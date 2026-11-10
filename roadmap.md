# FairyLightsStudio 构建系统路线图

> 状态：当前决策
> 最近更新：2026-11-10

## 目标

FairyLightsStudio 使用 Bazel 统一工作室项目的构建入口、跨项目依赖关系和适合纳入构建图的工具链。构建工具应尽可能由 Bazel 的 hermetic toolchain 管理，从而减少开发者本机环境差异，并为可重复构建、增量构建和远程缓存奠定基础。

但是，“Bazel 能够执行某条命令”并不意味着该生态已经得到 Bazel 的成熟支持。对缺少可靠规则、工具链和依赖模型的生态强行套用通用命令规则，只会把原有构建系统隐藏在 Bazel 后面，同时引入两套依赖图和更高的维护成本。

因此，本路线图采用渐进式纳管策略：成熟且适合 Bazel 的构建流程由 Bazel 管理；暂时缺少一等支持的现代前端流程继续使用其原生工具链，待迁移条件满足后再纳入 Bazel。

## 当前职责边界

| 范围 | 当前负责人 |
| --- | --- |
| 稀疏工作树和项目依赖闭包 | `fls` 消费 Bazel 项目图 |
| `fls` 与 Bazel 的版本和首次安装 | mise |
| Rust 与 proto codegen 构建图 | Bazel（`rules_rust` + 各工作区 `Cargo.lock` 的 crate_universe） |
| 当前 Rust 编译器入口 | mise 固定版本，经 Bazel toolchain 桥接 |
| Angular、Nx、Storybook 等现代前端工作流 | 原生工具链（Bun / npm） |
| 前端依赖解析和版本锁定 | `package.json` 与 `bun.lock` / `pnpm-lock.yaml` |
| 前端项目与其他工作室项目之间的粗粒度依赖 | Bazel `fls_project` 项目边界标记 |
| 暂时无法由 Bazel 合理接管的前端入口工具版本 | mise |

mise 只承担启动 Bazel 所必需的工具，以及 Bazel 当前无法合理接管的生态入口工具。只要构建工具已经由 Bazel hermetic toolchain 提供，就不应再通过 mise 重复安装和维护同一套版本。

## 为什么现代前端暂不接入 Bazel

2026-11 用 `aspect_rules_js 3.6.0` + `rules_nodejs 6.7.7` + Bazel 9.3.1 对 `Docs/Products/TeraPanel`（Rspress 2.1.1）做过 PoC（独立工程，未进本仓库）：

- **能构建**：pnpm 锁文件转换、hermetic Node 24.18.0 toolchain、原生 `@rspack/binding` 依赖都工作；关闭沙箱时能产出完整静态站（7 个页面）。
- **默认 sandbox 下稳定失败**：Rspress 的 ESM 入口在 runfiles 里触发 `@rspack/core` loader 的 `JsLoaderContext._module` 类型错误。这与 rules_js 官方已知问题 [#362 "ESM imports escape the sandbox & runfiles"](https://github.com/aspect-build/rules_js/issues/362)（仍 OPEN）吻合。
- 因此唯一可用形态必须关闭沙箱（`--spawn_strategy=local` 或 `tags = ["no-sandbox"]`），等于放弃 hermetic、远程缓存和输入精确追踪——而这正是接入 Bazel 的主要收益。
- 其它成本：`pnpm-workspace.yaml` 必须声明 `allowBuilds`；`js_binary` 不能直接指向包内 bin，需要额外的 launcher 脚本；`rspress dev`（开发服务器 + HMR）仍应保留原生 `pnpm dev`。

此外，文档站是叶子节点，原生构建在 1 秒级（7 页），CI 缓存收益很低。

**结论**：现代前端继续使用原生工具链。等 rules_js #362 关闭或 Rspack 能在默认 sandbox 下构建后，再按下面的触发条件重新评估；PoC 配置已记录在本文末尾。

## 当前前端方案

现代前端项目继续使用其原生工作流，例如：

```bash
bun install --frozen-lockfile
bun nx build <project>
bun nx test <project>
```

文档站（`Docs/Products`）使用 pnpm + Rspress：

```bash
cd Docs/Products && mise exec -- pnpm install && mise exec -- pnpm build
```

前端项目在仓库根用轻量 Bazel 标记声明与其他项目的粗粒度依赖。该标记只服务于工作区依赖发现，不尝试取代 Nx 的内部项目图，也不负责执行 Angular 构建：

```python
fls_project(
    name = "workspace",
    deps = [
        "//SomeOtherProject:workspace",
    ],
)
```

`fls add` 会让 Bazel 扫描所选项目目录下的全部 targets，并从查询结果计算跨项目依赖闭包；前端开发者仍然使用受上游支持的 Bun、Nx、Angular 和 Storybook 命令。`//Project:workspace` 是稳定的项目边界和粗粒度依赖桥梁，不代表“构建整个项目”。

## fls 的构建与发布边界

`fls` 自身的规范构建和测试入口是 Bazel：

```bash
bazel build //tools/fls:fls
bazel test //tools/fls:fls-test
```

Rust 版本由 mise 固定（1.97.1），Bazel 通过 `rules_rust` toolchain 使用同一版本。

GitHub Release 需要同时产出九个 host triple，其中包含 GNU/MUSL 和 RISC-V 交叉编译。发布阶段暂时使用 Cargo/cross 作为纯打包桥梁，但必须先通过 Bazel 测试，且两条路径编译同一份 Rust 源码和锁文件。等 Bazel 的 Rust 交叉工具链能够稳定覆盖完整发布矩阵后，应删除 Cargo/cross 例外，让发布资产也完全由 Bazel 产生。

## 重新评估前端接入 Bazel 的触发条件

前端迁移不设定主观日期。以下任一条件出现时，启动正式评估：

1. rules_js #362（ESM 逃逸 sandbox/runfiles）关闭，且 Rspack/Rspress 能在 Bazel 默认 sandbox 下完成构建；
2. rules_js 或 Bazel 官方提供可用的 ESM 前端构建规则，不需要关闭沙箱；
3. 前端构建时长或 CI 成本上升到原生工具链明显吃力的程度；
4. Angular、Nx 或 Storybook 出现稳定的 Bazel 官方/社区集成。

满足条件不代表立即迁移。应先选择一个规模较小的前端包做试点，比较首次/增量构建耗时、缓存命中、开发体验、CI 收益、结果一致性与维护成本；只有试点在正确性、性能、开发体验和维护成本上达到或超过现有方案，才逐步迁移其他前端项目。在迁移达到功能等价之前，原生前端命令必须保持可用。

## 定期复查

以下事件发生时，应重新评估现代前端支持：

- rules_js / rules_ts 发布重要版本，或上述 issue 关闭；
- Bazel 官方或 rules_js 增加通用 Web 前端构建规则；
- Angular、Nx 或 Bun 官方提供 Bazel 集成；
- 出现维护活跃、接口稳定且具备真实生产用户的社区规则集；
- 当前原生前端方案遇到 Bazel 能明确解决的跨项目构建或缓存瓶颈。

复查结论应更新到本文档，并附上试验仓库、基准数据或上游文档。迁移决定以可验证的工程收益为依据，而不是为了形式上的“全部使用 Bazel”。

## 参考资料

- [aspect rules_js](https://github.com/aspect-build/rules_js)
- [rules_js #362：ESM imports escape the sandbox & runfiles（OPEN）](https://github.com/aspect-build/rules_js/issues/362)
- [rules_js：pnpm 支持](https://github.com/aspect-build/rules_js/blob/main/docs/pnpm.md)
- [pnpm-lock.yaml 的两文档格式](https://pnpm.io/lockfile)
- [Rspress](https://rspress.dev/)
- [Bazel：Bzlmod](https://bazel.build/external/module)
