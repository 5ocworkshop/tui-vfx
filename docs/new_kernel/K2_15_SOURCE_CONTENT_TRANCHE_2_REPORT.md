# K2.15 source/content tranche 2 report

## Added source fixtures

Added under `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/sources/`:

- `source_ansi_sgr_basic.json`
- `source_image_binding_missing_asset.json`
- `source_procedural_dots_spinner_binding.json`

## Source evidence

- `source.ansi` remains bounded text evidence: common SGR is stripped, not converted into full styled-cell ANSI parity.
- `source.image` proves signal-backed asset resolution into deterministic missing-asset fallback evidence.
- `source.procedural` proves signal-backed generator selection for the bounded dots-spinner adapter.

## Content evidence

No new content family descriptors were added in this tranche. Existing content fixtures remain green through the graph/topology changes.

