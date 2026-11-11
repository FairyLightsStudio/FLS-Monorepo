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

## 产品切换入口（共享主题）

导航栏左上角的九宫格按钮可以在不同产品的文档站点之间跳转，实现在 `shared/theme/`，所有产品站点共用：

```
shared/theme/
├── index.tsx            # 包装 Rspress 默认 Layout，在 beforeNavTitle 注入入口
└── nav-icon/
    ├── index.tsx        # 九宫格按钮 + 弹出面板（悬停/点击展开，Esc 或点击外部关闭）
    ├── nav-config.ts    # 产品列表：名称、描述、图标、链接、状态
    └── assets/          # 产品图标（打包进主题，避免各站点重复放 public）
```

新增一个产品时：

1. 在 `shared/theme/nav-icon/nav-config.ts` 的 `products` 里加一条；已上线填 `url`、`status: 'ready'`，未上线用 `status: 'soon'`（面板里显示"即将上线"且不可点击）；
2. 图标放进 `shared/theme/nav-icon/assets/`，在配置里 `import` 后赋给 `logo`；没有图标可用 `emoji` 兜底；
3. 新站点在自己的 `rspress.config.ts` 里设置同一个主题目录：`themeDir: path.join(__dirname, '..', 'shared', 'theme')`；
4. 站点部署到子路径（如 `/terapanel/`）时，把 `url` 改成实际地址。

实现移植自 Rstack 文档站的 [`@rstack-dev/doc-ui` NavIcon](https://github.com/rstackjs/rstack-doc-ui)：antd `Popover`（`bottomLeft`、无箭头、hover 触发）+ `PopoverContent`/`LinkGroup`/`Link` 结构 + Rspress 自定义主题的 `beforeNavTitle` 插槽；产品列表换成我们自己的 `nav-config.ts`，样式用上游的面板尺寸/间距，并适配亮暗色。

> 上游的 `@rstack-dev/doc-ui` 不能直接 `npm i` 后替换产品：它的产品列表 `nav-config.json` 是打包进组件里的（且 header 写死 Rstack）。所以这里是把它的组件源码和样式 vendor 进共享主题，产品数据由我们控制。

## 说明

- 文档站由 pnpm + Rspress 原生工具链构建，Bazel 当前只负责项目边界（见 [roadmap.md](../../roadmap.md)；2026-11 的 rules_js 可行性调研结论也在那里）。
- `node_modules/`、`doc_build/` 等构建产物已被 `.gitignore` 与 `.bazelignore` 忽略。
- 根 `package.json` 用 `packageManager: pnpm@12.18.0` 锁定 pnpm 版本，因此 `pnpm-lock.yaml` 是 **pnpm 12 的两文档格式**（第一个是 pnpm 自身的 env 文档，最后一个是项目依赖图，两个文档都有 `lockfileVersion`）。这不是文件损坏：解析工具请用 YAML 的多文档 API 并取最后一个文档，见 [pnpm 文档](https://pnpm.io/lockfile)。
