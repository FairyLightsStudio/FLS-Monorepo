#!/usr/bin/env sh
set -eu
dir=$(cd "$(dirname "$0")/.." && pwd)
sed "s|__DSH_SCRATCH_DIR__|$dir|g" "$dir/cordis.yml.example" > "$dir/cordis.yml"
echo "$dir/cordis.yml"
