<!-- <FILE>docs/design/post-release/dynamic-light-shadow-primitive-spec.md</FILE> - <DESC>Post-release specification for a grid-aware Light primitive in tui-vfx-shadow that derives per-element shadow offsets from a global directional or positional light source, providing the canonical data type and projection math reused by gtd-factory's elevation-aware shadow build, the Madeira flag's surface-normal lighting, and any future tui-vfx consumer that needs coordinated dynamic shadows or surface shading.</DESC> -->
<!-- <VERS>VERSION: 0.2.2-draft</VERS> -->
<!-- <WCTX>Capture the post-release Light primitive at the tui-vfx layer where grid awareness, shadow rendering, and surface-shader signal seams already live, so a single canonical Light type drives both shadow offset projection and procedural surface lighting (e.g. the Madeira flag) instead of two parallel ad hoc light models.</WCTX>
<!-- <CLOG>0.2.2-draft: cross-reference the historical-graphics-techniques-addendum from §1 so future readers can locate the design-inspiration brainstorm (ANSI art, Amiga/C64 demoscene, Myst, Diablo, FFVII, Mode 7, Hypercard/Director/Flash compositing) that informs §10 and several sibling post-release specs. Metadata-only edit; no API or math changes.</CLOG> -->

# Dynamic Light primitive for tui-vfx-shadow

**Status: post-release project.** This is not release-blocking V3 work. Keep it as a deferred capability until the core V3 release gate, recipe migration, and as-built docs are stable. The companion downstream integration spec lives in gt-design at `docs/internal/specs/2026-04-30-dynamic-light-and-shadows-prd.md`. A design-inspiration addendum mining historical graphics-techniques (ANSI art, Amiga/C64 demoscene, Myst, Diablo, FFVII, Mode 7, Hypercard/Director/Flash compositing) lives alongside this spec at `historical-graphics-techniques-addendum.md` and motivates several of the §10 future-extensions items.

## 1. Purpose

Add a small grid-aware **`Light`** primitive at the `tui-vfx-shadow` layer plus the pure projection math that turns a `(Light, element_height, element_center)` triple into a continuous sub-cell `(offset_x, offset_y)` pair (with a `quantize_offset` adapter for renderers that consume integer cells) for the existing `ShadowConfig`.

This is the canonical place for the math because tui-vfx is already the grid-aware layer:

- `tui-vfx-shadow` already owns shadow geometry (`offset_x`, `offset_y`, `inset_*`, `falloff_*`, `ShadowEdges`, the three renderers).
- `tui-vfx-recipes` already exposes per-source surface shaders that read runtime-bindable light parameters (`shading.light_x`, `shading.light_y` in the Madeira flag's `cls_braille_flag_field.rs` shading-mode dispatcher).
- `tui-vfx-compositor` already speaks in cell coordinates and z-order.

Everything that should react to "where is the light" lives here. Putting `Light` anywhere else creates parallel light models that downstream consumers (gtd-factory, recipe shaders, hypothetical future tools) have to reconcile by hand.

## 2. Non-goals

1. **Theme/SSOT integration.** That belongs to gt-design's PRD. tui-vfx ships the primitive and the math; gt-design adds the policy field, the JSON shape, the `FactoryRenderRequest` override seam, and the optional Relative-Light focus coupling.
2. **Multiple lights.** One global light only. Per-element local lights are out of scope.
3. **Element-on-element shadow occlusion.** A high modal does not project a shadow onto the card behind it. tui-vfx-shadow's existing rendering paints into the destination grid; that does not change.
4. **Color grading / saturation.** The existing tui-vfx-v1 dramatic color shadow plan governs that and composes with this work; neither is a substitute for the other.
5. **Light *visualization*.** No "draw the sun in the corner" widget. That can be a recipe author's tool later if useful.

## 3. Conceptual model

```text
                    Light  ─── pure data
                     │
                     ├─── project_light_to_offset(light, level, element_center) → (f32, f32) ─┐
                     │                                                                        │
element height ──────┘                                                                        ├─→ Braille renderer (sub-cell, no quantization)
                                                                                              │
element center ─────────────────────────────────                                              └─→ quantize_offset → ShadowConfig.with_offset → rect/solid renderer
```

The primitive is a pure value plus a pure function. No state. No allocations. Mathematically this is the planar-shadow projection family (Blinn-style), in the ray-casting lineage that separates shadow visibility from primary visibility; the directional/positional split is the standard "light at infinity vs. point light" distinction (§4.1 captures the limit). The math runs in continuous (`f32`) space; a separate `quantize_offset` adapter rounds to integer cells for renderers that need it, so the Braille renderer (which already speaks sub-cell) skips the adapter entirely — keeping aliasing at the output device rather than the geometry layer.

Callers (gtd-factory's `shadow_def_to_config`, the flag shader, anything else) consult a `Light` they own and call `project_light_to_offset` to get an `(f32, f32)` sub-cell offset; cell-quantizing renderers pipe through `quantize_offset` to get the `(i8, i8)` they hand to the existing `ShadowConfig::with_offset`.

For surface shaders (the flag), the same `Light` exposes a normalized 2D direction via a `light.direction_at(point)` accessor — directional returns a fixed `(dx, dy)`; positional returns the unit vector from the surface point toward the light. This lets the flag's existing `shading.light_x` / `shading.light_y` floats become `Light::direction_at` calls without changing the shader's lighting math.

## 4. Data shape

```rust
/// Tagged union: callers pick directional or positional. Both kinds are
/// first-class and are valid inputs to project_light_to_offset.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Light {
    Directional(DirectionalLight),
    Positional(PositionalLight),
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectionalLight {
    /// 0..360, 0 = light from +x (right), 90 = +y (down).
    pub azimuth_deg: f32,
    /// 0..90, 0 = grazing horizon, 90 = overhead.
    pub pitch_deg: f32,
    /// 0..1, multiplies any consumer-side strength term.
    pub intensity: f32,
    /// Optional shift for warm/cool grading; no semantics imposed here.
    pub color_tint: Option<ColorTint>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PositionalLight {
    /// Cell coordinates in the grid space the consumer is rendering into.
    pub light_x: f32,
    pub light_y: f32,
    /// Height above the screen plane, in cell-equivalent units. Must exceed
    /// the tallest element height the caller will project; clamped at the
    /// projection site to avoid singularities.
    pub light_z: f32,
    /// Effective light radius in cell-equivalent units. Reserved for the
    /// future stochastic shadow sampler (§10), which will jitter N rays
    /// across a disc of this radius to produce soft penumbras (Cook 1984
    /// area-light framework). v1's `project_light_to_offset` ignores this
    /// field; the field is reserved here so adding the sampler later does
    /// not require a schema bump on already-authored JSON. Default 0.0 =
    /// ideal point light.
    #[serde(default)]
    pub radius: f32,
    pub intensity: f32,
    pub color_tint: Option<ColorTint>,
}
```

`Light` is `Copy` so per-frame override updates allocate nothing. `Serialize`/`Deserialize` so callers can persist or transport it without bespoke conversion.

### 4.1 Mathematical equivalence

A `Directional { azimuth, pitch, .. }` is the limit of a `Positional { light_x, light_y, light_z, .. }` as `light_z → ∞` along the direction `(cos(azimuth), sin(azimuth)) / sin(pitch)`. We keep both as separate variants because:

1. The directional path is cheaper at runtime (no element-position dependence; just trig and a multiply-by-level).
2. The calm/coherent aesthetic of directional is what most workspaces want.
3. "Infinite `light_z`" is awkward to express in JSON or to clamp safely.

Themes/consumers that want directional behaviour write a `Directional`; those that want position-tracking write a `Positional`. Both call the same projection function.

## 5. Projection math

```rust
/// Project a Light onto a per-element sub-cell shadow offset, given the
/// element's elevation (level, in cell-equivalent height units) and
/// screen-space center (in cell coordinates). Pure; no allocation. Returns
/// continuous `(f32, f32)` offsets so renderers that support sub-cell
/// positions (Braille) can consume the result directly; cell-quantizing
/// renderers (rect, solid) pipe the result through `quantize_offset`.
pub fn project_light_to_offset(
    light: &Light,
    level: i8,
    element_center: (f32, f32),
) -> (f32, f32) {
    match light {
        Light::Directional(d) => project_directional(d, level),
        Light::Positional(p) => project_positional(p, level, element_center),
    }
}

fn project_directional(light: &DirectionalLight, level: i8) -> (f32, f32) {
    let pitch_rad = light.pitch_deg.clamp(MIN_PITCH_DEG, MAX_PITCH_DEG).to_radians();
    let length_scale = (1.0 / pitch_rad.tan()).clamp(MIN_LENGTH_SCALE, MAX_LENGTH_SCALE);
    let az_rad = light.azimuth_deg.to_radians();
    let dx = az_rad.cos() * length_scale * level as f32;
    let dy = az_rad.sin() * length_scale * level as f32;
    (dx, dy)
}

fn project_positional(
    light: &PositionalLight,
    level: i8,
    element_center: (f32, f32),
) -> (f32, f32) {
    let h = level as f32;
    // light_z must exceed h to avoid singularity; clamp to a small positive
    // value if the caller hands us garbage.
    let denom = (light.light_z - h).max(MIN_LIGHT_Z_GAP);
    let scale = (h / denom).clamp(-MAX_LENGTH_SCALE, MAX_LENGTH_SCALE);
    let dx = (element_center.0 - light.light_x) * scale;
    let dy = (element_center.1 - light.light_y) * scale;
    (dx, dy)
}

/// Round a sub-cell offset to integer cells. Renderers that consume
/// `[i8; 2]` offsets (rect, solid) call this once on the projection result
/// before handing it to `ShadowConfig::with_offset`. The Braille renderer
/// consumes sub-cell offsets directly and does not call this. Pure; no
/// allocation.
pub fn quantize_offset(offset: (f32, f32)) -> (i8, i8) {
    (offset.0.round() as i8, offset.1.round() as i8)
}

const MIN_PITCH_DEG: f32 = 5.0;
const MAX_PITCH_DEG: f32 = 89.0;
const MIN_LENGTH_SCALE: f32 = 0.5;
const MAX_LENGTH_SCALE: f32 = 6.0;
const MIN_LIGHT_Z_GAP: f32 = 0.5;
```

### 5.1 Directional consequences

- `level=0` → `(0, 0)`
- `level=N` → length proportional to `N / tan(pitch)`, direction = azimuth, *independent of `element_center`*
- Grazing pitch (≤5°) clamped to keep length finite
- Overhead pitch (≥89°) clamped to keep `tan` numerically stable

### 5.2 Positional consequences

- `level=0` → `(0, 0)`
- Element directly under the light → shadow direction collapses (`element_center - light` ≈ 0)
- Element far from the light → shadow points away from the light's column, length grows with horizontal distance
- Element on the opposite side of the light → shadow flips direction
- Higher elements cast longer shadows than lower ones at the same horizontal distance, because `scale = h / (light_z - h)` grows with `h`
- `light_z ≤ level` is clamped via `MIN_LIGHT_Z_GAP` to avoid unbounded shadow lengths

### 5.3 Surface-direction accessor

For shader consumers (the Madeira flag, future surface shaders) that don't need an offset but do need a unit-vector light direction at a surface point:

```rust
impl Light {
    /// Returns a unit vector pointing from the surface point toward the
    /// light. Directional returns a fixed direction; positional returns
    /// a point-relative direction that varies across the surface.
    pub fn direction_at(&self, surface_point: (f32, f32)) -> (f32, f32) { ... }
}
```

This is the seam that retires the flag shader's hand-rolled `light_x` / `light_y` floats — a future minor revision of `cls_braille_flag_field.rs` can read a single `Light` parameter and ask it for its direction at the current sample point. Directional consumers see no change; positional consumers automatically get position-aware shading.

## 6. Reuse story for the Madeira flag

The flag shader's current shading dispatch (in `cls_braille_flag_field.rs` after commit `cc399c5` on master, 2026-04-30) reads two flat float params:

```json
"shading": {
  "mode": "lambert",
  "light_x": { "binding": "light_x", "default": 0.4 },
  "light_y": { "binding": "light_y", "default": 1.0 }
}
```

Under this primitive, that becomes:

```json
"shading": {
  "mode": "lambert",
  "light": {
    "kind": "directional",
    "azimuth_deg": { "binding": "light_azimuth_deg", "default": 60.0 },
    "pitch_deg":   { "binding": "light_pitch_deg",   "default": 55.0 },
    "intensity":   1.0
  }
}
```

The shader replaces its `light_direction(params)` helper with a `Light::deserialize → light.direction_at(surface_point)` call. Existing recipes can be migrated by a small one-shot transform; the `light_x` / `light_y` flat-param path stays as a deprecated fast lane until recipes are updated.

The win: a recipe that includes both a flag and any future tui-vfx-shadow consumer (e.g. an elevated card behind the flag) reads from **the same** `Light` value. Move the light azimuth at runtime — both the flag's diffuse term and the card's shadow offset reorient together, automatically coordinated.

## 7. Downstream integration contract

Consumers of `Light` must:

1. Own the `Light` value (or borrow it from a global). The primitive does not provide a global registry; that's a consumer concern.
2. Pass `element_center` in the same coordinate space their `ShadowConfig` will be applied in. tui-vfx-shadow does not transform coordinates.
3. Apply their own strength multiplier (using `light.intensity`) — the projection function does not modify alpha or color.
4. Decide their own clamping strategy if `level` ranges outside reasonable depth bands. The primitive's clamps protect numeric stability, not aesthetic sanity.

The downstream gt-design PRD specifies one such consumer (gtd-factory's elevation-aware shadow build, with theme-driven `Light` from SSOT and a runtime override seam through `FactoryRenderRequest`). Other consumers will follow the same contract.

## 8. Acceptance criteria

A reasonable v1 of this primitive delivers all of:

1. **Pure, allocation-free.** `Light` is `Copy`; `project_light_to_offset` performs no allocation and no I/O. Verifiable by `#[no_std]`-style review or a `cargo bench` baseline.
2. **Both kinds verified.** Unit tests cover: directional with overhead pitch (offset → 0), grazing pitch (offset clamped), all four cardinal azimuths (correct sign on dx/dy); positional with element directly under light (offset → 0), element far horizontally (offset large in correct direction), element opposite side of light (offset sign flipped), `light_z ≤ level` (clamped, finite output).
3. **Equivalence sanity.** A test asserts that as `light_z → very large` along a fixed direction, positional output approaches directional output for a representative grid of `(level, element_center)` pairs (within rounding tolerance).
4. **Reusable from a procedural shader.** The Madeira flag is migrated to consume `Light::direction_at` and the existing comparison test (`compare_shading_modes_across_cycle` in `tui-vfx-recipes` commit `ce3b95d`) continues to pass with no qualitative change to the per-cell brightness table.
5. **Reusable from a shadow renderer (both quantization paths).** A small example or test shows:
   - *Cell-quantized path*: `ShadowConfig::with_offset(quantize_offset(project_light_to_offset(&light, level, center)))` producing visually correct shadows under the rect and solid renderers, for two elevations under one shared light, in both directional and positional modes.
   - *Sub-cell path*: the Braille renderer consuming `project_light_to_offset(...)` directly, with smooth shadow movement when `light.azimuth_deg` (directional) or `light.light_x` (positional) is animated through small fractional-cell increments — confirming the §9.4 sub-cell promotion delivers the angular-smoothness payoff that motivated it.
6. **Cheap.** Per-element per-frame cost is two trig ops + a few multiplies (directional) or a subtract + divide + a few multiplies (positional). Both should benchmark below 1 µs per call on a representative dev machine.

## 9. Open questions

1. **Where should `Light` live within tui-vfx-shadow?** As a top-level export of the `tui-vfx-shadow` crate, or in a new `tui-vfx-shadow::light` submodule? Initial preference: submodule, mirroring `tui-vfx-shadow::renderers`.
2. **Should `direction_at` go on `Light` itself or on a thin wrapper?** Putting it on `Light` is convenient but slightly couples the primitive to surface-shader use cases. Wrapper is cleaner but adds a hop. Initial preference: on `Light`, since the cost is one `match` and the convenience is large.
3. **Does the primitive own `ColorTint` or just reference an opaque type?** The `color_tint: Option<ColorTint>` field is currently a stub. tui-vfx-shadow already has a `Color` type for the shadow color itself; we'd reuse that or define a sibling `ColorTint` for warm/cool shifts. Defer until a consumer needs it.
4. **Sub-cell precision via the Braille renderer.** *Resolved 0.2.0-draft.* The projection function returns `(f32, f32)` sub-cell offsets; a separate `quantize_offset` adapter rounds for cell-quantizing renderers. Reasoning: aliasing is a structural property of the sample layer, not a polish concern (Cook 1984 / Whitted 1980 both got anti-aliasing for free by keeping sample positions continuous and quantizing only at the output device — the Appel-pen-plotter pattern). The Braille renderer already supports sub-cell input (§11), so the alternative — staying at `(i8, i8)` — would have meant burning the renderer's existing sub-cell capability. Cost of the change: one extra `quantize_offset` call in cell-quantizing render paths and a `.round() as i8` removed from the projection function.
5. **Should `Light` be `#[repr(C)]` for FFI/recording?** Probably not yet; revisit if a recording or trace tool needs it.
6. **Migration story for the flag.** Hard cutover (rewrite the recipe), soft (accept both shapes for one release), or gated (recipe schema bump)? Decided in the flag's own follow-up commit.
7. **`Ray`-shaped accessor for future indirect/area-light consumers.** Should `Light::ray_to(surface_point) -> Ray { origin, direction, length }` eventually replace the standalone `direction_at` accessor? The Whitted/Kajiya lineage treats lights, eye rays, shadow rays, and bounce rays as one parameterized primitive (origin + direction + length), and any future consumer that needs inverse-square attenuation or one-bounce indirect (§10's "light-aware glow") will want the `length` term. Initial preference: defer to a future revision; v1 ships `direction_at` only since no current consumer needs `length`. The `Ray` shape lands cleanly when the first consumer requires it; until then the simpler accessor stays and `direction_at` is the public surface.

## 10. Future extensions (deferred)

- **Stochastic shadow sampling for area lights (Cook 1984).** A future `project_light_to_offset_samples(light, level, center, n) -> Vec<(f32, f32)>` jitters N offsets sampled across a disc of radius `light.radius` around the positional light (or an angular cone for directional); the renderer composites them for soft penumbras. The `radius` field is reserved in v1 (§4) so adding the sampler is API-additive and does not break in-flight JSON. Cell-quantizing renderers can approximate by averaging `quantize_offset` of each sample; the Braille renderer composites at sub-cell precision for genuine soft shadows.
- **Multiple lights with weighted blend.** Add a `LightSet` that sums per-light contributions. Useful for hero/showcase surfaces, expensive in the general case.
- **Element-on-element occlusion.** Real cast shadows from one element onto another. Two architectural options pre-evaluated in the gt-design PRD; unchanged here.
- **Light-aware glow.** The existing `glow_border` shadow technique could brighten the light-facing edge of an element under positional light. Composes naturally with this primitive.
- **Static light-field bake (Quake 1996 lightmap pattern).** When a `Light` is known to be static for the lifetime of a scene, a future `Light::bake(grid_extent: (u16, u16)) -> CachedLightField` can precompute per-cell direction and brightness once at scene-load time, with `direction_at` falling back to the analytic path when no cache exists. Only dynamic light deltas (cursor-tracked positional lights, animated azimuths) recompute per frame; the baked field is reused. The win is most pronounced for surface shaders (the Madeira flag) where every cell would otherwise trig-and-multiply per frame; the win for shadow projection is smaller because `project_light_to_offset` already runs only per-element. Composes with the LUT item below — the cache can store baked attenuation alongside baked direction.
- **Shared distance-attenuation LUT (Doom 1993 colormap pattern).** A future `AttenuationTable` keyed by `(distance, elevation)` and consulted at the cell level can replace per-cell analytic falloff with a table read. The same table can drive the `weather-ambient-field-spec.md` fog/depth-cue pass, so atmospheric fade and light falloff are mathematically consistent — the Doom trick that gave the engine its mood "for free" by making one mechanism produce both effects. Worth coordinating the table shape with the ambient-field spec before either lands so the two specs share a single attenuation primitive rather than landing parallel implementations.

## 11. Relationship to other tui-vfx work

- **`tui-vfx-shadow`'s existing `ShadowConfig`** — this primitive feeds offsets into the existing `with_offset` method; nothing about `ShadowConfig` changes.
- **The Braille shadow renderer (`renderers/cls_braille.rs`)** — already supports sub-cell offsets, which the v1 `(f32, f32)` projection signature feeds directly with no quantization step. Cell-quantizing renderers (rect, solid) wrap the result in `quantize_offset` (§5). When the §10 stochastic sampler lands, the Braille renderer is the natural composite target for genuine soft penumbras at sub-cell precision.
- **The Madeira flag shading dispatcher (`cls_braille_flag_field.rs` ≥ 0.6.0)** — first proving consumer for `Light::direction_at`. The reusability comment introduced in commit `cc399c5` already names the seam where this primitive lands.
- **`tui-vfx-v1-dramatic-color-shadow-plan.md`** (gt-design plans) — composes. That plan addresses shadow color/saturation; this primitive addresses shadow direction/length. Both can land independently; together they make shadows that point coherently AND look richer.

## 12. Decision boundaries

This spec does **not** decide:

- The exact module path for `Light` within `tui-vfx-shadow`.
- The `ColorTint` shape (defer to first consumer that needs it).
- Whether to bundle the flag migration with v1 or do it as a follow-up.
- Whether `Light` ships with a builder API or just plain struct literal construction.
- Whether to expose `project_light_to_offset` as a free function, a `Light` method, or both.

These belong in the implementation plan that follows this spec.

## 13. Next steps

1. **Review this spec** alongside the gt-design integration PRD.
2. **Resolve open questions** §9.1, §9.2 (they affect the public surface).
3. **Spike the projection function in tui-vfx-shadow** behind a feature flag, with unit tests covering §8.2's matrix. ~half-day task.
4. **Migrate the Madeira flag** to consume `Light::direction_at` once the primitive lands; verify the comparison test still passes.
5. **Hand off to the gt-design integration PRD** for the SSOT/factory wiring.

<!-- <FILE>docs/design/post-release/dynamic-light-shadow-primitive-spec.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.2-draft</VERS> -->
