<!-- <FILE>docs/design/tui-vfx-v3-glyph-particle-emitter-plan.md</FILE> - <DESC>Task 24 as-built plan and handoff for V3 glyph particle emitters.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Task 24 from the TTE capability audit: content-layer transient glyph spawner built on the per-cell motion substrate.</WCTX> -->
<!-- <CLOG>0.1.0: document the first-slice glyph particle emitter schema, runtime seams, debug recipes, tests, and remaining work.</CLOG> -->

# V3 glyph particle emitter plan

## 1. Why this exists

Per-cell motion moves cells that already exist in the source grid. TTE-inspired
BinaryPath, Spray, Burst, Confetti, Decrypt, and Rain need a second capability:
spawn transient glyphs that are not part of the authored source text, move them
through the same source-cell coordinate system, then let downstream masks,
filters, shaders, and style effects see the rendered particles.

The first slice is intentionally content-local and declarative:

```json
"content": {
  "glyph_emitters": [{
    "origin": { "type": "authored" },
    "spawn_count": 8,
    "glyph_palette": ["0", "1"],
    "color_palette": [{ "r": 80, "g": 220, "b": 255, "a": 255 }],
    "motion": { "enter": { "duration_ms": 1200, "from": { "type": "offscreen", "direction": "from_top" }, "to": { "type": "authored" } } },
    "lifetime_ms": 1400,
    "on_complete": "despawn",
    "concurrency": { "type": "random_sample", "fraction": 0.42 },
    "seed": 42
  }]
}
```

This mirrors TTE's mutable `EffectCharacter` model without importing its runtime
architecture: tui-vfx keeps immutable source cells, derives deterministic
particle actors from those cells, renders a sampled transient scene, and then
continues through the normal V3 pipeline.

## 2. First-slice contract

Schema home:

- `config.content.glyph_emitters[]` only.
- It applies to root message/content cells, not border/title/chrome.
- It runs after content effects and after optional `config.content.cell_motion`,
  then before the root pipeline.

Execution order:

```text
root message/content
  -> content.effect, if any
  -> config.content.cell_motion, if any
  -> config.content.glyph_emitters[], in authored order
  -> root masks/samplers/filters/shaders/style-effects
```

First-slice boundaries:

- Root content only. Scene-layer emitters are a follow-up once real recipes need
  particles inside layers.
- Enter/exit timing is inherited from the emitter's `motion` phases.
- Particle paths use `CellMotionSpec` placements, route, easing, snapping,
  stagger, affect, and validation rules.
- Palette selection and concurrency are deterministic from source actor identity,
  particle index, recipe/layer seed, and user `seed`.
- Particles overlay the current scene. They do not mutate source text and do not
  become persistent actors.

## 3. Rust surface

Core content crate types live in `tui_vfx_content::glyph_particles`:

```rust
pub struct GlyphParticleEmitterSpec {
    pub origin: CellPlacement,
    pub spawn_count: u16,
    pub glyph_palette: Vec<char>,
    pub color_palette: Vec<Color>,
    pub motion: CellMotionSpec,
    pub lifetime_ms: u64,
    pub on_complete: ParticleEndBehavior,
    pub concurrency: ParticleConcurrency,
    pub seed: u64,
}

pub enum ParticleEndBehavior {
    Despawn,
    FreezeInPlace,
    ConvergeToOrigin,
}

pub enum ParticleConcurrency {
    All,
    RandomSample { fraction: f32 },
    RoundRobin { stride: u16 },
}

pub fn emit_glyph_particles(
    scene: &SemanticScene,
    spec: &GlyphParticleEmitterSpec,
    timing: &CellMotionTiming,
    local_frame: Rect,
    options: &CellMotionOptions,
) -> GlyphParticleResult;
```

`GlyphParticleStats` reports source actor count, candidate particle count,
emitted count, concurrency skips, hidden-before-start count, completed count, and
clipped count.

## 4. TTE inspiration mapping

TTE `Spray`:

- TTE picks pending characters, places them at a spray origin, activates a path
  to input coordinates, and reveals a limited volume per tick.
- tui-vfx equivalent: `origin`, `motion.from`, `motion.to`, `stagger`, and
  `concurrency` derive a deterministic sampled particle set.

TTE `BinaryPath`:

- TTE spawns several binary glyphs per source character and moves them on
  right-angle paths with randomized ordering and capped concurrency.
- tui-vfx equivalent: `spawn_count: 8`, `glyph_palette: ["0", "1"]`,
  `route: rectilinear`, `stagger: random`, and `concurrency: random_sample`.

TTE sparks/bubbles/confetti:

- The same emitter can express transient decorative particles with different
  glyph/color palettes, `from` placements, and `on_complete` policies.

## 5. Debug recipes

Task 24 fixtures live in `tui-vfx-recipes`:

```text
recipes/debug_recipes/content/content_glyph_particles_base_spray.json
recipes/debug_recipes/content/content_glyph_particles_options_concurrency.json
recipes/debug_recipes/complex/complex_glyph_particles_binary_path.json
```

These prove:

- base emitter rendering over stable source text;
- palette selection and downstream filter visibility;
- random-sample concurrency and freeze completion;
- BinaryPath-inspired rectilinear binary particles plus downstream mask/shader/filter pipeline.

## 6. Validation baseline

From `/usr/projects/tui-vfx`:

```sh
cargo test -p tui-vfx-content --test test_glyph_particles
cargo test -p tui-vfx-content --lib
just docs-all-check
```

From `/usr/projects/tui-vfx-recipes`:

```sh
cargo test -p tui-vfx-recipes --lib
cargo test --test test_glyph_particles_root_runtime
cargo run -q -p pipeline-validator -- --rules --stages --phase all --sample-t 0.0,0.5,1.0 \
  recipes/debug_recipes/content/content_glyph_particles_base_spray.json \
  recipes/debug_recipes/content/content_glyph_particles_options_concurrency.json \
  recipes/debug_recipes/complex/complex_glyph_particles_binary_path.json
just docs-v3-check
```

## 7. Follow-ups

- Scene-layer `glyph_emitters` if recipes need particles inside layer-local
  source surfaces.
- Binding-backed particle origins for cursor/cell-driven emitters.
- Multi-track particle choreography for exact BinaryPath parity.
- Particle stats in probe truth surfaces, matching the existing cell-motion
  summary pattern.
- Faithful `recipes/tte_inspired/` recipes after Task 24 lands.

<!-- <FILE>docs/design/tui-vfx-v3-glyph-particle-emitter-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
