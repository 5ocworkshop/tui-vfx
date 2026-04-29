# ARCH-RESP-TO-PHASE_K2_1

## Acceptance

**Accepted.**

K2.1 is a good report-only checkpoint. It gives us a planning control surface without changing recipes, without altering K1/compositor work, and without weakening the existing K0 CLI player/report behavior.

The important result is not that migration is “done”; it is that we now have a measurable migration gap:

```text
legacy debug recipes: 603
canonical v3.1 debug recipes: 16
represented families: 8
unrepresented families: 11
partially represented families: 7
K0/K2 renderable canonical fixtures: 10
K0/K2 unsupported canonical fixtures: 6
```

That is the right level of honesty.

## Acceptance notes

The report correctly treats legacy recipes as **inventory evidence**, not as canonical semantic truth. The path/family inventory is enough for planning migration order, but it should not be interpreted as semantic coverage or parity.

The recommended queue has two `complex` entries with different meanings: one is “create a minimal v3.1 complex fixture,” and the other is “choose broader complex legacy replacement candidates.” That is fine for K2.1, but future reports should label those separately, for example:

```text
complex-minimal-fixture
complex-corpus-candidate-selection
```

No blocker.

## Read of current state

We now have these layers in place:

```text
J0: primitive migration pilot + first contract validator
J1: hardened validator + recursive fixture harness
J2: shared primitive descriptor pack + second-ring migration batch
K0: CLI player path for canonical v3.1 fixtures
K2.0: render/inventory evidence over canonical fixtures
K2.1: migration-gap report comparing legacy debug corpus to canonical v3.1 corpus
```

The current v3.1 fixture corpus is structurally valid and descriptor-backed, but still only partially renderable through the K0 player path:

```text
canonical fixtures validate: yes
descriptor pack resolves: yes
render CLI can inspect corpus: yes
all canonical fixtures visually supported: no
visual parity with legacy: not claimed
```

That is exactly where we should be before expanding migration.

## Architectural decision

Proceed to:

```text
Phase K2.2 — Visual Frame Substrate + Stable Frame Evidence
```

This is **not** a replacement for K0. It is an additive enhancement on top of the established CLI player capability.

K0 already gives us player authority. K2.2 should make that authority more inspectable by adding a stable visual-frame artifact that both humans and later tools can consume.

The sequencing reason is simple: before migrating more recipe families, we need better evidence from the fixtures we already have. Text-row output is useful, but not sufficient for visual debugging because it loses or obscures important per-cell data such as foreground, background, modifiers, roles, dimensions, and frame metadata.

## Phase K2.2 — Visual Frame Substrate + Stable Frame Evidence

### Objective

Add a stable visual-frame reporting substrate to the v3.1 player stack so canonical fixtures can produce machine-readable and human-inspectable frame evidence without changing existing K0 `render-recipe` behavior. The current text-row CLI output remains the regression authority; K2.2 adds a richer frame report alongside it.

### Implementation assignment

Implement K2.2 in `/usr/projects/tui-vfx`.

Add a visual-frame DTO in `tui-vfx-player`, not in the recipes repo. The DTO should represent one rendered frame or one unsupported render attempt in a stable JSON shape.

Recommended schema label:

```text
v3.1.player.visualFrameReport.1
```

Recommended report shape:

```text
schemaVersion
root
descriptorPacks
summary
frames[]
```

Each frame entry should include, at minimum:

```text
recipePath
status                         # rendered | unsupported | error
phase
sampleT
absoluteTimeMs
width
height
renderHash
nonEmptyCells
rows[]                         # compact glyph rows, preserving current human-readable value
cells[]                        # sparse non-default cells
unsupportedEffectIds[]
errors[]
warnings[]
```

Each sparse cell should include:

```text
x
y
glyph
foreground
background
modifiers
role                           # if available from the player surface
```

The exact field names can be adjusted to match existing player naming conventions, but the output must be deterministic and JSON-stable.

Add a new CLI command rather than changing default `render-recipe` output:

```bash
cargo run -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json
```

Also support recursive mode:

```bash
cargo run -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

The new command should reuse the existing K0/K2 rendering path. Do not create a second renderer.

The current `render-recipe` command must remain unchanged by default. Its existing output shape and summary behavior should continue to pass all existing tests.

For unsupported fixtures, `render-frame` should report `status: "unsupported"` with the unsupported effect ids. It should not treat unsupported primitives as hard errors unless the existing K0 render path already classifies them as errors.

### Required inputs / context

Use these implementation repo files as the player/control surface context:

```text
/usr/projects/tui-vfx/crates/tui-vfx-player/src/lib.rs
/usr/projects/tui-vfx/crates/tui-vfx-player/src/cls_player_migration_gap_report.rs
/usr/projects/tui-vfx/crates/tui-vfx-player/src/fnc_collect_debug_recipe_family_inventory.rs
/usr/projects/tui-vfx/crates/tui-vfx-player/src/fnc_build_migration_gap_report.rs
/usr/projects/tui-vfx/crates/tui-vfx-player-cli/src/cls_cli_options.rs
/usr/projects/tui-vfx/crates/tui-vfx-player-cli/src/fnc_parse_cli_options.rs
/usr/projects/tui-vfx/crates/tui-vfx-player-cli/src/fnc_run.rs
/usr/projects/tui-vfx/crates/tui-vfx-player-cli/src/fnc_print_usage.rs
/usr/projects/tui-vfx/crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
```

Use this descriptor pack:

```text
/usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json
```

Use this canonical fixture root:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

Use this legacy root only for comparison/report checks, not for rendering or mutation:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes
```

### Non-goals

Do not modify old recipes:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/**/*
```

Do not modify canonical migrated recipes unless a test fixture is explicitly needed and justified:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/**/*
```

Do not broaden migration in K2.2.

Do not add new descriptor ids unless the visual-frame substrate cannot represent existing K2/J2 fixtures without them. If a descriptor gap is found, report it instead of silently expanding scope.

Do not replace `render-recipe`.

Do not claim visual parity.

Do not build a full GUI in K2.2. The ratatui GUI path remains additive tooling; K2.2 is the stable frame evidence substrate that GUI/CLI/reporting can consume.

Do not import or depend on the legacy recipe runtime from `/usr/projects/tui-vfx-recipes`. That repo remains fixture/evidence input.

### Required verification

Run the existing player checks:

```bash
cargo fmt --package tui-vfx-player -- --check
cargo fmt --package tui-vfx-player-cli -- --check
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings
cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
```

Re-run existing K0/K2 commands and confirm behavior did not regress:

```bash
cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  > /tmp/tui-vfx-k22-render-report.json
```

Expected baseline should remain consistent with K2.1 unless the implementation deliberately fixed an adapter:

```text
total=16
rendered=10
unsupported=6
errors=0
```

Run inventory:

```bash
cargo run -q -p tui-vfx-player-cli -- inventory-recipes \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  > /tmp/tui-vfx-k22-inventory-report.json
```

Run migration gap:

```bash
cargo run -q -p tui-vfx-player-cli -- migration-gap \
  --legacy-root /usr/projects/tui-vfx-recipes/recipes/debug_recipes \
  --v31-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  > /tmp/tui-vfx-k22-migration-gap-report.json
```

Run the new visual-frame command on one fixture:

```bash
cargo run -q -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json \
  > /tmp/tui-vfx-k22-baseline-frame.json
```

Run it recursively:

```bash
cargo run -q -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  > /tmp/tui-vfx-k22-visual-frame-report.json
```

Verify the new report has:

```text
schemaVersion=v3.1.player.visualFrameReport.1
total=16
rendered=10
unsupported=6
errors=0
```

Also verify the frame report contains deterministic `rows[]` and sparse `cells[]` for rendered fixtures.

Run workspace and diff checks:

```bash
cargo test --workspace
git diff --check
```

Confirm recipes were not modified:

```bash
git -C /usr/projects/tui-vfx-recipes status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes
```

Expected output:

```text
# no output
```

### Deliverables

Add or update:

```text
crates/tui-vfx-player/src/...
crates/tui-vfx-player-cli/src/...
crates/tui-vfx-player-cli/tests/...
docs/VOCABULARY.md
docs/new_kernel/PHASE_K2_2_VISUAL_FRAME_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md
docs/new_kernel/K2_2_VISUAL_FRAME_EVIDENCE.md
```

The status memo should include:

```text
new command shape
new schema label
report shape
rendered/unsupported/error counts
confirmation that render-recipe output was preserved
confirmation that inventory-recipes and migration-gap still pass
paths to captured JSON outputs
recipe-root modification check
verification commands and results
```

### Draft implementer prompt

```text
Implement Phase K2.2 — Visual Frame Substrate + Stable Frame Evidence.

Context:
- Implementation repo: /usr/projects/tui-vfx
- Recipe repo: /usr/projects/tui-vfx-recipes
- Canonical v3.1 fixture root: /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
- Legacy debug recipe root, evidence only: /usr/projects/tui-vfx-recipes/recipes/debug_recipes
- Descriptor pack: /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json

Goal:
Add a stable visual-frame JSON report on top of the existing K0/K2 player CLI capability. This must not replace or alter the existing render-recipe default behavior. The current text-row output remains regression authority. K2.2 adds richer frame evidence for human and machine inspection.

Add a new player report schema:

  v3.1.player.visualFrameReport.1

Add a new CLI command:

  cargo run -p tui-vfx-player-cli -- render-frame \
    --descriptor-pack descriptors/v3.1/packs/primitive.json \
    <recipe.json>

and recursive mode:

  cargo run -p tui-vfx-player-cli -- render-frame \
    --descriptor-pack descriptors/v3.1/packs/primitive.json \
    --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes

The report should include:
- schemaVersion
- root
- descriptorPacks
- summary
- frames[]

Each frame entry should include:
- recipePath
- status: rendered | unsupported | error
- phase
- sampleT
- absoluteTimeMs
- width
- height
- renderHash
- nonEmptyCells
- rows[] as compact glyph rows
- cells[] as sparse non-default cells
- unsupportedEffectIds[]
- errors[]
- warnings[]

Each sparse cell should include:
- x
- y
- glyph
- foreground
- background
- modifiers
- role, if available from the player surface

Rules:
- Reuse the existing K0 player render path. Do not create a separate renderer.
- Preserve render-recipe behavior and tests.
- Preserve inventory-recipes and migration-gap behavior.
- Unsupported fixtures should report unsupported, not become hard errors.
- Do not mutate old recipes.
- Do not mutate canonical v3.1 recipes unless explicitly justified.
- Do not broaden migration.
- Do not claim visual parity.
- Do not build a GUI in this phase.
- Do not add legacy recipe-runtime dependencies.

Required verification:
  cargo fmt --package tui-vfx-player -- --check
  cargo fmt --package tui-vfx-player-cli -- --check
  cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings
  cargo test -p tui-vfx-player
  cargo test -p tui-vfx-player-cli

  cargo run -q -p tui-vfx-player-cli -- render-recipe \
    --descriptor-pack descriptors/v3.1/packs/primitive.json \
    --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
    > /tmp/tui-vfx-k22-render-report.json

  cargo run -q -p tui-vfx-player-cli -- inventory-recipes \
    --descriptor-pack descriptors/v3.1/packs/primitive.json \
    --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
    > /tmp/tui-vfx-k22-inventory-report.json

  cargo run -q -p tui-vfx-player-cli -- migration-gap \
    --legacy-root /usr/projects/tui-vfx-recipes/recipes/debug_recipes \
    --v31-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
    --descriptor-pack descriptors/v3.1/packs/primitive.json \
    > /tmp/tui-vfx-k22-migration-gap-report.json

  cargo run -q -p tui-vfx-player-cli -- render-frame \
    --descriptor-pack descriptors/v3.1/packs/primitive.json \
    /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json \
    > /tmp/tui-vfx-k22-baseline-frame.json

  cargo run -q -p tui-vfx-player-cli -- render-frame \
    --descriptor-pack descriptors/v3.1/packs/primitive.json \
    --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
    > /tmp/tui-vfx-k22-visual-frame-report.json

  cargo test --workspace
  git diff --check
  git -C /usr/projects/tui-vfx-recipes status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes

Expected corpus counts should remain:
  total=16
  rendered=10
  unsupported=6
  errors=0

Deliver:
- code changes in tui-vfx-player and tui-vfx-player-cli
- tests for render-frame single file, recursive mode, unsupported reporting, and preservation of render-recipe behavior
- docs/VOCABULARY.md update for VisualFrame / frame evidence / visual parity distinction
- docs/new_kernel/K2_2_VISUAL_FRAME_EVIDENCE.md
- docs/new_kernel/PHASE_K2_2_VISUAL_FRAME_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md
```

## Notes for future phases

After K2.2, the likely next decision should be based on the frame evidence:

```text
K2.3 — Primitive Adapter Reduction
```

Goal: reduce the six unsupported canonical primitive ids before migrating more recipes.

The migration-gap report says broad migration pressure is large, especially:

```text
content: 111 legacy recipes, 0 v3.1
filters: 98 legacy, 4 v3.1
shaders/primitives: 94 legacy, 1 v3.1
complex: 83 legacy, 0 v3.1
```

But we should not chase those until the current 16 canonical fixtures have stronger visual evidence and fewer unsupported player gaps.
