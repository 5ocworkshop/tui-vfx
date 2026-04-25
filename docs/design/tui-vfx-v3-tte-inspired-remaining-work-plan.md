<!-- <FILE>docs/design/tui-vfx-v3-tte-inspired-remaining-work-plan.md</FILE> - <DESC>Token-safe handoff for remaining TTE-inspired tui-vfx work after glyph particles.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Captured after Task 24 glyph particle emitter implementation and validation on 2026-04-25.</WCTX> -->
<!-- <CLOG>0.1.0: record completed work, verification evidence, and remaining packets for TTE-inspired recipe/effect coverage.</CLOG> -->

# TTE-inspired tui-vfx remaining-work plan

Last updated: 2026-04-25.

This is the token-loss handoff for the Claude/TTE-inspired shader/effect plan.
It assumes the latest worktree contains the Task 24 glyph particle emitter in
`tui-vfx` and its root V3 schema/runtime/debug-recipe integration in
`tui-vfx-recipes`.

## Completed baseline

### Task 23 — per-cell motion substrate

Status: complete by the other implementation team.

Evidence observed before Task 24 resumed:

- `/usr/projects/tui-vfx` commit: `854888a Enable V3 per-cell motion substrate`
- `/usr/projects/tui-vfx-recipes` commit: `9eed1a1 Carry V3 cell motion through recipes and tooling`

### Task 24 — glyph particle emitter

Status: implemented and validated in the current worktree.

Implemented surface:

- `tui_vfx_content::glyph_particles::GlyphParticleEmitterSpec`
- `ParticleEndBehavior`: `despawn`, `freeze_in_place`, `converge_to_origin`
- `ParticleConcurrency`: `all`, `random_sample`, `round_robin`
- `emit_glyph_particles(...) -> GlyphParticleResult`
- Root V3 authoring/normalized/compiled schema propagation via
  `config.content.glyph_emitters[]`
- Root preview/render integration before downstream masks/samplers/shaders/
  filters/style effects
- Direct compiled-V3 source-surface integration for deterministic previews
- Validation errors for invalid emitters
- Generated V3 API docs refreshed in `tui-vfx-recipes`
- Hand-maintained schema docs updated in
  `tui-vfx-recipes/docs/schema/SCHEMA_REFERENCE.md`

TTE inspiration mapped:

- `terminaltexteffects/effects/effect_binarypath.py`: spawns eight binary
  characters per source glyph, moves them along right-angle paths from outside
  the canvas, activates a capped subset of binary groups, then reveals the
  source glyph with a final wipe/brighten phase.
- `effect_spray.py`: starts characters at an origin, reveals a limited active
  volume, and moves them back to input coordinates.
- `effect_bubbles.py` / `effect_laseretch.py`: decorative transient glyphs that
  move independently of source text and then vanish or reveal source cells.

Task 24 debug recipes added in `/usr/projects/tui-vfx-recipes`:

- `recipes/debug_recipes/content/content_glyph_particles_base_spray.json`
- `recipes/debug_recipes/content/content_glyph_particles_options_concurrency.json`
- `recipes/debug_recipes/complex/complex_glyph_particles_binary_path.json`

Verification commands already run successfully:

```sh
# /usr/projects/tui-vfx
cargo test -p tui-vfx-content --test test_glyph_particles
cargo test -p tui-vfx-content --lib
just docs-all-check
git diff --check

# /usr/projects/tui-vfx-recipes
cargo test -p tui-vfx-recipes --lib
cargo test --test test_glyph_particles_root_runtime
just docs-v3-generate
just docs-v3-check
cargo run -q -p pipeline-validator -- --rules --stages --phase all --sample-t 0.0,0.5,1.0 \
  recipes/debug_recipes/content/content_glyph_particles_base_spray.json \
  recipes/debug_recipes/content/content_glyph_particles_options_concurrency.json \
  recipes/debug_recipes/complex/complex_glyph_particles_binary_path.json
git diff --check
```

Known check caveat: `just docs-all-check` in `/usr/projects/tui-vfx` emits a
pre-existing warning about `shaders.Highlighter` AI-hint params, but reports all
generated docs up to date.

## Remaining Task 25 — faithful TTE-inspired recipes

Goal: create user-facing recipe examples, not only debug fixtures, that prove
we can reproduce the important TTE visual idioms with the V3 architecture.
These should live in a durable non-debug recipe location once the project's
recipe taxonomy for examples is confirmed; if no better location exists, use a
new `recipes/tte_inspired/` directory in `tui-vfx-recipes`.

Recommended first recipe set:

1. **BinaryPath showcase**
   - Input: short technical phrase, e.g. `ACCESS GRANTED`, `BINARY PATH`, or
     `TRACE ROUTE`.
   - Root content: `glyph_emitters` with `spawn_count: 8`, `glyph_palette:
["0", "1"]`, `route: rectilinear`, `from: offscreen`, `to: authored`,
     random stagger, and capped `random_sample` concurrency.
   - Pipeline: diagonal or top-right-to-bottom-left wipe plus final foreground
     brighten/gradient. Keep this recognizably close to TTE BinaryPath: right
     angles, binary particles, capped active groups, and final source reveal.
   - Acceptance: validator passes; preview visibly shows binary particles
     entering from outside content and downstream pipeline sees those particles.

2. **Spray/converge showcase**
   - Root content: one emitter from a shared origin/corner or authored offset
     with `glyph_palette` such as `* + ·`, volume controlled by concurrency,
     staggered convergence back to source glyphs.
   - Pipeline: subtle color shader or filter that proves particles are real
     cells, not just an overlay outside the V3 pipeline.
   - Acceptance: source text remains legible/stable; particles are transient;
     border/chrome remains unaffected.

3. **Spark/laser-etch showcase**
   - Root content or scene-layer approximation: particles use spark glyphs
     (`*`, `✦`, `·`, `+`) and a short lifetime, with `despawn` or
     `freeze_in_place` depending on the intended etch moment.
   - Pipeline: beam/reveal mask or highlighter shader after particle pass.
   - Acceptance: short-lived particles do not persist beyond their lifetime,
     downstream effects can color them, and reduced density still reads at
     small terminal sizes.

Implementation notes for Task 25:

- Prefer composition over new primitives. Use `glyph_emitters`, existing
  `cell_motion`, existing masks, shaders, and filters first.
- Add recipe-local metadata with `intent_hints`, `visual_tags`,
  `expected_visual`, and `inspiration` to make demo-browser/search behavior
  useful.
- Follow debug recipe naming conventions for debug fixtures; for showcase
  recipes, use the chosen showcase directory's convention consistently.
- Validate each recipe with:

```sh
cargo run -q -p pipeline-validator -- --rules --stages --phase all --sample-t 0.0,0.5,1.0 <recipe-paths>
```

- If new recipe directories are introduced, update inventory docs or manifests
  that enumerate recipe locations.

## Remaining Task 27 — binding/range audit and schema completion

Task 27 is still separate from glyph particles. It should audit whether the new
V3 binding/plumbing fully covers row/column/modulo range use cases the plan
called out.

Suggested checklist:

1. Search schema/runtime for `RowRange`, `ColumnRange`, `Modulo`, dynamic range,
   and runtime binding resolution.
2. Verify normalized scope literals and dynamic binding scopes survive parse →
   normalize → validate → compile → render.
3. Add missing tests for:
   - dynamic row range binding;
   - dynamic column range binding;
   - modulo/remainder binding with both literal and runtime modulus;
   - interaction with glyph particles if scopes are used inside
     `glyph_emitters[].motion.enter.scope`.
4. If any range-binding feature is not plumbed, implement the smallest missing
   bridge and document exact schema paths.

Acceptance:

- Unit tests cover parse/normalize/validate/compile for each range binding.
- At least one debug recipe demonstrates a dynamic range controlling a visible
  V3 effect.
- Generated V3 docs are refreshed and `just docs-v3-check` passes.

## Glyph-particle follow-ups not required for Task 24

These are intentionally deferred unless a recipe requires them:

1. **Scene-layer glyph emitters**
   - Add `glyph_emitters` to scene-layer DTOs and compiled layer surfaces.
   - Run emitters after layer-local content/cell motion and before layer-local
     pipeline work.
   - Tests should prove root and layer emitters do not select each other's
     cells.

2. **Exact BinaryPath binary-string mode**
   - Current emitter samples from `glyph_palette`; it does not derive the exact
     8-bit binary representation of each source glyph.
   - If exact TTE parity matters, add an opt-in `glyph_source` mode such as
     `palette` vs `source_binary_byte`. Preserve `glyph_palette` as the simple
     default.

3. **Path waypoint/choreography support**
   - Current route support uses one `from`/`via`/`to` path. TTE BinaryPath builds
     multiple random right-angle waypoints per source character.
   - If exact multi-turn paths matter, add a deterministic waypoint route or a
     multi-track particle mode instead of hard-coding BinaryPath in recipes.

4. **Probe truth-surface stats**
   - `GlyphParticleStats` exists in the content crate but is not yet emitted by
     pipeline-validator/probe reports.
   - Add stats beside the existing cell-motion summaries when observability is
     needed for CI or docs.

5. **Color ergonomics**
   - `color_palette` currently uses raw `tui_vfx_types::Color` shape
     `{r,g,b,a}`. Recipes elsewhere often use `{type:"rgb",...}` for style
     colors.
   - If author ergonomics become a problem, add a recipe-local config wrapper
     that accepts design-system color shapes and lowers to raw colors.

## Dirty-worktree caution

At the time this handoff was written, both repositories contained unrelated or
pre-existing dirty files from parallel work. Do not revert or claim ownership of
files outside the glyph-particle paths without checking `git diff` carefully.

<!-- <FILE>docs/design/tui-vfx-v3-tte-inspired-remaining-work-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
