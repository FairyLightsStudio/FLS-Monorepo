use std::fs;
use std::path::Path;

use crate::{Error, Result};

pub const SUPPORTED_HOST_TRIPLES: &[&str] = &[
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
    "aarch64-pc-windows-msvc",
    "x86_64-pc-windows-msvc",
    "aarch64-unknown-linux-gnu",
    "aarch64-unknown-linux-musl",
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl",
    "riscv64gc-unknown-linux-gnu",
];

pub fn current_host_triple() -> String {
    let architecture = match std::env::consts::ARCH {
        "riscv64" => "riscv64gc",
        other => other,
    };
    let suffix = match std::env::consts::OS {
        "macos" => "apple-darwin",
        "windows" => "pc-windows-msvc",
        "linux" if cfg!(target_env = "musl") => "unknown-linux-musl",
        "linux" => "unknown-linux-gnu",
        other => return format!("{architecture}-unknown-{other}"),
    };
    format!("{architecture}-{suffix}")
}

pub fn ensure_supported_host() -> Result<String> {
    let triple = current_host_triple();
    if SUPPORTED_HOST_TRIPLES.contains(&triple.as_str()) {
        Ok(triple)
    } else {
        Err(Error::message(format!(
            "当前 host triple `{triple}` 不受 fls 支持；支持范围为：{}",
            SUPPORTED_HOST_TRIPLES.join(", ")
        )))
    }
}

pub fn pinned_buck2_version(root: &Path) -> Result<String> {
    let path = root.join("mise.toml");
    let contents = fs::read_to_string(&path).map_err(|source| Error::Read {
        path: path.clone(),
        source,
    })?;
    let document: toml::Value =
        toml::from_str(&contents).map_err(|source| Error::ParseToml { path, source })?;
    let buck2 = document
        .get("tools")
        .and_then(|tools| tools.get("buck2"))
        .ok_or_else(|| Error::message("mise.toml 没有固定 Buck2 版本"))?;
    if let Some(version) = buck2.as_str() {
        return Ok(version.to_owned());
    }
    buck2
        .get("version")
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| Error::message("mise.toml 中的 Buck2 版本不是精确字符串"))
}

pub fn ensure_buck2_version(root: &Path, actual: &str) -> Result<String> {
    let expected = pinned_buck2_version(root)?;
    if expected == "latest" || expected.contains('*') || expected.contains(' ') {
        return Err(Error::message(format!(
            "mise.toml 必须精确固定 Buck2 版本，当前值为 `{expected}`"
        )));
    }
    if actual.trim().is_empty() {
        return Err(Error::message("锁定的 Buck2 没有返回版本信息"));
    }
    Ok(expected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declared_host_matrix_has_no_duplicates() {
        let unique = SUPPORTED_HOST_TRIPLES
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(unique.len(), 9);
    }

    #[test]
    fn current_build_host_is_supported() {
        assert!(SUPPORTED_HOST_TRIPLES.contains(&current_host_triple().as_str()));
    }
}
