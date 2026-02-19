<!-- <FILE>docs/recipes/VALIDATOR_SPEC.md</FILE> - <DESC>Recipe validator specification</DESC> -->
<!-- <VERS>VERSION: 0.1.4</VERS> -->
<!-- <WCTX>Recipe validation planning</WCTX> -->
<!-- <CLOG>Document effect_schemas-based validation</CLOG> -->

# Recipe Validator Spec (v0)

## Scope
- **Input:** JSON recipes under `../tui-vfx-recipes/recipes/**`.
- **Reference:** `docs/generated/capabilities.json` and `docs/generated/effect_schemas.json` in `tui-vfx` repo.
- **Output:** Human + machine reports (no in-place edits).
- **Rule:** Missing non-key fields are **OK**.

## Validation Phases
### A) Parse & Resolve
- Parse each JSON file; report parse errors with file path.
- If `extends` exists, resolve path **relative to the recipe file**; fall back to recipes root if not found. Merge (child overrides parent).
- Detect extension cycles and missing parent file.

### B) Structural Checks (Type + Shape)
- Top-level keys present: `schema_version`, `id`, `title`, `description`, `version`, `config`.
- `config.message` string; `layout`/`lifecycle`/`border`/`pipeline` objects if present.
- `pipeline.enter/exit`: object with numeric `duration_ms`; optional `easing` and `snapping`.
  - Easing casing is mixed in existing recipes (snake_case and TitleCase). Validator should accept both or normalize.
- `content.effect` may be null (treated as no effect).
- `mask/filter/sampler`: `enter/exit/dwell` allowed to be **object or array**; empty array ok.
- `style`: `region` (string or object), `base_style` (object), optional effects, optional `spatial_shader`.
- `styles`: array of style blocks (each same as `style` shape); may include `interaction_states` (array) + `interaction_config` (object).

### C) Effect Validation (Capabilities + Schemas)
- For each effect object with `type`:
  - Validate effect `type` exists in the correct category (mask/filter/sampler/style/content/shader).
  - Validate **present** parameter names against schema fields; unknown params → warning.
  - Validate parameter value types using full schema when available.
- For `style` effects with `type = spatial`:
  - Validate `shader.type` against shader list.
  - Validate shader params similarly.
- For `pipeline.style.spatial_shader` or `styles[].spatial_shader`:
  - Treat as shader category and validate type/params.

### D) Severity
- **Error:** invalid JSON, unknown effect type, invalid structure (wrong type), broken `extends`.
- **Warning:** enum value mismatch (including casing differences if we choose strict), unsupported region shape, unknown parameter names under full schema validation.

## Outputs
- `docs/generated/recipes_validation.json` (machine):
  - Per-file: status, errors/warnings, extracted effect list, resolved `extends`.
- `docs/generated/recipes_validation.md` (human):
  - Summary counts, top offenders, per-file breakdown.

## Non-Goals (v0)
- No auto-rewrite of recipes.
- No CI integration yet.

<!-- <FILE>docs/recipes/VALIDATOR_SPEC.md</FILE> - <DESC>Recipe validator specification</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.4</VERS> -->
