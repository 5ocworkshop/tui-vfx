# ARCH-RESP-TO-PHASE_K2_18.md

## Review verdict

**ACCEPT K2.18, but the next cycle must produce visible end-to-end results.**

K2.18 did something useful: it stopped pretending unresolved work was implemented. It closed generic implementation queues and moved unresolved items into exact signed dispositions.

Current result:

```text
canonical fixtures:                 144
validate-recipe:                    144/144 valid
render-recipe:                      144/144 rendered, 0 unsupported, 0 errors
render-frame:                       144/144 rendered, 0 unsupported, 0 errors
fixture-qc:                         pass
primitive-field-coverage:           908/908 handled
primitive-adapter-gap:              75/75 rendered
schema-readiness:                   canDeclareSchemaReady=true
implementationBlocking:             0
explicitOwnerDecisionNeeded:         0

remaining exact signed work:
  descriptorBacklogSignedOff:        51
  backendHoldbackSignedOff:          118
```

That is an honest state. Now we need to stop making the user read more reports about why visual parity is not done. The next packet must produce **working, visible playback** and a **basic schema-generated studio control surface**.

The user’s stated priority is now:

```text
1. See colors.
2. See proper effects.
3. See real recipe playback through player IR into compositor.
4. See at least a basic studio UI/control example generated from schema/descriptor data.
5. Receive successful results first; supporting detail second.
```

That is the packet.

---

# Phase K2.19 — End-to-End Compositor Playback + Studio Control Pilot

## Executive goal

Deliver the first real v3.1 visual playback path:

```text
RecipeDocument v3.1
  -> RecipePlayer
  -> PlayerRenderIrReport
  -> PlayerRenderBackend
  -> compositor backend adapter
  -> colored/effectful terminal output or ratatui preview
```

And deliver the first generated studio control pilot:

```text
recipe + descriptor/schema metadata
  -> control catalog
  -> generated controls
  -> user/script changes value
  -> player re-renders
  -> preview/hash/output changes
```

This packet is not a planning packet. It is a **results packet**.

The final memo must begin with the successful visible results:

```text
- exact commands run
- exact recipes rendered
- backend used
- screenshots / ANSI snapshots / rendered output paths
- before/after studio control changes
- proof that visual output changed
```

Supporting reports come after that.

---

# Non-negotiable output standard

The implementer must not return a memo that only says “backend adapter is planned” or “studio controls are described.”

A successful packet must let the user run commands that visibly show compositor-backed color/effects.

A successful packet must include at least one generated studio/control example where changing a control changes the rendered output.

A failed packet must say **FAILED** at the top and list exactly what prevented visible playback. Do not bury failure behind process summaries.

---

# Required headline deliverables

## Deliverable 1 — Compositor backend adapter

Implement a player-owned backend path that can render at least a bounded set of v3.1 recipes through the existing compositor stack.

Preferred shape:

```text
crates/tui-vfx-player-backend-compositor/
```

or an equivalent backend module that preserves the boundary:

```text
tui-vfx-player-ui must not construct compositor internals.
tui-vfx-player core should not become a dumping ground for compositor-specific details.
The adapter should depend on player IR and compositor/types.
```

The backend must consume:

```text
PlayerRenderIrReport
```

and produce:

```text
PlayerRenderBackendOutput
```

with colored/styled cells or ANSI/render snapshot output.

## Deliverable 2 — CLI command for visual backend output

Add a command that renders a recipe through a chosen backend:

```bash
cargo run -q -p tui-vfx-player-cli -- render-backend \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --format ansi \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json"
```

Acceptable command names are flexible, but the result must be obvious and runnable.

Required formats:

```text
json      machine evidence
ansi      colored terminal output
text      fallback debug snapshot
```

JSON output must include at least:

```text
schemaVersion
backend
recipeId
recipePath
sample
rows
cells or styledCells
renderHash
backendHash
nonDefaultStyledCells
warnings
errors
```

ANSI output must visibly show color/style/effect evidence in a terminal.

## Deliverable 3 — Ratatui player UI backend selector

Add a minimal backend selector to the player UI:

```bash
cargo run -q -p tui-vfx-player-ui -- \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipes-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json" \
  --backend compositor
```

At minimum, the UI must be able to display compositor-backed colored/styled cells for supported recipes.

Do not let the UI construct compositor DTOs. The UI consumes player/backend output.

## Deliverable 4 — Working visual demo set

Render at least these recipe categories through the compositor backend:

```text
source/card/text baseline
filter/color style
mask/wipe or checkers
sampler/ripple or sineWave, if available through backend
shader/linearGradient
shader/borderSweep or highlighter
style/modulo or fade
scene/source fixture if feasible
```

Required minimum demo recipe list:

```text
baseline.json
filters/filter_tint.json
masks/mask_wipe.json
masks/mask_checkers.json
shaders/primitives/shader_linear_gradient_apply_to_both.json
shaders/primitives/shader_linear_gradient_diagonal.json
shaders/compositions/shader_border_sweep_position_binding.json
styles/style_modulo_horizontal_every_third_row.json
```

Use exact existing canonical paths under:

```text
$RECIPE_REPO/recipes/v3.1/debug_recipes/
```

Substitute only when a listed fixture path does not exist, and document the replacement.

## Deliverable 5 — Studio control pilot

Add a minimal generated control surface.

Acceptable initial shape:

```bash
cargo run -q -p tui-vfx-player-cli -- studio-snapshot \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json" \
  --backend compositor \
  --json
```

Or via UI snapshot/script mode:

```bash
cargo run -q -p tui-vfx-player-ui -- \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipes-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json" \
  --backend compositor \
  --studio \
  --script "set sweep_progress 0.75; render"
```

The result must prove:

```text
controls are derived from descriptor/schema/recipe data;
a numeric input becomes a slider or numeric field;
an enum input becomes a select/radio field where applicable;
a color input becomes a color/token field or placeholder control;
changing a generated value changes render output/hash;
the control panel does not infer effect behavior from raw ad-hoc recipe internals.
```

Required minimum studio examples:

```text
shader_border_sweep_position_binding:
  generated control: sweep_progress / position
  change value: 0.0 -> 0.75
  render hash changes

filter_pill_button_progress_binding:
  generated control: demo_progress / progress
  change value: 0.0 -> 1.0
  render hash changes

shader_linear_gradient_apply_to_both:
  generated controls: applyTo enum + gradient editor placeholder
  no full editor required, but catalog/control row must exist
```

---

# Work model: 10 result-focused lanes

```text
A. Results harness and demo commands
B. Backend adapter crate/seam
C. Player IR -> SemanticScene / OwnedGrid lowering
D. Recipe graph/effect -> CompositionSpec lowering
E. Compositor backend execution and output formats
F. CLI visual playback commands
G. Ratatui player UI backend integration
H. Studio control pilot
I. Demo fixtures/results/screenshots
J. QA, docs, de-slop, final results memo
```

Each lane must report **results first**:

```text
what works
exact command
exact output artifact
exact tests passed
remaining limits
```

Do not submit lane memos that only discuss architecture.

---

# Lane A — Results harness and demo commands

## Objective

Create a reproducible results harness that proves visual playback and studio control behavior.

## Required output directory

Use:

```text
/tmp/k219-visual-results/
```

Store:

```text
compositor JSON outputs
ANSI snapshots
text snapshots
studio control JSON
studio before/after render hashes
UI snapshot output if available
```

## Required script

Add or document a runnable script:

```bash
./scripts/k219_visual_demo.sh
```

or an equivalent checked-in command list in the status memo.

It must run:

```text
1. render compositor backend for demo recipes
2. render ANSI output for at least 3 recipes
3. render timeline samples for at least 2 animated/effect recipes
4. run studio control before/after for at least 2 recipes
5. print a final pass/fail table
```

## Acceptance

The final memo must include a table like:

| result                     | command | artifact                                            | pass/fail |
| -------------------------- | ------- | --------------------------------------------------- | --------- |
| compositor gradient        | `...`   | `/tmp/k219-visual-results/gradient.ansi`            | PASS      |
| compositor mask wipe       | `...`   | `/tmp/k219-visual-results/mask_wipe.json`           | PASS      |
| studio border sweep slider | `...`   | `/tmp/k219-visual-results/studio_border_sweep.json` | PASS      |

No artifact means no pass.

---

# Lane B — Backend adapter crate/seam

## Objective

Implement the compositor backend behind the player backend seam.

## Required architecture

Preferred structure:

```text
crates/tui-vfx-player-backend-compositor/
  src/lib.rs
  src/fnc_render_compositor_backend.rs
  src/fnc_lower_player_ir_to_semantic_scene.rs
  src/fnc_lower_recipe_graph_to_composition_spec.rs
```

Alternative structure is acceptable only if the dependency boundary remains clean.

Required boundary:

```text
tui-vfx-player:
  owns PlayerRenderIrReport
  owns PlayerRenderBackend trait / backend-neutral output

tui-vfx-player-backend-compositor:
  depends on tui-vfx-player
  depends on tui-vfx-types
  depends on tui-vfx-compositor
  lowers IR/recipe evidence into SemanticScene/CompositionSpec

tui-vfx-player-ui:
  consumes player/backend output
  no compositor DTO construction
```

## Required trait behavior

If `PlayerRenderBackend` already exists, implement it. If not, define the minimal stable trait:

```rust
pub trait PlayerRenderBackend {
    fn render(&self, input: &PlayerRenderIrReport) -> PlayerRenderBackendOutput;
}
```

`PlayerRenderBackendOutput` must include:

```text
backend id
rows
styled cells
render hash
warnings
errors
backend metadata
```

## Acceptance

```text
- compositor backend can be invoked from CLI
- text/styled-cell backend still works
- UI does not import compositor internals
- player core dependency direction stays sane
```

---

# Lane C — Player IR to SemanticScene / OwnedGrid lowering

## Objective

Lower `PlayerRenderIrReport` into compositor-ready data:

```text
PlayerRenderIrReport
  -> OwnedGrid
  -> RoleMap
  -> SemanticScene
```

## Required mapping

Map:

```text
text rows -> glyph cells
styled cells -> foreground/background/modifiers
roles -> RoleMap
scene/source provenance -> metadata or diagnostics only, not role identity
transparent/empty cells -> preserve/skip semantics
warnings/errors -> backend diagnostics
```

## Required role rule

Do not overload element identity into role identity.

```text
role: text, border, background, content, shadow
element: card, logo, spinner, ansi_layer
```

## Acceptance tests

Add tests proving:

```text
IR rows become OwnedGrid cells
styled foreground/background survive lowering
role map survives lowering
transparent/empty skip policy is preserved
scene provenance does not become role tags
```

---

# Lane D — Recipe graph/effect to CompositionSpec lowering

## Objective

Lower a bounded supported set of v3.1 graph/effect data into compositor `CompositionSpec`.

This is the first end-to-end backend slice. It does not need to support every descriptor.

## Required supported effects for this packet

Minimum support:

```text
filter.dim
filter.tint
filter.greyscale
filter.invert

mask.none
mask.wipe
mask.checkers
mask.dissolve if already straightforward

sampler.sineWave
sampler.ripple if feasible

shader.linearGradient
shader.borderSweep or shader.highlighter

style.baseStyleOverride
style.colorFade or existing style/base style evidence
```

Use descriptors already present and already player-handled when possible.

## Required behavior

For each supported effect:

```text
- resolve authored inputs through player value resolution
- map to CompositionSpec field or explicit backend adapter path
- emit warning if backend ignores a supported player-only field
- do not mark unsupported backend effects as rendered
```

## Important constraint

Do not mutate v3.1 DTOs to fit compositor internals.

The adapter lowers from accepted v3.1/player IR into compositor-compatible IR.

## Acceptance

At least these recipe categories must show nontrivial backend effects:

```text
linear gradient changes color cells
border sweep/highlighter changes color cells across samples
wipe/checkers changes visible cell coverage
tint/dim changes style values
```

---

# Lane E — Compositor backend execution and output formats

## Objective

Actually call the compositor and return visible output.

Use existing seams such as:

```text
tui_vfx_types::OwnedGrid
tui_vfx_types::SemanticScene
tui_vfx_compositor::pipeline::CompositionSpec
tui_vfx_compositor::pipeline::render_pipeline_with_spec
```

Use the exact available API names in the repo.

## Required outputs

Support:

```text
JSON backend output
ANSI colored output
plain text fallback
```

ANSI output may be approximate, but it must visibly encode foreground/background colors.

## Required hashes

Output must include:

```text
renderHash
backendHash
styleHash or cellHash if practical
nonDefaultStyledCells
changedCells for before/after studio demos
```

## Acceptance

```text
- compositor backend returns styled/color cells
- at least 3 demo recipes have nonDefaultStyledCells > 0
- at least 2 demo recipes produce different backendHash at different samples or control values
- backend errors are structured, not panics
```

---

# Lane F — CLI visual playback commands

## Objective

Expose the result through CLI.

## Required commands

Add or extend commands to support:

```bash
render-backend --backend compositor --format json
render-backend --backend compositor --format ansi
render-backend --backend compositor --sample-ms <N>
render-backend-timeline --backend compositor
```

Exact names can differ, but all functions must exist.

## Example commands required in final memo

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}

cargo run -q -p tui-vfx-player-cli -- render-backend \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --format ansi \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json"

cargo run -q -p tui-vfx-player-cli -- render-backend \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --format json \
  --sample-ms 750 \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json"

cargo run -q -p tui-vfx-player-cli -- render-backend-timeline \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --samples 5 \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/masks/mask_wipe.json"
```

## Acceptance

The final memo must show the commands and report pass/fail for each.

---

# Lane G — Ratatui player UI backend integration

## Objective

Make the player UI display compositor-backed output for supported recipes.

## Required behavior

Add:

```text
--backend text
--backend styled
--backend compositor
```

or equivalent.

UI must display:

```text
backend id
recipe id
sample time/phase
render hash/backend hash
warning count
styled/non-default cell count
```

## Required screenshot/snapshot mode

There must be a non-interactive snapshot mode for automated proof:

```bash
cargo run -q -p tui-vfx-player-ui -- \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipes-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json" \
  --backend compositor \
  --once
```

or:

```bash
just player-ui-once BACKEND=compositor RECIPE=...
```

## Acceptance

```text
- UI compiles and tests pass
- UI can render a compositor-backed preview
- UI does not construct compositor DTOs
- TestBackend or snapshot test proves draw succeeds
```

---

# Lane H — Studio control pilot

## Objective

Deliver a basic generated studio UI/control flow.

This is a pilot, not a full studio.

## Required control source

Controls must derive from:

```text
descriptor pack inputs
source descriptor inputs
effect descriptor inputs
graph parameters/signals where present
ValueKind
ValueSpec/range/allowedValues/unit/semantic/runtimeMutability/bindable/optional
```

Do not infer controls from random raw recipe payload keys without descriptor backing.

## Required generated controls

At minimum:

```text
number/range -> slider + numeric input
enum -> select
boolean -> toggle, if present in demo
color -> color/token control or placeholder row
gradient -> gradientEditor placeholder row
binding-capable input -> binding picker marker
optional input -> enable/disable marker
```

## Required commands

Add a command or UI snapshot that produces a control panel:

```bash
cargo run -q -p tui-vfx-player-cli -- studio-snapshot \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json" \
  --backend compositor \
  --json
```

Add support for scripted control mutation:

```bash
cargo run -q -p tui-vfx-player-cli -- studio-snapshot \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json" \
  --backend compositor \
  --set sweep_progress=0.75 \
  --json
```

Equivalent UI command is acceptable.

## Required proof

For each studio demo, record:

```text
control id
control kind
source descriptor/effect input
before value
after value
before backendHash
after backendHash
changedCells
```

## Minimum studio demos

```text
1. shader_border_sweep_position_binding
   control: sweep_progress / position
   before: 0.0
   after: 0.75
   hash must change

2. filter_pill_button_progress_binding
   control: demo_progress / progress
   before: 0.0
   after: 1.0
   hash must change

3. shader_linear_gradient_apply_to_both
   controls: applyTo enum and gradientEditor placeholder
   catalog/control rows must exist
```

## Acceptance

```text
- control catalog is recipe-specific
- generated controls are visible in CLI or UI snapshot
- setting at least two controls changes compositor backend output
- studio does not bypass descriptor/schema metadata
```

---

# Lane I — Demo fixtures, result artifacts, and visible proof

## Objective

Produce a user-facing demo bundle.

## Required artifacts

Create:

```text
/tmp/k219-visual-results/README.md
/tmp/k219-visual-results/baseline.ansi
/tmp/k219-visual-results/filter_tint.ansi
/tmp/k219-visual-results/mask_wipe_timeline.json
/tmp/k219-visual-results/mask_checkers.ansi
/tmp/k219-visual-results/linear_gradient.ansi
/tmp/k219-visual-results/border_sweep_sample_0.json
/tmp/k219-visual-results/border_sweep_sample_750.json
/tmp/k219-visual-results/studio_border_sweep_before_after.json
/tmp/k219-visual-results/studio_pill_button_before_after.json
/tmp/k219-visual-results/studio_gradient_controls.json
```

Use whatever exact filenames are produced, but the final memo must link/list them.

## Required visual result claims

Only claim success when:

```text
ANSI output exists
backend JSON exists
nonDefaultStyledCells > 0 for color/effect demos
backendHash changes for animated/control demos
warnings/errors are understood
```

## Acceptance

The final memo starts with this section:

```text
## Successful visible results
```

and lists concrete outputs.

---

# Lane J — QA, docs, de-slop, and final results memo

## Objective

Keep gates green while prioritizing working output.

## Required docs

Create:

```text
docs/new_kernel/K2_19_COMPOSITOR_BACKEND_RESULTS.md
docs/new_kernel/K2_19_PLAYER_IR_TO_COMPOSITOR_LOWERING.md
docs/new_kernel/K2_19_STUDIO_CONTROL_PILOT_RESULTS.md
docs/new_kernel/K2_19_VISUAL_DEMO_COMMANDS.md
docs/new_kernel/K2_19_BACKEND_LIMITATIONS_AND_HOLDBACKS.md
docs/new_kernel/K2_19_SCHEMA_API_DOCS_GATE.md
docs/new_kernel/PHASE_K2_19_VISUAL_PLAYBACK_STATUS_MEMO_TO_ARCHITECT.md
docs/new_kernel/PHASE_K2_19_REVIEW_AND_DESLOP_REPORT.md
```

Update if touched:

```text
docs/VOCABULARY.md
docs/v3.1-feature-contract-checklist.md
docs/new_kernel/INDEX.md
```

## Final memo required order

The final memo must be ordered exactly like this:

```text
1. SUCCESSFUL VISIBLE RESULTS
2. USER-RUNNABLE COMMANDS
3. STUDIO CONTROL RESULTS
4. WHAT WORKS END-TO-END
5. WHAT DOES NOT WORK YET
6. VERIFICATION MATRIX
7. FILES/CRATES TOUCHED
8. REVIEW AND DE-SLOP RESULTS
9. RECOMMENDED NEXT PACKET
```

Do not lead with background or process.

## Required honesty

The memo must explicitly state:

```text
Compositor-backed output works for the bounded supported set.
It is not full visual parity for every legacy recipe.
Unsupported backend effects remain signed holdbacks.
The UI consumes backend output and does not construct compositor internals.
Studio pilot is basic and generated from descriptors/schema, not a full authoring app.
```

---

# Acceptance criteria

## Required

```text
- At least 8 canonical recipes render through compositor backend.
- At least 3 compositor backend ANSI outputs visibly include color/styled cells.
- At least 2 animated/effect recipes produce different backendHash values across samples.
- At least 2 studio-generated controls can be changed and cause output/hash changes.
- Player UI can display compositor backend output in snapshot or interactive mode.
- CLI can render backend output as JSON and ANSI.
- PlayerRenderIrReport remains the handoff into backend rendering.
- UI does not construct compositor internals.
- validate-recipe remains green for full canonical corpus.
- render-recipe remains green for full canonical corpus.
- render-frame remains green for full canonical corpus.
- fixture-qc remains pass.
- primitive-field-coverage remains 0 unhandled.
- primitive-adapter-gap remains 0 unresolved for player evidence.
- implementation-readiness remains implementationBlocking=0.
```

## Preferred

```text
- 12+ recipes render through compositor backend.
- 5+ recipes have visible nonDefaultStyledCells > 0.
- Ratatui interactive playback works with --backend compositor.
- Studio snapshot shows controls beside preview.
- Studio control script can modify a slider and update preview in one command.
- A small demo README under /tmp/k219-visual-results explains exactly what to run.
```

## Hard stop conditions

Stop and report failure instead of pretending success if:

```text
- compositor backend cannot be called through player backend seam;
- UI must construct compositor DTOs to make output work;
- outputs are still plain rows with no styled/color cells;
- studio controls are hard-coded instead of descriptor/schema-derived;
- changing a studio control does not change output/hash;
- backend output panics on unsupported effects instead of structured warnings/errors;
- field coverage is kept green by marking unhandled fields as handled.
```

---

# Verification commands

Use portable paths:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
```

## Format and lint

```bash
cargo fmt \
  --package tui-vfx-player \
  --package tui-vfx-player-cli \
  --package tui-vfx-player-ui \
  --package tui-vfx-contract \
  --package tui-vfx-contract-cli \
  -- --check

cargo clippy \
  -p tui-vfx-player \
  -p tui-vfx-player-cli \
  -p tui-vfx-player-ui \
  -p tui-vfx-contract \
  -p tui-vfx-contract-cli \
  --all-targets --all-features -- -D warnings
```

Add compositor backend package to these commands if a new crate is created:

```bash
cargo fmt --package tui-vfx-player-backend-compositor -- --check

cargo clippy \
  -p tui-vfx-player-backend-compositor \
  --all-targets --all-features -- -D warnings
```

## Tests

```bash
cargo nextest run -p tui-vfx-player --no-fail-fast
cargo nextest run -p tui-vfx-player-cli --no-fail-fast
cargo nextest run -p tui-vfx-player-ui --no-fail-fast
cargo nextest run -p tui-vfx-contract --no-fail-fast
cargo nextest run -p tui-vfx-contract-cli --no-fail-fast
cargo nextest run --workspace --no-fail-fast
```

Fallback:

```bash
cargo test --workspace
```

## Existing corpus gates

```bash
cargo run -q -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- fixture-qc \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-field-coverage \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- implementation-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --include-blockers \
  --json
```

## New visual backend gates

Use exact command names implemented by the packet.

Expected examples:

```bash
cargo run -q -p tui-vfx-player-cli -- render-backend \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --format ansi \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json" \
  > /tmp/k219-visual-results/linear_gradient.ansi

cargo run -q -p tui-vfx-player-cli -- render-backend \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --format json \
  --sample-ms 750 \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json" \
  > /tmp/k219-visual-results/border_sweep_sample_750.json

cargo run -q -p tui-vfx-player-cli -- render-backend-timeline \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --samples 5 \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/masks/mask_wipe.json" \
  > /tmp/k219-visual-results/mask_wipe_timeline.json
```

## New studio gates

Expected examples:

```bash
cargo run -q -p tui-vfx-player-cli -- studio-snapshot \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json" \
  --backend compositor \
  --json \
  > /tmp/k219-visual-results/studio_border_sweep_controls.json

cargo run -q -p tui-vfx-player-cli -- studio-snapshot \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json" \
  --backend compositor \
  --set sweep_progress=0.75 \
  --json \
  > /tmp/k219-visual-results/studio_border_sweep_after.json
```

## UI backend gate

```bash
cargo run -q -p tui-vfx-player-ui -- \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipes-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json" \
  --backend compositor \
  --once
```

## Docs and cleanliness

```bash
cargo xtask docs generate
cargo xtask docs check
cargo xtask docs api
cargo xtask docs api-check
cargo xtask docs api-validate
cargo xtask audit configschema

git diff --check

git -C "$RECIPE_REPO" status --short -- recipes/debug_recipes
```

Legacy root must remain untouched.

---

# Expected final memo

Return:

```text
docs/new_kernel/PHASE_K2_19_VISUAL_PLAYBACK_STATUS_MEMO_TO_ARCHITECT.md
```

The memo must begin with:

```text
# Successful visible results
```

and include:

```text
- exact recipes rendered through compositor backend
- exact commands
- exact artifacts
- nonDefaultStyledCells counts
- before/after backend hashes for studio controls
- UI snapshot/playback proof
- unsupported backend effects, if any
- verification matrix
- review/de-slop results
- next recommended packet
```

Do not lead with summaries. Lead with results.

---

# What this packet should close out

A successful K2.19 closes the “we only have text-grid/player evidence” concern for a bounded set of recipes.

It will not complete all visual parity, but it should prove the full shape works:

```text
recipe -> player IR -> compositor backend -> visible colored/effectful playback
```

It should also close the first studio-control uncertainty by proving:

```text
schema/descriptors -> generated controls -> changed value -> changed render
```

After K2.19, the remaining work becomes much clearer:

```text
1. Expand compositor lowering to more descriptor families.
2. Move backendHoldbackSignedOff families into real backend evidence.
3. Add richer studio controls and live interaction.
4. Add visual review/oracle comparison.
5. Implement template expansion above canonical recipe validation.
```

The immediate priority is not more classification. The immediate priority is visible playback and a working generated control pilot.
