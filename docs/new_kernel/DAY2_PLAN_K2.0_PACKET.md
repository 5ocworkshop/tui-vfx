## Day plan

Today should be treated as a **control-surface day**, not a blind migration day. The current blocker is not that we lack recipes; it is that we do not yet have a repeatable way to answer:

```text
Which v3.1 fixtures exist?
Which descriptor ids do they exercise?
Which source ids do they require?
Which primitives are descriptor-covered?
Which primitives are adapter-backed?
Which recipes render, are unsupported, or error?
Which old debug families are not represented yet?
```

Once that exists, compositor wiring and migration can proceed in a measured way.

---

# Target state by end of day

The desired end-of-day state should be:

```text
K0 remains the automation authority.
K1 remains a thin visual shell over K0.
A machine-readable inventory gate exists for v3.1 debug_recipes.
The migration gap from old debug_recipes to v3.1 is explicit.
The unsupported primitive list is explicit.
The first compositor-backed visual-frame plan is scoped.
The first complex fixture path is identified.
The next recipe-family migration roadmap is documented.
```

I would not start by porting the full old corpus. I would first make the corpus measurable, then expand adapters and recipes against that measurement.

---

# Work sequence

## Block 1 — Add K0 v3.1 fixture inventory gate

This is the first packet.

Add a report-only command or mode to the K0 player CLI that inventories canonical v3.1 debug fixtures and descriptor coverage without changing render behavior.

It should report, per recipe:

```text
path
recipe id
source ids used
graph effect ids used
descriptor coverage for each source/effect id
current K0 render status
unsupported adapter diagnostics
```

And aggregate:

```text
total fixtures
rendered count
unsupported count
error count
descriptor effect ids represented by fixtures
descriptor effect ids not represented by fixtures
source ids represented
unsupported effect ids
```

This becomes the day’s control surface.

Expected result right now should remain approximately:

```text
total=16
rendered=10
unsupported=6
errors=0
descriptor effect ids represented=14/14
```

No compositor work in this packet.

---

## Block 2 — Document migration gap from old debug_recipes

Once the inventory gate exists, add or generate a lightweight migration-gap memo/report that compares:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

The current gap should be represented at the family level:

```text
content        old present, v3.1 absent
scene          old present, v3.1 absent
shadows        old present, v3.1 absent
complex        old present, v3.1 absent
signals        old present, v3.1 absent
easings        old present, v3.1 absent
subcell_shapes old present, v3.1 absent
motion_routes  old present, v3.1 absent
loopback       old present, v3.1 absent
```

This should not migrate files yet. It should establish the roadmap queue.

---

## Block 3 — Prepare K0 visual-frame substrate

After inventory is stable, add a K0-owned visual frame substrate while preserving the current text rows.

The key architectural move is:

```text
source.card / source.text
  -> OwnedGrid + RoleMap
  -> SemanticScene
```

The existing `Vec<String>` rows should remain available for CLI regression. Do not break existing hashes/statuses unless the report schema is explicitly versioned.

This packet should prepare the player to eventually call:

```text
render_pipeline_with_spec(...)
```

but it does not need to implement all effect lowering yet.

---

## Block 4 — First compositor-backed primitive adapters

Add a small adapter registry that maps canonical v3.1 effect ids to compositor `CompositionSpec` pieces.

Start with primitives that already exist in the descriptor pack and fixtures:

```text
mask.none
mask.wipe
mask.checkers
sampler.sineWave
filter.dim or filter.tint
```

Then expand to:

```text
filter.greyscale
filter.invert
mask.dissolve
sampler.ripple
shader.linearGradient
shader.borderSweep
style.baseStyleOverride
style.colorFade
```

The success metric should be inventory-driven: unsupported count decreases only when a real adapter exists.

---

## Block 5 — K1 cell-aware preview

Once K0 can return visual cells, teach K1 to display those cells in ratatui instead of only displaying `rows.join("\n")`.

K1 should still call K0. It should not create a second render path.

The preview behavior should be:

```text
if K0 visual cells exist:
    blit cells into ratatui preview area
else:
    display legacy K0 text rows
```

---

## Block 6 — First complex v3.1 fixture

Do not start with the old `complex_full_pipeline` as-is.

Create a canonical minimal complex fixture first:

```text
recipes/v3.1/debug_recipes/complex/complex_minimal_pipeline.json
```

It should exercise the currently representable v3.1 primitive families:

```text
source.card
mask.wipe
sampler.sineWave
filter.tint or filter.dim
shader.linearGradient
style.baseStyleOverride or style.colorFade
```

Only after that works visually should the old `complex_full_pipeline` be migrated as a parity/replacement candidate.

---

# First work packet

## Packet name

```text
K2.0 — K0 v3.1 Debug Fixture Inventory Gate
```

## Objective

Add a repeatable, machine-readable K0 inventory/reporting gate for canonical v3.1 debug recipes.

This packet must not change rendering semantics. It should only make the current fixture, descriptor, source, effect, and adapter-support status visible.

## Architectural rule

K0 remains the automation authority.

Do not make K1 responsible for discovering recipe coverage. K1 can display information later, but the inventory report belongs in the K0 player/CLI path.

## Implementation repo

```text
/usr/projects/tui-vfx
```

## Recipe corpus

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

## Descriptor pack

```text
/usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json
```

## Likely files to touch

```text
crates/tui-vfx-player/src/
crates/tui-vfx-player-cli/src/fnc_run.rs
crates/tui-vfx-player-cli/src/fnc_parse_cli_options.rs
crates/tui-vfx-player-cli/src/fnc_print_usage.rs
crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
```

Add new K0 library files if that keeps OFPF sizing clean, for example:

```text
crates/tui-vfx-player/src/cls_player_inventory_report.rs
crates/tui-vfx-player/src/cls_player_inventory_recipe.rs
crates/tui-vfx-player/src/fnc_inventory_recipe_file.rs
crates/tui-vfx-player/src/fnc_inventory_recipe_paths.rs
```

Use exact names that fit the repo’s OFPF conventions.

## Suggested CLI shape

Keep the existing render command intact.

Add either a new command:

```bash
cargo run -p tui-vfx-player-cli -- inventory-recipes \
  --recursive \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

Or a render submode:

```bash
cargo run -p tui-vfx-player-cli -- render-recipe \
  --inventory \
  --recursive \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

I prefer a separate `inventory-recipes` command because it is report-only and should not overload frame rendering.

## Required report shape

The report should be JSON by default, similar to the existing player reports.

Suggested schema label:

```text
v3.1.player.inventory.1
```

Top-level shape:

```json
{
  "schemaVersion": "v3.1.player.inventory.1",
  "root": "...",
  "descriptorPacks": [
    {
      "id": "v3.1.primitive",
      "path": "descriptors/v3.1/packs/primitive.json"
    }
  ],
  "summary": {
    "totalRecipes": 16,
    "rendered": 10,
    "unsupported": 6,
    "errors": 0,
    "descriptorEffectIds": 14,
    "representedEffectIds": 14,
    "unrepresentedEffectIds": 0,
    "unsupportedEffectIds": 6,
    "sourceIds": 1
  },
  "recipes": [],
  "effects": [],
  "sources": []
}
```

Per recipe:

```json
{
  "path": ".../filters/filter_dim.json",
  "recipeId": "debugFilterDim",
  "status": "rendered",
  "sourceIds": ["source.card"],
  "effectIds": ["filter.dim"],
  "descriptorCoveredEffectIds": ["filter.dim"],
  "missingDescriptorEffectIds": [],
  "unsupportedEffectIds": [],
  "errors": []
}
```

Per effect aggregate:

```json
{
  "id": "shader.linearGradient",
  "descriptorCovered": true,
  "representedByRecipes": true,
  "adapterStatus": "unsupported",
  "recipePaths": [
    ".../shaders/primitives/shader_linear_gradient.json"
  ]
}
```

Per source aggregate:

```json
{
  "id": "source.card",
  "descriptorCovered": true,
  "representedByRecipes": true,
  "adapterStatus": "supported",
  "recipePaths": []
}
```

## Adapter-status classification

For now, derive adapter status from K0 behavior, not from wishful descriptor presence.

The current expected classification is:

```text
supportedVisible:
  mask.wipe
  mask.checkers

supportedNoopSmoke:
  filter.dim
  filter.greyscale
  filter.invert
  filter.tint
  mask.none
  sampler.sineWave

unsupported:
  mask.dissolve
  sampler.ripple
  shader.borderSweep
  shader.linearGradient
  style.baseStyleOverride
  style.colorFade
```

The report can serialize this as:

```text
visible
noop
unsupported
missingDescriptor
error
```

or similar. The important thing is to distinguish descriptor coverage from runtime adapter support.

## Required behavior

The inventory command should:

```text
load descriptor packs through existing K0 load_descriptor_catalog
collect recipe paths through existing K0 collect_recipe_paths
deserialize canonical RecipeDocument JSON
collect source ids from recipe.sources
collect effect ids from recipe.graph.nodes
validate descriptor coverage against DescriptorCatalog
render through existing RecipePlayer path to get current status
include unsupported adapter diagnostics from PlayerFrameReport.errors
aggregate effect/source coverage
print stable JSON
```

Do not duplicate validation logic that already exists in `RecipeDocument::validate_with_catalog` or `RecipePlayer::render_recipe`.

## Non-goals

Do not implement new visual adapters in this packet.

Do not modify recipes.

Do not modify the descriptor pack unless the inventory code proves an actual malformed descriptor.

Do not wire the compositor yet.

Do not change K1.

Do not remove or replace current `render-recipe`.

## Acceptance criteria

The packet is acceptable when:

```text
existing K0 render CLI still works
existing K1 UI tests still pass
new inventory command emits JSON
inventory command works recursively on v3.1 debug_recipes
report shows 16 current canonical v3.1 fixtures
report shows every primitive descriptor effect id represented by at least one fixture
report separates descriptor coverage from runtime adapter support
report lists the current unsupported effect ids explicitly
report has tests for at least baseline, one rendered primitive, and one unsupported primitive
```

Expected current unsupported ids:

```text
mask.dissolve
sampler.ripple
shader.borderSweep
shader.linearGradient
style.baseStyleOverride
style.colorFade
```

## Verification commands

Run from:

```text
/usr/projects/tui-vfx
```

Commands:

```bash
cargo fmt --package tui-vfx-player -- --check
cargo fmt --package tui-vfx-player-cli -- --check
cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --recursive \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
cargo run -q -p tui-vfx-player-cli -- inventory-recipes \
  --recursive \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

If the implementer uses a different command name than `inventory-recipes`, update the evidence memo accordingly.

## Required evidence memo

At completion, write:

```text
docs/new_kernel/PHASE_K2_0_INVENTORY_GATE_STATUS_MEMO_TO_ARCHITECT.md
```

The memo should include:

```text
new command shape
new report schema version
files touched
summary counts from v3.1 debug_recipes
list of represented descriptor effect ids
list of unsupported effect ids
confirmation that rendering behavior did not change
confirmation that no recipe files were modified
verification commands and pass/fail results
recommended next adapter packet based on the report
```

## Draft implementer prompt

```text
Implement K2.0: K0 v3.1 Debug Fixture Inventory Gate.

Use /usr/projects/tui-vfx as the implementation repo.
Use /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/ as the canonical fixture corpus.
Use /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json as the descriptor pack.

Do not change K0 rendering semantics.
Do not modify recipes.
Do not modify K1.
Do not wire the compositor in this packet.

Add a report-only K0 player CLI command, preferably:

cargo run -p tui-vfx-player-cli -- inventory-recipes \
  --recursive \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes

The command must load descriptor packs through the existing K0 descriptor loader, collect recipe paths through the existing K0 path collector, deserialize canonical RecipeDocument JSON, collect source ids and graph effect ids, validate descriptor coverage, render each recipe through the existing RecipePlayer path, and emit a stable machine-readable JSON inventory report.

The report must distinguish:
- descriptor-covered effect ids
- missing descriptor effect ids
- represented effect ids
- unrepresented descriptor effect ids
- runtime adapter-supported ids
- runtime unsupported ids
- per-recipe rendered/unsupported/error status

Expected current recursive inventory:
- total recipes: 16
- rendered: 10
- unsupported: 6
- errors: 0
- unsupported effect ids:
  - mask.dissolve
  - sampler.ripple
  - shader.borderSweep
  - shader.linearGradient
  - style.baseStyleOverride
  - style.colorFade

Keep the existing render-recipe CLI behavior intact.

Add tests covering:
- baseline inventory entry has no effects and rendered status
- a supported visible primitive such as masks/mask_wipe.json is represented and rendered
- an unsupported primitive such as shader_linear_gradient.json is represented, descriptor-covered, and runtime unsupported
- recursive inventory summary reports the current fixture totals

At completion, write:
docs/new_kernel/PHASE_K2_0_INVENTORY_GATE_STATUS_MEMO_TO_ARCHITECT.md

The memo must include the command shape, report schema version, files touched, summary counts, unsupported ids, verification commands, and confirmation that no recipe files were modified.
```
