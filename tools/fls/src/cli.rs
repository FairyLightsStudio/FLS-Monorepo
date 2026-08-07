use std::collections::BTreeSet;
use std::io::{self, IsTerminal, Write};
use std::path::Path;

use clap::{Args, Parser, Subcommand};

use crate::doctor;
use crate::output;
use crate::workspace::{ChangeKind, ChangeRequest, ProjectStatus, Workspace};
use crate::{Error, Result};

#[derive(Debug, Parser)]
#[command(
    name = "fls",
    version = "0.1.0",
    about = "FairyLightsStudio 单仓库选择性工作树工具"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// 显式选择项目，并自动检出其 Buck2 依赖闭包
    Add(SelectionArgs),
    /// 取消显式选择项目；仍被依赖的项目会继续保留
    Remove(DestructiveSelectionArgs),
    /// 用给定集合替换显式项目选择
    Set(DestructiveSelectionArgs),
    /// 列出项目及其来源状态
    List(OutputArgs),
    /// 根据当前 HEAD 重新计算依赖闭包
    Reconcile(ReconcileArgs),
    /// 检查 Git、Buck2、清单、项目图、稀疏检出与 hooks
    Doctor(OutputArgs),
}

#[derive(Clone, Debug, Args)]
pub struct SelectionArgs {
    #[arg(value_name = "PROJECT")]
    pub projects: Vec<String>,
    #[arg(long, conflicts_with = "projects")]
    pub all: bool,
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long)]
    pub json: bool,
}

#[derive(Clone, Debug, Args)]
pub struct DestructiveSelectionArgs {
    #[arg(value_name = "PROJECT")]
    pub projects: Vec<String>,
    #[arg(long, conflicts_with = "projects")]
    pub all: bool,
    /// 确认允许从工作树移除干净项目
    #[arg(long)]
    pub yes: bool,
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long)]
    pub json: bool,
}

#[derive(Clone, Debug, Args)]
pub struct OutputArgs {
    #[arg(long)]
    pub json: bool,
}

#[derive(Clone, Debug, Args)]
pub struct ReconcileArgs {
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long)]
    pub json: bool,
    #[arg(long, hide = true)]
    pub hook: bool,
}

impl Cli {
    pub fn wants_json(&self) -> bool {
        match &self.command {
            Command::Add(args) => args.json,
            Command::Remove(args) | Command::Set(args) => args.json,
            Command::List(args) | Command::Doctor(args) => args.json,
            Command::Reconcile(args) => args.json,
        }
    }
}

pub fn run(cli: Cli) -> Result<i32> {
    let mut workspace = Workspace::open(Path::new("."))?;
    match cli.command {
        Command::List(args) => {
            let statuses = workspace.statuses();
            if args.json {
                output::json(&serde_json::json!({
                    "projects": statuses,
                    "warnings": workspace.warnings,
                }))?;
            } else {
                output::print_statuses(&statuses, &workspace.warnings);
            }
            Ok(0)
        }
        Command::Doctor(args) => {
            workspace.ensure_metadata()?;
            let report = doctor::run(&workspace);
            if args.json {
                output::json(&report)?;
            } else {
                output::print_doctor(&report);
            }
            Ok(if report.healthy { 0 } else { 1 })
        }
        Command::Add(args) => {
            workspace.ensure_metadata()?;
            let projects = selection_for_add(&workspace, &args)?;
            execute_change(
                &workspace,
                ChangeRequest {
                    kind: ChangeKind::Add,
                    projects,
                },
                args.dry_run,
                args.json,
                true,
            )
        }
        Command::Remove(args) => {
            workspace.ensure_metadata()?;
            let projects = selection_for_remove(&workspace, &args)?;
            execute_change(
                &workspace,
                ChangeRequest {
                    kind: ChangeKind::Remove,
                    projects,
                },
                args.dry_run,
                args.json,
                args.yes,
            )
        }
        Command::Set(args) => {
            workspace.ensure_metadata()?;
            let projects = selection_for_set(&workspace, &args)?;
            execute_change(
                &workspace,
                ChangeRequest {
                    kind: ChangeKind::Set,
                    projects,
                },
                args.dry_run,
                args.json,
                args.yes,
            )
        }
        Command::Reconcile(args) => {
            workspace.ensure_metadata()?;
            let result = execute_change(
                &workspace,
                ChangeRequest {
                    kind: ChangeKind::Reconcile,
                    projects: BTreeSet::new(),
                },
                args.dry_run,
                args.json,
                true,
            );
            if args.hook {
                match result {
                    Ok(code) => Ok(code),
                    Err(error) => {
                        eprintln!("fls reconcile hook 警告：{error}");
                        Ok(0)
                    }
                }
            } else {
                result
            }
        }
    }
}

fn execute_change(
    workspace: &Workspace,
    request: ChangeRequest,
    dry_run: bool,
    json: bool,
    confirmed: bool,
) -> Result<i32> {
    let plan = workspace.plan(request)?;
    if dry_run {
        if json {
            output::json(&plan)?;
        } else {
            output::print_plan(&plan);
        }
        return Ok(0);
    }
    if !plan.removed_projects.is_empty() && !confirmed && !confirm_removal(&plan.removed_projects)?
    {
        return Err(Error::message(
            "操作已取消；非交互环境请检查 `--dry-run` 后使用 `--yes` 确认",
        ));
    }
    let outcome = workspace.apply(plan)?;
    if json {
        output::json(&outcome)?;
    } else {
        output::print_outcome(&outcome);
    }
    Ok(0)
}

fn selection_for_add(workspace: &Workspace, args: &SelectionArgs) -> Result<BTreeSet<String>> {
    if args.all {
        return Ok(workspace.all_projects());
    }
    if !args.projects.is_empty() {
        return workspace.resolve_inputs(&args.projects);
    }
    let choices = workspace
        .statuses()
        .into_iter()
        .filter(|status| status.status != "explicit" && status.status != "unavailable")
        .collect::<Vec<_>>();
    interactive_select("选择要显式添加的项目", &choices, false)
}

fn selection_for_remove(
    workspace: &Workspace,
    args: &DestructiveSelectionArgs,
) -> Result<BTreeSet<String>> {
    if args.all {
        return Ok(workspace.state.explicit_projects.clone());
    }
    if !args.projects.is_empty() {
        return workspace.resolve_removal_inputs(&args.projects);
    }
    let choices = workspace
        .statuses()
        .into_iter()
        .filter(|status| status.status == "explicit" || status.status == "unavailable")
        .collect::<Vec<_>>();
    interactive_select("选择要取消显式选择的项目", &choices, false)
}

fn selection_for_set(
    workspace: &Workspace,
    args: &DestructiveSelectionArgs,
) -> Result<BTreeSet<String>> {
    if args.all {
        return Ok(workspace.all_projects());
    }
    if !args.projects.is_empty() {
        return workspace.resolve_inputs(&args.projects);
    }
    let choices = workspace
        .statuses()
        .into_iter()
        .filter(|status| status.status != "unavailable")
        .collect::<Vec<_>>();
    interactive_select("选择新的显式项目集合", &choices, true)
}

fn interactive_select(
    title: &str,
    choices: &[ProjectStatus],
    show_defaults: bool,
) -> Result<BTreeSet<String>> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err(Error::message(
            "没有提供项目且当前不是交互终端；请传入项目 ID 或使用 `--all`",
        ));
    }
    println!("{title}：");
    for (index, choice) in choices.iter().enumerate() {
        let selected = show_defaults && choice.status == "explicit";
        let marker = if selected { "x" } else { " " };
        println!(
            "  {:>2}. [{}] {:<28} {}",
            index + 1,
            marker,
            choice.id,
            choice.description
        );
    }
    if choices.is_empty() {
        return Ok(BTreeSet::new());
    }
    if show_defaults {
        print!("输入编号（空格或逗号分隔，all 表示全部，直接回车保留勾选）：");
    } else {
        print!("输入编号（空格或逗号分隔，all 表示全部）：");
    }
    io::stdout()
        .flush()
        .map_err(|source| Error::message(format!("无法刷新终端输出: {source}")))?;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|source| Error::message(format!("无法读取选择: {source}")))?;
    let input = input.trim();
    if input.is_empty() {
        return Ok(if show_defaults {
            choices
                .iter()
                .filter(|choice| choice.status == "explicit")
                .map(|choice| choice.id.clone())
                .collect()
        } else {
            BTreeSet::new()
        });
    }
    if input.eq_ignore_ascii_case("all") {
        return Ok(choices.iter().map(|choice| choice.id.clone()).collect());
    }
    let mut selected = BTreeSet::new();
    for token in input.split(|character: char| character == ',' || character.is_whitespace()) {
        if token.is_empty() {
            continue;
        }
        let index: usize = token
            .parse()
            .map_err(|_| Error::message(format!("无效选择 `{token}`；请输入列表中的编号")))?;
        let choice = choices.get(index.saturating_sub(1)).ok_or_else(|| {
            Error::message(format!("选择 `{index}` 超出 1..={} 的范围", choices.len()))
        })?;
        selected.insert(choice.id.clone());
    }
    Ok(selected)
}

fn confirm_removal(projects: &BTreeSet<String>) -> Result<bool> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Ok(false);
    }
    print!(
        "将从工作树移除这些干净项目：{}。继续？[y/N] ",
        projects.iter().cloned().collect::<Vec<_>>().join(", ")
    );
    io::stdout()
        .flush()
        .map_err(|source| Error::message(format!("无法刷新终端输出: {source}")))?;
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .map_err(|source| Error::message(format!("无法读取确认: {source}")))?;
    Ok(matches!(
        answer.trim().to_ascii_lowercase().as_str(),
        "y" | "yes"
    ))
}
