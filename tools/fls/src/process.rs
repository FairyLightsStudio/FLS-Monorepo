use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::process::{Command, Stdio};

use crate::{Error, Result};

#[derive(Debug)]
pub struct Output {
    pub success: bool,
    pub status: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub fn run<I, S>(program: &str, args: I, cwd: &Path, stdin: Option<&[u8]>) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_with_env(program, args, cwd, stdin, &[])
}

pub fn run_with_env<I, S>(
    program: &str,
    args: I,
    cwd: &Path,
    stdin: Option<&[u8]>,
    env: &[(OsString, OsString)],
) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let arguments: Vec<OsString> = args
        .into_iter()
        .map(|argument| argument.as_ref().to_owned())
        .collect();
    let display = format_command(program, &arguments);
    let output = run_allow_failure_with_env(program, &arguments, cwd, stdin, env)?;
    if !output.success {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        let details = match (stderr.is_empty(), stdout.is_empty()) {
            (false, _) => format!(":\n{stderr}"),
            (true, false) => format!(":\n{stdout}"),
            (true, true) => String::new(),
        };
        return Err(Error::Command {
            command: display,
            status: output
                .status
                .map(|code| code.to_string())
                .unwrap_or_else(|| "被信号终止".to_owned()),
            details,
        });
    }
    Ok(output)
}

pub fn run_allow_failure<I, S>(
    program: &str,
    args: I,
    cwd: &Path,
    stdin: Option<&[u8]>,
) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_allow_failure_with_env(program, args, cwd, stdin, &[])
}

pub fn run_allow_failure_with_env<I, S>(
    program: &str,
    args: I,
    cwd: &Path,
    stdin: Option<&[u8]>,
    env: &[(OsString, OsString)],
) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let arguments: Vec<OsString> = args
        .into_iter()
        .map(|argument| argument.as_ref().to_owned())
        .collect();
    let display = format_command(program, &arguments);
    let mut command = Command::new(program);
    command
        .args(&arguments)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .envs(env.iter().cloned());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    } else {
        command.stdin(Stdio::null());
    }

    let mut child = command
        .spawn()
        .map_err(|source| Error::message(format!("无法启动 `{display}`: {source}")))?;
    if let Some(input) = stdin {
        use std::io::Write;
        child
            .stdin
            .take()
            .expect("已请求管道 stdin")
            .write_all(input)
            .map_err(|source| {
                Error::message(format!("无法向 `{display}` 写入标准输入: {source}"))
            })?;
    }
    let output = child
        .wait_with_output()
        .map_err(|source| Error::message(format!("无法等待 `{display}`: {source}")))?;
    Ok(Output {
        success: output.status.success(),
        status: output.status.code(),
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

fn format_command(program: &str, arguments: &[OsString]) -> String {
    std::iter::once(OsStr::new(program))
        .chain(arguments.iter().map(OsString::as_os_str))
        .map(|part| part.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ")
}
