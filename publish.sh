#!/usr/bin/env bash
set -euo pipefail

CRATES=(
  tui-vfx-types
  tui-vfx-core-macros
  tui-vfx-debug
  tui-vfx-core
  tui-vfx-geometry
  tui-vfx-shadow
  tui-vfx-content
  tui-vfx-style
  tui-vfx-compositor
  tui-vfx
)

MAX_RETRIES=5
INITIAL_WAIT=30
RETRY_WAIT=120

for i in "${!CRATES[@]}"; do
  crate="${CRATES[$i]}"
  echo "=== Publishing $crate ($(( i + 1 ))/${#CRATES[@]}) ==="

  attempt=0
  while true; do
    output=$(cargo publish -p "$crate" 2>&1) && break
    if echo "$output" | grep -q "already exists"; then
      echo "  Already published — skipping."
      break
    fi
    attempt=$(( attempt + 1 ))
    if (( attempt >= MAX_RETRIES )); then
      echo "  FAILED after $MAX_RETRIES attempts. Aborting."
      echo "$output"
      exit 1
    fi
    echo "  Attempt $attempt failed — waiting ${RETRY_WAIT}s before retry..."
    sleep "$RETRY_WAIT"
  done

  if (( i < ${#CRATES[@]} - 1 )); then
    echo "  Waiting ${INITIAL_WAIT}s for crates.io index..."
    sleep "$INITIAL_WAIT"
  fi
done

echo ""
echo "All crates published."
