Based on the memo, I’d accept **K2.0** as complete for the bounded inventory packet.

The important new standing rule should become part of every future packet:

```text
For every touched file:
- look for small refactors that reduce complexity
- keep code readable and OFPF-sized
- improve naming when it clarifies intent
- add or update rustdoc on public APIs
- add schemars details where the touched type participates in schema generation
- avoid unrelated broad rewrites
```

## Updated day sequence

Now that K2.0 exists, the next sequence should be:

```text
K2.0  Inventory gate                                  DONE
K2.1  Migration gap + recipe-family roadmap            NEXT
K2.2  K0 visual-frame substrate                        AFTER K2.1
K2.3  First compositor-backed primitive adapters        AFTER K2.2
K2.4  K1 cell-aware preview blit                        AFTER K2.3
K2.5  First v3.1 complex fixture                        AFTER FIRST ADAPTERS
K2.6  Family-by-family recipe migration queue           AFTER GAP REPORT + FIRST COMPLEX
```

I would not jump straight into compositor wiring yet. The inventory gate now tells us what the current v3.1 set contains. The next missing control surface is the old/new recipe-family gap and migration roadmap.

---

# Recommended next work packet

## Packet name

```text
K2.1 — Debug Recipe Migration Gap + Porting Roadmap Gate
```

## Objective

Add a small, repeatable report that compares:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

and produces a machine-readable migration gap summary plus a human-readable status memo.

This should answer:

```text
Which old debug recipe families exist?
Which v3.1 debug recipe families exist?
Which families are unrepresented in v3.1?
Which families are partially represented?
Which families are ready for adapter work?
Which families require schema/descriptor decisions before migration?
Which old fixtures are obvious candidates for the next migration batch?
```

This packet should still be **report-only**. No recipe migration yet.

## Architectural rule

Do not depend on the legacy runtime.

The report may inspect legacy recipe JSON files as JSON documents or paths, but it should not revive legacy execution semantics, old preview fallback, or old runtime loading.

K2.1 is about inventory and planning, not parity rendering.

## Implementation repo

```text
/usr/projects/tui-vfx
```

## Recipe repo inputs

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

## Likely implementation shape

Add a K0/player-side report command, preferably:

```bash
cargo run -p tui-vfx-player-cli -- migration-gap \
  --legacy-root /usr/projects/tui-vfx-recipes/recipes/debug_recipes \
  --v31-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  --descriptor-pack descriptors/v3.1/packs/primitive.json
```

Alternative name is fine, but keep it distinct from `render-recipe` and `inventory-recipes`.

## Suggested files to touch

```text
crates/tui-vfx-player/src/lib.rs
crates/tui-vfx-player/src/cls_player_migration_gap_report.rs
crates/tui-vfx-player/src/fnc_collect_debug_recipe_family_inventory.rs
crates/tui-vfx-player/src/fnc_build_migration_gap_report.rs
crates/tui-vfx-player-cli/src/fnc_run.rs
crates/tui-vfx-player-cli/src/fnc_parse_cli_options.rs
crates/tui-vfx-player-cli/src/fnc_print_usage.rs
crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
docs/new_kernel/PHASE_K2_1_MIGRATION_GAP_STATUS_MEMO_TO_ARCHITECT.md
```

Use exact names that fit the repo’s conventions. Keep new modules OFPF-sized.

## Required report shape

Suggested schema label:

```text
v3.1.player.migrationGap.1
```

Top-level fields:

```json
{
  "schemaVersion": "v3.1.player.migrationGap.1",
  "legacyRoot": "...",
  "v31Root": "...",
  "summary": {
    "legacyRecipes": 0,
    "v31Recipes": 0,
    "representedFamilies": 0,
    "unrepresentedFamilies": 0,
    "partiallyRepresentedFamilies": 0,
    "readyFamilies": 0,
    "blockedFamilies": 0
  },
  "families": [],
  "recommendedQueue": []
}
```

Per family:

```json
{
  "family": "filters",
  "legacyCount": 98,
  "v31Count": 4,
  "coverage": "partial",
  "knownV31EffectIds": [
    "filter.dim",
    "filter.greyscale",
    "filter.invert",
    "filter.tint"
  ],
  "status": "adapterExpansionReady",
  "blockers": [],
  "recommendedNextCandidates": []
}
```

Use stable string values for `coverage`:

```text
none
partial
represented
notApplicable
```

Use stable string values for `status`:

```text
adapterExpansionReady
schemaDecisionNeeded
descriptorDecisionNeeded
migrationCandidateReady
ownerAuditNeeded
notYetClassified
```

## Family classification guidance

Use path-based family classification first. Do not overfit to legacy internals.

Expected broad family buckets:

```text
baseline
filters
masks
samplers
shaders/primitives
shaders/compositions
styles
content
scene
shadows
complex
event_driven_dwell
signals
easings
subcell_shapes
motion_routes
loopback
bindable_rates
fixtures
other
```

The report should identify these current high-level statuses:

```text
baseline              represented
event_driven_dwell    partial / represented pilot
filters               partial
masks                 partial
samplers              partial
shaders/primitives    partial
shaders/compositions  partial
styles                partial
content               none
scene                 none
shadows               none
complex               none
signals               none
easings               none
subcell_shapes        none
motion_routes         none
loopback              none
```

## Recommended queue logic

The report should produce a conservative next queue, not a full migration promise.

Suggested queue order:

```text
1. complex minimal fixture
2. remaining primitive adapter blockers
3. content pilot
4. scene pilot
5. shadow pilot
6. complex legacy replacement candidates
7. signals/easings/motion routes
8. subcell/loopback/other advanced families
```

This queue can be hard-coded initially if clearly labeled as a recommendation from current inventory evidence.

## Cross-cutting refactor/documentation requirement

For every touched file:

```text
- keep functions small and named by purpose
- prefer DTO structs over ad hoc JSON assembly when public/report shapes stabilize
- add rustdoc to public report structs and public helper functions
- keep serialization field names stable with serde rename_all where appropriate
- add schemars derives/details only if the crate/type already participates in schema generation or the touched contract type already uses schemars
- update usage text when adding command flags
- add focused tests rather than broad snapshot tests
```

Do not perform unrelated formatting or workspace-wide cleanup.

## Non-goals

Do not migrate recipe files.

Do not edit `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/`.

Do not edit `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/`.

Do not add compositor adapters.

Do not modify K1.

Do not reinterpret old recipe semantics beyond path/family/candidate inventory.

## Acceptance criteria

K2.1 is acceptable when:

```text
new migration-gap command exists
command accepts legacy root and v3.1 root
command emits stable JSON
report includes per-family legacy/v3.1 counts
report identifies unrepresented v3.1 families
report recommends a conservative migration queue
existing K2.0 inventory-recipes still works
existing render-recipe still works
no recipe files are modified
status memo is written
```

## Suggested verification commands

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

cargo run -q -p tui-vfx-player-cli -- migration-gap \
  --legacy-root /usr/projects/tui-vfx-recipes/recipes/debug_recipes \
  --v31-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  --descriptor-pack descriptors/v3.1/packs/primitive.json
```

If the implementer uses a different command name, the memo should record the actual command.

## Required evidence memo

At completion, write:

```text
docs/new_kernel/PHASE_K2_1_MIGRATION_GAP_STATUS_MEMO_TO_ARCHITECT.md
```

The memo should include:

```text
new command shape
new schema label
files touched
legacy root and v3.1 root inspected
family count summary
unrepresented families
partially represented families
recommended migration queue
verification commands and pass/fail results
confirmation that no recipes were modified
recommended next packet
```

## Draft implementer prompt

```text
Implement K2.1: Debug Recipe Migration Gap + Porting Roadmap Gate.

Use /usr/projects/tui-vfx as the implementation repo.

Inputs:
- legacy root: /usr/projects/tui-vfx-recipes/recipes/debug_recipes
- v3.1 root: /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
- descriptor pack: /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json

Do not migrate recipes.
Do not modify either recipe root.
Do not modify K1.
Do not wire the compositor.
Do not depend on the legacy runtime.

Add a report-only K0 player CLI command, preferably:

cargo run -p tui-vfx-player-cli -- migration-gap \
  --legacy-root /usr/projects/tui-vfx-recipes/recipes/debug_recipes \
  --v31-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes \
  --descriptor-pack descriptors/v3.1/packs/primitive.json

The command should emit stable JSON with schemaVersion:

v3.1.player.migrationGap.1

The report must include:
- legacy root
- v3.1 root
- summary counts
- per-family legacy counts
- per-family v3.1 counts
- coverage classification: none, partial, represented, notApplicable
- status classification: adapterExpansionReady, schemaDecisionNeeded, descriptorDecisionNeeded, migrationCandidateReady, ownerAuditNeeded, notYetClassified
- recommended conservative migration queue

Use path-based family classification first. Do not attempt deep legacy semantic lowering.

For every file touched:
- look for small refactors that reduce complexity
- keep code readable and maintainable
- add rustdoc to public structs/functions
- add schemars details only where appropriate for touched schema-bearing types
- update usage text
- keep tests focused

Expected high-level result:
- current v3.1 root has baseline, event_driven_dwell, filters, masks, samplers, shaders, and styles represented
- content, scene, shadows, complex, signals, easings, subcell_shapes, motion_routes, loopback, and other advanced families remain unrepresented or not yet classified

Keep K2.0 inventory-recipes and K0 render-recipe behavior intact.

At completion, write:

docs/new_kernel/PHASE_K2_1_MIGRATION_GAP_STATUS_MEMO_TO_ARCHITECT.md

The memo must include the command shape, schema label, files touched, counts, unrepresented families, migration queue, verification results, and confirmation that recipe files were not modified.
```
