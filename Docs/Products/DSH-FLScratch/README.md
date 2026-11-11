# DSH-FLScratch 文档站

把 DeepSeek Harness 当成增强版 Scratch IDE 的产品文档。结构与泰拉面板文档站一致：

- `docs/`：Markdown 内容（`index.md` 是首页，`_nav.json` 是顶部导航，`guide/_meta.json` 是侧边栏顺序）；
- 主题来自 `../shared/theme`（`rspress.config.ts` 里 `themeDir` 指向它，导航栏产品切换自动生效）；
- 品牌图（`docs/public/DSHFLScratch*.webp`）由 `DesignAssets/DSH-FLScratch/鲸娘女仆举着绿旗_透明背景.png` 与 Google Sans Flex SemiBold 合成，尺寸与泰拉面板 wordmark 对齐（204px 高、文本 cap height 119px）。

本地开发：

```bash
cd Docs/Products
mise exec -- pnpm install
mise exec -- pnpm dev:dsh-flscratch    # http://localhost:5173 默认端口由 rspress 决定
```
