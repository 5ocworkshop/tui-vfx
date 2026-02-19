#!/usr/bin/env bash
set -euo pipefail

CRATES=(
#  tui-vfx-types
#  tui-vfx-core-macros
#  tui-vfx-debug
#  tui-vfx-core
#  tui-vfx-geometry
#  tui-vfx-shadow
#  tui-vfx-content
#  tui-vfx-style
#  tui-vfx-compositor
  tui-vfx
)

for i in "${!CRATES[@]}"; do
  crate="${CRATES[$i]}"
  echo "=== Publishing $crate ($(( i + 1 ))/${#CRATES[@]}) ==="
  cargo publish -p "$crate"
  if (( i < ${#CRATES[@]} - 1 )); then
    echo "  Waiting 30s for crates.io index..."
    sleep 30
  fi
done

echo ""
echo "All crates published."
