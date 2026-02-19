<!-- <FILE>docs/recipes/RECIPE_SCHEMA_SUMMARY.md</FILE> - <DESC>Observed JSON recipe schema summary</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>Recipe validation planning</WCTX> -->
<!-- <CLOG>Add schema variants for styles, masks, and casing</CLOG> -->

# Recipe JSON Schema Summary (Observed)

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
