# 智能体们可以在这里记一些代码所没记载的项目背景信息

# 保持此文件最新、及时删除过时内容。此文件不需要记录历史。历史由git commit日志记录。

## 构建边界

- Bazel 9.3.2 目前只负责 Rust（`rules_rust` + 各工作区 `Cargo.lock` 的 crate_universe）与 proto codegen。
  **没有**接入 JS/Node 规则（没有 `aspect_rules_js` / `rules_ts` / `npm_translate_lock`）。
- 现代前端（Angular/Nx/Bun、Rspress 文档站）走原生工具链；Bazel 对它们只提供
  `fls_project(name = "workspace")` 项目边界标记（`//tools/bazel:fls_project.bzl`）。
- `fls` 用 `bazel query deps(//Project/...)` 计算粗粒度项目依赖闭包，所以每个顶层项目目录
  必须有 `BUILD.bazel`，哪怕里面没有真正的构建 target。

## 目录与文档

- `Docs/AboutUs/`：官网（Angular + pnpm，`cd Docs/AboutUs && mise exec -- pnpm install && mise exec -- pnpm start`）。
- `Docs/Products/`：产品文档，一个产品一个 Rspress 站点（`Docs/Products/README.md`）。
  `cd Docs/Products && mise exec -- pnpm install && mise exec -- pnpm dev`；构建产物在 `<产品>/doc_build/`。
- 前端项目统一用 pnpm（不再用 bun/npm/yarn）：`Docs/AboutUs`、`Docs/Products`、`TeraPanel/teraclient-web`、`PoWCAPTCHA/web-component` 的 `package.json` 都用 `packageManager: pnpm@12.19.0`。
- `PoWCAPTCHA/web-component` 依赖 BSR 上的 `@buf/*` 包，因此该目录有 `.npmrc` 把 `@buf` scope 指到 `https://buf.build/gen/npm/v1/`（公开包，无需 token）。
- `Docs/` 在 `fls-projects.toml` 里是单个项目；新增文档站不需要改 fls 清单。
- 旧的 `TeraPanelDocsSite/` 已删除（内容迁移到 `Docs/Products/TeraPanel`），git 历史里可找回。

## 历史与清理

- 2026-11 从 Buck2 迁到 Bazel（`1543192`、`c182530`）。`third-party/rust`（reindeer 生成的
  BUCK 依赖元数据）、`buck-out`、`BUCK` 文件都已删除；Bazel 依赖由各工作区 Cargo.lock 解析。
- 旧站点/旧路径删除后都还能从 git 历史找回，不必为“留备份”保留无用目录。

## 工具版本

- mise：`mise.toml` 用宽松版本（bazel 9.3、rust 1.99、node 26、pnpm 12、fls 0.1），精确版本由 `mise.lock` 锁定（当前 bazel 9.3.2 / rust 1.99.1 / node 26.13.0 / pnpm 12.19.0 / fls 0.1.1）。
  更新工具：`mise lock --bump [工具]`（发版不到 24h 的版本需要 `MISE_MINIMUM_RELEASE_AGE=0`），安装：`mise install --locked`（仅项目工具）或 `mise install`。
- `mise.lock` 只为 8 个常用平台生成（`[settings] lockfile_platforms`，riscv64 上不跑这些开发工具）。
- `fls` 的发布流程：bump `tools/fls/Cargo.toml` 与 `tools/fls/src/cli.rs`（clap 版本是硬编码的）、
  打 `fls-v*` tag 触发 `.github/workflows/release-fls.yml`，再更新 `mise.toml` 与 `mise.lock`。

## 前端接入 Bazel 的调研结论（2026-11）

- 用 `aspect_rules_js 3.6.0` + `rules_nodejs 6.7.7` 对 Rspress 站点做过 PoC：构建能跑通，
  但默认 linux-sandbox 下因 rules_js #362（ESM 逃逸 sandbox/runfiles）稳定失败，
  只有 `--spawn_strategy=local` 才成功，等于放弃 hermetic 与远端缓存。
- 因此现代前端继续走原生工具链；结论与重新评估触发条件写在 `roadmap.md`。
- 所有 pnpm 项目的 `pnpm-lock.yaml` 都是 pnpm 12 的**两文档**格式（`packageManager` 锁定触发）：
