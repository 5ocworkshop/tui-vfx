<!-- <FILE>docs/recipes/SCHEMA_EXPORT_DESIGN.md</FILE> - <DESC>Plan for full schema export for validation</DESC> -->
<!-- <VERS>VERSION: 0.1.2</VERS> -->
<!-- <WCTX>Recipe validation planning</WCTX> -->
<!-- <CLOG>Note effect_schemas.json output path</CLOG> -->

# Schema Export Design (Full Validation)

## Goal
Move from key-parameter validation to **full field validation** using `ConfigSchema` metadata.

## Current State
- `capabilities.json` includes effect names and **key parameters** only.
- Key parameters are adequate for surface validation, not full schema enforcement.

## Export (Implemented)
### Output
- `docs/generated/effect_schemas.json` (written by `cargo xtask docs generate`)
  - Includes `version` and per-category `enum_schema` + `variants` mappings.

## Schema Source
- Use `tui_vfx_core::ConfigSchema::schema()` on each effect type.
- Serialize `SchemaNode` (Struct/Enum/Primitive/Option/Vec/Box/Opaque) with:
  - `json_value` for enum variants (for snake_case mapping).
  - `json_key` for renamed fields.
  - `optional`, `default`, and `range` where available.

## Mapping Rules
- Recipe `type` should match the enum variant `json_value` (serde rename/rename_all); fall back to variant name when absent.\n+- Field names use `json_key` when present; otherwise struct field name.

## Caveats
- Serde aliases/custom deserialization are **not** represented in ConfigSchema; schema may be stricter than parser behavior.

## Benefits
- Validate **all** fields (including optionality and ranges).
- Enables migration hints (rename, type change, enum deprecations).
- Stable machine contract for AI tooling.

## Risks / Pitfalls
- Some schemas may contain `Opaque` nodes; validation must treat as unknown.
- Variant name vs json_value mismatch must be handled explicitly.

<!-- <FILE>docs/recipes/SCHEMA_EXPORT_DESIGN.md</FILE> - <DESC>Plan for full schema export for validation</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.2</VERS> -->
