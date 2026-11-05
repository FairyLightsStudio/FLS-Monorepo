use std::collections::BTreeSet;
use std::path::Path;

use crate::environment;
use crate::manifest::Manifest;
use crate::process;
use crate::{Error, Result};

#[derive(Clone, Debug)]
pub struct DependencyResolution {
    pub projects: BTreeSet<String>,
    pub unknown_root_packages: BTreeSet<String>,
    pub labels: Vec<String>,
}

pub fn version(root: &Path) -> Result<String> {
    // Prefer the pinned .bazelversion so fls reports the version it will
    // actually resolve with; fall back to `bazel --version`.
    if let Ok(pinned) = environment::pinned_bazel_version(root) {
        if !pinned.trim().is_empty() {
            return Ok(pinned);
        }
    }
    let output = run(root, ["--version"])?;
    let text = String::from_utf8(output.stdout)
        .map_err(|source| Error::message(format!("Bazel 版本输出不是有效的 UTF-8: {source}")))?;
    // `bazel --version` prints e.g. "bazel 9.3.1".
    text.split_whitespace()
        .last()
        .map(str::to_owned)
        .filter(|version| !version.is_empty())
        .ok_or_else(|| Error::message(format!("无法从 Bazel 版本输出解析版本号：{text:?}")))
}

pub fn resolve(
    root: &Path,
    manifest: &Manifest,
    explicit: &BTreeSet<String>,
) -> Result<DependencyResolution> {
    if explicit.is_empty() {
        return Ok(DependencyResolution {
            projects: BTreeSet::new(),
            unknown_root_packages: BTreeSet::new(),
            labels: Vec::new(),
        });
    }
    let expressions = explicit
        .iter()
        .filter_map(|id| manifest.projects.get(id))
        .map(|project| format!("//{}/...", project.path))
        .collect::<Vec<_>>();
    if expressions.is_empty() {
        return Ok(DependencyResolution {
            projects: BTreeSet::new(),
            unknown_root_packages: BTreeSet::new(),
            labels: Vec::new(),
        });
    }
    let query = format!("deps(set({}))", expressions.join(" "));
    let output = run(
        root,
        [
            "query",
            query.as_str(),
            "--output=label",
            "--noshow_progress",
            "--color=no",
        ],
    )?;
    let text = String::from_utf8(output.stdout)
        .map_err(|source| Error::message(format!("无法解析 Bazel query 的输出: {source}")))?;
    let labels: Vec<String> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect();
    classify_labels(manifest, labels)
}

pub fn verify_workspace_target(root: &Path, project_path: &str) -> Result<()> {
    let target = format!("//{project_path}:workspace");
    run(root, ["query", target.as_str(), "--noshow_progress", "--color=no"])?;
    Ok(())
}

fn run<I, S>(root: &Path, args: I) -> Result<process::Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut command = Vec::new();
    command.extend(
        args.into_iter()
            .map(|argument| argument.as_ref().to_owned()),
    );
    process::run("bazel", command, root, None)
}

fn classify_labels(manifest: &Manifest, labels: Vec<String>) -> Result<DependencyResolution> {
    let control_paths: BTreeSet<String> = manifest
        .control_paths
        .iter()
        .map(|path| path.to_ascii_lowercase())
        .collect();
    let mut projects = BTreeSet::new();
    let mut unknown_root_packages = BTreeSet::new();
    for label in &labels {
        // Bazel main-repo labels look like "//UserCenter:workspace" (no
        // "root//" prefix, unlike Buck2). External repos ("@@…//…",
        // "@…//…") never map to local projects.
        if label.starts_with('@') {
            continue;
        }
        let Some(package) = label.strip_prefix("//") else {
            continue;
        };
        let package = package.split(':').next().unwrap_or(package);
        let top_level = package.split('/').next().unwrap_or(package);
        if top_level.is_empty() {
            continue;
        }
        if let Some(project) = manifest.project_by_path(top_level) {
            projects.insert(project.id.clone());
        } else if !control_paths.contains(&top_level.to_ascii_lowercase()) {
            unknown_root_packages.insert(top_level.to_owned());
        }
    }
    Ok(DependencyResolution {
        projects,
        unknown_root_packages,
        labels,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::manifest::Project;

    #[test]
    fn classifies_only_root_repository_projects() {
        let manifest = Manifest {
            schema_version: 1,
            projects: BTreeMap::from([(
                "UserCenter".to_owned(),
                Project {
                    id: "UserCenter".to_owned(),
                    path: "UserCenter".to_owned(),
                    description: String::new(),
                },
            )]),
            control_paths: vec!["third-party".to_owned()],
            warnings: Vec::new(),
            mutation_supported: true,
        };
        let resolution = classify_labels(
            &manifest,
            vec![
                "//UserCenter:workspace".to_owned(),
                "//third-party/rust:serde".to_owned(),
                "toolchains//:rust".to_owned(),
                "@@rules_rust+//rust:defs.bzl".to_owned(),
                "//Unregistered:target".to_owned(),
            ],
        )
        .unwrap();
        assert_eq!(
            resolution.projects,
            BTreeSet::from(["UserCenter".to_owned()])
        );
        assert_eq!(
            resolution.unknown_root_packages,
            BTreeSet::from(["Unregistered".to_owned()])
        );
    }
}
