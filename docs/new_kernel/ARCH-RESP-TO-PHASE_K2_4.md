# Phase K2.5 Work Packet — Styled Primitive Adapter Burn-down + Player Tooling/Validation PRD

Date: 2026-04-29
Implementation repo: `/usr/projects/tui-vfx`
Recipe repo: `${RECIPE_REPO:-../tui-vfx-recipes}`
Prior packet: K2.4 — Styled-Cell Visual Substrate
Current packet: K2.5 — Styled Primitive Adapter Burn-down + Tooling/Validation PRD Capture

## Rolling context

Completed today:

```text
K2.1 — migration-gap report
K2.2 — visual-frame report substrate
K2.3 — text-grid primitive adapter burn-down
K2.4 — styled-cell visual substrate foundation
```

Current packet:

```text
K2.5 — styled/color/role primitive adapter burn-down
K2.5 companion lane — player tooling and validation PRD capture
```

Coming next:

```text
K2.6 — frame timeline / frame diff controls
K2.7+ — selectors, runtime params, trace/debug surfaces, SQLite/xray only after useful trace data exists
```

## Executive goal

Use the K2.4 styled-cell substrate to stop treating styled/color/role primitives as generically blocked by missing substrate.

Target remaining primitive ids:

```text
shader.borderSweep
shader.linearGradient
style.baseStyleOverride
style.colorFade
```

Preferred end state:

```text
render-recipe:          total=16 rendered=16 unsupported=0 errors=0
inventory-recipes:      totalRecipes=16 rendered=16 unsupported=0 errors=0 unsupportedEffectIds=0
render-frame:           total=16 rendered=16 unsupported=0 errors=0
primitive-adapter-gap:  totalEffects=14 rendered=14 blockedByStyledCellSubstrate=0 stillUnsupported=0 blockedBySemanticDecision=0
```

If a primitive cannot honestly render because of a semantic gap, do **not** fake it. Move it from `blockedByStyledCellSubstrate` to a more accurate outcome such as `blockedBySemanticDecision`, with a precise reason and regression coverage. K2.5 should eliminate stale “substrate missing” blockers because K2.4 established the substrate.

## Non-goals

Do not claim visual parity.

Do not modify legacy recipes:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/recipes/debug_recipes
```

Do not modify canonical fixture recipes unless explicitly needed and separately justified:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/recipes/v3.1/debug_recipes
```

Do not port legacy rendering source.

Do not introduce dependencies from `tui-vfx-player` or `tui-vfx-player-cli` onto legacy recipes tooling crates.

Do not implement SQLite, trace streams, frame diff, runtime-param injection, canvas simulation, QC gates, and schema-doc generation all in K2.5. Capture that roadmap in the PRD; implement only the minimal pieces needed for styled primitive validation.

## Core invariant

A primitive may be marked rendered only when the player evidence can honestly represent it.

For styled/color/role primitives, that means rendered frames must show real styled-cell evidence:

```text
substrate=styledCell
cellSource=styledCells
styleKnown=true
```

Rows must remain present for compact human-readable evidence.

Frame evidence remains contract-native player evidence, not oracle parity.

## Lane A — Styled primitive adapter burn-down

### A1. Establish RED tests first

Add failing tests before implementation for the four remaining primitive ids.

Use canonical fixtures from:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/recipes/v3.1/debug_recipes
```

Likely fixture targets:

```text
styles/style_color_fade.json
styles/style_role_scope_border.json
shaders/primitives/shader_linear_gradient.json
shaders/compositions/shader_border_sweep.json
```

Tests should prove, for each fixture that can honestly render:

```text
status=rendered
styleKnown=true
substrate=styledCell
cellSource=styledCells
rows[] still present
cells[] contains at least one styled non-default cell
unsupportedEffectIds[] is empty for that frame
```

Also add a hash regression:

```text
Changing style evidence must be able to change renderHash even when rows are unchanged.
```

This prevents styled-cell changes from becoming invisible to regression reports.

### A2. Implement or classify each styled primitive honestly

#### `style.colorFade`

Expected adapter class:

```text
styledCell
```

Required behavior:

```text
- Resolve descriptor/fixture inputs through existing canonical input resolution.
- Apply deterministic color interpolation to selected cells.
- Preserve glyphs and rows.
- Mark styleKnown=true when style data is written.
- Emit sparse cells with real foreground/background evidence.
```

Do not assume old field names. Read the canonical descriptor pack and fixture input names.

#### `style.baseStyleOverride`

Expected adapter class:

```text
styledCell
```

Required behavior:

```text
- Apply foreground/background/modifier overrides to selected cells.
- Preserve glyphs and rows.
- Mark styleKnown=true when style data is written.
- If the fixture uses role scope and the player lacks enough role provenance, either:
  - add a minimal honest role source for the relevant canonical source/fixture, or
  - classify the unsupported case as blockedBySemanticDecision with a precise reason.
```

Do not silently apply a role-scoped override to all cells.

#### `shader.linearGradient`

Expected adapter class:

```text
styledCell
```

Required behavior:

```text
- Compute deterministic color/style evidence from cell position and descriptor inputs.
- Apply to selected cells.
- Preserve glyphs and rows.
- Mark styleKnown=true.
```

This does not need to match the legacy shader visually. It must be deterministic, descriptor-driven, and honest.

#### `shader.borderSweep`

Expected adapter class:

```text
styledCell
```

Required behavior:

```text
- Apply deterministic styled-cell evidence to the scope/cells represented by the canonical fixture.
- Prefer border/edge-local behavior when source/fixture evidence supports it.
- Preserve glyphs and rows.
- Mark styleKnown=true.
```

If true border semantics require role/source facts that the current player does not yet produce, classify that limitation explicitly rather than faking a full border sweep.

### A3. Update primitive adapter gap classification

Update the adapter gap report so the previous four blockers no longer say the substrate is missing.

Expected outcomes after successful implementation:

```text
shader.borderSweep      rendered
shader.linearGradient   rendered
style.baseStyleOverride rendered
style.colorFade         rendered
```

If any remain blocked, their outcome must be one of:

```text
blockedBySemanticDecision
stillUnsupported
```

with a reason that names the missing semantic decision or missing adapter behavior.

No remaining K2.5 target should report:

```text
blockedByStyledCellSubstrate
```

unless K2.4’s styled-cell substrate is demonstrably not available to production frame construction, which would be a regression.

### A4. Preserve K2.3 text-grid adapters

The following must remain rendered:

```text
mask.dissolve
sampler.ripple
```

They may still report row/text-grid provenance when no styled adapter writes style:

```text
substrate=textGrid
cellSource=rows
styleKnown=false
```

Do not force all rendered frames to become styled-cell frames.

### A5. Keep visual-frame provenance precise

For any frame with styled adapter output:

```text
substrate=styledCell
cellSource=styledCells
styleKnown=true
```

For row-derived frames with no styled-cell writes:

```text
substrate=textGrid
cellSource=rows
styleKnown=false
```

Rows remain present in both cases.

## Lane B — Player tooling and validation PRD capture

Create a formal PRD document:

```text
docs/new_kernel/K2_PLAYER_TOOLING_VALIDATION_PRD.md
```

This PRD captures useful capabilities from existing recipes tooling as oracle inspiration, not as source to port and not as canonical authority.

### B1. PRD purpose statement

Include this principle:

```text
Adopt capability patterns, not source code or legacy validation authority.
```

The clean-room authority remains:

```text
tui-vfx-contract-cli     structural validation
tui-vfx-player-cli       canonical player/render/inventory/migration evidence
tui-vfx-player           contract-native player library
```

### B2. Current clean-room tooling state

Summarize current commands:

```text
validate-recipe          tui-vfx-contract-cli
render-recipe            tui-vfx-player-cli
render-frame             tui-vfx-player-cli
inventory-recipes        tui-vfx-player-cli
migration-gap            tui-vfx-player-cli
primitive-adapter-gap    tui-vfx-player-cli
```

Summarize current report schemas:

```text
v3.1.validator.report.1
v3.1.player.renderReport.1
v3.1.player.inventoryReport.1
v3.1.player.migrationGap.1
v3.1.player.visualFrameReport.1
v3.1.player.primitiveAdapterGap.1
```

### B3. Legacy tooling inventory to capture

Include these source roots as reference only:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/pipeline-validator
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-probe
${RECIPE_REPO:-../tui-vfx-recipes}/tools/tui-vfx-trace
${RECIPE_REPO:-../tui-vfx-recipes}/tools/tui-vfx-horseman
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-source-capture
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-signals-doc
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-validator
```

#### `pipeline-validator`

Capture these capabilities:

```text
--format text|json
--stage parse|profile|render|shader|output
--phase entering|dwelling|exiting|all
--sample-t values
--frames N
--dump
--dump-normalized
--explore-normalized
--lowering-report
--migration-equivalence-report
--trace
--rules
--rules-file FILE
--strict
--strict-contracts
--errors-only
--stages
--probe
--probe-cells all|non-empty|modified
--probe-frames N
--probe-diff-to T
--probe-causation
--probe-sqlite-query SQL
--probe-widget-cell X,Y
--runtime-params-json JSON|@FILE
--compiled-v3-source-text TEXT|@FILE
--compiled-v3-procedurals none|stock
--debug-recipes-qc
--bench
--iterations N
--canvas RRGGBB
--canvas-content empty|sentinel|lorem
```

Clean-room classification:

```text
Adopt now or soon:
- mode-based command architecture
- schema-labeled JSON envelopes
- recursive corpus handling
- phase/sample/frame controls
- runtime parameter injection design
- debug corpus QC concept
- canvas/substrate flags after styled-cell/composition substrate matures

Adapt later:
- rule files
- stage masks
- trace mode
- probe SQL
- benchmark mode
- migration equivalence reports

Reject as direct port:
- legacy parse/profile/render stage names as canonical authority
- legacy strict-contract meanings when they differ from v3.1 validation
```

#### `recipe-probe`

Capture:

```text
--format json
--phase entering|dwelling|exiting
--sample-t N
--cells all|non-empty|modified
--with-causation
--frames N
--diff-to T
--sqlite-query SQL
--widget-cell X,Y
--canvas RRGGBB
--canvas-content empty|sentinel|lorem
```

Capture evidence concepts:

```text
per-cell traces
content/style/compositor stage accounting
operational analysis
lifecycle analysis
motion analysis
focus-cell root cause
SQLite xray tables
runtime parameter resolution tables
binding resolution diagnostics
cell root-cause summaries
```

Clean-room classification:

```text
Adopt soon:
- frame timeline semantics
- frame diff semantics
- cell selector vocabulary
- focus-cell inspection vocabulary

Adapt later:
- causation traces once player has trace events
- SQLite xray once player evidence includes enough structured events
- canvas simulation after styled-cell/composition substrate is real
```

#### `tui-vfx-trace`

Capture:

```text
--recipe PATH
--theme NAME
--frames N
--select all|cell:x,y|rect:x,y,w,h|role:name|layer:id|recipe:id
--stages lifecycle|resolution|composition|pipeline|sampler|mask|shader|filter|shadow|all|none
--from-ms N
--to-ms N
--format ndjson|report
--output PATH
```

Clean-room classification:

```text
Adopt later:
- selector grammar
- stage-mask vocabulary
- time-windowed trace capture
- NDJSON for long trace streams
- JSON report envelopes for summaries
```

#### `tui-vfx-horseman`

Capture:

```text
--json
--project-root PATH
--corpus DIR
recipe.json positional path
```

Report schemas:

```text
tui_vfx_horseman.summary.v1
tui_vfx_horseman.corpus_summary.v1
```

Clean-room classification:

```text
Already mostly covered by render-recipe, inventory-recipes, migration-gap, and primitive-adapter-gap.
Adopt lightweight corpus summary ergonomics.
Do not adopt legacy fallback authority.
```

#### `recipe-source-capture`

Capture:

```text
--output FILE
--cwd DIR
--allow-nonzero
-- <command...>
```

Output schema:

```text
tui_vfx.command_capture.v1
```

Clean-room classification:

```text
Adopt later as offline authoring/oracle fixture capture.
Do not make command execution runtime playback behavior.
```

#### `recipe-signals-doc`

Capture:

```text
--write
--check
```

Clean-room classification:

```text
Adopt generated docs drift checks for player/report schemas once schemas stabilize.
```

Potential future shape:

```text
cargo run -p tui-vfx-player-cli -- report-schema-docs --check
cargo run -p tui-vfx-player-cli -- report-schema-docs --write
```

#### `recipe-validator`

Capture only this conclusion:

```text
Deprecated. Do not adopt as a new path.
```

Clean-room classification:

```text
Use only as a negative example.
Keep canonical validation in tui-vfx-contract-cli and tui-vfx-player-cli.
```

### B4. Adoption matrix

Add this table or equivalent to the PRD:

| Feature family              | Legacy source                        | Clean-room target                           | Priority                           |
| --------------------------- | ------------------------------------ | ------------------------------------------- | ---------------------------------- |
| Schema-labeled JSON reports | `pipeline-validator`, `horseman`     | already active; standardize                 | immediate                          |
| Recursive corpus commands   | `pipeline-validator`, `horseman`     | already active; keep                        | immediate                          |
| Phase/sample controls       | `pipeline-validator`, `recipe-probe` | `render-frame --phase --sample-t`           | high                               |
| Frame timelines             | `recipe-probe`                       | `frame-timeline` or `render-frame --frames` | high after K2.5                    |
| Frame diff                  | `recipe-probe`                       | `frame-diff` / `render-frame --diff-to`     | high after K2.5                    |
| Cell selectors              | `recipe-probe`, `trace`              | `--cells`, `--select`                       | medium                             |
| Runtime params              | `pipeline-validator`                 | `--runtime-params-json`                     | medium/high                        |
| Canvas simulation           | `pipeline-validator`, `recipe-probe` | `--canvas`, `--canvas-content`              | medium after composition substrate |
| Stage masks                 | `trace`                              | `trace-frame --stages`                      | later                              |
| NDJSON trace streams        | `trace`                              | `trace-frame --format ndjson`               | later                              |
| SQLite xray                 | `recipe-probe`, `pipeline-validator` | xray export/query                           | later                              |
| QC summary                  | `pipeline-validator`, `horseman`     | `fixture-qc`                                | medium                             |
| Bench/iterations            | `pipeline-validator`                 | player benchmark mode                       | later                              |
| Command capture             | `recipe-source-capture`              | offline authoring/oracle fixture capture    | later                              |
| Docs drift checks           | `recipe-signals-doc`                 | `report-schema-docs --check`                | medium                             |
| Deprecated validator        | `recipe-validator`                   | reject                                      | immediate                          |

### B5. Proposed clean-room command taxonomy

Include current commands:

```text
render-recipe
render-frame
inventory-recipes
migration-gap
primitive-adapter-gap
```

Include near-term candidates:

```text
render-frame --phase <enter|dwell|exit>
render-frame --sample-t <number>
render-frame --frames <N>
render-frame --diff-to <sampleT>
render-frame --cells <all|non-empty|modified>
render-frame --runtime-params-json <JSON|@FILE>
render-frame --canvas <RRGGBB>
render-frame --canvas-content <empty|sentinel|lorem>
```

Include later candidates:

```text
trace-frame
probe-frame
frame-diff
frame-timeline
fixture-qc
report-schema-docs
```

### B6. Proposed future report schemas

Document as future candidates, not implemented in K2.5:

```text
v3.1.player.frameTimelineReport.1
v3.1.player.frameDiffReport.1
v3.1.player.traceReport.1
v3.1.player.traceStream.1
v3.1.player.fixtureQcReport.1
v3.1.player.schemaDocsReport.1
```

### B7. PRD scope guard

Include this exact scope guard or equivalent:

```text
The tooling PRD must not turn legacy tooling into v3.1 authority.
It classifies legacy tooling as oracle inspiration only.
Any new clean-room CLI feature must report through schema-labeled v3.1 player or contract reports.
```

## Documentation deliverables

Create or update:

```text
docs/new_kernel/K2_5_STYLED_PRIMITIVE_ADAPTER_EVIDENCE.md
docs/new_kernel/K2_PLAYER_TOOLING_VALIDATION_PRD.md
docs/new_kernel/PHASE_K2_5_STYLED_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md
docs/VOCABULARY.md
```

The K2.5 status memo must include the rolling context section:

```text
Completed today:
- K2.1 migration-gap
- K2.2 visual-frame report
- K2.3 primitive adapter burn-down
- K2.4 styled-cell substrate foundation

Current packet:
- K2.5 styled/color/role primitive adapter burn-down
- K2.5 tooling/validation PRD capture

Coming next:
- K2.6 frame timeline / frame diff
- Later trace/debug/SQLite/QC surfaces
```

Update vocabulary only for new public terms that are actually introduced or formalized. Planning-only future terms may be added under deferred tooling terms if needed.

## Expected files likely touched

Player library:

```text
crates/tui-vfx-player/src/lib.rs
crates/tui-vfx-player/src/fnc_apply_graph_effects.rs
crates/tui-vfx-player/src/fnc_resolve_effect_input.rs
crates/tui-vfx-player/src/fnc_player_inventory_adapter_status.rs
crates/tui-vfx-player/src/fnc_classify_primitive_adapter_gap.rs
crates/tui-vfx-player/src/fnc_build_visual_frame.rs
crates/tui-vfx-player/tests/test_fnc_recipe_player.rs
```

Likely new helpers, if needed:

```text
crates/tui-vfx-player/src/fnc_apply_style_color_fade.rs
crates/tui-vfx-player/src/fnc_apply_style_base_style_override.rs
crates/tui-vfx-player/src/fnc_apply_shader_linear_gradient.rs
crates/tui-vfx-player/src/fnc_apply_shader_border_sweep.rs
```

Player CLI/tests:

```text
crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
```

Docs:

```text
docs/new_kernel/K2_5_STYLED_PRIMITIVE_ADAPTER_EVIDENCE.md
docs/new_kernel/K2_PLAYER_TOOLING_VALIDATION_PRD.md
docs/new_kernel/PHASE_K2_5_STYLED_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md
docs/VOCABULARY.md
```

## Acceptance criteria

K2.5 is accepted when:

```text
1. The four remaining styled/color/role primitive ids no longer report stale blockedByStyledCellSubstrate.
2. Any primitive marked rendered has direct visual-frame evidence with styleKnown=true when it writes style/color/role.
3. render-frame continues to include rows[] for every rendered frame.
4. render-recipe, inventory-recipes, render-frame, migration-gap, and primitive-adapter-gap still work.
5. K2_PLAYER_TOOLING_VALIDATION_PRD.md exists and captures all legacy tooling capabilities listed in this packet.
6. The PRD explicitly classifies legacy tooling as oracle inspiration only.
7. recipe-validator is explicitly marked non-adopted/deprecated.
8. No legacy tooling crate becomes a dependency of tui-vfx-player or tui-vfx-player-cli.
9. No recipe files are modified.
10. Status memo includes completed/current/coming context.
```

Preferred corpus result:

```text
total=16
rendered=16
unsupported=0
errors=0
```

If unsupported remains, acceptance requires a precise semantic blocker explanation, tests for the blocker, and no stale substrate-blocker language.

## Verification commands

Use portable paths:

```bash
export RECIPE_REPO="${RECIPE_REPO:-../tui-vfx-recipes}"
export PACK="descriptors/v3.1/packs/primitive.json"
export V31_ROOT="$RECIPE_REPO/recipes/v3.1/debug_recipes"
export LEGACY_ROOT="$RECIPE_REPO/recipes/debug_recipes"
export TMP="${TMPDIR:-/tmp}"
```

Run formatting and tests:

```bash
cargo fmt --package tui-vfx-player --package tui-vfx-player-cli -- --check
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings
cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
cargo test --workspace
git diff --check
```

Run path guard:

```bash
rg -n '"/usr/projects/tui-vfx-recipes' crates/tui-vfx-player/tests crates/tui-vfx-player-cli/tests
```

Expected: no output.

Run dependency guard:

```bash
cargo tree -p tui-vfx-player
cargo tree -p tui-vfx-player-cli
```

Confirm no dependencies on legacy tooling crates such as:

```text
pipeline-validator
recipe-probe
tui-vfx-trace
tui-vfx-horseman
recipe-validator
```

Run report commands:

```bash
cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --descriptor-pack "$PACK" \
  --recursive "$V31_ROOT" \
  > "$TMP/tui-vfx-k25-render-report.json"

cargo run -q -p tui-vfx-player-cli -- inventory-recipes \
  --descriptor-pack "$PACK" \
  --recursive "$V31_ROOT" \
  > "$TMP/tui-vfx-k25-inventory-report.json"

cargo run -q -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack "$PACK" \
  --recursive "$V31_ROOT" \
  > "$TMP/tui-vfx-k25-visual-frame-report.json"

cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack "$PACK" \
  --recursive "$V31_ROOT" \
  > "$TMP/tui-vfx-k25-primitive-adapter-gap-report.json"

cargo run -q -p tui-vfx-player-cli -- migration-gap \
  --legacy-root "$LEGACY_ROOT" \
  --v31-root "$V31_ROOT" \
  --descriptor-pack "$PACK" \
  > "$TMP/tui-vfx-k25-migration-gap-report.json"
```

Confirm recipe roots are untouched:

```bash
git -C "$RECIPE_REPO" status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes
```

Expected: no output.

## Review and de-slop requirements

After implementation:

```text
1. Run formal review over production code, tests, docs, and any justfile/command changes.
2. Resolve blocking findings.
3. Run AI de-slop over touched files.
4. Re-run all verification commands.
5. Record review/de-slop results in the K2.5 status memo.
```

The status memo should include final report summaries and captured artifact paths under `${TMPDIR:-/tmp}`.

## TDD process requirement

Start with tests.

Minimum RED coverage before implementation:

```text
- one player integration test per styled primitive target or per canonical fixture
- one CLI/report test proving remaining target ids no longer appear as stale styled-cell blockers
- one visual-frame assertion proving styleKnown=true and styledCell provenance for a styled fixture
- one regression that text-grid-only fixtures are not forced to styleKnown=true
```

Then implement to GREEN.

## Final implementer prompt

```text
Implement Phase K2.5 in /usr/projects/tui-vfx.

Use the K2.4 styled-cell substrate to burn down the remaining styled/color/role primitive adapter blockers:
shader.borderSweep, shader.linearGradient, style.baseStyleOverride, and style.colorFade.

Do not claim visual parity. Do not modify recipe files. Do not port legacy source. Do not add dependencies on legacy recipes tooling crates.

A primitive may be marked rendered only if the player evidence can honestly represent it. For styled/color/role primitives, rendered visual frames must carry substrate=styledCell, cellSource=styledCells, and styleKnown=true, while preserving rows[].

Also create docs/new_kernel/K2_PLAYER_TOOLING_VALIDATION_PRD.md. This PRD must capture the existing recipes tooling capabilities from pipeline-validator, recipe-probe, tui-vfx-trace, tui-vfx-horseman, recipe-source-capture, recipe-signals-doc, and recipe-validator. Classify each capability as adopt now, adapt later, defer, or reject. Treat legacy tooling as oracle inspiration only, not v3.1 authority.

Keep rolling context in the K2.5 status memo:
completed K2.1/K2.2/K2.3/K2.4, current K2.5, upcoming K2.6 frame timeline/diff and later trace/debug surfaces.

Run and record full verification:
fmt, clippy, player tests, CLI tests, workspace tests, diff check, path portability, dependency guard, render-recipe, inventory-recipes, render-frame, primitive-adapter-gap, migration-gap, and recipe-root cleanliness.
```
