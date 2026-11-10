# ProductsDoc

彩灯工作室所有产品的文档，一个产品一个 [Rspress](https://rspress.dev/) 站点。

## 目录结构

```
ProductsDoc/
├── pnpm-workspace.yaml   # pnpm workspace，每个产品是一个包
├── package.json          # 根脚本：统一 build / dev / preview
└── TeraPanel/            # 泰拉面板文档站
    ├── rspress.config.ts
    ├── docs/             # Markdown / MDX 与静态资源（public/）
    └── package.json
```

## 本地开发

先进入仓库根目录并让 mise 提供固定的 Node 版本：

```bash
mise exec -- pnpm install
mise exec -- pnpm dev      # 开发服务器
mise exec -- pnpm build    # 产物在 <产品>/doc_build/
```

## 新增一个产品的文档

1. 复制 `TeraPanel/` 为 `<产品名>/`，改掉 `package.json` 里的 `name`、`rspress.config.ts` 里的 `title`/`icon`/`logo`；
2. 在 `pnpm-workspace.yaml` 的 `packages` 里加上该目录；
3. 在根 `package.json` 的脚本里按需增加对应 `--filter`；
4. 在仓库根的 [`fls-projects.toml`](../fls-projects.toml) 登记该产品文档目录，并在该目录放一个 `BUILD.bazel`（`fls_project(name = "workspace")`），以便 `fls` 与 Bazel 识别项目边界。

## 说明

- `TeraPanelDocsSite/` 是迁移前的旧站点，内容已复制到这里；确认新站点无误后可以删掉旧目录。
- `node_modules/`、`doc_build/` 等构建产物已被 `.gitignore` 与 `.bazelignore` 忽略。
