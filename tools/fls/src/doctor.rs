use std::collections::BTreeSet;

use serde::Serialize;

use crate::buck;
use crate::environment;
use crate::hooks;
use crate::manifest;
use crate::workspace::Workspace;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Pass,
    Warning,
    Fail,
}

#[derive(Clone, Debug, Serialize)]
pub struct Check {
    pub name: String,
    pub status: CheckStatus,
    pub message: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Report {
    pub healthy: bool,
    pub checks: Vec<Check>,
}

impl Report {
    fn push(&mut self, name: &str, status: CheckStatus, message: impl Into<String>) {
        if matches!(status, CheckStatus::Fail) {
            self.healthy = false;
        }
        self.checks.push(Check {
            name: name.to_owned(),
            status,
            message: message.into(),
        });
    }
}

pub fn run(workspace: &Workspace) -> Report {
    let mut report = Report {
        healthy: true,
        checks: Vec::new(),
    };

    match workspace.git.version() {
        Ok(version) => report.push("git", CheckStatus::Pass, version),
        Err(error) => report.push("git", CheckStatus::Fail, error.to_string()),
    }
    match environment::ensure_supported_host() {
        Ok(triple) => report.push("host-triple", CheckStatus::Pass, triple),
        Err(error) => report.push("host-triple", CheckStatus::Fail, error.to_string()),
    }
    match buck::version(&workspace.git.root).and_then(|actual| {
        environment::ensure_buck2_version(&workspace.git.root, &actual)
            .map(|expected| format!("{actual}（mise 固定 {expected}）"))
    }) {
        Ok(version) => report.push("buck2", CheckStatus::Pass, version),
        Err(error) => report.push("buck2", CheckStatus::Fail, error.to_string()),
    }

    if workspace.manifest.schema_version == manifest::CURRENT_SCHEMA {
        report.push(
            "manifest-schema",
            CheckStatus::Pass,
            format!("schema {}", manifest::CURRENT_SCHEMA),
        );
    } else {
        report.push(
            "manifest-schema",
            CheckStatus::Fail,
            workspace
                .manifest
                .warnings
                .first()
                .cloned()
                .unwrap_or_else(|| "清单 schema 不受支持".to_owned()),
        );
    }

    match workspace.validate_registered_paths() {
        Ok(()) => report.push(
            "project-paths",
            CheckStatus::Pass,
            format!(
                "{} 个项目路径均由 Git 跟踪",
                workspace.manifest.projects.len()
            ),
        ),
        Err(error) => report.push("project-paths", CheckStatus::Fail, error.to_string()),
    }

    let all_projects = workspace.all_projects();
    match buck::resolve(&workspace.git.root, &workspace.manifest, &all_projects) {
        Ok(resolution) if resolution.unknown_root_packages.is_empty() => report.push(
            "dependency-graph",
            CheckStatus::Pass,
            format!(
                "Buck2 已解析 {} 个项目、{} 个 targets",
                resolution.projects.len(),
                resolution.labels.len()
            ),
        ),
        Ok(resolution) => report.push(
            "dependency-graph",
            CheckStatus::Fail,
            format!(
                "依赖图引用未登记的顶层路径：{}",
                resolution
                    .unknown_root_packages
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ),
        Err(error) => report.push("dependency-graph", CheckStatus::Fail, error.to_string()),
    }

    let mut missing_markers = BTreeSet::new();
    for project in workspace.manifest.projects.values() {
        if buck::verify_workspace_target(&workspace.git.root, &project.path).is_err() {
            missing_markers.insert(project.id.clone());
        }
    }
    if missing_markers.is_empty() {
        report.push(
            "workspace-targets",
            CheckStatus::Pass,
            "所有项目都提供 :workspace 边界 target",
        );
    } else {
        report.push(
            "workspace-targets",
            CheckStatus::Fail,
            format!(
                "缺少可查询的 :workspace target：{}",
                missing_markers.into_iter().collect::<Vec<_>>().join(", ")
            ),
        );
    }

    if workspace.state_exists() {
        if workspace.state_mutation_supported() {
            report.push(
                "state-schema",
                CheckStatus::Pass,
                format!("schema {}", workspace.state.schema_version),
            );
        } else {
            report.push(
                "state-schema",
                CheckStatus::Fail,
                format!(
                    "本地状态 schema {} 不受当前 fls 支持",
                    workspace.state.schema_version
                ),
            );
        }
        match workspace.expected_sparse_patterns() {
            Ok(expected)
                if workspace.sparse_snapshot().enabled
                    && normalized_patterns(&expected)
                        == normalized_patterns(&workspace.sparse_snapshot().patterns) =>
            {
                report.push(
                    "sparse-checkout",
                    CheckStatus::Pass,
                    "稀疏检出规则与 fls 状态一致",
                )
            }
            Ok(_) => report.push(
                "sparse-checkout",
                CheckStatus::Fail,
                "稀疏检出规则与 fls 状态不一致；请运行 `fls reconcile`",
            ),
            Err(error) => report.push("sparse-checkout", CheckStatus::Fail, error.to_string()),
        }
    } else {
        report.push(
            "state-schema",
            CheckStatus::Warning,
            "当前工作树尚未创建本地 fls 状态",
        );
        report.push(
            "sparse-checkout",
            CheckStatus::Warning,
            "当前工作树尚未写入 fls 状态；第一次修改命令会采用现有检出内容",
        );
    }

    match hooks::inspect(&workspace.git) {
        Ok(warnings) if warnings.is_empty() => report.push(
            "git-hooks",
            CheckStatus::Pass,
            "post-checkout、post-merge、post-rewrite hooks 已接入",
        ),
        Ok(warnings) => report.push("git-hooks", CheckStatus::Warning, warnings.join(" ")),
        Err(error) => report.push("git-hooks", CheckStatus::Warning, error.to_string()),
    }

    for warning in &workspace.warnings {
        report.push("compatibility", CheckStatus::Warning, warning.clone());
    }
    report
}

fn normalized_patterns(patterns: &str) -> BTreeSet<&str> {
    patterns
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
}
