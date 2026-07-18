# AGENTS.md — 本项目 AI Agent 协作经验

## 项目概述

`UserCenter` 是基于 ConnectRPC (Rust) 的微服务项目，通过 protobuf 定义服务。

---

## 经验 1：Cargo build script 必须在项目根目录

**问题**：将 build script 放在 `src/build.rs` 中，Cargo 不会识别。

**原因**：Cargo 要求 build script 的路径是 `<package_root>/build.rs`，而非 `src/` 目录下。

**解决**：将 `src/build.rs` 移到项目根目录，即 `<package_root>/build.rs`。

```bash
mv src/build.rs build.rs
```

---

## 经验 2：`connectrpc::include_generated!()` 是 v0.6+ 才有的宏

**问题**：使用 `connectrpc::include_generated!()` 时报 `cannot find include_generated in connectrpc`。

**原因**：该宏在 connectrpc 0.3.x 中不存在，需要 v0.6+。项目原本依赖的是 `connectrpc = "0.3"`。

**解决**：升级依赖到最新版本：

```toml
# Cargo.toml
[dependencies]
connectrpc = { version = "0.6", features = ["axum", "client", "tls"] }

[build-dependencies]
connectrpc-build = "0.6"
```

---

## 经验 3：升级 connectrpc 时，axum 等传递依赖也要同步升级

**问题**：升级 connectrpc 到 v0.6 后，`axum::serve` 找不到。

**原因**：connectrpc v0.6 内部依赖 axum v0.8，但项目显式依赖的是 axum v0.6。`axum::serve` 是 axum 0.7+ 才有的函数。

**解决**：将显式的 axum 依赖也升级到与 connectrpc 兼容的版本：

```toml
axum = { version = "0.8", features = ["tokio", "http1"] }
```

---

## 经验 4：buf vs build.rs — 两种代码生成方式

| 方式 | 优点 | 适用场景 |
|---|---|---|
| `buf generate` | 生成代码可提交到 repo、无额外编译依赖 | 团队协作、CI/CD |
| `build.rs` (connectrpc-build) | 零配置、proto 变更自动重新生成 | 个人项目、快速原型 |

`build.rs` 方式的核心结构：

```rust
// build.rs
fn main() {
    connectrpc_build::Config::new()
        .files(&["testproto/greet.proto"])
        .includes(&["testproto/"])
        .include_file("_connectrpc.rs")
        .compile()
        .unwrap();
}
```

```rust
// src/main.rs 或 src/lib.rs —— 引入生成的代码
pub mod proto {
    connectrpc::include_generated!();
}
```

---

## 经验 5：lib.rs 和 main.rs 共存时都要处理 proto 模块

项目同时有 `lib.rs` 和 `main.rs`，两者都定义了 `pub mod proto`。如果 lib.rs 中指向 buf 生成的旧路径而 build script 已切换为 connectrpc-build，会导致 lib 编译失败。需要两边统一使用 `connectrpc::include_generated!()`。
