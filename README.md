# FairyLightsStudio Monorepo

这是 FairyLightsStudio 的统一单仓库。项目代码直接属于同一份 Git 历史，不再使用 Git submodule；Buck2 提供跨项目依赖图和统一构建入口，`fls` 负责按需生成安全的稀疏工作树。

## 第一次只检出一个项目

先安装并启用 [mise](https://mise.jdx.dev/)，然后执行：

```bash
git clone --filter=blob:none --sparse git@github.com:FairyLightsStudio/FLS-Monorepo.git
cd FLS-Monorepo
mise trust
mise install
mise exec -- fls add TeraPanel
```

`fls add TeraPanel` 会扫描 `//TeraPanel/...` 下的全部 Buck2 targets，再通过 `buck2 uquery` 计算完整依赖闭包。例如 TeraPanel 依赖 UserCenter 时，两者都会被检出，而且都是同一仓库内可编辑、可提交的普通目录。

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
| `fls doctor` | 检查 host、Git、Buck2、清单、完整依赖图、稀疏规则和 hooks |

`add`、`remove`、`set` 在没有参数时提供交互式多选，也支持 `--all`、`--dry-run` 和 `--json`。会移除项目的 `remove`/`set` 需要交互确认或 `--yes`。

项目内存在已跟踪、已暂存或未跟踪修改时，主动移除会整体失败。Git hook 触发的 `reconcile` 会把这种项目标为 `retained`，而不是删除文件。被 Git 忽略的构建输出不会由 `fls` 删除。

被移除项目的业务源码会离开工作树，但其 `BUCK`/`.bzl` 元数据仍会保留，以便 Buck2 在不下载全部业务文件的情况下计算完整项目图。因此看到仅含 `BUCK` 的项目目录是正常现象。

## 配置与状态

- [`fls-projects.toml`](fls-projects.toml) 只登记项目 ID、顶层路径和始终需要的控制目录。项目依赖不在这里重复维护。
- 每个顶层项目必须提供 `//Project:workspace`，并通过 Buck2 `deps` 声明跨项目关系。
- target 和 `deps` 的形状必须是静态的，并覆盖 `select()` 的所有分支；`glob()` 只能用于填充 `srcs`，不能根据业务文件是否已检出来决定 target 或依赖是否存在。
- 显式选择保存在当前工作树的 `<git-dir>/fls/state.toml`，不会提交，也不会与其他 Git worktree 共用。
- 工具版本由 [`mise.toml`](mise.toml) 精确固定。`fls` 不会自更新，也不会管理或终止 `buckd`。
- `fls` 不执行 `fetch`、`pull`、`commit`、`reset` 或 `clean`；远程同步和提交仍由 Git/Jujutsu 负责。

`fls-projects.toml` 当前及上一版 schema 可读取；上一版会提示迁移且 CI 失败。过新或过旧的 schema 会禁止工作树修改，但 `list` 和 `doctor` 仍可用于诊断。本地状态只保留当前及上一版，成功迁移时原子覆写，不创建备份。

## 构建与测试 fls

Buck2 是 `fls` 的规范构建入口：

```bash
mise exec -- buck2 build //tools/fls:fls
mise exec -- buck2 test //tools/fls:fls-test
mise run doctor
```

发布流程提供九个固定 host triple 的预编译资产：Apple Darwin 与 Windows MSVC 的 x86_64/aarch64，以及 Linux GNU/MUSL 的 x86_64/aarch64和 Linux GNU 的 riscv64gc。`doctor` 会拒绝不在该矩阵中的 host。

Rust 第三方依赖由 `third-party/rust/Cargo.lock` 锁定，Reindeer 生成带 SHA256 的 `http_archive` 规则；全新工作树执行查询不需要先运行 `reindeer vendor`。更新依赖时执行：

```bash
cd third-party/rust
CARGO_HOME="$PWD/.cargo" cargo fetch --locked
reindeer buckify
```

随后提交 Cargo.lock 与生成的 BUCK。

现代 Angular/Nx/Bun 项目为什么暂不交给 Buck2 构建，以及未来的迁移条件，见 [`roadmap.md`](roadmap.md)。
