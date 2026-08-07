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
    let output = run(root, ["--version"])?;
    String::from_utf8(output.stdout)
        .map(|value| value.trim().to_owned())
        .map_err(|source| Error::message(format!("Buck2 版本输出不是有效的 UTF-8: {source}")))
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
        .map(|project| format!("'//{}/...'", project.path))
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
        ["uquery", query.as_str(), "--json", "--console=simple"],
    )?;
    let labels: Vec<String> = serde_json::from_slice(&output.stdout).map_err(|source| {
        Error::message(format!("无法解析 Buck2 uquery 的 JSON 输出: {source}"))
    })?;
    classify_labels(manifest, labels)
}

pub fn verify_workspace_target(root: &Path, project_path: &str) -> Result<()> {
    let target = format!("//{project_path}:workspace");
    run(
        root,
        ["uquery", target.as_str(), "--json", "--console=simple"],
    )?;
    Ok(())
}

fn run<I, S>(root: &Path, args: I) -> Result<process::Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let version = environment::pinned_buck2_version(root)?;
    let mut command = vec![
        std::ffi::OsString::from("--locked"),
        std::ffi::OsString::from("exec"),
        std::ffi::OsString::from(format!("buck2@{version}")),
        std::ffi::OsString::from("--"),
        std::ffi::OsString::from("buck2"),
    ];
    command.extend(
        args.into_iter()
            .map(|argument| argument.as_ref().to_owned()),
    );
    process::run("mise", command, root, None)
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
        let Some(package) = label.strip_prefix("root//") else {
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
                "root//UserCenter:workspace".to_owned(),
                "root//third-party/rust:serde".to_owned(),
                "toolchains//:rust".to_owned(),
                "root//Unregistered:target".to_owned(),
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
