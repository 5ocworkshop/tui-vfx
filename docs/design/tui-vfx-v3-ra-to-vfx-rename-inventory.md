<!-- <FILE>docs/design/tui-vfx-v3-ra-to-vfx-rename-inventory.md</FILE> - <DESC>Working inventory for the Rust-side Ra→Vfx rename. Enumerates the wire-format type families slated to change under Decision 4 and notes where the remaining live references currently sit during cutover.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Chapter 40 Decision 4 explicitly adopts the Ra→Vfx rename but defers the exhaustive inventory to a follow-on planning pass. This file is that inventory seed so the rename can proceed methodically instead of by ad hoc grep sessions.</WCTX> -->
<!-- <CLOG>0.1.0: initial inventory. Captures the adopted rename targets from Chapter 40 and records the currently observed live-reference buckets in tui-vfx proper.</CLOG> -->

# tui-vfx V3 Ra→Vfx rename inventory

This document is a working inventory for the Rust-side rename adopted in:

- `docs/design/tui-vfx-v3-upgrade-plan/40_decisions.md`
  - **Decision 4: Rename `Ra*` prefix to `Vfx*`**

The goal is to make the rename executable in deliberate slices instead of
re-deriving the same grep output every time work resumes.

---

## 1. Adopted rename targets

Decision 4 explicitly names the following type families:

- `RaRecipeConfig` → `VfxRecipeConfig`
- `RaPipelineConfig` → `VfxPipelineConfig`
- `RaStylePipelineConfig` → `VfxStylePipelineConfig`
- `RaMaskConfig` → `VfxMaskConfig`
- `RaFilterConfig` → `VfxFilterConfig`
- `RaSamplerConfig` → `VfxSamplerConfig`
- `RaStyleEffect` → `VfxStyleEffect`
- `RaBaseStyle` → `VfxBaseStyle`
- `RaClock` → `VfxClock`
- `RaContinuousConfig` → `VfxContinuousConfig`
- `RaSceneConfig` → `VfxSceneConfig`
- `RaLifecycleConfig` → `VfxLifecycleConfig`
- `RaContentConfig` → `VfxContentConfig`

This list is the minimum committed surface. Other `Ra*` names discovered during
implementation should be added here as they are confirmed.

---

## 2. Current live-reference buckets in `tui-vfx` proper

At the time of this inventory seed, the remaining non-archive `Ra*` references
in `tui-vfx` proper are mostly:

### 2.1 Planning / design docs

- `docs/design/tui-vfx-v3-upgrade-plan/40_decisions.md`
- `docs/design/tui-vfx-v3-upgrade-plan/100_tooling_ci_migration.md`
- `docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md`
- `docs/design/tui-vfx-v3-upgrade-plan/80_open_questions.md`
- `docs/design/tui-vfx-v3-migration-findings-memo-claude.md`
- `docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md`

Most of these are expected because they describe either:

- the historical V2 surface
- the explicit rename decision itself
- open migration work

### 2.2 Library-side code comments / rustdocs

- `crates/tui-vfx-types/src/recipe_id.rs`
- `crates/tui-vfx-types/src/layer_id.rs`

These are live code comments that should track the V3 direction rather than
teaching stale names as the future target.

### 2.3 Working prompts / kickoff docs

- `docs/design/ai-native-tooling-kickoff-prompt.md`

These should talk about the V3 naming direction while still acknowledging the
cutover state when necessary.

---

## 3. Buckets that are intentionally historical

The following are expected to keep `Ra*` references because they are historical
archives or explicitly describe the V2 world:

- `docs/v2-spec-archive/**`
- archived migration snapshots / historical logs that are explicitly about V2

Those should not be mass-renamed.

---

## 4. Recommended execution order

The rename should proceed in this order:

1. **Live docs/comments/prompts in `tui-vfx` proper**
   - low risk
   - keeps the repo from teaching stale terminology

2. **`tui-vfx-recipes` public Rust type surface**
   - main wire-format rename event
   - strongest downstream impact

3. **Downstream consumers**
   - validator / probe / trace / demos
   - gt-design integration points

4. **Generated docs / authoring guides / prompts**
   - ensure the extracted/generated layer matches the renamed code

5. **Endgame cleanup**
   - remove cutover caveats that were temporarily needed while both naming
     worlds coexisted

---

## 5. Current rule during cutover

Until the full rename lands:

- new code should avoid introducing fresh `Ra*`-named public helpers
- docs should prefer `Vfx*` as the target naming direction
- legacy names can still be mentioned when the cutover state itself is the fact
  being described

That keeps the migration direction consistent without pretending the rename is
already fully complete.

<!-- <FILE>docs/design/tui-vfx-v3-ra-to-vfx-rename-inventory.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
