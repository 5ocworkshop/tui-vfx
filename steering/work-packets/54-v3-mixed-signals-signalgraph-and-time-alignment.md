# Packet 54 — V3 mixed-signals signal-graph and time alignment

## Task first
Resolve the next bounded mixed-signals/V3 contract seam for signal-graph shape and time vocabulary so V3 stops carrying tentative parallel assumptions.

## Why this matters
The migration log still flags signal-graph JSON shape as a major blocker, and the docs still describe a build-target wrapper form that needs alignment with upstream `mixed-signals`. At the same time, V3 timing now distinguishes `phase_t`, `loop_t`, and `absolute_t`, so the contract needs one explicit shared vocabulary instead of partial local conventions.

## Success condition
- one bounded contract seam between V3 and `mixed-signals` is resolved and documented in code/docs
- the chosen signal/time vocabulary is explicit and testable
- no broad cross-repo rewrite in one pass

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md`
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-schema-overview.md`
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-motion-spec.md`
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-spatial-field-hint-plan.md`
- `/usr/projects/mixed-signals/steering/INTENTIONS.md`
- the exact `mixed-signals` signal/time types touched by the chosen seam
- the exact `tui-vfx-recipes/src/v3/` or runtime files consuming that seam

## Exact write scope
- only the exact mixed-signals and V3 files needed for the chosen signal/time contract slice
- the smallest supporting tests and doc updates required to make the new contract explicit

## Out of scope
- all spatial leaves and field-hint delivery in one packet
- whole-engine timing rewrites
- broad recipe-corpus migration

## Must-read docs in order
1. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md`
2. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-schema-overview.md`
3. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-motion-spec.md`
4. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-spatial-field-hint-plan.md`
5. `/usr/projects/mixed-signals/steering/INTENTIONS.md`
6. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
7. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
8. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
9. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`

## Recommended first steps
1. Name the exact signal-graph/time seam before editing.
2. State which vocabulary becomes canonical (`phase_t`, `loop_t`, `absolute_t`, `clock`, etc.).
3. Prove one consumer path uses the contract correctly.
4. Document the cutover so downstream crates stop guessing.

## Verification required
- focused mixed-signals tests for the chosen seam
- focused V3/runtime tests proving the same seam is consumed correctly
- explicit doc/rustdoc update showing the canonical vocabulary

## Task reminder
Your task is still: resolve one bounded signal/time contract seam, not solve the entire mixed-signals/V3 integration in one jump.
