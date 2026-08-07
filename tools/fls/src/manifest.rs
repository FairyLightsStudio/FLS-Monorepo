use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

pub const MANIFEST_FILE: &str = "fls-projects.toml";
pub const CURRENT_SCHEMA: u32 = 1;
pub const PREVIOUS_SCHEMA: u32 = 0;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ManifestFile {
    #[serde(default)]
    schema_version: u32,
    workspace: WorkspaceFile,
    projects: BTreeMap<String, ProjectFile>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct WorkspaceFile {
    #[serde(default)]
    control_paths: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct ProjectFile {
    path: String,
    #[serde(default)]
    description: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Project {
    pub id: String,
    pub path: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct Manifest {
    pub schema_version: u32,
    pub projects: BTreeMap<String, Project>,
    pub control_paths: Vec<String>,
    pub warnings: Vec<String>,
    pub mutation_supported: bool,
}

impl Manifest {
    pub fn load(root: &Path) -> Result<Self> {
        let path = root.join(MANIFEST_FILE);
        let contents = fs::read_to_string(&path).map_err(|source| Error::Read {
            path: path.clone(),
            source,
        })?;
        let raw: ManifestFile = toml::from_str(&contents).map_err(|source| Error::ParseToml {
            path: path.clone(),
            source,
        })?;
        let mut warnings = Vec::new();
        let mutation_supported = match raw.schema_version {
            CURRENT_SCHEMA => true,
            PREVIOUS_SCHEMA => {
                warnings.push(format!(
                    "{MANIFEST_FILE} 使用上一版 schema {}；请更新到 schema {}。读取和诊断仍可使用，但 CI 应视为失败。",
                    PREVIOUS_SCHEMA, CURRENT_SCHEMA
                ));
                true
            }
            version if version > CURRENT_SCHEMA => {
                warnings.push(format!(
                    "{MANIFEST_FILE} schema {version} 比当前 fls 支持的 schema {CURRENT_SCHEMA} 新；已禁止修改工作树。"
                ));
                false
            }
            version => {
                warnings.push(format!(
                    "{MANIFEST_FILE} schema {version} 太旧；当前仅支持 schema {PREVIOUS_SCHEMA} 和 {CURRENT_SCHEMA}，已禁止修改工作树。"
                ));
                false
            }
        };

        let mut projects = BTreeMap::new();
        let mut folded_ids = BTreeSet::new();
        let mut paths = BTreeSet::new();
        for (id, project) in raw.projects {
            validate_identifier(&id)?;
            validate_top_level_path(&project.path, "项目路径")?;
            let folded = id.to_ascii_lowercase();
            if !folded_ids.insert(folded) {
                return Err(Error::message(format!(
                    "{MANIFEST_FILE} 中的项目 ID `{id}` 与另一个 ID 仅大小写不同"
                )));
            }
            if !paths.insert(project.path.to_ascii_lowercase()) {
                return Err(Error::message(format!(
                    "{MANIFEST_FILE} 中的项目路径 `{}` 重复",
                    project.path
                )));
            }
            projects.insert(
                id.clone(),
                Project {
                    id,
                    path: project.path,
                    description: project.description,
                },
            );
        }
        if projects.is_empty() {
            return Err(Error::message(format!("{MANIFEST_FILE} 没有声明任何项目")));
        }

        let project_paths: BTreeSet<String> = projects
            .values()
            .map(|project| project.path.to_ascii_lowercase())
            .collect();
        let mut control_paths = Vec::new();
        let mut seen_control_paths = BTreeSet::new();
        for control_path in raw.workspace.control_paths {
            validate_top_level_path(&control_path, "控制路径")?;
            let folded = control_path.to_ascii_lowercase();
            if project_paths.contains(&folded) {
                return Err(Error::message(format!(
                    "控制路径 `{control_path}` 与项目路径重叠"
                )));
            }
            if seen_control_paths.insert(folded) {
                control_paths.push(control_path);
            }
        }
        control_paths.sort();

        Ok(Self {
            schema_version: raw.schema_version,
            projects,
            control_paths,
            warnings,
            mutation_supported,
        })
    }

    pub fn resolve_id(&self, input: &str) -> Option<&str> {
        self.projects
            .keys()
            .find(|id| id.eq_ignore_ascii_case(input))
            .map(String::as_str)
    }

    pub fn project_by_path(&self, path: &str) -> Option<&Project> {
        self.projects
            .values()
            .find(|project| project.path.eq_ignore_ascii_case(path))
    }

    pub fn require_mutation_support(&self) -> Result<()> {
        if self.mutation_supported {
            Ok(())
        } else {
            Err(Error::message(format!(
                "当前 {MANIFEST_FILE} schema 不受此版本 fls 支持；请先更新 fls 或迁移配置"
            )))
        }
    }

    pub fn known_paths(&self) -> BTreeMap<String, String> {
        self.projects
            .iter()
            .map(|(id, project)| (id.clone(), project.path.clone()))
            .collect()
    }
}

fn validate_identifier(id: &str) -> Result<()> {
    if id.is_empty()
        || !id.is_ascii()
        || !id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
    {
        return Err(Error::message(format!(
            "项目 ID `{id}` 非法；只能使用 ASCII 字母、数字、连字符和下划线"
        )));
    }
    Ok(())
}

fn validate_top_level_path(path: &str, kind: &str) -> Result<()> {
    let candidate = PathBuf::from(path);
    let mut components = candidate.components();
    let first = components.next();
    if path.is_empty()
        || path.contains('\\')
        || !matches!(first, Some(Component::Normal(_)))
        || components.next().is_some()
        || path == ".git"
    {
        return Err(Error::message(format!(
            "{kind} `{path}` 非法；必须是仓库根目录下的单层相对路径"
        )));
    }
    if !path.is_ascii()
        || !path.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-' || byte == b'.'
        })
    {
        return Err(Error::message(format!(
            "{kind} `{path}` 非法；只能使用 ASCII 字母、数字、点、连字符和下划线"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_lookup_is_ascii_case_insensitive() {
        let manifest = Manifest {
            schema_version: CURRENT_SCHEMA,
            projects: BTreeMap::from([(
                "TeraPanel".to_owned(),
                Project {
                    id: "TeraPanel".to_owned(),
                    path: "TeraPanel".to_owned(),
                    description: String::new(),
                },
            )]),
            control_paths: Vec::new(),
            warnings: Vec::new(),
            mutation_supported: true,
        };
        assert_eq!(manifest.resolve_id("terapanel"), Some("TeraPanel"));
    }

    #[test]
    fn top_level_paths_reject_nested_paths() {
        assert!(validate_top_level_path("nested/path", "项目路径").is_err());
        assert!(validate_top_level_path("Project", "项目路径").is_ok());
    }
}
