Understood. When you send a phase/status memo back to me, I will respond with the full next work packet unless you explicitly tell me to stand by.

````md
# FILE: ARCH-RESP-TO-PHASE_K2_3.md
<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_3.md</FILE> - <DESC>Architect response and next work packet after K2.3 primitive adapter burn-down</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Respond to K2.3 and assign K2.4 styled-cell visual substrate packet.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — accept K2.3 and define K2.4 styled-cell visual substrate packet.</CLOG> -->

# Architect Response to Phase K2.3

Date: 2026-04-29  
Repo: `/usr/projects/tui-vfx`  
Prior packet: K2.3 — Primitive Adapter Burn-down  
Next packet: K2.4 — Styled-Cell Visual Substrate Foundation

## Rolling context

Completed today:

```text
K2.1: migration-gap report established the legacy/v3.1 corpus planning surface.
K2.2: render-frame report established stable visual-frame evidence.
K2.3: primitive adapter burn-down reduced unsupported primitive ids from 6 to 4.
````

Current state:

```text
render-recipe:      total=16 rendered=12 unsupported=4 errors=0
inventory-recipes:  totalRecipes=16 rendered=12 unsupported=4 errors=0
render-frame:       total=16 rendered=12 unsupported=4 errors=0
```

Remaining unsupported primitive ids:

```text
shader.borderSweep
shader.linearGradient
style.baseStyleOverride
style.colorFade
```

These are correctly blocked because the current visual-frame substrate is still:

```text
substrate=textGrid
cellSource=rows
styleKnown=false
```

Coming next:

```text
K2.4 should add honest styled-cell frame evidence.
K2.5 can then unblock style/color/role primitive adapters.
Later K2.x packets can add frame diff/timeline/trace/debug tooling inspired by existing recipes tools.
```

## Executive response

K2.3 is accepted.

The important architectural result is not merely that two more primitives render. The important result is that the player now distinguishes honest text-grid support from styled-cell blockers. That distinction prevents us from prematurely claiming support for effects that require color, modifier, or role evidence.

The next packet should therefore **not** add another glyph-only adapter. The next packet should establish the styled-cell substrate that lets the player report real foreground/background/modifier/role values in `VisualFrame` entries.

This is the necessary gate before claiming support for:

```text
shader.linearGradient
shader.borderSweep
style.baseStyleOverride
style.colorFade
```

## Phase K2.4 assignment

Proceed with:

```text
Phase K2.4 — Styled-Cell Visual Substrate Foundation
```

The goal is to upgrade player frame evidence from row-derived sparse cells to real styled-cell sparse cells while preserving the existing row output, current CLI commands, and K0/K2 regression authority.

This packet is substrate work, not visual parity work.

## Purpose

K2.4 should make `render-frame` capable of emitting a visual frame whose sparse cells carry real style/role data produced by the clean-room player path.

The desired transition is:

```text
Before K2.4:
  substrate=textGrid
  cellSource=rows
  styleKnown=false
  foreground=transparent placeholder
  background=transparent placeholder
  modifiers=[]
  role=null

After K2.4, for recipes supported by the styled-cell substrate:
  substrate=styledCell
  cellSource=surfaceCells or styledCells
  styleKnown=true
  foreground=<actual value>
  background=<actual value>
  modifiers=<actual modifiers>
  role=<actual role or null if role is still genuinely unavailable>
```

Rows must remain present because they are useful for compact human-readable evidence and existing report consumers.

## Non-goals

Do not claim visual parity.

Do not wire the legacy compositor as the authority for v3.1.

Do not modify recipe files in:

```text
../tui-vfx-recipes/recipes/debug_recipes
../tui-vfx-recipes/recipes/v3.1/debug_recipes
```

Do not replace existing commands:

```text
render-recipe
inventory-recipes
migration-gap
render-frame
primitive-adapter-gap
```

Do not mark the four remaining styled-cell-blocked primitives as rendered unless the new substrate can honestly emit the necessary style/role evidence.

Do not build the ratatui GUI in this packet. K1/rataui GUI remains additive UI work on top of CLI/player capability; K2.4 is the player evidence substrate.

## Required design constraints

K2.4 must preserve the current report schema label unless the top-level shape changes incompatibly:

```text
v3.1.player.visualFrameReport.1
```

If the existing schema can represent the new substrate by filling existing fields, keep the schema label unchanged. If a breaking shape change is unavoidable, stop and document the reason before changing it.

The preferred outcome is additive filling of existing fields:

```text
substrate
cellSource
styleKnown
foreground
background
modifiers
role
```

The existing text rows remain canonical compact evidence:

```text
rows[]
```

Styled sparse cells are additional richer evidence:

```text
cells[]
```

## Required implementation shape

### 1. Add an internal styled-cell frame representation

Create or extend player-side frame DTOs so the renderer can carry both:

```text
rows[]
sparse styled cells[]
```

The internal representation should support:

```text
x
y
glyph
foreground
background
modifiers
role
```

Prefer a player-owned DTO rather than leaking contract or legacy runtime structures into the report layer.

Potential files, adjust names as appropriate:

```text
crates/tui-vfx-player/src/cls_player_styled_cell.rs
crates/tui-vfx-player/src/cls_player_styled_grid.rs
crates/tui-vfx-player/src/fnc_collect_styled_visual_cells.rs
```

If existing `PlayerVisualCell` can be extended without confusion, reuse it.

### 2. Define default style semantics

The player needs explicit defaults so sparse-cell collection is deterministic.

Recommended defaults:

```text
glyph: ' '
foreground: transparent or canonical default foreground, but be consistent
background: transparent or canonical default background, but be consistent
modifiers: []
role: null or Background, but be explicit in docs
```

The important rule is that sparse cells should include non-default cells only. If a cell appears in `cells[]`, it must be because at least one observable field differs from the default.

Document the default rule in:

```text
docs/VOCABULARY.md
docs/new_kernel/K2_4_STYLED_CELL_SUBSTRATE_EVIDENCE.md
```

### 3. Convert supported text-grid adapters to styled-cell output

Existing text-grid adapters should still work. For K2.4, they may initially produce styled cells with default style values.

The goal is not yet to style every primitive. The goal is to ensure the frame substrate can carry styles honestly once adapters provide them.

Existing rendered ids should remain rendered:

```text
source.card
filter.dim
filter.tint
filter.invert
filter.greyscale
mask.none
mask.wipe
mask.checkers
mask.dissolve
sampler.sineWave
sampler.ripple
```

If any of these cannot yet produce true color values, they may still use default style values, but the report must honestly distinguish whether style data is known.

### 4. Make `styleKnown` precise

Do not set `styleKnown=true` merely because the cell schema has style fields.

Set `styleKnown=true` only when the substrate is capable of carrying actual style evidence for that frame.

A reasonable K2.4 split is:

```text
substrate=styledCell
cellSource=styledCells
styleKnown=true
```

for frames produced by the new substrate, even if many cells still use defaults.

If the implementation only wraps old rows into styled cells with placeholder style values, keep:

```text
substrate=textGrid
cellSource=rows
styleKnown=false
```

The key acceptance gate is that at least one controlled fixture/test must prove a real non-placeholder style value can appear in a sparse visual cell.

### 5. Add a tiny controlled styled-cell fixture or synthetic player test

Do not mutate the v3.1 recipe corpus just to create a test.

Use one of these approaches:

```text
Option A: Add a player unit test that builds a styled grid/frame directly and verifies sparse cell serialization.
Option B: Add a temp-file canonical recipe inside a test only, not committed to the recipes repo.
Option C: If an existing canonical fixture can honestly produce style data through a supported adapter, use it.
```

The test must prove:

```text
foreground is not placeholder
or background is not placeholder
or modifiers are non-empty
or role is non-null
```

At least one of these must be true in emitted `cells[]`.

### 6. Preserve current K2 reports

The following reports must continue to work:

```text
render-recipe
inventory-recipes
migration-gap
render-frame
primitive-adapter-gap
```

Expected corpus counts may remain:

```text
rendered=12
unsupported=4
errors=0
```

It is acceptable if `render-frame` now reports richer substrate metadata for the rendered 12 fixtures, as long as the output is honest and tests are updated intentionally.

### 7. Do not unblock styled-cell adapters yet unless substrate proof is real

K2.4 should not be primarily an adapter burn-down packet.

If the substrate proof is strong and one simple style adapter can be honestly implemented with minimal scope, it may be included only as a small proof. But the recommended approach is:

```text
K2.4: styled-cell substrate
K2.5: style/color/role adapter burn-down
```

This keeps the gate clean.

## Required report/docs

Create:

```text
docs/new_kernel/K2_4_STYLED_CELL_SUBSTRATE_EVIDENCE.md
docs/new_kernel/PHASE_K2_4_STYLED_CELL_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md
```

Update:

```text
docs/VOCABULARY.md
```

The evidence doc should explain:

```text
- What substrate values now mean.
- What cellSource values now mean.
- When styleKnown is true.
- What fields are still placeholders, if any.
- Whether role evidence is real or deferred.
- Why this still does not prove visual parity.
```

The status memo must include the rolling context block:

```text
Completed today:
- K2.1 migration-gap
- K2.2 visual-frame report
- K2.3 primitive adapter burn-down

Current packet:
- K2.4 styled-cell substrate foundation

Coming next:
- K2.5 styled/color/role primitive adapter burn-down
- Later: frame diff/timeline/trace/debug surfaces
```

## Acceptance criteria

K2.4 is complete when all are true:

| Criterion                                   | Required result                                                                                     |
| ------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| Existing player commands still work         | `render-recipe`, `inventory-recipes`, `migration-gap`, `render-frame`, `primitive-adapter-gap` pass |
| VisualFrame cells can carry real style data | At least one test proves non-placeholder style/modifier/role evidence in `cells[]`                  |
| Rows remain present                         | `rows[]` still emitted for rendered frames                                                          |
| Provenance is honest                        | `substrate`, `cellSource`, and `styleKnown` are accurate                                            |
| No fake adapter support                     | The four styled-cell-blocked ids are not marked rendered unless truly supported                     |
| Current corpus remains clean                | No mutation under old or canonical recipe roots                                                     |
| Docs updated                                | Vocabulary and K2.4 evidence/status docs written                                                    |
| Tests pass                                  | Player, CLI, workspace, fmt, clippy, diff hygiene all pass                                          |

## Verification commands

Use portable recipe repo pathing:

```bash
export RECIPE_REPO="${RECIPE_REPO:-../tui-vfx-recipes}"
export TMPDIR="${TMPDIR:-/tmp}"
```

Run formatting and linting:

```bash
cargo fmt --package tui-vfx-player --package tui-vfx-player-cli -- --check
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings
```

Run tests:

```bash
cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
cargo test --workspace
```

Run current report commands:

```bash
cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  > "$TMPDIR/tui-vfx-k24-render-report.json"

cargo run -q -p tui-vfx-player-cli -- inventory-recipes \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  > "$TMPDIR/tui-vfx-k24-inventory-report.json"

cargo run -q -p tui-vfx-player-cli -- migration-gap \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  > "$TMPDIR/tui-vfx-k24-migration-gap-report.json"

cargo run -q -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  > "$TMPDIR/tui-vfx-k24-visual-frame-report.json"

cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  > "$TMPDIR/tui-vfx-k24-primitive-adapter-gap-report.json"
```

Confirm recipe corpus was not modified:

```bash
git -C "$RECIPE_REPO" status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes
```

Expected output:

```text
# no output
```

Run diff hygiene:

```bash
git diff --check
```

## Review expectations

After implementation, run a focused review before final status:

```text
1. Verify substrate labels are honest.
2. Verify styleKnown is not true for row-derived placeholder style.
3. Verify no styled/color/role effect is marked rendered without real style evidence.
4. Verify all existing K2 commands still emit stable schema-labeled JSON.
5. Verify docs describe the limits clearly.
```

Then run an AI de-slop pass over touched files. The de-slop pass should remove confusing names, overbroad helper functions, stale comments, and unnecessary compatibility assumptions, but must not broaden scope.

## Draft implementation prompt

Use this prompt for the implementation agent:

```text
You are working in /usr/projects/tui-vfx.

Implement Phase K2.4 — Styled-Cell Visual Substrate Foundation.

Rolling context:
- K2.1 added migration-gap reporting.
- K2.2 added render-frame visual-frame reports derived from text-grid rows.
- K2.3 reduced primitive unsupported ids from 6 to 4 by adding honest text-grid adapters for mask.dissolve and sampler.ripple.
- Remaining unsupported ids are shader.borderSweep, shader.linearGradient, style.baseStyleOverride, and style.colorFade because they require styled-cell evidence.

Goal:
Add an honest styled-cell frame substrate to tui-vfx-player so VisualFrame sparse cells can carry real foreground/background/modifier/role evidence while preserving rows[], existing CLI commands, and all K0/K2 report authority.

Do not claim visual parity.
Do not modify recipes under ../tui-vfx-recipes/recipes/debug_recipes or ../tui-vfx-recipes/recipes/v3.1/debug_recipes.
Do not build the ratatui GUI in this packet.
Do not mark styled/color/role primitives as rendered unless real styled-cell evidence supports them.

Tasks:
1. Add or extend player-side styled-cell/styled-grid DTOs.
2. Preserve rows[] while emitting sparse visual cells from the styled substrate.
3. Make substrate, cellSource, and styleKnown precise and honest.
4. Add at least one test proving cells[] can contain non-placeholder style/modifier/role evidence.
5. Preserve render-recipe, inventory-recipes, migration-gap, render-frame, and primitive-adapter-gap commands.
6. Update docs/VOCABULARY.md.
7. Add docs/new_kernel/K2_4_STYLED_CELL_SUBSTRATE_EVIDENCE.md.
8. Add docs/new_kernel/PHASE_K2_4_STYLED_CELL_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md with a rolling context block.

Verification:
Use RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}.
Run fmt, clippy, player tests, CLI tests, workspace tests, the five player CLI report commands, git diff --check, and recipe-root cleanliness check.

Acceptance:
K2.4 passes only if at least one test proves real non-placeholder style/modifier/role evidence in VisualFrame cells[], rows[] remain present, substrate provenance is honest, and the four remaining styled-cell-blocked primitive ids are not falsely promoted.
```

## Expected K2.4 status memo summary shape

The final K2.4 memo should report:

```text
schemaVersion for render-frame
whether substrate stayed v3.1.player.visualFrameReport.1
rendered/unsupported/error counts
which frames or tests prove styled-cell evidence
whether styleKnown is true for any emitted frame
whether roles are real or still deferred
remaining unsupported primitive ids
verification matrix
recipe-root cleanliness
review/de-slop results
```

## Recommended next phase after K2.4

If K2.4 lands cleanly, proceed to:

```text
K2.5 — Styled Primitive Adapter Burn-down
```

K2.5 should attempt honest support for some or all of:

```text
shader.linearGradient
shader.borderSweep
style.baseStyleOverride
style.colorFade
```

Only proceed with K2.5 after K2.4 proves the player can emit real styled-cell evidence.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_3.md</FILE> - <DESC>Architect response and next work packet after K2.3 primitive adapter burn-down</DESC> -->

<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->

```
```
