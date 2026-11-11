#!/usr/bin/env python3
"""生成 DSH-FLScratch 品牌图：SVG（矢量源）→ WebP（站点用）。

依赖：fontTools（uv venv 里装）、rsvg-convert、Pillow。

设计参数（与泰拉面板 wordmark 对齐）：
- 画布高 204px，图标高 200px、左边距 2px；
- 文字用 Google Sans Flex Regular（可变字体实例化 wght=400），cap height 119px；
- 画布宽度按文字实际墨迹右边缘收紧，右侧不留空白。

用法：uv run --with fonttools --with pillow python generate-logo.py
"""
import subprocess
from pathlib import Path

from PIL import Image
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont

HERE = Path(__file__).resolve().parent
DOCS_PUBLIC = HERE.parent.parent / "Docs" / "Products" / "DSH-FLScratch" / "docs" / "public"
NAV_ASSET = HERE.parent.parent / "Docs" / "Products" / "shared" / "theme" / "nav-icon" / "assets" / "dsh-flscratch.webp"
FONT = Path.home() / ".local/share/fonts/GoogleSansFlex-VariableFont_GRAD,ROND,opsz,slnt,wdth,wght.ttf"
WHALE_SRC = HERE / "鲸娘女仆举着绿旗_透明背景.png"
WHALE_PNG = HERE / "dsh-flscratch-whale.png"

TEXT = "DSH-FLScratch"
CANVAS_H = 204
ICON_H = 200
ICON_X = 2
GAP = 74
TEXT_CAP_H = 119
TEXT_BASELINE = 53 + TEXT_CAP_H  # cap 顶在 y=53


def prepare_whale() -> Image.Image:
    whale = Image.open(WHALE_SRC).convert("RGBA")
    whale = whale.crop(whale.getbbox())
    h = ICON_H * 2  # 2x 供 SVG 以 200px 显示
    w = round(whale.width * h / whale.height)
    whale = whale.resize((w, h), Image.LANCZOS)
    whale.save(WHALE_PNG, optimize=True)
    return whale


def text_paths() -> tuple[list[tuple[str, float]], float, float]:
    """返回 [(path_d, cursor_units)]、单位/px 缩放、总 advance。"""
    font = TTFont(FONT)
    font = instantiateVariableFont(font, {"wght": 400}, inplace=False)
    glyph_set = font.getGlyphSet()
    cmap = font.getBestCmap()
    units = font["head"].unitsPerEm
    scale = TEXT_CAP_H / font["OS/2"].sCapHeight  # px per font unit
    cursor = 0.0
    paths: list[tuple[str, float]] = []
    for ch in TEXT:
        name = cmap[ord(ch)]
        pen = SVGPathPen(glyph_set)
        glyph_set[name].draw(pen)
        paths.append((pen.getCommands(), cursor))
        cursor += glyph_set[name].width
    return paths, scale, cursor


def build_svg(width: float, color: str, whale: Image.Image) -> str:
    paths, scale, _advance = text_paths()
    icon_w = ICON_H * whale.width / whale.height
    body = "\n".join(
        f'    <path transform="translate({x:.1f} 0)" d="{d}"/>' for d, x in paths
    )
    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="{width:.0f}" height="{CANVAS_H}" viewBox="0 0 {width:.1f} {CANVAS_H}">
  <!-- 品牌资源源文件：由 generate-logo.py 生成，文字已转路径，不依赖本机字体 -->
  <image href="dsh-flscratch-whale.png" x="{ICON_X}" y="2" width="{icon_w:.1f}" height="{ICON_H}"/>
  <g transform="translate({ICON_X + icon_w + GAP:.1f} {TEXT_BASELINE}) scale({scale:.6f} -{scale:.6f})" fill="{color}">
{body}
  </g>
</svg>
"""


def build_icon_svg(whale: Image.Image) -> str:
    side = 512
    pad = round(side * 0.06)
    inner = side - 2 * pad
    w = inner * whale.width / whale.height
    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="{side}" height="{side}" viewBox="0 0 {side} {side}">
  <!-- 品牌图标：由 generate-logo.py 生成 -->
  <image href="dsh-flscratch-whale.png" x="{(side - w) / 2:.1f}" y="{pad}" width="{w:.1f}" height="{inner}"/>
</svg>
"""


def render(svg: Path, png: Path) -> None:
    subprocess.run(["rsvg-convert", "-o", str(png), str(svg)], check=True)


def main() -> None:
    whale = prepare_whale()
    icon_w = ICON_H * whale.width / whale.height
    text_left = ICON_X + icon_w + GAP
    _, scale, advance = text_paths()
    rough_w = text_left + advance * scale + 8

    DOCS_PUBLIC.mkdir(parents=True, exist_ok=True)

    # 先给宽一点画布渲染，量出文字墨迹右边缘，再把画布收紧到墨迹处
    tmp = HERE / "_measure.svg"
    for color, suffix in (("#000000", "light"), ("#ffffff", "dark")):
        tmp.write_text(build_svg(rough_w, color, whale))
        png = HERE / f"_measure-{suffix}.png"
        render(tmp, png)
        bbox = Image.open(png).getbbox()
        tight_w = bbox[2]  # 右边缘，无额外空白
        svg = HERE / f"DSHFLScratchLogo-{suffix}.svg"
        svg.write_text(build_svg(tight_w, color, whale))
        out_png = HERE / f"_DSHFLScratchLogo-{suffix}.png"
        render(svg, out_png)
        out = DOCS_PUBLIC / f"DSHFLScratchLogo-{suffix}.webp"
        Image.open(out_png).convert("RGBA").save(out, "WEBP", lossless=True, quality=100, exact=True)
        print(f"{svg.name}: {tight_w}x{CANVAS_H} -> {out.name}")

    icon_svg = HERE / "DSHFLScratchIcon.svg"
    icon_svg.write_text(build_icon_svg(whale))
    render(icon_svg, HERE / "_icon.png")
    Image.open(HERE / "_icon.png").convert("RGBA").save(
        DOCS_PUBLIC / "DSHFLScratchIcon.webp", "WEBP", lossless=True, quality=100, exact=True
    )
    icon = Image.open(HERE / "_icon.png").convert("RGBA").resize((256, 256), Image.LANCZOS)
    icon.save(NAV_ASSET, "WEBP", lossless=True, quality=100, exact=True)
    print("icon + nav asset done")

    for p in HERE.glob("_*"):
        p.unlink()


if __name__ == "__main__":
    main()
