<!-- <FILE>docs/INDEX.md</FILE> - <DESC>Documentation table of contents</DESC> -->
<!-- <VERS>VERSION: 1.6.0</VERS> -->
<!-- <WCTX>feat/cursor-primitive T29: link new Cursor primitive section</WCTX> -->
<!-- <CLOG>MINOR: Add a link to the Cursor primitive section in CAPABILITIES_REFERENCE.md</CLOG>

# Documentation Index

## Hand-Maintained
- [TERMINAL_MOTION_HEURISTICS.md](TERMINAL_MOTION_HEURISTICS.md) — Canonical terminal-specific motion, depth, and compositing heuristics for effect and recipe design
- [API_HAND.md](API_HAND.md) — Original hand-maintained API reference
- [CAPABILITIES_REFERENCE.md](CAPABILITIES_REFERENCE.md) — Hand-maintained capabilities reference
- [HOWTO_SHADOWS.md](HOWTO_SHADOWS.md) — Shadow rendering guide and integration patterns
- [PIPELINE_VALIDATOR_LLM_GUIDE.md](PIPELINE_VALIDATOR_LLM_GUIDE.md) — How an LLM should use the `pipeline-validator` CLI (in the sibling `tui-vfx-recipes` repo) to inspect recipe rendering, diagnose shader bugs, and verify per-cell output
- [PIPELINE_PROBE_LLM_GUIDE.md](PIPELINE_PROBE_LLM_GUIDE.md) — How an LLM or user should use the engine-side `pipeline-probe` CLI to inspect direct `ProbeSceneSpec` inputs as structured JSON/NDJSON
- [PIPELINE_PROBE_WISHLIST.md](PIPELINE_PROBE_WISHLIST.md) — Prioritized wishlist for finishing the dream AI-native recipe debug tool, including the current diminishing-returns assessment
- [RECIPE_AUTHORING_WORKFLOW.md](RECIPE_AUTHORING_WORKFLOW.md) — Canonical staged workflow for building complex recipes one effect at a time, validating each layer, then flattening to a single final file
- [RECIPE_VISUAL_QA.md](RECIPE_VISUAL_QA.md) — Canonical visual checklist for manually previewing and signing off complex probe-validation recipes
- [design/pipeline-probe-design.md](design/pipeline-probe-design.md) — The current phase-1 design and rollout plan for engine-owned AI-native observability
- [RESEARCH_DESIGN_EXCELLENCE.md](RESEARCH_DESIGN_EXCELLENCE.md) — Cross-domain research summary for subtle, premium terminal polish and the current recommendation shortlist
- [DESIGN_EXCELLENCE_USAGE_GUIDE.md](DESIGN_EXCELLENCE_USAGE_GUIDE.md) — Research-backed guidance on when and how often subtle effects should be used, including fatigue management and deterministic-vs-stochastic rules
- [Cursor primitive](CAPABILITIES_REFERENCE.md#cursor-primitive-since-040) — General-purpose cursor primitive with grow-in + wake trail; powers `TypewriterCursor` and standalone caret overlays

## Generated (via `cargo xtask docs`)
- [generated/API.md](generated/API.md) — Auto-generated API reference from code + TOML templates
- [generated/CAPABILITIES.md](generated/CAPABILITIES.md) — Auto-generated capabilities inventory
- [generated/ai-context.md](generated/ai-context.md) — Condensed AI context prompt
- [generated/capabilities.json](generated/capabilities.json) — Machine-readable effect inventory
- [generated/effect_schemas.json](generated/effect_schemas.json) — Full ConfigSchema per effect

<!-- <FILE>docs/INDEX.md</FILE> - <DESC>Documentation table of contents</DESC> -->
<!-- <VERS>END OF VERSION: 1.5.0</VERS> -->
