use std::collections::{BTreeMap, BTreeSet};
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};

use crate::manifest::Manifest;
use crate::process;
use crate::{Error, Result};

#[derive(Clone, Debug)]
pub struct Git {
    pub root: PathBuf,
    pub git_dir: PathBuf,
}

#[derive(Clone, Debug)]
pub struct SparseSnapshot {
    pub enabled: bool,
    pub patterns: String,
}

#[derive(Clone, Debug, Default)]
pub struct Dirtiness {
    pub blocking: bool,
    pub ignored: bool,
}

impl Git {
    pub fn discover(start: &Path) -> Result<Self> {
        let root_output = process::run("git", ["rev-parse", "--show-toplevel"], start, None)?;
        let root = PathBuf::from(text(&root_output.stdout, "Git 仓库根路径")?);
        let git_dir_output = process::run("git", ["rev-parse", "--absolute-git-dir"], &root, None)?;
        let git_dir = PathBuf::from(text(&git_dir_output.stdout, "Git 工作树元数据路径")?);
        Ok(Self { root, git_dir })
    }

    pub fn state_directory(&self) -> PathBuf {
        self.git_dir.join("fls")
    }

    pub fn state_path(&self) -> PathBuf {
        self.state_directory().join("state.toml")
    }

    pub fn head(&self) -> Result<String> {
        let output = self.run(["rev-parse", "HEAD"], None)?;
        text(&output.stdout, "HEAD")
    }

    pub fn version(&self) -> Result<String> {
        let output = self.run(["--version"], None)?;
        text(&output.stdout, "Git 版本")
    }

    pub fn tracked_files(&self) -> Result<Vec<String>> {
        let output = self.run(["ls-files", "-z"], None)?;
        split_nul_paths(&output.stdout)
    }

    pub fn top_level_directories(&self) -> Result<BTreeSet<String>> {
        Ok(self
            .tracked_files()?
            .into_iter()
            .filter_map(|path| path.split_once('/').map(|(first, _)| first.to_owned()))
            .collect())
    }

    pub fn sparse_snapshot(&self) -> Result<SparseSnapshot> {
        let config = process::run_allow_failure(
            "git",
            ["config", "--bool", "core.sparseCheckout"],
            &self.root,
            None,
        )?;
        let enabled = config.success
            && String::from_utf8_lossy(&config.stdout)
                .trim()
                .eq_ignore_ascii_case("true");
        if !enabled {
            return Ok(SparseSnapshot {
                enabled: false,
                patterns: String::new(),
            });
        }
        let path_output = self.run(["rev-parse", "--git-path", "info/sparse-checkout"], None)?;
        let mut path = PathBuf::from(text(&path_output.stdout, "稀疏检出规则路径")?);
        if path.is_relative() {
            path = self.root.join(path);
        }
        let patterns = fs::read_to_string(&path).map_err(|source| Error::Read { path, source })?;
        Ok(SparseSnapshot { enabled, patterns })
    }

    pub fn inferred_materialized_projects(
        &self,
        manifest: &Manifest,
        snapshot: &SparseSnapshot,
    ) -> BTreeSet<String> {
        if !snapshot.enabled {
            return manifest.projects.keys().cloned().collect();
        }
        let full_paths: BTreeSet<&str> = snapshot
            .patterns
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                line.strip_prefix('/')
                    .and_then(|line| line.strip_suffix('/'))
                    .filter(|line| !line.contains('/'))
            })
            .collect();
        manifest
            .projects
            .iter()
            .filter(|(_, project)| full_paths.contains(project.path.as_str()))
            .map(|(id, _)| id.clone())
            .collect()
    }

    pub fn sparse_patterns(
        &self,
        manifest: &Manifest,
        project_paths: &BTreeMap<String, String>,
        materialized: &BTreeSet<String>,
    ) -> Result<String> {
        let mut patterns = BTreeSet::new();
        for tracked in self.tracked_files()? {
            let path = Path::new(&tracked);
            let root_file = path.components().count() == 1;
            let metadata = path
                .file_name()
                .and_then(OsStr::to_str)
                .map(|name| name == "BUCK" || name.ends_with(".bzl"))
                .unwrap_or(false);
            if root_file || metadata {
                patterns.insert(format!("/{tracked}"));
            }
        }
        for control_path in &manifest.control_paths {
            patterns.insert(format!("/{control_path}/"));
        }
        for id in materialized {
            let path = manifest
                .projects
                .get(id)
                .map(|project| project.path.as_str())
                .or_else(|| project_paths.get(id).map(String::as_str))
                .ok_or_else(|| {
                    Error::message(format!(
                        "状态中的项目 `{id}` 没有已知路径，无法安全生成稀疏检出规则"
                    ))
                })?;
            patterns.insert(format!("/{path}/"));
        }
        let mut result = patterns.into_iter().collect::<Vec<_>>().join("\n");
        result.push('\n');
        Ok(result)
    }

    pub fn apply_sparse_patterns(&self, patterns: &str) -> Result<()> {
        let env = [(OsString::from("FLS_RECONCILING"), OsString::from("1"))];
        process::run_with_env(
            "git",
            ["sparse-checkout", "set", "--no-cone", "--stdin"],
            &self.root,
            Some(patterns.as_bytes()),
            &env,
        )?;
        Ok(())
    }

    pub fn add_sparse_patterns(&self, patterns: &str) -> Result<()> {
        let env = [(OsString::from("FLS_RECONCILING"), OsString::from("1"))];
        process::run_with_env(
            "git",
            ["sparse-checkout", "reapply", "--no-cone"],
            &self.root,
            None,
            &env,
        )?;
        process::run_with_env(
            "git",
            ["sparse-checkout", "add", "--stdin"],
            &self.root,
            Some(patterns.as_bytes()),
            &env,
        )?;
        Ok(())
    }

    pub fn restore_sparse(&self, snapshot: &SparseSnapshot) -> Result<()> {
        if snapshot.enabled {
            self.apply_sparse_patterns(&snapshot.patterns)
        } else {
            let env = [(OsString::from("FLS_RECONCILING"), OsString::from("1"))];
            process::run_with_env(
                "git",
                ["sparse-checkout", "disable"],
                &self.root,
                None,
                &env,
            )?;
            Ok(())
        }
    }

    pub fn dirtiness(&self, path: &str) -> Result<Dirtiness> {
        let blocking = self.run_status(
            [
                "status",
                "--porcelain=v2",
                "-z",
                "--untracked-files=all",
                "--",
                path,
            ],
            None,
        )?;
        let ignored = self.run_status(
            [
                "status",
                "--porcelain=v2",
                "-z",
                "--ignored",
                "--untracked-files=all",
                "--",
                path,
            ],
            None,
        )?;
        Ok(Dirtiness {
            blocking: !blocking.stdout.is_empty(),
            ignored: ignored
                .stdout
                .split(|byte| *byte == 0)
                .any(|record| record.starts_with(b"! ")),
        })
    }

    pub fn hooks_path(&self) -> Result<(PathBuf, bool)> {
        let configured = process::run_allow_failure(
            "git",
            ["config", "--path", "core.hooksPath"],
            &self.root,
            None,
        )?;
        if configured.success {
            let raw = text(&configured.stdout, "core.hooksPath")?;
            let path = PathBuf::from(raw);
            return Ok((
                if path.is_absolute() {
                    path
                } else {
                    self.root.join(path)
                },
                true,
            ));
        }
        let output = self.run(["rev-parse", "--git-path", "hooks"], None)?;
        let path = PathBuf::from(text(&output.stdout, "Git hooks 路径")?);
        Ok((
            if path.is_absolute() {
                path
            } else {
                self.root.join(path)
            },
            false,
        ))
    }

    fn run<I, S>(&self, args: I, stdin: Option<&[u8]>) -> Result<process::Output>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        process::run("git", args, &self.root, stdin)
    }

    fn run_status<I, S>(&self, args: I, stdin: Option<&[u8]>) -> Result<process::Output>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        process::run("git", args, &self.root, stdin)
    }
}

fn text(bytes: &[u8], description: &str) -> Result<String> {
    String::from_utf8(bytes.to_vec())
        .map(|text| text.trim().to_owned())
        .map_err(|source| Error::message(format!("{description} 不是有效的 UTF-8: {source}")))
}

fn split_nul_paths(bytes: &[u8]) -> Result<Vec<String>> {
    bytes
        .split(|byte| *byte == 0)
        .filter(|part| !part.is_empty())
        .map(|part| {
            String::from_utf8(part.to_vec())
                .map_err(|source| Error::message(format!("Git 路径不是有效的 UTF-8: {source}")))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Project;

    #[test]
    fn inference_only_accepts_full_top_level_patterns() {
        let manifest = Manifest {
            schema_version: 1,
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
        let git = Git {
            root: PathBuf::new(),
            git_dir: PathBuf::new(),
        };
        let full = SparseSnapshot {
            enabled: true,
            patterns: "/TeraPanel/\n/TeraPanel/BUCK\n".to_owned(),
        };
        assert!(git
            .inferred_materialized_projects(&manifest, &full)
            .contains("TeraPanel"));
        let metadata_only = SparseSnapshot {
            enabled: true,
            patterns: "/TeraPanel/BUCK\n".to_owned(),
        };
        assert!(git
            .inferred_materialized_projects(&manifest, &metadata_only)
            .is_empty());
    }
}
