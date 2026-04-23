# Packet 56 — V3 motion-path and offscreen-origin support

## Task first
Close the major V2→V3 motion gap by landing one bounded motion-path / from-to support slice aligned with the current V3 time model.

## Why this matters
The migration log still calls motion-path and offscreen `from`/`to` support a major unresolved gap. The active V3 time model is much clearer now, but geometry-aware entry/exit trajectories are still not fully expressed in the V3 runtime/schema path.

## Success condition
- one bounded motion-path / offscreen-origin slice lands
- the slice aligns with the current `phase_t` / `loop_t` / `absolute_t` timing model
- one representative motion-bearing recipe path is proven

## Mode
BLOCKER_MODE

## Task-scope paths for grounding
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md`
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-motion-spec.md`
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-vanishing-edge-spec.md`
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-schema-overview.md`
- `/usr/projects/tui-vfx/crates/tui-vfx-geometry/src/types/`
- the exact `tui-vfx-recipes/src/v3/` authoring/compile files touched by the chosen motion slice

## Exact write scope
- only the exact motion schema/runtime files needed for the chosen slice
- the smallest representative recipe/test/probe proof surfaces
- the narrowest rustdoc/doc updates needed to keep the time/motion contract consistent

## Out of scope
- a broad motion-system redesign
- unrelated scene/effect work
- whole-corpus recipe migration

## Must-read docs in order
1. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-motion-spec.md`
2. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-vanishing-edge-spec.md`
3. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md`
4. `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-schema-overview.md`
5. `/usr/projects/tui-vfx/steering/INTENTIONS.md`
6. `/usr/projects/gt-design/.omx/context/v3-managed-briefing-20260423T170500Z.md`
7. `/usr/projects/global_prompts/standards/40_ofpf_standards.md`
8. `/usr/projects/global_prompts/standards/50_tdd_protocol.md`

## Verification required
- focused motion-bearing tests or deterministic render proofs for the chosen slice
- explicit proof that the slice consumes the current V3 timing model correctly
- clear note on whether `motion_path`, `from/to`, `snapping`, or `edge_crossing` were advanced

## Task reminder
Your task is still: land one bounded motion-path/offscreen-origin slice, not solve every deferred motion design concern.
