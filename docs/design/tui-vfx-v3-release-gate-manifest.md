<!-- <FILE>docs/design/tui-vfx-v3-release-gate-manifest.md</FILE> - <DESC>First concrete V3 release-gate fixture manifest seed for Track D / Chapter 60.</DESC> -->
<!-- <VERS>VERSION: 0.2.2</VERS> -->
<!-- <WCTX>Define the first checked-in manifest shape for the six V3 release gates so evidence capture and CI can bind to stable fixture IDs without overclaiming approved outcomes.</WCTX> -->
<!-- <CLOG>0.2.2: add the checked probe_alarm_lighthouse evidence-record template and keep the smoke handoff small.</CLOG> -->

# V3 release-gate manifest seed

This document is the first concrete fixture manifest for Track D / `D5-RELEASE-MANIFESTS`.

It is a **seed**, not a captured-evidence report. It defines stable fixture IDs,
recipe targets, expected evidence types, and ownership rules so later work in
`D6-RELEASE-EVIDENCE` and `D7-CI-CUTOVER` can bind to a checked-in contract.

Companion machine-readable seed:

- [`tui-vfx-v3-release-gate-manifest.seed.json`](tui-vfx-v3-release-gate-manifest.seed.json)
- [Release-gate evidence records](../tooling/release-gate-evidence.md)

## Why this shape exists now

- The [V3 execution DAG](tui-vfx-v3-execution-dag.md) marks release manifests as
  a prerequisite for evidence capture and CI cutover.
- The [V3 release-gate policy](tui-vfx-v3-release-gate-policy.md) already fixes
  the allowed outcome states and approval ownership split.
- The remaining blocker is a checked-in fixture manifest shape that names the
  critical gate entries without pretending the evidence is already captured.

## Scope and boundary

- The manifest lives in `tui-vfx` because the release-gate policy and Chapter 60
  gate definition live here.
- Concrete recipe fixtures remain in `tui-vfx-recipes`; this manifest points at
  those paths instead of copying them.
- This seed does **not** assign outcomes yet. Evidence capture and any
  `accepted_change` decisions happen in later work.

## Allowed outcome states

These are the only accepted per-fixture outcomes when evidence is attached.
They come directly from the accepted V3 release-gate policy.

| State | Meaning |
| --- | --- |
| `pass` | V3 output matches the expected tolerance for the fixture. |
| `fail` | Regression or unresolved mismatch. |
| `accepted_change` | Intended difference with checked-in rationale and approval. |
| `stale_fixture` | Reference capture is outdated and needs explicit recapture approval. |
| `not_applicable` | The gate does not apply to the targeted recipe or surface. |

## Evidence type catalog

These evidence types are intentionally small and map to existing tooling
surfaces. D6 can add artifact paths later without changing the fixture IDs.

| Evidence type | Purpose | Canonical tool / surface |
| --- | --- | --- |
| `render_capture_png` | Human visual render capture for library/debug fixtures. | `demo.rs`, `play_recipe.rs`, or equivalent checked capture workflow |
| `gtd_surface_capture` | Human-approved representative GT-Design surface capture. | GT-Design representative preview / capture flow |
| `probe_report_json` | Structured recipe-aware probe report and frame-diff evidence. | `recipe-probe` or `pipeline-validator --probe` |
| `trace_report_json` | Structured lifecycle / resolution / composition / pipeline trace evidence. | `tui-vfx-trace --format report` |
| `lowering_report_json` | Structured lowering / canonicalization truth surface for role-aware checks. | `pipeline-validator --lowering-report --format json` |

## Ownership defaults

These defaults come from the accepted policy and should be reused unless a
future manifest revision records a narrower override.

| Gate | Default owner | Approval notes |
| --- | --- | --- |
| `shadow` | tui-vfx implementation lead | Library-level `accepted_change` can be proposed here; stale recapture still requires explicit approval. |
| `offscreen` | tui-vfx implementation lead | Same approval split as `shadow`. |
| `probe` | tui-vfx tooling lead | Probe semantics and snapshot expectations live with tooling ownership. |
| `trace` | tui-vfx tooling lead | Trace schema additions may be acceptable; semantic drift still needs explicit rationale. |
| `gtd_integration` | project owner / GTD lead | Representative fixture selection and any product-visible `accepted_change` require owner approval. |
| `role_aware_lowering` | tui-vfx implementation lead | Library-level role-lowering changes may be classified here unless they become product-visible GTD drift. |

Global rule: `stale_fixture` always means explicit recapture approval is needed;
never recapture silently.

## Fixture manifest entries

### 1. Shadow gate

| Fixture ID | Recipe target | Required evidence | Ownership / approval notes |
| --- | --- | --- | --- |
| `shadow_surface_base` | `tui-vfx-recipes:recipes/debug_recipes/scene/scene_layer_surface_shadow.json` | `render_capture_png`, `probe_report_json` | Base scene-level shadow truth surface. Implementation-lead owned. |
| `shadow_surface_pipeline` | `tui-vfx-recipes:recipes/debug_recipes/scene/scene_layer_surface_shadow_pipeline.json` | `render_capture_png`, `probe_report_json`, `trace_report_json` | Covers shadow behavior with the scene-layer pipeline active. |
| `shadow_edge_crossing` | `tui-vfx-recipes:recipes/debug_recipes/motion/toast_shadow_edge_crossing.json` | `render_capture_png`, `probe_report_json`, `trace_report_json` | Critical bridge between shadow, motion envelope, and vanishing-edge behavior. |

### 2. Offscreen / slide gate

| Fixture ID | Recipe target | Required evidence | Ownership / approval notes |
| --- | --- | --- | --- |
| `offscreen_content_slide_shift` | `tui-vfx-recipes:recipes/debug_recipes/content/content_slide_shift.json` | `render_capture_png`, `probe_report_json` | Smallest clear slide fixture. Good first smoke target. |
| `offscreen_scene_full_stack` | `tui-vfx-recipes:recipes/debug_recipes/scene/scene_layer_full_stack.json` | `render_capture_png`, `probe_report_json`, `trace_report_json` | Multi-layer/offscreen composition coverage. |
| `offscreen_follow_lag` | `tui-vfx-recipes:recipes/debug_recipes/motion/scene_layer_follow_lag.json` | `render_capture_png`, `probe_report_json` | Motion/offscreen behavior without GTD-product coupling. |

### 3. Probe gate

| Fixture ID | Recipe target | Required evidence | Ownership / approval notes |
| --- | --- | --- | --- |
| `probe_alarm_lighthouse` | `tui-vfx-recipes:recipes/vfx-probe-validation/alarm_lighthouse.json` | `probe_report_json` | Part of the dedicated probe-validation corpus. Tooling-lead owned. First command-backed smoke target for D6. |
| `probe_midnight_switchboard` | `tui-vfx-recipes:recipes/vfx-probe-validation/midnight_switchboard.json` | `probe_report_json` | Good multi-style / multi-stage probe reference. |
| `probe_wormhole_pageant` | `tui-vfx-recipes:recipes/vfx-probe-validation/wormhole_pageant.json` | `probe_report_json` | High-stage-activity probe corpus member for repeated diff evidence. |

### 4. Trace gate

| Fixture ID | Recipe target | Required evidence | Ownership / approval notes |
| --- | --- | --- | --- |
| `trace_path_primitive` | `tui-vfx-recipes:recipes/debug_recipes/shaders/primitives/shader_trace_path.json` | `trace_report_json` | Primitive-level trace fixture; should stay narrow and legible. |
| `trace_propagation_primitive` | `tui-vfx-recipes:recipes/debug_recipes/shaders/primitives/shader_trace_propagation.json` | `trace_report_json` | Companion primitive that exercises trace propagation semantics. |
| `trace_scene_role_scope` | `tui-vfx-recipes:recipes/debug_recipes/scene/scene_layer_role_scope_pipeline.json` | `trace_report_json`, `lowering_report_json` | Bridges trace expectations to scene/lowering behavior instead of only shader primitives. |

### 5. GT-Design integration gate

These are **provisional representative candidates**, not owner-approved final
selection. The point of the seed is to establish the manifest shape and nominate
concrete downstream targets early enough for D6 wiring.

| Fixture ID | Recipe target | Required evidence | Ownership / approval notes |
| --- | --- | --- | --- |
| `gtd_toast_success_restrained` | `tui-vfx-recipes:recipes/gt-design/restrained/R01_fuji_enso_success.json` | `gtd_surface_capture`, `probe_report_json` | Candidate restrained success/toast fixture. Final representative-set approval belongs to the project owner / GTD lead. |
| `gtd_modal_midrange` | `tui-vfx-recipes:recipes/gt-design/mid-range/M05_blueprint_schematic_reveal_modal.json` | `gtd_surface_capture`, `probe_report_json` | Candidate modal fixture with richer staged behavior. |
| `gtd_drawer_bold` | `tui-vfx-recipes:recipes/gt-design/bold/B07_flw_art_glass_drawer.json` | `gtd_surface_capture`, `probe_report_json` | Candidate drawer / slide surface. |
| `gtd_progress_midrange` | `tui-vfx-recipes:recipes/gt-design/mid-range/M09_stuttgart_instrument_cluster_progress.json` | `gtd_surface_capture`, `probe_report_json` | Candidate progress / overlay representative. |

### 6. Role-aware lowering gate

| Fixture ID | Recipe target | Required evidence | Ownership / approval notes |
| --- | --- | --- | --- |
| `role_scope_scene_pipeline` | `tui-vfx-recipes:recipes/debug_recipes/scene/scene_layer_role_scope_pipeline.json` | `lowering_report_json`, `render_capture_png`, `trace_report_json` | Primary role-aware scene lowering fixture. |
| `role_scope_border_style` | `tui-vfx-recipes:recipes/debug_recipes/styles/style_role_scope_border.json` | `lowering_report_json`, `render_capture_png` | Narrow style-role fixture for border-specific lowering. |
| `role_scope_parallel_conflict` | `tui-vfx-recipes:recipes/debug_recipes/complex/complex_parallel_role_scopes.json` | `lowering_report_json`, `probe_report_json`, `trace_report_json` | Stress fixture for overlapping/disjoint role scopes and conflict handling. |

## Evidence record handoff

D6 evidence records use the small JSON sidecar shape documented in
[Release-gate evidence records](../tooling/release-gate-evidence.md). The record
keys by `fixture_id`, names the command that produced each artifact, and stores
large probe/trace/render outputs as separate artifacts. Start with
`probe_alarm_lighthouse` as the first command-backed smoke record and keep the
sidecar tiny. Probe and frame-diff evidence must reuse `recipe-probe` / `pipeline-validator --probe` and the
existing SQLite xray surface; do not add a second diff schema for Chapter 60.

The checked template for that sidecar lives at
[`../tooling/probe_alarm_lighthouse.evidence.record.template.json`](../tooling/probe_alarm_lighthouse.evidence.record.template.json).
Copy it to `artifacts/release-gates/probe_alarm_lighthouse/evidence.record.json`
when the smoke capture is produced; do not check in the generated artifact.

## Companion JSON expectations

The JSON seed mirrors this document and is intended to be the future automation
entry point.

It should remain limited to:

- stable `fixture_id` values
- gate name
- repo/path target
- required evidence type list
- ownership / approval notes
- allowed outcome-state vocabulary

It should **not** grow captured evidence blobs or CI-specific runtime state in
place. Those belong in generated outputs or later evidence records keyed by
`fixture_id`.

## Next handoff

1. D6 can attach evidence records keyed to these fixture IDs using `docs/tooling/release-gate-evidence.md`, starting with the command-backed `probe_alarm_lighthouse` smoke record and its tiny sidecar.
2. GTD owner can confirm, replace, or shrink the provisional GTD fixture set
   without changing the overall manifest shape.
3. D7 can consume the JSON seed for CI enumeration once evidence capture exists.

<!-- <FILE>docs/design/tui-vfx-v3-release-gate-manifest.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.2</VERS> -->
