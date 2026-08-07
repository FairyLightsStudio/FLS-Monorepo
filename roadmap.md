# FairyLightsStudio 构建系统路线图

> 状态：当前决策
> 最近更新：2026-08-07

## 目标

FairyLightsStudio 使用 Buck2 统一工作室项目的构建入口、跨项目依赖关系和适合纳入构建图的工具链。构建工具应尽可能由 Buck2 的 hermetic toolchain 管理，从而减少开发者本机环境差异，并为可重复构建、增量构建和远程缓存奠定基础。

但是，“Buck2 能够执行某条命令”并不意味着该生态已经得到 Buck2 的成熟支持。对缺少可靠规则、工具链和依赖模型的生态强行套用通用命令规则，只会把原有构建系统隐藏在 Buck2 后面，同时引入两套依赖图和更高的维护成本。

因此，本路线图采用渐进式纳管策略：成熟且适合 Buck2 的构建流程由 Buck2 管理；暂时缺少一等支持的现代前端流程继续使用其原生工具链，待迁移条件满足后再纳入 Buck2。

## 当前职责边界

| 范围 | 当前负责人 |
| --- | --- |
| 稀疏工作树和项目依赖闭包 | `fls` 消费 Buck2 项目图 |
| `fls` 与 Buck2 的版本和首次安装 | mise |
| Rust、Python、C/C++ 等构建图 | Buck2 |
| 当前 Rust 编译器入口 | mise 固定版本，经 Buck2 system toolchain 桥接 |
| Angular、Nx、Storybook 等现代前端工作流 | Bun 与 Nx |
| 前端依赖解析和版本锁定 | `package.json` 与 `bun.lock` |
| 前端项目与其他工作室项目之间的粗粒度依赖 | 轻量 Buck2 桥接 target |
| 暂时无法由 Buck2 管理的前端入口工具版本 | mise |

mise 只承担启动 Buck2 所必需的工具，以及 Buck2 当前无法合理接管的生态入口工具。只要构建工具已经由 Buck2 hermetic toolchain 提供，就不应再通过 mise 重复安装和维护同一套版本。

## 为什么现代前端暂不由 Buck2 管理

截至 2026-08-07，Buck2 开源 prelude 中的 JavaScript 规则主要围绕 `js_library`、`js_bundle`、worker 工具以及 Metro/移动端资源集成展开。它们不构成 Angular、Nx 和 Bun 工作区的完整替代方案。

当前缺少的一等能力包括：

- Bun、npm 或 pnpm lockfile 与 workspace 的原生依赖解析；
- Angular compiler、Angular CLI builders 和 `ng-packagr` 的成熟规则；
- Nx project graph、`affected`、插件推断和任务缓存语义的可靠集成；
- Storybook、Vite、Webpack 或其他现代前端构建链的标准规则；
- 前端测试、开发服务器、热更新和 watch mode 的良好交互体验；
- 与前端包发布、版本管理及生态插件兼容的稳定方案。

可以使用 `genrule` 或自定义规则调用 `bun nx build`，但这类封装暂时存在以下问题：

- Buck2 与 Nx 同时维护任务图和缓存边界，职责重复；
- `node_modules`、动态插件加载和隐式输入难以得到精确追踪；
- 依赖安装若发生在构建 action 中，会引入网络访问和缓存不可重复问题；
- 粗暴声明整个工作区为输入会削弱 Buck2 的增量构建优势；
- 开发服务器、watch mode、IDE 集成和交互式工具的体验可能退化；
- 为跟进 Angular、Nx、Bun 和 Storybook 的快速更新，需要长期维护大量自定义规则。

在这些问题解决以前，把原生命令包进 Buck2 只会形成表面统一，不会带来真正统一、可靠的构建图。

## 当前前端方案

现代前端项目继续使用其原生工作流，例如：

```bash
bun install --frozen-lockfile
bun nx build <project>
bun nx test <project>
```

前端项目可以在根目录提供轻量 Buck2 桥接 target，用于声明它依赖的其他 FairyLightsStudio 项目。桥接 target 只服务于工作区依赖发现，不尝试取代 Nx 的内部项目图，也不负责执行 Angular 构建。

概念示例：

```python
fls_project(
    name = "workspace",
    deps = [
        "root//SomeOtherProject:workspace",
    ],
)
```

`fls add` 会让 Buck2 扫描所选项目目录下的全部 targets，并从查询结果计算跨项目依赖闭包；前端开发者仍然使用受上游支持的 Bun、Nx、Angular 和 Storybook 命令。`//Project:workspace` 是稳定的项目边界和粗粒度依赖桥梁，不代表“构建整个项目”。

## fls 的构建与发布边界

`fls` 自身的规范构建和测试入口是 Buck2。当前开源 Buck2 system Rust toolchain 仍从 PATH 获取编译器，因此 Rust 版本暂由 mise 精确固定；未来会把 Rust 编译器与标准库进一步收进 hermetic toolchain。

GitHub Release 需要同时产出九个 host triple，其中包含 GNU/MUSL 和 RISC-V 交叉编译。发布阶段暂时使用 Cargo/cross 作为纯打包桥梁，但必须先通过 Buck2 测试，且两条路径编译同一份 Rust 源码和锁文件。等 Buck2 的开源 Rust 交叉工具链能够稳定覆盖完整发布矩阵后，应删除 Cargo/cross 例外，让发布资产也完全由 Buck2 产生。

## 迁移到 Buck2 的触发条件

前端迁移不设定主观日期。只有当 Buck2 或可信赖的社区规则满足下列条件时，才启动正式迁移评估：

1. 能够从 lockfile 可重复地解析和物化前端依赖，不依赖构建期间的非受控网络访问；
2. 支持常见 workspace 布局，并能准确表达包之间的依赖关系；
3. Node 或 Bun 工具链能够按操作系统和 CPU 架构固定版本、校验来源并作为 hermetic toolchain 使用；
4. Angular 编译、库打包、测试和开发构建具有稳定规则或清晰的官方集成路径；
5. 能与 Nx 项目图协作，或者能够完整替代当前实际使用的 Nx 能力，而不是重复维护两套图；
6. 增量构建、远程缓存和输入追踪能够覆盖 TypeScript、资源文件、生成代码和前端配置；
7. Storybook、测试、开发服务器和 watch mode 不出现明显体验倒退；
8. Linux、macOS 和 Windows 的行为一致，并具备可维护的升级与故障诊断方式；
9. 规则接口和维护状态足够稳定，不需要 FairyLightsStudio 长期追随上游内部实现细节。

满足条件不代表立即迁移。应先选择一个规模较小的前端包进行验证，至少比较以下指标：

- 首次构建和增量构建耗时；
- 冷缓存与热缓存命中情况；
- 本地开发、测试和 Storybook 体验；
- CI 配置与远程缓存收益；
- 构建结果的一致性；
- 自定义规则数量和后续维护成本；
- 与现有发布流程的兼容性。

只有试点在正确性、性能、开发体验和维护成本上达到或超过现有 Bun/Nx 方案，才逐步迁移其他前端项目。在迁移达到功能等价之前，原生前端命令必须保持可用。

## 定期复查

以下事件发生时，应重新评估现代前端支持：

- Buck2 发布包含 JavaScript、TypeScript、Node、Bun 或 Web 构建相关的重要能力；
- Buck2 官方 prelude 增加通用 Web 前端规则或 hermetic Node/Bun toolchain；
- Angular、Nx 或 Bun 官方提供 Buck2 集成；
- 出现维护活跃、接口稳定且具备真实生产用户的社区规则集；
- 当前 Bun/Nx 方案遇到 Buck2 能明确解决的跨项目构建或缓存瓶颈。

复查结论应更新到本文档，并附上试验仓库、基准数据或上游文档。迁移决定以可验证的工程收益为依据，而不是为了形式上的“全部使用 Buck2”。

## 参考资料

- [Buck2：Toolchain](https://buck2.build/docs/concepts/toolchain/)
- [Buck2：Writing Toolchains](https://buck2.build/docs/rule_authors/writing_toolchains/)
- [Buck2 Prelude：JavaScript rules](https://github.com/facebook/buck2/tree/main/prelude/js)
