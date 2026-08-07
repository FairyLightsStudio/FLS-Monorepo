use std::io::{self, Write};

use serde::Serialize;

use crate::doctor::{CheckStatus, Report};
use crate::workspace::{ApplyOutcome, Plan, ProjectStatus};
use crate::{Error, Result};

pub fn json<T: Serialize>(value: &T) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    serde_json::to_writer_pretty(&mut handle, value)
        .map_err(|source| Error::message(format!("无法生成 JSON 输出: {source}")))?;
    writeln!(handle).map_err(|source| Error::message(format!("无法写入标准输出: {source}")))
}

pub fn print_plan(plan: &Plan) {
    println!("计划：{:?}", plan.command);
    print_set("显式项目", &plan.explicit_projects);
    print_set("解析后项目", &plan.resolved_projects);
    print_set("将检出", &plan.added_projects);
    print_set("将移除", &plan.removed_projects);
    if !plan.retained_projects.is_empty() {
        println!("保留项目：");
        for (id, reason) in &plan.retained_projects {
            println!("  - {id}: {reason}");
        }
    }
    for warning in &plan.warnings {
        eprintln!("警告：{warning}");
    }
}

pub fn print_outcome(outcome: &ApplyOutcome) {
    if outcome.plan.changes_worktree() {
        println!(
            "工作树已更新：新增 {} 个项目，移除 {} 个项目。",
            outcome.plan.added_projects.len(),
            outcome.plan.removed_projects.len()
        );
    } else {
        println!("工作树已经符合请求，无需增删项目。");
    }
    println!("显式选择：{}", join(&outcome.plan.explicit_projects));
    println!("依赖闭包：{}", join(&outcome.plan.resolved_projects));
    for warning in &outcome.warnings {
        eprintln!("警告：{warning}");
    }
}

pub fn print_statuses(statuses: &[ProjectStatus], warnings: &[String]) {
    for status in statuses {
        let path = status.path.as_deref().unwrap_or("-");
        println!(
            "{:<28} {:<12} {:<28} {}",
            status.id, status.status, path, status.description
        );
        if let Some(reason) = &status.reason {
            println!("  原因：{reason}");
        }
    }
    for warning in warnings {
        eprintln!("警告：{warning}");
    }
}

pub fn print_doctor(report: &Report) {
    for check in &report.checks {
        let symbol = match check.status {
            CheckStatus::Pass => "PASS",
            CheckStatus::Warning => "WARN",
            CheckStatus::Fail => "FAIL",
        };
        println!("[{symbol}] {:<20} {}", check.name, check.message);
    }
    if report.healthy {
        println!("doctor 检查通过。");
    } else {
        println!("doctor 检查未通过；修改命令可能拒绝服务。");
    }
}

fn print_set(name: &str, values: &std::collections::BTreeSet<String>) {
    println!("{name}：{}", join(values));
}

fn join(values: &std::collections::BTreeSet<String>) -> String {
    if values.is_empty() {
        "（无）".to_owned()
    } else {
        values.iter().cloned().collect::<Vec<_>>().join(", ")
    }
}

#[derive(Serialize)]
pub struct ErrorEnvelope<'a> {
    pub error: &'a str,
}
