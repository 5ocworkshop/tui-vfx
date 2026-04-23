# Packet 15 — generated docs freshness gate hardening

## Objective
Harden the V3 generated-docs freshness gate so future drift is easier to detect and diagnose.

## Why this matters
A freshness gate that fails opaquely or misses common drift patterns will decay in value over time.

## Mode
BLOCKER_MODE

## Success condition
- the gate failure mode is clearer and/or the generator catches the currently known drift classes more reliably
- no runtime or schema behavior changes beyond docs/tooling surfaces

## In scope
- `/usr/projects/tui-vfx-recipes/tools/fnc_generate_v3_docs.py`
- `justfile` or docs check entrypoint if directly involved
- `/usr/projects/tui-vfx-recipes/docs/generated/` only if regeneration is required

## Out of scope
- runtime/library code
- broad V3 docs content rewrites
- recipe changes

## Recommended first steps
1. Reproduce a freshness check locally.
2. Inspect how the generator discovers public V3 surfaces.
3. Identify one concrete weakness in failure diagnosis or coverage.
4. Make the smallest tooling-only improvement.

## Verification required
- `python3 tools/fnc_generate_v3_docs.py --write`
- `just docs-v3-check`
- any focused generator tests if they exist

## Reporting format
Report:
- what was weak before
- what improved
- exact files changed
- exact commands run
- any remaining blind spot in the generator

## Task reminder
Your task is still: harden docs freshness tooling, not redesign V3 docs architecture.
