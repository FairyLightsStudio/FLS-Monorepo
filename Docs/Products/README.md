# Products（产品文档）

彩灯工作室所有产品的文档，一个产品一个 [Rspress](https://rspress.dev/) 站点。

## 目录结构

```
Docs/
├── AboutUs/              # 官网（Angular 应用，见 Docs/AboutUs/README.md）
└── Products/             # 产品文档（本目录）
    ├── pnpm-workspace.yaml   # pnpm workspace，每个产品是一个包
    ├── package.json          # 根脚本：统一 build / dev / preview
    └── TeraPanel/            # 泰拉面板文档站
        ├── rspress.config.ts
        ├── docs/             # Markdown / MDX 与静态资源（public/）
        └── package.json
```

## 本地开发

先进入本目录并让 mise 提供固定的 Node 版本：

```bash
mise exec -- pnpm install
mise exec -- pnpm dev      # 开发服务器
mise exec -- pnpm build    # 产物在 <产品>/doc_build/
```

## 新增一个产品的文档

1. 复制 `TeraPanel/` 为 `<产品名>/`，改掉 `package.json` 里的 `name`、`rspress.config.ts` 里的 `title`/`icon`/`logo`；
2. 在 `pnpm-workspace.yaml` 的 `packages` 里加上该目录；
3. 在根 `package.json` 的脚本里按需增加对应 `--filter`。

`Docs/` 在 [`fls-projects.toml`](../../fls-projects.toml) 里是单个项目（`Docs`），项目边界由 [`Docs/BUILD.bazel`](../BUILD.bazel) 的 `fls_project(name = "workspace")` 声明；新增站点不需要再改 fls 清单。

## 说明

- 文档站由 pnpm + Rspress 原生工具链构建，Bazel 当前只负责项目边界（见 [roadmap.md](../../roadmap.md)）。
- `node_modules/`、`doc_build/` 等构建产物已被 `.gitignore` 与 `.bazelignore` 忽略。
