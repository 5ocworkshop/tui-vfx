<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-probe-fidelity-bisect.md</FILE> - <DESC>Junior-ready execution packet for handoff §8.7 — bisect and fix the probe-path fidelity regression in tui-vfx-recipes that returns modified_cells=0 for canonical schema_v1 recipes whose shaders are known-good. Captures the symptom, suspect commit window, hypothesis enumeration with the role-map cutover as the leading candidate, fix-shape options keyed off bisect outcome, and acceptance criteria including a regression test that pins the contract.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Restore the truthful-probe contract before the observability bus reaches its parity-test phase. Schema_v1 probe path returns modified_cells=0 for recipes targeting Role(Text); the probe wraps a no-info RoleMap::all_background source-roles map around the destination scene at run_probe entry, so any role-scoped scope matches zero cells regardless of shader correctness.</WCTX> -->
<!-- <CLOG>1.0.0: initial packet — pre-flight, current-state audit of the probe pipeline (build_probe_scene_from_recipe, build_unified_probe_report_from_recipe, run_probe, build_probe_spec_from_preview, strip_spatial_effects), bisect protocol with 2026-04-14 a90c51f as the known-good anchor and today as bad, hypothesis ranking with H1 (the A.2 role-aware run_probe cutover at ec872a4) as the leading candidate to test first, fix-shape options per cause class, acceptance criteria with a binding regression test against btop_focused_row_demo.json, sequencing with the observability bus parity-test phase.</CLOG> -->

# Packet — Probe-path fidelity bisect (handoff §8.7)

> **Source finding.** `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md` v1.3.0 §8.7 ("Item 9 — Probe-path fidelity regression").
>
> **Companion.** `docs/design/tui-vfx-pipeline-observability.md` v0.2.0 — the observability bus's parity-test phase needs a working probe to A/B against. Landing this packet before the bus's US-009 (bench-gate verification) is ideal but not strictly required.
>
> **Related findings to keep distinct.** §8.6 (the focused_row_btop role-map mismatch in production) and §8.9 (legacy `region: TextOnly` lowering to `Role(Text)`) are adjacent bugs in the *production* render path, not the probe path. The probe regression has the same flavor (role-map mismatch) but a different surface (probe always renders with all-Background source roles, regardless of producer). The fix here lives in the probe pipeline, not in the production lowering.

---

## Goal & motivation

Restore the truthful-probe contract. The probe currently returns `modified_cells: 0` for `recipes/btop_focused_row_demo.json` despite `shader_count: 1` and a recipe DF-010 (closed 2026-04-14) demonstrated as actively producing per-cell color changes. The regression appeared between DF-010's close and 2026-04-26.

When the probe lies about whether a recipe modifies cells, the entire validator workflow loses signal. "Probe says 0" no longer means "shader broken"; it means "either shader broken OR probe regressed, indistinguishable from the CLI." The DF-010-style investigation pattern — diff a probe before and after a code change, infer cause from the delta — is degraded.

The observability bus design (`tui-vfx-pipeline-observability.md`) calls for a parity-test phase that asserts the bus's view of the pipeline matches the probe's. With a broken probe, parity tests test nothing.

This packet does not execute the bisect itself. It captures symptom, suspect window, fix-shape options, and acceptance criteria so the executor can land the fix in a focused half-session.

## Scope

**In scope.**

- The schema_v1 probe path. Specifically `src/probe/fnc_build_probe_scene_from_recipe.rs`, `src/probe/fnc_build_unified_probe_report_from_recipe.rs`, the surrounding `strip_*` helpers, and the upstream `crates/tui-vfx-probe/src/orc_run_probe.rs::run_probe` they hand off to.
- The reproducer recipe `recipes/btop_focused_row_demo.json` and its sibling `themes/eichler/recipes/focused_row_btop.json` (lives in `gt-design`, not `tui-vfx-recipes`; the recipe schema is identical).
- A regression test pinning a non-zero `modified_cells` count for the canonical reference recipe.

**Out of scope.**

- The compiled-V3 direct-bridge probe path (`src/v3/compile/fnc_build_probe_scene_spec_from_compiled_plan.rs`). The handoff frames this path as "appears unaffected." See §Risks & gotchas — current evidence says both paths route through the same `run_probe`, so the same role-map placeholder applies. If the bisect lands the fix at the `run_probe` level, both paths benefit; if the fix lands in `build_probe_scene_from_recipe`, the V3 builder needs the same treatment in a follow-on packet. Confirm during the bisect.
- The validator's CLI surface (`tools/pipeline-validator/`, `tools/recipe-probe/`). The CLI plumbing parses arguments correctly; only the per-cell render fidelity broke.
- The §8.6 production-path role-map mismatch (`ContentShell::card` tagging every cell with `Surface`/`Background`). Same flavor, different surface, separate fix.
- The §8.9 legacy `region: TextOnly` lowering. The reproducer recipe uses `region: "TextOnly"`; this is the input to the probe regression but is not the regression itself. §8.9 is deferred to the V3 cutover.

**Crates touched (anticipated, pending bisect).**

- `tui-vfx` workspace — `crates/tui-vfx-probe/src/orc_run_probe.rs` (top hypothesis lives here).
- `tui-vfx-recipes` workspace — `src/probe/fnc_build_probe_scene_from_recipe.rs`, `src/probe/fnc_build_unified_probe_report_from_recipe.rs`, possibly the V3 builder.
- New regression test — `tools/recipe-probe/tests/` or `crates/tui-vfx-recipes/src/probe/test_*.rs`.

## Pre-work checklist

```bash
# Daemon health.
ofpf-status

# Load both workspaces (the probe runs across them).
ofpf-load --root /usr/projects/tui-vfx-recipes
ofpf-load --root /usr/projects/tui-vfx

# Re-read the source finding.
sed -n '234,246p' /usr/projects/tui-vfx/docs/design/tui-vfx-2026-04-26-handoff-outstanding.md

# Read the observability companion (parity-test phase needs this fix).
grep -n "probe\|parity" /usr/projects/tui-vfx/docs/design/tui-vfx-pipeline-observability.md | head -20

# Confirm the reproducer recipe still uses region: TextOnly (it lowers to Role(Text);
# this is what makes the regression visible — coordinate-scoped recipes hide the bug).
python3 -c "import json; d=json.load(open('/usr/projects/tui-vfx-recipes/recipes/btop_focused_row_demo.json')); print(d['config']['pipeline']['style']['region'])"
# Expect: TextOnly

# Confirm today's symptom before doing anything else.
cd /usr/projects/tui-vfx-recipes
cargo run -q -p recipe-probe -- /usr/projects/tui-vfx-recipes/recipes/btop_focused_row_demo.json \
  --phase dwelling --sample-t 1.0 --with-causation --widget-cell 4,3 \
  | grep -E "modified_cells|shader_count|shader_effects"
# Expect: shader_count: 1, shader_effects: ["FocusedRowGradient#1"], modified_cells: 0

# OFPF inspect every file in the audit. ofpf-inspect mandatory before edits.
ofpf-inspect crates/tui-vfx-recipes/src/probe/fnc_build_probe_scene_from_recipe.rs
ofpf-inspect crates/tui-vfx-recipes/src/probe/fnc_build_unified_probe_report_from_recipe.rs
ofpf-inspect crates/tui-vfx-recipes/src/probe/fnc_strip_spatial_effects.rs
ofpf-inspect crates/tui-vfx-recipes/src/probe/fnc_build_probe_spec_from_preview.rs
ofpf-inspect crates/tui-vfx-probe/src/orc_run_probe.rs

# Find probe entry call sites (sanity check the CLI hands off to the unified report builder).
grep -rn "build_unified_probe_report_from_recipe\|build_probe_scene_from_recipe" \
  /usr/projects/tui-vfx-recipes --include="*.rs" | head -10
```

## Bisect protocol

The bisect is mechanical. The executor automates it with the reproducer.

**Bisect bounds.**

- **Good (anchor):** the last commit dated 2026-04-14 or earlier where the reproducer shows `modified_cells > 0`. Candidate: `a90c51f Explain dynamic recipe bugs at parameter and cell level` (2026-04-14, the day DF-010 closed). Verify by checking out the commit and running the reproducer; if it returns `modified_cells > 0`, lock it in as good.
- **Bad (current):** today's HEAD on `master` in both `tui-vfx` and `tui-vfx-recipes`. The reproducer above already confirmed `modified_cells: 0`.

**Identify the bisect window in `tui-vfx-recipes`.**

```bash
cd /usr/projects/tui-vfx-recipes
git log --since="2026-04-13" --until="2026-04-27" --oneline -- \
  src/probe/ tools/recipe-probe/ tools/pipeline-validator/
```

The window has roughly 50 commits on the recipes side. The probe also depends on `tui-vfx-probe` and `tui-vfx-compositor` in the `tui-vfx` workspace; surface that history too:

```bash
cd /usr/projects/tui-vfx
git log --pretty=format:"%h %ad %s" --date=short --since="2026-04-13" --until="2026-04-27" -- \
  crates/tui-vfx-probe/ crates/tui-vfx-compositor/src/pipeline/
```

**Run the bisect.**

```bash
cd /usr/projects/tui-vfx-recipes
git bisect start
git bisect bad HEAD
git bisect good <known-good-sha>
# Automate with the reproducer:
git bisect run bash -c '
  cd /usr/projects/tui-vfx-recipes &&
  cargo build -p recipe-probe --quiet 2>/dev/null || exit 125;
  out=$(cargo run -q -p recipe-probe -- /usr/projects/tui-vfx-recipes/recipes/btop_focused_row_demo.json \
    --phase dwelling --sample-t 1.0 --with-causation --widget-cell 4,3 2>/dev/null \
    | grep "modified_cells" | head -1);
  echo "$out" | grep -q ": 0," && exit 1 || exit 0;
'
```

Note the exit code semantics: `git bisect run` treats exit 0 as good, exit 1 as bad, exit 125 as skip. Build failures must skip rather than fail-as-bad to avoid blaming an unrelated transient breakage. The `cargo build` line above bails with 125 if the build is broken at that revision.

**Cross-workspace caveat.** If the bisect lands on a commit in `tui-vfx-recipes` that depends on a breaking change in `tui-vfx`, the reproducer fails to build and the bisect skips. Watch for a window of skipped revisions; if the bisect can't isolate, the regression may live in the `tui-vfx` workspace (most likely candidate: the A.2 role-aware compositor cutover at `ec872a4`, dated 2026-04-20). Run a parallel bisect against `tui-vfx` if needed:

```bash
cd /usr/projects/tui-vfx
git bisect start
git bisect bad HEAD
git bisect good <pre-A.2-sha-circa-2026-04-19>
git bisect run bash -c '
  cd /usr/projects/tui-vfx-recipes &&
  cargo build -p recipe-probe --quiet 2>/dev/null || exit 125;
  out=$(cargo run -q -p recipe-probe -- /usr/projects/tui-vfx-recipes/recipes/btop_focused_row_demo.json \
    --phase dwelling --sample-t 1.0 --with-causation --widget-cell 4,3 2>/dev/null \
    | grep "modified_cells" | head -1);
  echo "$out" | grep -q ": 0," && exit 1 || exit 0;
'
```

**Suspect commits to verify first.** Per §Hypothesis enumeration the order is H1, then H4, then H2/H3.

- `tui-vfx@ec872a4` (2026-04-20, "Sub-plan A Phase A.2 — compositor + StyleRegion hard cutover") — H1 candidate. The commit's CLOG on `crates/tui-vfx-probe/src/orc_run_probe.rs:0.7.0` reads: "MINOR — migrate call to the new `render_pipeline_with_spec` signature. Source roles default to `RoleMap::all_background(w, h)` (probe has no semantic info)." Strongest single candidate. **Verify before bisecting.**
- `tui-vfx-recipes@4326395` (2026-04-26, "Wire loopback merge into probe scene builders (L5 follow-on)") — the §8.7 commentary flagged this as structurally suspicious because it touches the probe builder. Confirmed shape: 4 added lines on the legacy schema_v1 path (just `loopback_fired_keys: Vec::new()`), 47 lines on the V3 path. Unlikely to be the cause but verify quickly by checking out `4326395^` and re-running the reproducer.

## Current-state audit

Captured 2026-04-26 from manual reads. The librarian was loaded against `tui-vfx`; `ofpf-inspect` should be re-run against `tui-vfx-recipes` after `ofpf-load --root /usr/projects/tui-vfx-recipes`.

### The schema_v1 probe pipeline

The CLI dispatches via `tools/recipe-probe/src/main.rs:run()` at line 91. For schema_v1 recipes (the reproducer is one), control reaches `run_legacy_probe_mode` at line 119, which calls `build_unified_probe_report_from_recipe`.

`crates/tui-vfx-recipes/src/probe/fnc_build_unified_probe_report_from_recipe.rs:26` (193 LOC) constructs three reference grids (`base_grid`, `content_grid`, `styled_grid`) by progressively un-stripping effects from `preview_item`, then calls `build_probe_scene_from_recipe(config, scene_request)?` and hands the resulting `ProbeSceneSpec` to `tui_vfx_probe::run_probe`.

`crates/tui-vfx-recipes/src/probe/fnc_build_probe_scene_from_recipe.rs:22` (94 LOC, 0.3.0) constructs the `ProbeSceneSpec` with:

- `source` — rendered from `strip_spatial_effects(preview_item)` (shaders / spatial style layers stripped, content preserved).
- `destination` — an empty buffer optionally canvas-filled.
- `composition` — built from `build_probe_spec_from_preview(&preview_item, &plan)`, which collects `shader_layers` from `preview_item.profile.effective_style_layers()` and the cursor layer.
- `loopback_fired_keys: Vec::new()` (the L5 follow-on field; legacy V2 path leaves it empty).

`crates/tui-vfx-probe/src/orc_run_probe.rs:run_probe` (line 47, 271 LOC) takes the spec and:

1. Unpacks `source` and `destination` into `OwnedGrid`s.
2. **Constructs `source_roles = RoleMap::all_background(source_w, source_h)` (line 91 in current 0.7.0).** This is the load-bearing line for H1.
3. Wraps `destination_grid` in `SemanticScene::from_grid_with_default_role(destination_grid, RoleTag::Background)`.
4. Calls `render_pipeline_with_spec(&source, &source_roles, &mut destination_scene, ..., &composition, Some(&mut inspector))`.
5. Diffs each cell as `is_modified = final_cell != source_cell` to compute `modified_cells`.

If the pipeline produces no per-cell change in the destination — because every shader scope evaluates against the all-Background role map and matches zero cells — every `final_cell` equals the cleared destination cell, every diff returns false, and `modified_cells = 0`.

### `strip_spatial_effects` (the source-buffer companion)

`crates/tui-vfx-recipes/src/probe/fnc_strip_spatial_effects.rs:10` (56 LOC, 0.1.0) clears masks/samplers/filters and filters every style layer's effect to keep only those where `effect.shader().is_none()`. Shaders ARE stripped from the source. So a working probe has source-without-shader and destination-with-shader, and the diff exposes shader-induced cell changes.

### The scope of the reproducer recipe

`recipes/btop_focused_row_demo.json` declares `config.pipeline.style.region: "TextOnly"`. Per the legacy lowering this becomes `StyleRegion::Text`, which lowers to scope `Role(Text)`. **No cells in the probe's destination scene carry `RoleTag::Text` — the role map is all-Background by construction.** The shader's scope predicate evaluates to false for every cell.

### The compiled-V3 probe path

`crates/tui-vfx-recipes/src/v3/compile/fnc_build_probe_scene_spec_from_compiled_plan.rs:125 (build_probe_scene_spec_from_compiled_plan_timed_with_overrides)` produces a `ProbeSceneSpec` whose `destination` comes from `build_destination_grid(width, height)` (a freshly cleared `OwnedGrid`). It hands off to the same `run_probe`. The same `RoleMap::all_background` rendering applies. A V3 recipe whose scope is role-based would also report `modified_cells: 0`. The V3 path "appears unaffected" only because most V3 reference recipes use coordinate-based scopes; verify during the bisect.

## Hypothesis enumeration

Ordered by strength of evidence. The executor verifies the top hypothesis first, before bisecting.

**H1 (top hypothesis — verify before bisecting). The A.2 cutover at `tui-vfx@ec872a4` (2026-04-20) introduced the `RoleMap::all_background` placeholder in `run_probe` as a "Sub-plan C will replace this" stopgap. Sub-plan C never landed.** The role-aware compositor signature requires a source-roles map; the probe has no upstream to provide one for legacy recipes; the placeholder makes any role-scoped recipe report zero modified cells. Predictions: every role-scoped recipe (`Role(Text)`, `Role(Surface)`, etc.) reports `modified_cells: 0`; coordinate-scoped recipes (`Cell(x, y)`, `RowRange`, `Rect`, `All`) still work. **Reproducer test: run the probe against any all-cells recipe (e.g. one using `region: All`) — if those report `modified_cells > 0`, H1 is confirmed.**

**H4. A schema_v1 → V3 normalization regression silently dropped the shader from the lowered pipeline.** Predictions: `shader_count` would also be zero. The reproducer reports `shader_count: 1`. **Falsified by the reproducer output.** Listed for completeness in case the bisect surprises.

**H2. `apply_composition` in the probe path (the geometric role-inference fallback referenced in §8.6) now strips spatial effects too aggressively.** The §8.6 commentary mentions `apply_composition` vs `apply_composition_with_roles`. If the probe uses the geometric inference path and that path lost the shader, `shader_count` would still be 1 (built from `composition.shader_layers`) but the destination would be unchanged. Less likely than H1 because `run_probe` calls `render_pipeline_with_spec` directly, not `apply_composition`.

**H3. The diff comparison logic (`is_modified = final_cell != source_cell`) regressed.** Possible if a recent refactor changed `Cell` equality to ignore some field. Easy to check by inspecting `tui_vfx_types::Cell`'s `PartialEq`. Lowest probability.

If the bisect lands somewhere unexpected, the executor reports the surprise commit and the fix-shape decision tree below selects the matching response.

## Once-bisect-identifies-cause: fix-shape options

Each fix shape is listed against the cause class it addresses. Do not pre-commit; let the bisect dictate.

**If H1 (role-map placeholder).** The probe has no upstream role information for legacy schema_v1 recipes. Three fix shapes, ranked by surgical-ness:

- **Shape A (recommended). Lift the source-roles map from the recipe's content rather than defaulting all-Background.** The probe builder already constructs `source_buffer` by rendering `strip_spatial_effects(preview_item)`. After that render, infer roles from the rendered grid using the geometric/glyph-based inference (`Content(Text)` cells where the buffer has non-empty glyphs, `Background` elsewhere). Pass the inferred map to `run_probe` via a new `ProbeSceneSpec.source_roles` field, defaulted to `None` (existing behavior preserved). When `Some`, `run_probe` uses it instead of `RoleMap::all_background`. Cost: one new optional field on `ProbeSceneSpec`; one inference helper in the recipes-side probe; modest test churn. Honors the probe's grid-first contract — the probe is the lowest-truth surface and should not depend on a producer's role map.
- **Shape B. Promote the inference to a probe-internal default.** Move the inference inside `run_probe` itself: when the destination scene's role map is all-Background AND the source grid has glyph content, infer `Text` roles on glyph cells. Cost: keeps the recipes-side code unchanged. Risk: probe behavior diverges from production behavior (production has a real role map from the producer); the probe becomes an "honest probe except for this case" which is the kind of thing future debuggers will trip on.
- **Shape C. Wait for Sub-plan C.** The CLOG on `orc_run_probe.rs:0.7.0` says Sub-plan C delivers real role-tagging. If Sub-plan C is queued and close, defer the probe fix and let it ride that work. Risk: §8.7 has been an open finding for at least the 2026-04-26 session; "close" is unverified. **Surface the question to the user before picking C.**

**If H2 (`apply_composition` over-strips).** Identify the change that altered which effects survive into the destination buffer, narrow to the minimal revert that restores destination-with-shader. If the change was deliberate (e.g. the loopback-merge change deliberately changed semantics), the fix may be in the probe's reference-buffer construction, not in the production change.

**If H3 (diff regressed).** Restore the `Cell` equality contract or compare on a field-by-field basis in `run_probe`'s diff loop. Add a unit test on `Cell::eq` pinning the contract.

**If H4 (lowering dropped shader).** The reproducer falsifies this; if it lands here anyway, the bisect surfaced something the reproducer's metadata missed. Investigate by dumping the lowered pipeline at the probe entry.

## Acceptance criteria

- [ ] Reproducer shows `modified_cells > 0` for `recipes/btop_focused_row_demo.json` (the schema_v1 path). The recipe's `falloff_distance: 4` over 12 rows plus the `Role(Text)` scope predicts roughly 8-10 modified cells per the dwelling phase frame — exact count depends on the inference; assert `>= 1` in tests, capture the actual count in the commit message.
- [ ] Reproducer shows `modified_cells > 0` for `themes/eichler/recipes/focused_row_btop.json` (the gt-design copy of the same recipe shape, also schema_v1). Run from the gt-design checkout: `cd /usr/projects/gt-design && cargo run -q -p recipe-probe -- themes/eichler/recipes/focused_row_btop.json --phase dwelling --sample-t 1.0 --with-causation --widget-cell 4,3 | grep modified_cells`.
- [ ] Existing probe tests pass unchanged across both workspaces (`cargo test -p tui-vfx-probe`, `cargo test -p tui-vfx-recipes`). Coordinate-scoped recipes (those that already worked) still work — no regression in the working paths.
- [ ] One regression test added that asserts `build_unified_probe_report_from_recipe(...).summary.modified_cells > 0` for the canonical reference recipe. Location: `crates/tui-vfx-recipes/src/probe/test_*.rs` (peer-test pattern). The test loads the recipe, builds the probe report, asserts the count, and includes a failure message naming the file (`recipes/btop_focused_row_demo.json — probe-fidelity contract`) so the next regression is named at the test failure rather than requiring archaeology.
- [ ] If H1 lands and the fix is Shape A, the new `ProbeSceneSpec.source_roles` field has a rustdoc paragraph naming the contract (`None` = all-Background placeholder, `Some(map)` = author-supplied or inferred roles), and the recipes-side helper that does the inference is a peer-tested `fnc_*` file with at least three tests covering text-rich, text-empty, and partial-text grids.
- [ ] Clean build: `cargo build --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` clean across both `tui-vfx` and `tui-vfx-recipes`. No `#[allow]` suppressions added. Pre-existing warnings count as breakage per `feedback_clean_build_no_warnings`.
- [ ] No inert schema fields. If the fix adds a field to `ProbeSceneSpec`, it is consumed by `run_probe` in this packet, not parsed-and-ignored.
- [ ] Rustdoc audited on every public item touched per `feedback_rustdoc_when_editing`. The `RoleMap::all_background` placeholder comment in `run_probe` is updated to reflect the post-fix contract or removed if the placeholder is gone.
- [ ] Handoff doc `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md` §8.7 updated to mark item 9 done with a one-line note pointing at the fix commit.

## Verification commands

```bash
# Reproducer (the truth gate).
cd /usr/projects/tui-vfx-recipes
cargo run -q -p recipe-probe -- /usr/projects/tui-vfx-recipes/recipes/btop_focused_row_demo.json \
  --phase dwelling --sample-t 1.0 --with-causation --widget-cell 4,3 \
  | grep -E "modified_cells|shader_count|shader_effects"
# Expect: shader_count: 1, modified_cells: > 0.

cd /usr/projects/gt-design
cargo run -q -p recipe-probe -- themes/eichler/recipes/focused_row_btop.json \
  --phase dwelling --sample-t 1.0 --with-causation --widget-cell 4,3 \
  | grep modified_cells
# Expect: modified_cells: > 0.

# Build + test + clippy across both workspaces.
cd /usr/projects/tui-vfx       && cargo build --workspace && cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings
cd /usr/projects/tui-vfx-recipes && cargo build --workspace && cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings

# Confirm the regression test exists and exercises the contract.
cd /usr/projects/tui-vfx-recipes
cargo test -p tui-vfx-recipes probe_fidelity_btop_focused_row -- --nocapture
```

## Rollback plan

Per-bisect-result.

- **If the bisect identifies a single commit and the fix is a small revert.** `git revert <sha>` on the offending commit, re-run the reproducer, commit. Add the regression test as a separate commit so the test lands even if the revert is later reworked.
- **If the regression is intentional (e.g. the A.2 role-map cutover deliberately changed semantics and the probe was expected to follow up).** The fix lives in the probe's reference-buffer construction or in a new role-inference helper, not in reverting the change. Land the fix forward; do not revert A.2. Document in the commit message that the deferred Sub-plan C work has been satisfied for the probe's needs.
- **If the bisect is inconclusive (skipped revisions or no clean transition).** Run the parallel bisect on `tui-vfx`. If both bisects are inconclusive, halt and surface to the user with the full list of skipped revisions and the most recent passing reproducer output.
- **Recyclebin discipline.** Any file replaced or retired moves to `recyclebin/` mirroring the source path. No `rm`.

## Risks & gotchas

- **The schema_v1 path is shared with the gt-design loader (per handoff §8.5). Any fix must not break loading.** `gt-design`'s producer pipeline calls into the same `tui-vfx-recipes` surfaces; if the fix changes the `ProbeSceneSpec` shape, gt-design's compile chain needs to catch the additive field. Verify with `cd /usr/projects/gt-design && cargo build --workspace` after each interim commit.

- **The compiled-V3 path "appears unaffected" but routes through the same `run_probe`. Both paths probably exhibit the regression for any role-scoped recipe.** Most V3 reference recipes use coordinate-based scopes, hiding the bug. If the fix lands at the `run_probe` level (likely for H1 Shape B), both paths benefit. If the fix lands in the recipes-side `build_probe_scene_from_recipe` (H1 Shape A), the V3 builder needs the same treatment in a follow-on packet. Add a probe-vs-V3 parity check to the regression test if Shape A is chosen.

- **The §8.9 finding (`region: TextOnly` lowers to `Role(Text)`) is the root reason role-based scopes appear at all in legacy recipes.** §8.9 is deferred to the V3 cutover. Do not attempt to fix §8.9 in this packet — it ripples across every legacy recipe author who used `region: TextOnly`. Assume the role-scoped scope is the input contract this packet must honor.

- **The bisect window crosses workspaces.** `tui-vfx` and `tui-vfx-recipes` evolve independently; the regression may live in one and surface in the other. The cross-workspace bisect protocol above handles this. Watch for skipped revisions in the bisect output and pivot to the other workspace if the recipes-side bisect is inconclusive.

- **Build failures during bisect must skip, not fail-as-bad.** The `cargo build` step in the `git bisect run` script bails with exit 125 on build failure. Without the skip, transient breakages get blamed for the regression and the bisect produces a wrong answer.

- **The reproducer reports `shader_count: 1` even when broken. Do not interpret a non-zero `shader_count` as evidence the probe is working.** `shader_count` is computed from the composition spec (the recipe's declared shaders), not from execution evidence. `modified_cells` is the truth signal.

- **Per §8.8 (handoff item 10), `--runtime-params-json` is silently dropped on schema_v1.** The reproducer does not exercise runtime params. If a future probe regression test needs runtime params, fix §8.8 first — otherwise the test result is the wrong-shaped truth.

- **Clean-build memory rule.** Resolve every warning at the root. No per-site `#[allow]` per `feedback_no_landmines`. Pre-existing warnings count.

## Sequencing note

- This packet **should land before** the observability bus reaches its parity-test phase (`tui-vfx-pipeline-observability.md` US-009 bench-gate verification). The bus needs a working probe to A/B against; without this fix, parity tests test nothing.
- This packet is **independent** of the §8.6 production-path role-map mismatch fix (path B at `ContentShell::card`). Both touch role-tagging contracts; the production fix is a gt-design concern, this fix is a probe concern.
- This packet is **independent** of the §8.9 `region: TextOnly` lowering fix. §8.9 is deferred to the V3 cutover; this packet honors the existing lowering as input.
- The parallel observability session may already have noticed this regression while building the bus design. Coordinate via the handoff doc — the observability author named the probe regression in `tui-vfx-pipeline-observability.md` v0.2.0's motivation prose. If they have an in-flight branch touching `run_probe`, sync before bisecting to avoid duplicate work.

<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-probe-fidelity-bisect.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
