# FairyLightsStudio Monorepo

这是 FairyLightsStudio 的统一单仓库。项目代码直接属于同一份 Git 历史，不再使用 Git submodule；Bazel 提供跨项目依赖图和统一构建入口，`fls` 负责按需生成安全的稀疏工作树。

## 第一次只检出一个项目

先安装并启用 [mise](https://mise.jdx.dev/)，然后执行：

```bash
git clone --filter=blob:none --sparse git@github.com:FairyLightsStudio/FLS-Monorepo.git
cd FLS-Monorepo
mise trust
mise install
mise exec -- fls add TeraPanel
```

`fls add TeraPanel` 会扫描 `//TeraPanel/...` 下的全部 Bazel targets，再通过 `bazel query deps(...)` 计算完整依赖闭包。例如 TeraPanel 依赖 UserCenter 时，两者都会被检出，而且都是同一仓库内可编辑、可提交的普通目录。

如果已经完整克隆了仓库，第一次执行修改命令会采用当前全部项目，不会让已有目录突然消失。要收缩工作树，应先预览：

```bash
mise exec -- fls set TeraPanel --dry-run
mise exec -- fls set TeraPanel --yes
```

## fls 命令

| 命令 | 作用 |
| --- | --- |
| `fls add [PROJECT...]` | 幂等地添加显式项目，并自动检出依赖 |
| `fls remove [PROJECT...]` | 取消显式选择；仍被依赖的项目不会移除 |
| `fls set [PROJECT...]` | 替换显式选择集合 |
| `fls list` | 显示 `explicit`、`dependency`、`retained`、`available`、`unavailable` 状态 |
| `fls reconcile` | 在 HEAD 改变后重新计算依赖闭包，不获取远端更新 |
| `fls doctor` | 检查 host、Git、Bazel、清单、完整依赖图、稀疏规则和 hooks |

`add`、`remove`、`set` 在没有参数时提供交互式多选，也支持 `--all`、`--dry-run` 和 `--json`。会移除项目的 `remove`/`set` 需要交互确认或 `--yes`。

项目内存在已跟踪、已暂存或未跟踪修改时，主动移除会整体失败。Git hook 触发的 `reconcile` 会把这种项目标为 `retained`，而不是删除文件。被 Git 忽略的构建输出不会由 `fls` 删除。

被移除项目的业务源码会离开工作树，但其 `BUILD.bazel`/`.bzl` 元数据仍会保留，以便 Bazel 在不下载全部业务文件的情况下计算完整项目图。因此看到仅含 `BUILD.bazel` 的项目目录是正常现象。

## 配置与状态

- [`fls-projects.toml`](fls-projects.toml) 只登记项目 ID、顶层路径和始终需要的控制目录。项目依赖不在这里重复维护。
- 每个顶层项目必须提供 `//Project:workspace`，并通过 Bazel `deps` 声明跨项目关系。
- target 和 `deps` 的形状必须是静态的，并覆盖 `select()` 的所有分支；`glob()` 只能用于填充 `srcs`，不能根据业务文件是否已检出来决定 target 或依赖是否存在。
- 显式选择保存在当前工作树的 `<git-dir>/fls/state.toml`，不会提交，也不会与其他 Git worktree 共用。
- 工具版本由 [`mise.toml`](mise.toml) 声明（宽松版本）并由 [`mise.lock`](mise.lock) 锁定精确版本；Bazel 同时以 `.bazelversion`（9.3.2）作为权威版本号。安装用 `mise install --locked`。`fls` 不会自更新，也不会管理或终止 Bazel server（`bazel shutdown` 由用户决定）。
- `fls` 不执行 `fetch`、`pull`、`commit`、`reset` 或 `clean`；远程同步和提交仍由 Git/Jujutsu 负责。

`fls-projects.toml` 当前及上一版 schema 可读取；上一版会提示迁移且 CI 失败。过新或过旧的 schema 会禁止工作树修改，但 `list` 和 `doctor` 仍可用于诊断。本地状态只保留当前及上一版，成功迁移时原子覆写，不创建备份。

## 构建与测试 fls

Bazel 是 `fls` 的规范构建入口：

```bash
bazel build //tools/fls:fls
bazel test //tools/fls:fls-test
bazel-bin/tools/fls/fls doctor
# 或：mise run build-fls / mise run test-fls / mise run doctor
```

发布流程提供九个固定 host triple 的预编译资产：Apple Darwin 与 Windows MSVC 的 x86_64/aarch64，以及 Linux GNU/MUSL 的 x86_64/aarch64和 Linux GNU 的 riscv64gc。`doctor` 会拒绝不在该矩阵中的 host。

Rust 第三方依赖由各 Cargo 工作区自己的 `Cargo.lock` 锁定，Bazel 通过 `rules_rust` 的 `crate_universe` 直接从 lockfile 解析并拉取 crate（带校验和），无需本地 vendor 目录。更新某个工作区的依赖后只需提交该工作区的 `Cargo.lock`，Bazel 下次构建自动重新解析。

Bazel 已成为本仓唯一构建系统（Buck2 相关文件已于迁移中删除）。Angular/Nx 等前端项目当前的构建状态见 [`roadmap.md`](roadmap.md)。

## 文档

工作室的文档统一放在 [`Docs/`](Docs/)：

- [`Docs/AboutUs/`](Docs/AboutUs/)：官网，Angular 应用（pnpm + Angular CLI）。
- [`Docs/Products/`](Docs/Products/)：产品文档，一个产品一个 [Rspress](https://rspress.dev/) 站点（目前有泰拉面板），开发方式见 [`Docs/Products/README.md`](Docs/Products/README.md)。

## 前端（pnpm）

所有前端项目统一用 pnpm（`package.json` 中 `packageManager: pnpm@12.19.0`），Node 版本由 [mise.toml](mise.toml) 固定（当前 Node 26）。在仓库根目录即可安装/构建：

```bash
mise exec -- pnpm --dir Docs/AboutUs install
mise exec -- pnpm --dir Docs/AboutUs run build

mise exec -- pnpm --dir Docs/Products install
mise exec -- pnpm --dir Docs/Products --filter terapanel-docs build   # 指定某个产品站点
mise exec -- pnpm --dir Docs/Products -r build                        # 全部产品站点

mise exec -- pnpm --dir TeraPanel/teraclient-web run build
mise exec -- pnpm --dir PoWCAPTCHA/web-component run build
```

- `Docs/Products` 是唯一的 pnpm workspace（每个产品一个包）；其余前端项目是单包项目。
- `PoWCAPTCHA/web-component` 的 `@buf/*` 依赖来自 BSR，目录内的 [`.npmrc`](PoWCAPTCHA/web-component/.npmrc) 已把 `@buf` scope 指向 `https://buf.build/gen/npm/v1/`（公开包，无需 token）。
- 每个项目的 `pnpm-lock.yaml` 都是 pnpm 12 的两文档格式（env 文档 + 项目依赖图），解析工具请用 YAML 多文档 API。
- Bazel 只负责 Rust/proto 与项目边界，前端构建不走 Bazel（原因见 [roadmap.md](roadmap.md)）。

## Bazel 构建

Rust 工作区的 Bazel 构建图与迁移前的目标一一对齐：

```bash
bazel build //...
bazel test //...
```

- `MODULE.bazel`：rules_rust 0.75.0、Rust 1.99.1/edition 2024、按需 rules_proto（protobuf 29.1）。
- 每个 Cargo 工作区一个 crate_universe 仓库（消费各自的 `Cargo.lock`）：UserCenter、PoWCAPTCHA、ChatHistoryCat、tools/fls。
- `cargo_build_script` 覆盖 2 处 proto codegen：UserCenter（`testproto/greet.proto`）、pow-captcha-server（`contracts/pow_captcha/v1/*.proto`，well-known include 由 `wellknown-include` genrule 提供）。
- `//tools/bazel:fls_project.bzl` 提供 `fls_project(name = "workspace")` 标记规则，供 `fls` 用 `bazel query //Project:workspace` 校验项目边界。
- TeraPanel 部分纳入：workspace 表里补上了 `crates/terapanel` 已引用但缺失的 `sqlx` 声明，因此现在有 `Cargo.lock`，可编译的两个 crate 已接入（`//TeraPanel/crates/shared-types:shared-types`、`//TeraPanel/crates/terad:terad`）。`crates/terapanel` 本身仍不构建——源码不完整（`config.rs`/`db.rs` 为空，缺 `state.rs`/`api.rs`，未声明 `volo_http`，引用不存在的 `tera_common`），待应用代码补齐后再加入。
- 基础配置：`.bazelversion`（9.3.2）、`.bazelignore`、`./.bazelrc`；`/.gitignore` 增补 `/bazel-*`。

