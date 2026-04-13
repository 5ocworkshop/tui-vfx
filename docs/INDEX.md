<!-- <FILE>docs/INDEX.md</FILE> - <DESC>Documentation table of contents</DESC> -->
<!-- <VERS>VERSION: 1.3.0</VERS> -->
<!-- <WCTX>Add pipeline-probe direct-engine debugging guide reference</WCTX> -->
<!-- <CLOG>MINOR: Link the new PIPELINE_PROBE_LLM_GUIDE.md alongside the existing pipeline-validator guide and design doc</CLOG> -->

# Documentation Index

## Hand-Maintained
- [TERMINAL_MOTION_HEURISTICS.md](TERMINAL_MOTION_HEURISTICS.md) — Canonical terminal-specific motion, depth, and compositing heuristics for effect and recipe design
- [API_HAND.md](API_HAND.md) — Original hand-maintained API reference
- [CAPABILITIES_REFERENCE.md](CAPABILITIES_REFERENCE.md) — Hand-maintained capabilities reference
- [HOWTO_SHADOWS.md](HOWTO_SHADOWS.md) — Shadow rendering guide and integration patterns
- [PIPELINE_VALIDATOR_LLM_GUIDE.md](PIPELINE_VALIDATOR_LLM_GUIDE.md) — How an LLM should use the `pipeline-validator` CLI (in the sibling `tui-vfx-recipes` repo) to inspect recipe rendering, diagnose shader bugs, and verify per-cell output
- [PIPELINE_PROBE_LLM_GUIDE.md](PIPELINE_PROBE_LLM_GUIDE.md) — How an LLM or user should use the engine-side `pipeline-probe` CLI to inspect direct `ProbeSceneSpec` inputs as structured JSON/NDJSON
- [design/pipeline-probe-design.md](design/pipeline-probe-design.md) — The current phase-1 design and rollout plan for engine-owned AI-native observability

## Generated (via `cargo xtask docs`)
- [generated/API.md](generated/API.md) — Auto-generated API reference from code + TOML templates
- [generated/CAPABILITIES.md](generated/CAPABILITIES.md) — Auto-generated capabilities inventory
- [generated/ai-context.md](generated/ai-context.md) — Condensed AI context prompt
- [generated/capabilities.json](generated/capabilities.json) — Machine-readable effect inventory
- [generated/effect_schemas.json](generated/effect_schemas.json) — Full ConfigSchema per effect

<!-- <FILE>docs/INDEX.md</FILE> - <DESC>Documentation table of contents</DESC> -->
<!-- <VERS>END OF VERSION: 1.3.0</VERS> -->
