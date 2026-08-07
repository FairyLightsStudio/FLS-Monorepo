use std::fs;
use std::path::Path;

use crate::git::Git;
use crate::{Error, Result};

const MARKER: &str = "# fls-managed-hook:v1";
const HOOKS: &[&str] = &["post-checkout", "post-merge", "post-rewrite"];

pub fn install(git: &Git) -> Result<Vec<String>> {
    let (hooks_path, configured) = git.hooks_path()?;
    if configured {
        return Ok(vec![format!(
            "检测到自定义 core.hooksPath（{}）；fls 不会写入或覆盖自定义 hooks，请自行调用 `fls reconcile --hook`。",
            hooks_path.display()
        )]);
    }
    fs::create_dir_all(&hooks_path).map_err(|source| Error::Write {
        path: hooks_path.clone(),
        source,
    })?;
    let mut warnings = Vec::new();
    for hook in HOOKS {
        let path = hooks_path.join(hook);
        if path.exists() {
            let existing = fs::read_to_string(&path).map_err(|source| Error::Read {
                path: path.clone(),
                source,
            })?;
            if !existing.contains(MARKER) {
                warnings.push(format!(
                    "未覆盖已有 hook `{}`；请在其中调用 `fls reconcile --hook`。",
                    path.display()
                ));
                continue;
            }
        }
        fs::write(&path, hook_script()).map_err(|source| Error::Write {
            path: path.clone(),
            source,
        })?;
        make_executable(&path)?;
    }
    Ok(warnings)
}

pub fn inspect(git: &Git) -> Result<Vec<String>> {
    let (hooks_path, configured) = git.hooks_path()?;
    if configured {
        return Ok(vec![format!(
            "自定义 core.hooksPath 为 {}，fls 无法确认 reconcile hook 已接入。",
            hooks_path.display()
        )]);
    }
    let mut warnings = Vec::new();
    for hook in HOOKS {
        let path = hooks_path.join(hook);
        match fs::read_to_string(&path) {
            Ok(contents) if contents.contains(MARKER) => {}
            Ok(_) => warnings.push(format!(
                "{} 已存在但不由 fls 管理；请手动接入 `fls reconcile --hook`。",
                path.display()
            )),
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => warnings.push(format!(
                "缺少 {}；下一次成功执行 fls 修改命令时会尝试安装。",
                path.display()
            )),
            Err(source) => {
                return Err(Error::Read { path, source });
            }
        }
    }
    Ok(warnings)
}

fn hook_script() -> &'static str {
    "#!/bin/sh\n\
# fls-managed-hook:v1\n\
if [ \"${FLS_RECONCILING:-}\" = \"1\" ]; then\n\
    exit 0\n\
fi\n\
if command -v mise >/dev/null 2>&1; then\n\
    mise exec -- fls reconcile --hook\n\
else\n\
    fls reconcile --hook\n\
fi\n"
}

#[cfg(unix)]
fn make_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let metadata = fs::metadata(path).map_err(|source| Error::Read {
        path: path.to_owned(),
        source,
    })?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(permissions.mode() | 0o111);
    fs::set_permissions(path, permissions).map_err(|source| Error::Write {
        path: path.to_owned(),
        source,
    })
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) -> Result<()> {
    Ok(())
}
