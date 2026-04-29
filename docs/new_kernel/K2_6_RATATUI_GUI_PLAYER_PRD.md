<!-- <FILE>docs/new_kernel/K2_6_RATATUI_GUI_PLAYER_PRD.md</FILE> - <DESC>Product requirements for the v3.1 Ratatui GUI player over clean-room player evidence</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Define the additive human-facing v3.1 Ratatui GUI player PRD without changing player code or recipe corpus files.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — add reviewed legacy-tooling inspiration boundaries for GUI workflow design.</CLOG> -->

# K2.6 Ratatui GUI Player PRD

## 1. Product purpose

The Ratatui GUI player is the human-facing v3.1 recipe review surface for tui-vfx. It lets a developer browse canonical v3.1 `RecipeDocument` files, preview contract-native frames, inspect descriptor and player evidence, and understand migration gaps without leaving the terminal.

The product exists because the current CLI player is a strong regression authority but not a practical visual review environment. Humans need to see the recipe, scrub lifecycle time, compare phases, reload quickly, and inspect why a recipe did or did not render. The GUI player turns the existing player reports into an interactive workflow while keeping the CLI/player model authoritative.

Primary users:

- migration authors converting legacy debug recipes to v3.1;
- tui-vfx maintainers validating descriptor coverage and adapter behavior;
- reviewers checking visual parity, unsupported fields, and evidence reports;
- future studio users editing recipe controls generated from descriptors and signals.

## 2. Non-goals

The GUI player is not a replacement schema interpreter. It must not infer behavior from raw legacy recipes, legacy runtime crates, or old schema semantics.

Out of scope for the first product slice:

- no mutation of recipe corpus files;
- no dependency from clean-room player crates to legacy `tui-vfx-recipes` runtime crates;
- no source-port of the legacy demo implementation;
- no fallback path that makes legacy recipes validate as v3.1;
- no hand-coded effect-specific inspector that bypasses descriptor metadata;
- no full studio authoring surface in K2.7;
- no claim of visual parity unless backed by player-owned evidence reports.

## 3. Runtime authority boundary

The GUI player is additive over the clean-room v3.1 player stack:

- schema authority: `schemas/v3.1/contract/` for stable v3.1 contract surfaces;
- proof-pipeline authority, only where explicitly needed: `schemas/v3.1/next/`;
- Rust contract authority: `crates/tui-vfx-contract/`;
- not-yet-promoted proof concepts: `crates/tui-vfx-next/`;
- descriptor authority: `descriptors/v3.1/packs/primitive.json` and other `descriptors/v3.1/` artifacts;
- canonical fixture corpus: a configurable recipe root such as `$RECIPE_REPO/recipes/v3.1/debug_recipes`; `../tui-vfx-recipes/recipes/v3.1/debug_recipes/` is only the local sibling-checkout example used by this workspace;
- render/evidence authority: `tui-vfx-player` reports and frame data.

The GUI may read recipe source evidence and migrated fixtures, but it must treat only canonical v3.1 `RecipeDocument` values and descriptor packs as runtime inputs. It may cite `../tui-vfx-recipes/examples/demo.rs` as UX inspiration, not as schema, runtime, or dependency authority.

The boundary rule is simple: if the GUI cannot explain a displayed behavior from a v3.1 recipe, descriptor pack, or player report, it should show an explicit unsupported/evidence warning instead of guessing.

## 4. Relationship to existing CLI player

The CLI player remains the regression authority. The GUI is a terminal interface over the same facts.

Current CLI/report surfaces the GUI should consume or mirror:

- `render-recipe` for single or recursive frame rendering;
- `render-frame` / `v3.1.player.visualFrameReport.1` for frame rows, sparse cells, provenance, render hash, non-empty cells, and substrate metadata;
- `inventory-recipes` / `v3.1.player.inventory.1` for recipe and descriptor inventory;
- `primitive-adapter-gap` / `v3.1.player.primitiveAdapterGap.1` for unsupported primitive adapter evidence;
- `primitive-field-coverage` / `v3.1.player.primitiveFieldCoverage.1` for authored field consumption and descriptor coverage;
- `migration-gap` / `v3.1.player.migrationGap.1` for represented and unrepresented legacy families;
- `render-timeline` / `v3.1.player.frameTimeline.1` for sampled frame sequences;
- `render-frame-diff` / `v3.1.player.frameDiff.1` for pairwise frame evidence.

The GUI should not hide CLI failures. If a player report fails or returns warnings, the corresponding GUI pane must show that state inline and keep the underlying report inspectable.

## 5. Required screens/panes

The MVP should use a keyboard-first Ratatui layout with clear pane focus and portable terminal sizing.

Required panes:

1. **Recipe browser / file picker** — browses canonical v3.1 recipes, supports recursive fixture roots, search/filter, directory refresh, and direct file load.
2. **Preview pane** — renders the current `tui-vfx-player` frame substrate into Ratatui cells, including styled-cell metadata when evidence says style is known.
3. **Recipe metadata pane** — shows recipe id, title, description, schema version, descriptor pack references, active scene/layer, selected element, and source path.
4. **Descriptor/effect/node inventory pane** — lists source instances, graph nodes, descriptor ids, domains, inputs, outputs, lifecycle metadata, scope support, and current adapter status.
5. **Timeline pane** — shows phase, sample time, frame index, elapsed time, normalized phase time, loop time, and available timeline samples.
6. **Evidence/debug pane** — shows render hash, non-empty cells, warnings, unsupported effects, field-coverage gaps, report schema versions, and descriptor provenance.
7. **Help modal** — lists global, browser, preview, timeline, evidence, and future-editing bindings.
8. **Status strip** — always-visible concise state: focused pane, recipe status, playback state, motion mode, active descriptor pack, latest warning, and help hint.

## 6. Required controls

Global controls:

- quit;
- switch focus between panes;
- open help modal;
- cycle layout density when terminal space is constrained;
- copy or reveal the active evidence/report path when persisted reports exist.

Browser controls:

- move selection with arrows and `j`/`k`;
- page up/down;
- top/bottom navigation;
- open file or directory;
- parent directory;
- search/filter;
- toggle recursive/flat listing;
- reload current directory from disk;
- load selected recipe.

Preview and playback controls:

- play/pause;
- restart;
- reload active recipe from disk;
- close preview;
- freeze/motion-disabled mode;
- select phase: enter, dwell, exit;
- scrub `sampleT` backward/forward;
- scrub frame timeline index;
- jump to first, previous, next, and last sampled frame;
- trigger supported recipe controls such as contract-native triggers or bindings when present;
- resize/rerender when terminal dimensions change.

Evidence controls:

- switch between render, inventory, adapter-gap, field-coverage, migration-gap, timeline, and diff views;
- expand/collapse warnings;
- jump from a warning to the relevant node, descriptor input, or recipe path;
- toggle compact and verbose JSON-like views without requiring a shell round trip.

## 7. Recipe-driven UI generation

The GUI must derive its controls from recipe and descriptor data rather than hard-coded effect families.

Recipe-driven generation sources:

- `RecipeDocument.metadata` for title, description, status, and human context;
- descriptor pack references for which descriptor packs were intended;
- source instances and graph nodes for inventory rows;
- descriptor `inputs`, `outputs`, lifecycle, domain, scope support, and write support for parameter panels;
- bindings, triggers, signals, and value sources for runtime controls;
- player reports for unsupported, handled, unhandled, and missing coverage states.

Generated UI should be deterministic. Given the same recipe, descriptor packs, frame sample request, and terminal size, the visible inventory and control ordering should be stable.

## 8. Descriptor/parameter/signal/control model

The descriptor model is the basis for current inspection and future editing.

Parameter display requirements:

- show descriptor id, input name, display name, description, value kind, default, range, allowed values, unit, semantic, bindability, and runtime mutability;
- distinguish authored value, defaulted value, bound value, and unsupported value;
- show whether the current player adapter handled an authored field;
- show whether a descriptor declares a field that no current recipe uses;
- show missing descriptor inputs for authored fields not present in the pack.

Signal and binding requirements:

- show value-source shape without collapsing elapsed time and normalized phase time into one concept;
- show signal ids, graph value ids, triggers, latches, reset boundaries, and target bindings when present in the v3.1 contract;
- treat user controls as writes to contract-native controls or bindings, not ad-hoc legacy runtime parameter maps;
- if a control cannot be represented by the current v3.1 model, classify it as `schemaDecisionNeeded`, `descriptorExpansionNeeded`, `adapterNeeded`, or `semanticReviewNeeded` instead of inventing a GUI-only field.

Future editable controls should be generated from descriptors and recipe graph metadata. For MVP, most controls may be read-only except lifecycle sampling, playback, reload, and any contract-native trigger/control already supported by player evidence.

## 9. Playback lifecycle controls

The GUI must expose lifecycle state directly because visual review depends on phase control.

Required lifecycle controls and displays:

- enter/dwell/exit phase selector;
- `sampleT` scrubber with numeric display;
- elapsed time display separate from normalized phase time;
- loop time display where loop sampling is active;
- frame timeline scrubber over sampled reports;
- play/pause and restart;
- freeze/motion-disabled mode that renders a stable representative frame;
- reload-from-disk that reparses the active v3.1 recipe and reruns player evidence without preserving stale warnings;
- terminal resize rerender that makes viewport and substrate dimensions explicit.

The GUI should make current sampling parameters visible in the status strip and evidence pane so a screenshot can be traced back to the same CLI sample request.

## 10. Debug/evidence panels

Debug panels are product requirements, not developer-only extras. The GUI must make uncertainty visible.

Required evidence fields:

- report schema version;
- recipe path and descriptor pack path/provenance;
- render status;
- render hash;
- non-empty cell count;
- substrate kind, cell source, style-known state, and frame dimensions;
- unsupported effect ids;
- primitive adapter gaps;
- primitive field coverage gaps;
- migration-gap family status;
- timeline sample count and current sample index;
- frame diff summary when comparing two samples;
- parse, validation, descriptor, adapter, render, and IO errors.

Warnings should be grouped by source: recipe validation, descriptor lookup, adapter support, authored-field coverage, render substrate, migration classification, and IO/reload. Each warning should name the affected path, node id or descriptor id when available, and recommended next action.

## 11. Migration/parity workflow support

The GUI should support the migration loop without becoming the migration authority.

Required workflow support:

- browse canonical migrated fixtures under a repo-relative v3.1 root;
- optionally display related legacy source path as evidence, not runtime input;
- show migration-gap family status and recommendation;
- show which descriptor or adapter addition would unblock a recipe;
- compare timeline samples for the same v3.1 recipe;
- display frame diffs between two v3.1 samples or between two player runs when evidence exists;
- preserve CLI-equivalent sample parameters for bug reports;
- surface visual parity as `pending`, `needsHumanReview`, or `evidenceBacked`, not as implicit success.

The GUI should make it easy for a reviewer to answer: what recipe is loaded, what contract data was consumed, what fields were ignored, what frame was rendered, and what still needs migration work.

## 12. Accessibility / keyboard model

The player is keyboard-first and must remain usable in narrow or high-contrast terminals.

Requirements:

- every action has a keyboard path;
- pane focus is visible without relying only on color;
- status and warning text should be readable without animation;
- motion-disabled/freeze mode is always available;
- help modal is available globally;
- controls use predictable keys and avoid same-key conflicting meanings across panes;
- screen layouts degrade to stacked panes on narrow terminals;
- important state is represented as text as well as color;
- avoid rapid flashing by default;
- preserve raw-mode cleanup and terminal restoration on errors;
- report paths and identifiers are copyable or displayed plainly enough for issue reports.

## 13. Expected JSON/report dependencies

The GUI should depend on stable player/library APIs where possible, but its data contract should match the JSON report vocabulary so CLI and GUI evidence stay aligned.

Expected dependencies:

- v3.1 recipe contract: `schemas/v3.1/contract/recipe.schema.json` and `tui_vfx_contract::RecipeDocument`;
- descriptor catalog: `schemas/v3.1/contract/descriptor-catalog.schema.json`, descriptor-pack references, and `descriptors/v3.1/packs/primitive.json`;
- frame evidence: `v3.1.player.frame.1` and `v3.1.player.visualFrameReport.1`;
- inventory evidence: `v3.1.player.inventory.1`;
- adapter evidence: `v3.1.player.primitiveAdapterGap.1`;
- field coverage evidence: `v3.1.player.primitiveFieldCoverage.1`;
- migration evidence: `v3.1.player.migrationGap.1`;
- timeline evidence: `v3.1.player.frameTimeline.1`;
- diff evidence: `v3.1.player.frameDiff.1`.

The GUI should preserve report schema versions in displayed evidence. If a report shape changes, the GUI should fail loudly or show an explicit incompatible-report state rather than silently dropping fields.

## 14. MVP scope

MVP is the first shippable Ratatui GUI skeleton after this PRD.

MVP must include:

- Ratatui app shell with browser/preview/evidence panes;
- canonical v3.1 recipe loading;
- descriptor pack loading and provenance display;
- preview rendering through `tui-vfx-player` frame evidence;
- play/pause, restart, reload-from-disk, phase selection, `sampleT` scrub, and motion-disabled/freeze mode;
- render hash and non-empty cell display;
- unsupported, adapter-gap, and field-coverage warning display;
- help modal and status strip;
- no legacy runtime crate dependency;
- tests or deterministic fixtures proving the GUI data model consumes player evidence rather than legacy recipe internals.

MVP may defer:

- editing recipe values;
- persisted workspace/session files;
- full visual parity comparison;
- live multi-recipe batch dashboards;
- oracle screenshot integration;
- SQLite or trace storage;
- plugin-style descriptor packs beyond the current primitive pack.

## 15. Future studio path

The GUI should be structured so MVP inspection can grow into a studio without a rewrite.

Future path:

1. **Inspector** — current PRD scope: browse, preview, scrub, and inspect evidence.
2. **Review cockpit** — batch migration queues, family coverage, adapter gaps, field coverage, timeline/diff comparisons, and parity review status.
3. **Control studio** — editable controls generated from descriptor inputs, signals, bindings, triggers, ranges, units, and runtime mutability.
4. **Recipe authoring studio** — safe writes to user-selected workspace or export-copy v3.1 recipe documents with validation, preview, diff, and rollback; canonical fixture and source recipe corpora remain read-only unless a future owner-approved packet changes that boundary.
5. **Evidence archive** — persisted reports, trace history, screenshots, and CI-comparable frame fingerprints.

The architecture should keep these layers separate. Studio editing must grow from contract-native descriptors and player evidence, not from hard-coded effect widgets.

## Reviewed tooling inspiration boundaries

The GUI PRD borrows workflow ideas from reviewed legacy tooling without adopting legacy runtime authority:

| Source | Borrow conceptually | Do not borrow |
| --- | --- | --- |
| `../tui-vfx-recipes/examples/demo.rs` | Browser/preview split, keyboard workflow, help modal, status strip, reload, pause, motion-disabled mode, phase/sample scrubbing, trigger controls, render hash diagnostics, canvas substrate. | Legacy recipe loading, fallback paths, old schema semantics, runtime crate dependency, hard-coded effect inspection. |
| `../tui-vfx-recipes/tools/pipeline-validator` | Mode-based CLI shape, staged validation/report mindset, explicit failure categories. | Legacy pipeline semantics as v3.1 authority. |
| `../tui-vfx-recipes/tools/recipe-probe` | Timeline, diff, focus-cell, and causation inspection concepts for human debugging. | Probe internals or legacy runtime behavior as player authority. |
| `../tui-vfx-recipes/tools/tui-vfx-trace` | Selector, stage-mask, and NDJSON trace-stream ideas for later evidence panes. | Trace-stage machinery in the first GUI skeleton. |
| `../tui-vfx-recipes/tools/tui-vfx-horseman` | Compact corpus summaries and batch health dashboards. | Legacy summary fields as canonical v3.1 vocabulary. |
| `../tui-vfx-recipes/tools/recipe-source-capture` | Reproducible generated-source artifact ideas for future migration audit trails. | Generated legacy source as canonical runtime input. |
| `../tui-vfx-recipes/tools/recipe-signals-doc` | Generated-doc drift checks for future descriptor/signal panels. | Old signal docs as v3.1 schema authority. |
| `../tui-vfx-recipes/tools/recipe-validator` | Historical validation warnings as migration evidence. | New authority; this tool remains deprecated for clean-room v3.1 decisions. |

## `demo.rs` lessons: borrow and do not borrow

`../tui-vfx-recipes/examples/demo.rs` is a useful UX oracle because it is a working keyboard-first TUI for human recipe playback. It is not v3.1 authority.

Borrow conceptually:

- **Browser/preview split layout.** The left browser and right preview reduce mode switching and make recipe exploration fast.
- **Keyboard-first workflow.** `Tab`, navigation keys, search, reload, pause, scrub, and close actions are discoverable and efficient.
- **Help modal.** A global `?` modal keeps the dense key map usable.
- **Status strip.** The browser status line shows mode, filter, selection, paused state, motion-disabled state, and help hint.
- **Reload active recipe from disk.** Fast reload is essential for migration and tuning.
- **Pause/resume and motion-disabled mode.** Human review needs both temporal control and stable representative frames.
- **Phase and sample scrubbing.** Direct control over phase and `sample_t` makes lifecycle debugging practical.
- **Event trigger controls.** The `t` dwell-binding trigger demonstrates why contract-native trigger/control display matters.
- **Render hash and diagnostics display.** The direct v3 preview shows render hash, non-empty cell count, phase, sample time, source, families, and schema state; the clean-room GUI should keep these evidence concepts.
- **Canvas substrate concept.** A visible preview substrate helps reviewers see transparency, masking, and empty-cell behavior.

Do not borrow directly:

- **Legacy recipe loading authority.** The demo uses legacy recipe loading and cutover fallback paths; the clean-room GUI must load canonical v3.1 contracts.
- **Legacy fallback paths.** Fallbacks are useful historical evidence but not a v3.1 runtime behavior.
- **Runtime crate dependency.** The GUI must not depend on legacy `tui-vfx-recipes` runtime crates.
- **Old schema semantics.** The clean-room GUI must not copy old phase, effect, or pipeline assumptions when v3.1 descriptors disagree.
- **Hard-coded effect inspection.** The demo enumerates legacy style/shader/mask/sampler/filter accessors; the new GUI should derive inventory and controls from descriptors, graph nodes, and player reports.
- **Ad-hoc runtime parameter maps.** Trigger/control actions must map to v3.1 bindings, triggers, signals, or explicitly reported model gaps.

## Acceptance checklist

- The GUI uses Ratatui as the terminal UI layer.
- The GUI consumes canonical v3.1 recipes and descriptor packs.
- `tui-vfx-player` frame evidence is the render/evidence substrate.
- CLI commands remain regression authority and are not replaced.
- Unsupported, migration, adapter, and field-coverage diagnostics are inline.
- Screens include browser, preview, metadata, inventory, timeline, evidence, help, and status surfaces.
- Controls include browse, load, reload, preview, pause, scrub, phase selection, timeline controls, and freeze mode.
- Parameter/signal/control panels are generated from descriptors and contract-native recipe metadata.
- Accessibility requirements include keyboard coverage, visible focus, text state, and motion-disabled mode.
- Future studio controls grow from descriptors and player evidence.
- `../tui-vfx-recipes/examples/demo.rs` is cited only as UX inspiration.

<!-- <FILE>docs/new_kernel/K2_6_RATATUI_GUI_PLAYER_PRD.md</FILE> - <DESC>Product requirements for the v3.1 Ratatui GUI player over clean-room player evidence</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
