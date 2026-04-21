# <FILE>docs/INDEX.md</FILE> - <DESC>Documentation index for tui-vfx-recipes</DESC>
# <VERS>VERSION: 0.13.0</VERS>
# <WCTX>Point recipe authors at the canonical upstream ownership split and add tui-vfx-trace as the end-to-end trace surface beside validation and probe tooling.</WCTX>
# <CLOG>MINOR: Add tui-vfx-trace to the recipes docs index and keep the existing Preview / validator / probe ownership guidance intact.</CLOG>

# Documentation Index

## Start Here
- Canonical terminal heuristics live in the sibling `tui-vfx` repo:
  `../../tui-vfx/docs/TERMINAL_MOTION_HEURISTICS.md`
- **Debugging recipe rendering as an LLM:**
  - Preview / demo browser is the canonical recipe player for human sign-off.
  - `tools/recipe-probe/` is the recipe-side adapter CLI for the engine probe. Read [RECIPE_PROBE_GUIDE.md](RECIPE_PROBE_GUIDE.md).
  - `tools/tui-vfx-trace/` is the end-to-end trace CLI for lifecycle + resolution + composition + pipeline evidence. Read `tools/tui-vfx-trace/README.md` and the sibling engine doc `../../tui-vfx/docs/PIPELINE_TRACE_LLM_GUIDE.md`.
  - `tools/pipeline-validator/` remains the recipe-aware validator and now owns upstream-native `--debug-recipes-qc` for reusable fixtures and concrete export validation. Read the sibling engine doc `../../tui-vfx/docs/PIPELINE_VALIDATOR_LLM_GUIDE.md`.
  - The direct engine probe itself is documented in the sibling engine doc `../../tui-vfx/docs/PIPELINE_PROBE_LLM_GUIDE.md`.
  - `../recipes/vfx-probe-validation/` is the fastest curated corpus for repeatable manual and structured probe smoke tests. Read [../recipes/vfx-probe-validation/README.md](../recipes/vfx-probe-validation/README.md).
  - Canonical recipe workflow and visual QA guidance now live in the sibling `tui-vfx` repo:
    - `../../tui-vfx/docs/RECIPE_AUTHORING_WORKFLOW.md`
    - `../../tui-vfx/docs/RECIPE_VISUAL_QA.md`
    - `../../tui-vfx/docs/PIPELINE_PROBE_WISHLIST.md`
  - Recipe-aware diagnostics now live alongside the adapter surface in this repo:
    - `src/probe/fnc_collect_recipe_dwell_diagnostics.rs`
    - `src/probe/fnc_collect_recipe_report_diagnostics.rs`
    - `src/probe/fnc_collect_probe_operational_analysis.rs`
    - `src/probe/fnc_collect_probe_timeline_motion_analysis.rs`
    - `tests/test_probe_recipe_diagnostics.rs`
    - `tests/test_probe_report_diagnostics.rs`
    - `tests/test_probe_operational_analysis.rs`
    - `tests/test_probe_motion_analysis.rs`
    - `tests/test_probe_widget_cell_focus.rs`

## This Repo
- Recipe loading, parsing, preview, and validation live here.
- `tools/tui-vfx-trace/` is the source of truth for unified end-to-end trace capture across lifecycle, resolution, composition, and pipeline stages.
- `tools/pipeline-validator/` is the source of truth for recipe-aware validation and now has a delegated `--probe` mode; `tools/recipe-probe/` is the standalone adapter CLI for direct probe reports; `tools/recipe-validator/` is the deprecated predecessor.
- `tools/pipeline-validator/ --debug-recipes-qc` is the upstream-owned QC bundle for debug fixtures and resolved concrete exports; it does not absorb GTD token semantics.
- The canonical engine behavior, effect inventory, and terminal-motion guidance
  live in `tui-vfx`.

# <FILE>docs/INDEX.md</FILE> - <DESC>Documentation index for tui-vfx-recipes</DESC>
# <VERS>END OF VERSION: 0.13.0</VERS>
