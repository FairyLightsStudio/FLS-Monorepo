#!/usr/bin/env bash
# onboarding.sh — Buck2 + Reindeer 新项目一键配置脚本
#
# 用法：在项目根目录运行 ./onboarding.sh
# 脚本会检查 Buck2、Reindeer、Rust 工具链是否安装，然后自动初始化 third-party 依赖。
#
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

banner() {
    echo -e "${CYAN}${BOLD}"
    echo "╔════════════════════════════════════════╗"
    echo "║   FlightStudio Buck2 环境初始化       ║"
    echo "╚════════════════════════════════════════╝"
    echo -e "${NC}"
}

info()    { echo -e "${GREEN}[✓]${NC} $1"; }
warn()    { echo -e "${YELLOW}[!]${NC} $1"; }
fail()    { echo -e "${RED}[✗]${NC} $1"; exit 1; }
step()    { echo -e "\n${BOLD}→ $1${NC}"; }

check_cmd() {
    if command -v "$1" &> /dev/null; then
        info "$1: $(command -v "$1")"
        return 0
    else
        fail "$1 未安装，请先安装 $1"
        return 1
    fi
}

# ──────────────────────────────────────────────
banner

# ── Step 1: 检查 Buck2 ───────────────────────
step "第一步：Buck2"

if check_cmd buck2; then
    BUCK2_VERSION=$(buck2 --version 2>&1 || true)
    echo "  ${BUCK2_VERSION}"
fi

# ── Step 2: 检查 Reindeer ────────────────────
step "第二步：Reindeer"

if check_cmd reindeer; then
    REINDEER_VERSION=$(reindeer --version 2>&1 || true)
    echo "  ${REINDEER_VERSION}"
fi

# ── Step 3: 检查 Rust 工具链 ─────────────────
step "第三步：Rust 工具链"

if ! check_cmd rustc; then
    fail "请安装 Rust: https://rustup.rs"
fi
echo "  $(rustc --version)"

if ! check_cmd cargo; then
    fail "请安装 Cargo (通常随 Rust 一起安装)"
fi
echo "  $(cargo --version)"

# ── Step 4: 确认 third-party 目录 ────────────
step "第四步：确认 third-party 目录"

THIRD_PARTY_DIR="third-party/rust"
if [ ! -f "$THIRD_PARTY_DIR/reindeer.toml" ]; then
  fail "找不到 $THIRD_PARTY_DIR/reindeer.toml，请在项目根目录运行本脚本"
fi
info "找到 $THIRD_PARTY_DIR/reindeer.toml"

# ── Step 5: vendor ───────────────────────────
step "第五步：下载第三方依赖源码（vendor）"

# 清理旧的 Cargo.lock 确保重新解析
if [ -f "$THIRD_PARTY_DIR/Cargo.lock" ]; then
    rm "$THIRD_PARTY_DIR/Cargo.lock"
fi

echo "  正在运行 reindeer vendor ...（可能需要几分钟）"
if reindeer --third-party-dir="$THIRD_PARTY_DIR" vendor; then
    info "vendor 完成"
else
    fail "vendor 失败，请检查上方错误信息"
fi

# ── Step 6: 补 cargo config ───────────────────
step "第六步：补 cargo config（rsproxy 兼容）"

LOCAL_CONFIG="$THIRD_PARTY_DIR/.cargo/config.toml"

# `reindeer vendor` always regenerates .cargo/config.toml from scratch,
# only writing the crates-io → vendored-sources mapping. If the user's
# global Cargo config redirects crates-io to an rsproxy mirror (common
# in mainland China), the global replace-with takes precedence over the
# local one, causing buckify's offline metadata resolution to fail.
# The override below is harmless for direct crates.io users — if no
# redirect chain points to rsproxy-sparse, this section is dead code.
# If a chain does point to it, it correctly routes everything to vendor.
if ! grep -q 'rsproxy-sparse' "$LOCAL_CONFIG" 2>/dev/null; then
    cat >> "$LOCAL_CONFIG" << 'EOF'

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
replace-with = "vendored-sources"
EOF
    info "已追加 rsproxy-sparse 覆盖到 $LOCAL_CONFIG"
else
    info "rsproxy-sparse 覆盖已存在，跳过"
fi

# ── Step 7: buckify ──────────────────────────
step "第七步：生成 BUCK 文件（buckify）"

echo "  正在运行 reindeer buckify ..."
if reindeer --third-party-dir="$THIRD_PARTY_DIR" buckify; then
    info "buckify 完成"
else
    fail "buckify 失败，请检查上方错误信息"
fi

# ── Step 8: 构建验证 ─────────────────────────
step "第八步：构建验证"

echo "  正在构建 //$THIRD_PARTY_DIR:tokio ..."
if buck2 build "//$THIRD_PARTY_DIR:tokio" 2>&1 | tail -5; then
    info "tokio 构建通过"
else
    warn "tokio 构建失败，请检查上方错误信息"
fi

# ── 完成 ─────────────────────────────────────
echo
echo -e "${GREEN}${BOLD}========================================${NC}"
echo -e "${GREEN}${BOLD}  🎉 环境配置完成！${NC}"
echo -e "${GREEN}${BOLD}========================================${NC}"
echo
echo "  运行以下命令试试你的第一个 Buck2 构建："
echo
echo -e "    ${CYAN}buck2 build //third-party/rust:tokio${NC}"
echo
echo "  以后加新依赖的流程："
echo "    1. 编辑 $THIRD_PARTY_DIR/Cargo.toml"
echo "    2. 运行 ./onboarding.sh"
echo
