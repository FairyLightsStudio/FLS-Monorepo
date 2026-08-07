use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Serialize;

use crate::buck;
use crate::environment;
use crate::git::{Git, SparseSnapshot};
use crate::hooks;
use crate::manifest::Manifest;
use crate::state::{self, State, StateLock};
use crate::{Error, Result};

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeKind {
    Add,
    Remove,
    Set,
    Reconcile,
}

#[derive(Clone, Debug)]
pub struct ChangeRequest {
    pub kind: ChangeKind,
    pub projects: BTreeSet<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Plan {
    pub command: ChangeKind,
    pub explicit_projects: BTreeSet<String>,
    pub resolved_projects: BTreeSet<String>,
    pub retained_projects: BTreeMap<String, String>,
    pub added_projects: BTreeSet<String>,
    pub removed_projects: BTreeSet<String>,
    pub warnings: Vec<String>,
    #[serde(skip)]
    next_state: State,
    #[serde(skip)]
    sparse_patterns: String,
    #[serde(skip)]
    expected_head: String,
    #[serde(skip)]
    expected_state: String,
    #[serde(skip)]
    old_sparse: SparseSnapshot,
}

impl Plan {
    pub fn changes_worktree(&self) -> bool {
        !self.added_projects.is_empty() || !self.removed_projects.is_empty()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ApplyOutcome {
    pub plan: Plan,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProjectStatus {
    pub id: String,
    pub path: Option<String>,
    pub description: String,
    pub status: String,
    pub reason: Option<String>,
}

pub struct Workspace {
    pub git: Git,
    pub manifest: Manifest,
    pub state: State,
    pub warnings: Vec<String>,
    state_mutation_supported: bool,
    state_exists: bool,
    sparse: SparseSnapshot,
}

impl Workspace {
    pub fn open(start: &Path) -> Result<Self> {
        let git = Git::discover(start)?;
        let manifest = Manifest::load(&git.root)?;
        let sparse = git.sparse_snapshot()?;
        let loaded = state::load(&git.state_path())?;
        let mut warnings = manifest.warnings.clone();
        warnings.extend(loaded.warnings.clone());
        let state_exists = loaded.state.is_some();
        let state = match loaded.state {
            Some(mut state) => {
                for (id, path) in manifest.known_paths() {
                    state.known_project_paths.entry(id).or_insert(path);
                }
                state
            }
            None => Self::adopt_state(&git, &manifest, &sparse)?,
        };
        Ok(Self {
            git,
            manifest,
            state,
            warnings,
            state_mutation_supported: loaded.mutation_supported,
            state_exists,
            sparse,
        })
    }

    fn adopt_state(git: &Git, manifest: &Manifest, sparse: &SparseSnapshot) -> Result<State> {
        let selected = git.inferred_materialized_projects(manifest, sparse);
        let mut state = State::current();
        state.explicit_projects = selected.clone();
        state.resolved_projects = selected;
        state.known_project_paths = manifest.known_paths();
        state.resolved_at_head = git.head()?;
        Ok(state)
    }

    pub fn ensure_metadata(&mut self) -> Result<()> {
        if !self.sparse.enabled {
            return Ok(());
        }
        let tracked = self.git.tracked_files()?;
        let missing = tracked.iter().any(|tracked| {
            let path = Path::new(tracked);
            let metadata = path
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name == "BUCK" || name.ends_with(".bzl"))
                .unwrap_or(false);
            metadata && !self.git.root.join(path).exists()
        });
        if !missing {
            return Ok(());
        }
        let patterns = tracked
            .into_iter()
            .filter(|tracked| {
                Path::new(tracked)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name == "BUCK" || name.ends_with(".bzl"))
                    .unwrap_or(false)
            })
            .map(|tracked| format!("/{tracked}"))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        self.git.add_sparse_patterns(&patterns)?;
        self.sparse = self.git.sparse_snapshot()?;
        self.warnings.push(
            "为了让 Buck2 解析完整项目图，fls 已补齐工作树中的 BUCK 与 .bzl 元数据。".to_owned(),
        );
        Ok(())
    }

    pub fn resolve_inputs(&self, inputs: &[String]) -> Result<BTreeSet<String>> {
        let mut result = BTreeSet::new();
        for input in inputs {
            let id = self.manifest.resolve_id(input).ok_or_else(|| {
                Error::message(format!("未知项目 `{input}`；可使用 `fls list` 查看项目 ID"))
            })?;
            result.insert(id.to_owned());
        }
        Ok(result)
    }

    pub fn resolve_removal_inputs(&self, inputs: &[String]) -> Result<BTreeSet<String>> {
        let mut result = BTreeSet::new();
        for input in inputs {
            if let Some(id) = self.manifest.resolve_id(input) {
                result.insert(id.to_owned());
                continue;
            }
            let id = self
                .state
                .explicit_projects
                .iter()
                .find(|id| id.eq_ignore_ascii_case(input))
                .ok_or_else(|| {
                    Error::message(format!("未知项目 `{input}`；可使用 `fls list` 查看项目 ID"))
                })?;
            result.insert(id.clone());
        }
        Ok(result)
    }

    pub fn all_projects(&self) -> BTreeSet<String> {
        self.manifest.projects.keys().cloned().collect()
    }

    pub fn statuses(&self) -> Vec<ProjectStatus> {
        let materialized = self.state.materialized_projects();
        let mut statuses = self
            .manifest
            .projects
            .iter()
            .map(|(id, project)| {
                let (status, reason) = if self.state.explicit_projects.contains(id) {
                    ("explicit", None)
                } else if let Some(reason) = self.state.retained_projects.get(id) {
                    ("retained", Some(reason.clone()))
                } else if self.state.resolved_projects.contains(id) {
                    ("dependency", None)
                } else {
                    ("available", None)
                };
                ProjectStatus {
                    id: id.clone(),
                    path: Some(project.path.clone()),
                    description: project.description.clone(),
                    status: status.to_owned(),
                    reason,
                }
            })
            .collect::<Vec<_>>();
        for id in self
            .state
            .explicit_projects
            .iter()
            .filter(|id| !self.manifest.projects.contains_key(*id))
        {
            statuses.push(ProjectStatus {
                id: id.clone(),
                path: self.state.known_project_paths.get(id).cloned(),
                description: String::new(),
                status: "unavailable".to_owned(),
                reason: Some("项目仍被显式选择，但当前清单已不再声明它".to_owned()),
            });
        }
        statuses.sort_by(|left, right| {
            left.id
                .to_ascii_lowercase()
                .cmp(&right.id.to_ascii_lowercase())
        });
        debug_assert!(materialized
            .iter()
            .all(|id| statuses.iter().any(|status| &status.id == id)));
        statuses
    }

    pub fn plan(&self, request: ChangeRequest) -> Result<Plan> {
        self.manifest.require_mutation_support()?;
        if !self.state_mutation_supported {
            return Err(Error::message(
                "当前本地状态 schema 不受此版本 fls 支持；修改工作树已被拒绝",
            ));
        }
        let _host = environment::ensure_supported_host()?;
        let _git_version = self.git.version()?;
        let buck_version = buck::version(&self.git.root)?;
        environment::ensure_buck2_version(&self.git.root, &buck_version)?;
        self.validate_registered_paths()?;
        let mut warnings = self.warnings.clone();
        let mut explicit = self.state.explicit_projects.clone();
        match request.kind {
            ChangeKind::Add => explicit.extend(request.projects),
            ChangeKind::Remove => {
                for id in request.projects {
                    if !explicit.remove(&id) {
                        return Err(Error::message(format!(
                            "项目 `{id}` 不是显式选择；依赖项目应通过移除其上游项目来释放"
                        )));
                    }
                }
            }
            ChangeKind::Set => {
                let unavailable = explicit
                    .iter()
                    .filter(|id| !self.manifest.projects.contains_key(*id))
                    .cloned()
                    .collect::<BTreeSet<_>>();
                explicit = request.projects;
                explicit.extend(unavailable);
            }
            ChangeKind::Reconcile => {}
        }

        let available_explicit = explicit
            .iter()
            .filter(|id| self.manifest.projects.contains_key(*id))
            .cloned()
            .collect::<BTreeSet<_>>();
        let resolution = buck::resolve(&self.git.root, &self.manifest, &available_explicit)?;
        if !resolution.unknown_root_packages.is_empty() {
            return Err(Error::message(format!(
                "Buck2 依赖图引用了未登记的顶层项目：{}。请先在 fls-projects.toml 中登记完整项目图。",
                resolution
                    .unknown_root_packages
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            )));
        }
        for id in &resolution.projects {
            let project = self
                .manifest
                .projects
                .get(id)
                .ok_or_else(|| Error::message(format!("Buck2 解析到了清单外项目 `{id}`")))?;
            buck::verify_workspace_target(&self.git.root, &project.path)?;
        }

        let mut retained = BTreeMap::new();
        for unavailable in explicit
            .iter()
            .filter(|id| !self.manifest.projects.contains_key(*id))
        {
            retained.insert(
                unavailable.clone(),
                "项目不在当前清单中；在显式 remove 前保留本地文件".to_owned(),
            );
        }
        let current_materialized = self.state.materialized_projects();
        let candidates = current_materialized
            .difference(&resolution.projects)
            .filter(|id| !retained.contains_key(*id))
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut removable = BTreeSet::new();
        for id in candidates {
            let Some(path) = self.project_path(&id) else {
                retained.insert(id, "缺少已知项目路径，无法安全移除".to_owned());
                continue;
            };
            let dirtiness = self.git.dirtiness(path)?;
            if dirtiness.blocking {
                match request.kind {
                    ChangeKind::Reconcile => {
                        retained.insert(id, "存在已跟踪、暂存或未跟踪的本地修改".to_owned());
                    }
                    _ => {
                        return Err(Error::message(format!(
                            "项目 `{id}`（{path}）存在已跟踪、暂存或未跟踪的本地修改；整个操作已取消"
                        )));
                    }
                }
            } else {
                removable.insert(id.clone());
                if dirtiness.ignored {
                    warnings.push(format!(
                        "项目 `{id}` 中存在被 Git 忽略的输出；稀疏检出不会删除这些文件，目录可能保留。"
                    ));
                }
            }
        }

        let mut target_materialized = resolution.projects.clone();
        target_materialized.extend(retained.keys().cloned());
        let added_projects = target_materialized
            .difference(&current_materialized)
            .cloned()
            .collect();
        let removed_projects = current_materialized
            .difference(&target_materialized)
            .filter(|id| removable.contains(*id))
            .cloned()
            .collect();
        let head = self.git.head()?;
        let mut next_state = State::current();
        next_state.explicit_projects = explicit.clone();
        next_state.resolved_projects = resolution.projects.clone();
        next_state.retained_projects = retained.clone();
        next_state.known_project_paths = self.state.known_project_paths.clone();
        next_state
            .known_project_paths
            .extend(self.manifest.known_paths());
        next_state.resolved_at_head = head.clone();
        let sparse_patterns = self.git.sparse_patterns(
            &self.manifest,
            &next_state.known_project_paths,
            &target_materialized,
        )?;

        Ok(Plan {
            command: request.kind,
            explicit_projects: explicit,
            resolved_projects: resolution.projects,
            retained_projects: retained,
            added_projects,
            removed_projects,
            warnings,
            next_state,
            sparse_patterns,
            expected_head: head,
            expected_state: self.state.fingerprint()?,
            old_sparse: self.sparse.clone(),
        })
    }

    pub fn apply(&self, plan: Plan) -> Result<ApplyOutcome> {
        let _lock = StateLock::acquire(&self.git.state_directory())?;
        let current_head = self.git.head()?;
        if current_head != plan.expected_head {
            return Err(Error::message(
                "计算计划后 HEAD 已改变；为避免应用过期依赖图，本次操作已取消，请重试",
            ));
        }
        let current_loaded = state::load(&self.git.state_path())?;
        let current_state = match current_loaded.state {
            Some(state) => state,
            None => self.state.clone(),
        };
        if current_state.fingerprint()? != plan.expected_state {
            return Err(Error::message(
                "计算计划后本地 fls 状态已改变；本次操作已取消，请重试",
            ));
        }

        self.git.apply_sparse_patterns(&plan.sparse_patterns)?;
        if let Err(error) = state::write(&self.git.state_path(), &plan.next_state) {
            let rollback = self.git.restore_sparse(&plan.old_sparse);
            return match rollback {
                Ok(()) => Err(error),
                Err(rollback_error) => Err(Error::message(format!(
                    "写入 fls 状态失败，且无法恢复原稀疏检出规则。原错误：{error}；恢复错误：{rollback_error}"
                ))),
            };
        }

        let mut warnings = plan.warnings.clone();
        match hooks::install(&self.git) {
            Ok(hook_warnings) => warnings.extend(hook_warnings),
            Err(error) => warnings.push(format!(
                "工作树已经更新，但安装 Git reconcile hooks 失败：{error}"
            )),
        }
        Ok(ApplyOutcome { plan, warnings })
    }

    fn project_path(&self, id: &str) -> Option<&str> {
        self.manifest
            .projects
            .get(id)
            .map(|project| project.path.as_str())
            .or_else(|| self.state.known_project_paths.get(id).map(String::as_str))
    }

    pub fn state_exists(&self) -> bool {
        self.state_exists
    }

    pub fn state_mutation_supported(&self) -> bool {
        self.state_mutation_supported
    }

    pub fn sparse_snapshot(&self) -> &SparseSnapshot {
        &self.sparse
    }

    pub fn expected_sparse_patterns(&self) -> Result<String> {
        self.git.sparse_patterns(
            &self.manifest,
            &self.state.known_project_paths,
            &self.state.materialized_projects(),
        )
    }

    pub fn validate_registered_paths(&self) -> Result<()> {
        let top_level = self.git.top_level_directories()?;
        let missing = self
            .manifest
            .projects
            .values()
            .filter(|project| !top_level.contains(&project.path))
            .map(|project| project.path.clone())
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            return Err(Error::message(format!(
                "以下登记项目没有任何被 Git 跟踪的文件：{}",
                missing.join(", ")
            )));
        }
        Ok(())
    }
}
