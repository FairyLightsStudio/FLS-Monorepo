use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use atomic_write_file::AtomicWriteFile;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

pub const CURRENT_SCHEMA: u32 = 1;
pub const PREVIOUS_SCHEMA: u32 = 0;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct State {
    #[serde(default)]
    pub schema_version: u32,
    #[serde(default)]
    pub explicit_projects: BTreeSet<String>,
    #[serde(default)]
    pub resolved_projects: BTreeSet<String>,
    #[serde(default)]
    pub retained_projects: BTreeMap<String, String>,
    #[serde(default)]
    pub known_project_paths: BTreeMap<String, String>,
    #[serde(default)]
    pub resolved_at_head: String,
}

#[derive(Debug)]
pub struct LoadedState {
    pub state: Option<State>,
    pub warnings: Vec<String>,
    pub mutation_supported: bool,
}

impl State {
    pub fn current() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA,
            ..Self::default()
        }
    }

    pub fn materialized_projects(&self) -> BTreeSet<String> {
        self.resolved_projects
            .iter()
            .chain(self.retained_projects.keys())
            .cloned()
            .collect()
    }

    pub fn fingerprint(&self) -> Result<String> {
        toml::to_string(self)
            .map_err(|source| Error::message(format!("无法序列化 fls 状态: {source}")))
    }
}

pub fn load(path: &Path) -> Result<LoadedState> {
    if !path.exists() {
        return Ok(LoadedState {
            state: None,
            warnings: Vec::new(),
            mutation_supported: true,
        });
    }
    let contents = fs::read_to_string(path).map_err(|source| Error::Read {
        path: path.to_owned(),
        source,
    })?;
    let mut state: State = toml::from_str(&contents).map_err(|source| Error::ParseToml {
        path: path.to_owned(),
        source,
    })?;
    let mut warnings = Vec::new();
    let mutation_supported = match state.schema_version {
        CURRENT_SCHEMA => true,
        PREVIOUS_SCHEMA => {
            warnings.push(format!(
                "本地 fls 状态使用上一版 schema {}，将在下一次成功修改时迁移到 schema {}。",
                PREVIOUS_SCHEMA, CURRENT_SCHEMA
            ));
            state.schema_version = CURRENT_SCHEMA;
            true
        }
        version if version > CURRENT_SCHEMA => {
            warnings.push(format!(
                "本地 fls 状态 schema {version} 比当前支持的 schema {CURRENT_SCHEMA} 新；已禁止修改工作树。"
            ));
            false
        }
        version => {
            warnings.push(format!(
                "本地 fls 状态 schema {version} 太旧；当前仅支持 schema {PREVIOUS_SCHEMA} 和 {CURRENT_SCHEMA}。"
            ));
            false
        }
    };
    Ok(LoadedState {
        state: Some(state),
        warnings,
        mutation_supported,
    })
}

pub fn write(path: &Path, state: &State) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| Error::message("fls 状态路径没有父目录"))?;
    fs::create_dir_all(parent).map_err(|source| Error::Write {
        path: parent.to_owned(),
        source,
    })?;
    let contents = toml::to_string_pretty(state)
        .map_err(|source| Error::message(format!("无法序列化 fls 状态: {source}")))?;
    let mut file = AtomicWriteFile::open(path).map_err(|source| Error::Write {
        path: path.to_owned(),
        source,
    })?;
    file.write_all(contents.as_bytes())
        .map_err(|source| Error::Write {
            path: path.to_owned(),
            source,
        })?;
    file.commit().map_err(|source| Error::Write {
        path: path.to_owned(),
        source,
    })
}

pub struct StateLock {
    _file: File,
    pub path: PathBuf,
}

impl StateLock {
    pub fn acquire(directory: &Path) -> Result<Self> {
        fs::create_dir_all(directory).map_err(|source| Error::Write {
            path: directory.to_owned(),
            source,
        })?;
        let path = directory.join("lock");
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&path)
            .map_err(|source| Error::Write {
                path: path.clone(),
                source,
            })?;
        match file.try_lock() {
            Ok(()) => Ok(Self { _file: file, path }),
            Err(std::fs::TryLockError::WouldBlock) => Err(Error::Locked),
            Err(std::fs::TryLockError::Error(source)) => Err(Error::Write { path, source }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn materialized_projects_include_retained_projects() {
        let mut state = State::current();
        state.resolved_projects.insert("TeraPanel".to_owned());
        state
            .retained_projects
            .insert("UserCenter".to_owned(), "存在未提交修改".to_owned());
        assert_eq!(
            state.materialized_projects(),
            BTreeSet::from(["TeraPanel".to_owned(), "UserCenter".to_owned()])
        );
    }
}
