我们使用 Bazel

```bash
bazel build //UserCenter:UserCenter-lib
bazel build //UserCenter:UserCenter
```

依赖由 `crate_universe` 从 `UserCenter/Cargo.lock` 解析（见根 `MODULE.bazel`）。
