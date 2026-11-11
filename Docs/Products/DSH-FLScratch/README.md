# DSH-FLScratch 文档站

结构与泰拉面板文档站一致：

- `docs/`：Markdown 内容（`index.md` 是首页，`_nav.json` 是顶部导航，`guide/_meta.json` 是侧边栏顺序）；
- 主题来自 `../shared/theme`（`rspress.config.ts` 里 `themeDir` 指向它，导航栏产品切换自动生效）；
- 品牌图源文件在 `DesignAssets/DSH-FLScratch/`：`DSHFLScratchLogo-{light,dark}.svg`（文字已转路径，不依赖本机字体）+ `DSHFLScratchIcon.svg` + 裁好的 `dsh-flscratch-whale.png`；`generate-logo.py` 负责生成这些 SVG 并用 `rsvg-convert` 转成 `docs/public/*.webp`。参数与泰拉面板 wordmark 对齐（204px 高、文本 cap height 119px、Google Sans Flex **Regular**），画布宽度按文字墨迹右边缘收紧，右侧不留空白。
- 重新生成：`uv run --with fonttools --with pillow python generate-logo.py`（需要 `rsvg-convert`；WebP 保存加了 `exact=True`，避免透明像素被塞进 RGB 导致预览发绿）。

本地开发：

```bash
cd Docs/Products
mise exec -- pnpm install
mise exec -- pnpm dev:dsh-flscratch    # http://localhost:5173 默认端口由 rspress 决定
```
