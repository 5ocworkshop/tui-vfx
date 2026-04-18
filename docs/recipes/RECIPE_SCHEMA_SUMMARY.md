<!-- <FILE>docs/recipes/RECIPE_SCHEMA_SUMMARY.md</FILE> - <DESC>Observed JSON recipe schema summary</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Document the highlighter-vs-focus_field choice for per-region static bg paint: highlighter sweeps with phase t and produces a visible wipe-in at every enter→dwell boundary, focus_field is geometric and stays put</WCTX> -->
<!-- <CLOG>MINOR: Extend the per-region bg callout with concrete shader recommendations — focus_field for static fills, highlighter only when a sweep is desired — and explain the phase-restart wipe-in artifact authors hit when they reach for highlighter as a flat-paint tool. Also add: enter and dwell are independent t clocks, so any t-driven shader needs to be set on both phases</CLOG> -->

# Recipe JSON Schema Summary (Observed)

Before authoring recipes, read [`../TERMINAL_MOTION_HEURISTICS.md`](../TERMINAL_MOTION_HEURISTICS.md).
Schema knowledge alone is not enough; many effects fail because they ignore
terminal-specific perception, cell ownership, and compositing constraints.

## Top-Level Keys
- **Required (observed in all):** `schema_version`, `id`, `title`, `description`, `version`, `config`.
  - Evidence: `../tui-vfx-recipes/recipes/progress_scanner.json:2-8`.
- **Optional:** `last_updated` (not always present), `extends` for inheritance.
  - Evidence: `../tui-vfx-recipes/recipes/wargames/wargames_shall_we_play.json:3`.

## `config` Keys (Common)
- **Required in practice:** `message`, `layout`, `lifecycle`, `pipeline`, `border`.
  - Evidence: `../tui-vfx-recipes/recipes/progress_scanner.json:8-27`.
- **Optional:** `time`, `content`, `requires_primitives`.
  - Evidence (`time`): `../tui-vfx-recipes/recipes/progress_scanner.json:18-21`.
  - Evidence (`content`): `../tui-vfx-recipes/recipes/debug_recipes/content/content_morph.json:22-30`.

## `layout`
- Typical keys: `width`, `height`, `anchor`, optional `mode`, optional `wrap`.
- Evidence: `../tui-vfx-recipes/recipes/progress_scanner.json:10-14`.

## `lifecycle`
- Observed key: `auto_dismiss_ms`.
- Evidence: `../tui-vfx-recipes/recipes/progress_scanner.json:15-16`.

## `border`
- Typical keys: `type`, `trim`, `padding`, `title`, `title_position`, `title_alignment`, `center_content`, `frame`, `custom_chars`.
- Evidence (padding/title): `../tui-vfx-recipes/recipes/hll_leave_server.json:20-28`.

## `time`
- Observed keys: `loop`, `loop_period_ms`.
- Evidence: `../tui-vfx-recipes/recipes/progress_scanner.json:18-21`.

## `content`
- Keys: `mode`, `effect` (typed object).
- Evidence: `../tui-vfx-recipes/recipes/debug_recipes/content/content_morph.json:22-30`.

## `pipeline`
### Structure
- Core: `enter`, `exit`, `mask`, `sampler`, `filter`, `style` or `styles`.
  - Evidence (single `style`): `../tui-vfx-recipes/recipes/progress_scanner.json:26-142`.
  - Evidence (`styles` array): `../tui-vfx-recipes/recipes/hll_leave_server.json:95-185`.

### `enter` / `exit`
- Typical keys: `duration_ms`, `easing`, optional `snapping`.
- Evidence: `../tui-vfx-recipes/recipes/progress_scanner.json:27-39`.

### `mask` / `filter` / `sampler`
- Each contains `enter`/`exit`/`dwell`.
- Phase value can be **object** or **array** (including empty array).
  - Evidence (mask arrays): `../tui-vfx-recipes/recipes/hll_leave_server.json:40-44`.
  - Evidence (mask arrays, stacked effects): `../tui-vfx-recipes/recipes/multi_effect_cinema_reveal.json:47-58`.
  - Evidence (filter arrays): `../tui-vfx-recipes/recipes/hll_leave_server.json:57-83`.
  - Evidence (sampler dict): `../tui-vfx-recipes/recipes/hll_leave_server.json:46-55`.

### `style`
- Keys: `region`, `base_style`, optional `enter_effect`/`dwell_effect`/`exit_effect`, optional `spatial_shader`.
- `region` is usually a string, but can be a typed object (e.g., `RowRange` or `Cells`).
  - Evidence (string): `../tui-vfx-recipes/recipes/progress_scanner.json:65-67`.
  - Evidence (RowRange object): `../tui-vfx-recipes/recipes/hll_admin_message.json:118-123`.
- Evidence (style block): `../tui-vfx-recipes/recipes/progress_scanner.json:65-142`.
- Evidence (`spatial_shader`): `../tui-vfx-recipes/recipes/coin_get.json:112-117`.

### `styles` (array)
- Each item is a style block (region/base_style/effects), with richer regions (e.g., `RowRange`, `Cells`).
- Some items include `interaction_states` (array) and `interaction_config` (object).
  - Evidence: `../tui-vfx-recipes/recipes/hll_leave_server.json:243-256`.
- Evidence: `../tui-vfx-recipes/recipes/hll_leave_server.json:95-185`.

#### ⚠️ Only the first layer's `base_style` is applied

**Authoring pitfall.** You can list many `{ region, base_style, … }` entries in `styles[]`, but the renderer only reads `base_style` (static `foreground`/`background`/`added_modifiers`) from `styles[0]`. Every subsequent layer contributes only its *effects* — `enter_effect`, `dwell_effect`, `exit_effect`, `spatial_shader` — scoped to its `region`. Static per-region fg/bg in `styles[1..]` are silently dropped.

Consequences:
- **Per-region static fg** (e.g., dark-red trim row, cream text cells over a scrim) cannot be expressed with `base_style`. Use a spatial shader (`LinearGradient`, `Highlighter`, `glisten_band` with matching head/tail) scoped to the region to paint a flat color into the foreground.
- **Per-region background** is not expressible through `base_style`, but it can be painted by shaders that support `apply_to: background`:
  - **`focus_field`** — *prefer this for static bg fills.* Set `shape: "rect"` with `rect_x`/`rect_y`/`rect_width`/`rect_height` covering the region, plus `feather: 0`, `intensity: 1.0`, `pulse_speed: 0`, `apply_to: "background"`, `falloff: "linear"`. Geometry-only — no `t` term — so the bg is solid from the very first frame and never wipes.
  - **`highlighter`** — only choose this when you actually want a sweep-in animation. It is a *time-driven* sweep with `mode: fill`/`band`; `head_pos = effective_t * (axis_len + band_width)`. Even at `speed: 10` (the clamp ceiling) the sweep takes a small but visible fraction of the phase to saturate. Use the highlighter's apply-to-bg form for hover-style underline reveals or progressive scrim drops, not for "always-on" patches.
  - **`focus_field`** and **`glow`** also write bg. **`linear_gradient`** writes fg only.

- **Enter and dwell have independent `t` clocks.** A `dwell_effect` shader does *not* continue from where the `enter_effect` left off — `t` resets to 0 at the phase boundary. Any time-driven shader (highlighter sweep, glisten band, neon flicker, …) that you want active in *both* phases must be declared in *both* `enter_effect` and `dwell_effect` slots on the layer. Authoring symptom: the effect appears, completes during enter, and then visibly restarts when dwell begins — looking like a glitch or a left-to-right wipe.

- **Debugging signal for these issues.** `pipeline-validator --dump --stage output --phase entering --sample-t 0.05/0.5/0.99 -vvv` and the same across `--phase dwelling`. Inspect the `Per-row bg brightness` and `Per-row fg brightness` rasters at multiple sample-ts within each phase — a static paint should be uniform across all sample times; a sweep-in shows progressive coverage that looks like `DDDDDDD...DDDDLLLL...LLLL` filling left-to-right as t advances.
- **Debugging signal.** `pipeline-validator --dump --stage output -vvv` will show every cell carrying `styles[0]`'s fg/bg regardless of the per-region `base_style` you wrote. If your cells don't match your later layers' colors, this is why — it is not a filter or shader side-effect.

Evidence:
- `../tui-vfx-recipes/src/preview/fnc_preview_from_config.rs:234-237` — `build_appearance_from_v2` explicitly `.first()`-picks the base style.
- `../tui-vfx-recipes/src/recipe_schema/config.rs:1876-1912` — `to_animation_profile()` copies each layer's `region` + `enter_effect`/`dwell_effect`/`exit_effect` into `StyleLayer`s. The tui-vfx `StyleLayer` struct (`../tui-vfx/crates/tui-vfx-style/src/models/cls_style_layer.rs`) has no `base_style` field.

Workaround pattern:
1. Put the most common fg/bg combination in `styles[0].base_style` so it applies to the largest region.
2. For each deviating region, add another layer with `dwell_effect.type = spatial` and a shader that paints the desired foreground (use a `linear_gradient` with matching start/end for a flat repaint).
3. Accept that the background is uniform across the widget. If a design truly requires per-region bg, raise it as a schema enhancement — the current renderer cannot deliver it without changes.

## Enum Casing Variants (Observed)
- Some fields (e.g., easing) use snake_case, others use TitleCase in older recipes.
  - Evidence (snake_case): `../tui-vfx-recipes/recipes/effect_parity_morph_density_reveal.json:36-39`.
  - Evidence (TitleCase): `../tui-vfx-recipes/recipes/hll_leave_server.json:216-218`.

## Colors (Observed Shapes)
- Named types: `{ "type": "light_blue" }`.
  - Evidence: `../tui-vfx-recipes/recipes/debug_recipes/content/content_morph.json:68-70`.
- RGB with type: `{ "type": "rgb", "r":.., "g":.., "b":.. }`.
  - Evidence: `../tui-vfx-recipes/recipes/progress_scanner.json:68-73`.
- RGB without type: `{ "r":.., "g":.., "b":.. }`.
  - Evidence: `../tui-vfx-recipes/recipes/hll_leave_server.json:99-103`.

<!-- <FILE>docs/recipes/RECIPE_SCHEMA_SUMMARY.md</FILE> - <DESC>Observed JSON recipe schema summary</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.1</VERS> -->
