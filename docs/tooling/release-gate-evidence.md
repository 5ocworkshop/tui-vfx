<!-- <FILE>docs/tooling/release-gate-evidence.md</FILE> - <DESC>Executable evidence-record contract and command map for V3 Chapter 60 release gates.</DESC> -->
<!-- <VERS>VERSION: 0.1.6</VERS> -->
<!-- <WCTX>Make the seeded Chapter 60 release-gate manifest actionable by mapping each evidence type to existing tooling and the D5/D6/D7 gate lanes.</WCTX> -->
<!-- <CLOG>0.1.6: add the just-based probe_alarm_lighthouse smoke check as the smallest headless gate rehearsal.</CLOG> -->

# Release-gate evidence records

This guide turns the V3 release-gate manifest seed into an executable evidence
capture contract. It does not add a second frame-diff system. Probe and frame
comparison evidence continues to use `recipe-probe` / `pipeline-validator --probe`
and the existing SQLite xray surface documented in
[Probe database and frame diffs](probe-database-and-frame-diffs.md).

Use this guide for Track D / `D6-RELEASE-EVIDENCE` after selecting a fixture from
[`tui-vfx-v3-release-gate-manifest.seed.json`](../design/tui-vfx-v3-release-gate-manifest.seed.json).

## Gate alignment

This is the gate-level view of the same command map. It keeps the evidence
commands tied back to the punch list and DAG lanes without introducing a new
capture schema.

| Gate lane | Existing command surface | Evidence types | First fixture IDs |
|---|---|---|---|
| `V3-CI02` / `D5-RELEASE-MANIFESTS` / `D6-RELEASE-EVIDENCE` / probe | `cargo run -q -p recipe-probe -- <recipe> --format json --phase dwelling --sample-t <t> --with-causation` or `pipeline-validator --probe` | `probe_report_json` | `probe_alarm_lighthouse`, `probe_midnight_switchboard`, `probe_wormhole_pageant` |
| `V3-CI02` / `D5-RELEASE-MANIFESTS` / `D6-RELEASE-EVIDENCE` / trace | `cargo run -q -p tui-vfx-trace -- --recipe <recipe> --format report` | `trace_report_json` | `trace_path_primitive`, `trace_propagation_primitive`, `trace_scene_role_scope` |
| `V3-CI02` / `D5-RELEASE-MANIFESTS` / `D6-RELEASE-EVIDENCE` / shadow | `cargo run --example play_recipe -- <recipe>` or `cargo run --example demo -- <recipe>` plus `recipe-probe` | `render_capture_png`, `probe_report_json` | `shadow_surface_base`, `shadow_surface_pipeline`, `shadow_edge_crossing` |
| `V3-CI02` / `D5-RELEASE-MANIFESTS` / `D6-RELEASE-EVIDENCE` / offscreen | same render surfaces plus `recipe-probe` / `tui-vfx-trace` as needed | `render_capture_png`, `probe_report_json`, `trace_report_json` | `offscreen_content_slide_shift`, `offscreen_scene_full_stack`, `offscreen_follow_lag` |
| `V3-CI02` / `D5-RELEASE-MANIFESTS` / `D6-RELEASE-EVIDENCE` / gtd_integration | GTD representative preview / capture flow | `gtd_surface_capture`, `probe_report_json` | `gtd_toast_success_restrained`, `gtd_modal_midrange`, `gtd_drawer_bold`, `gtd_progress_midrange` |
| `V3-CI02` / `D5-RELEASE-MANIFESTS` / `D6-RELEASE-EVIDENCE` / role_aware_lowering | `pipeline-validator --lowering-report --format json` plus render/trace/probe as needed | `lowering_report_json`, `render_capture_png`, `trace_report_json` | `role_scope_scene_pipeline`, `role_scope_border_style`, `role_scope_parallel_conflict` |

Start with the probe and trace fixtures. They are the smallest command-backed
captures and give D6 enough evidence to validate the record shape before the
render and GTD lanes widen the scope.

## Current V3-CI02 evidence status

As of 2026-04-24, command-backed headless evidence records exist for every
non-GTD fixture that can be advanced without product-owner visual judgment. The
prior `probe_midnight_switchboard` technical blocker is resolved in
`tui-vfx-recipes` commit `8a2eca7`; remaining CI02 blockers are visual-capture or
GTD-fixture approval items, not known headless probe failures.

| Gate | Current record status | Notes |
|---|---|---|
| `shadow` | 3 pass | Existing render/probe/trace evidence records cover all seeded shadow fixtures. |
| `trace` | 3 pass | Existing trace/lowering evidence records cover all seeded trace fixtures. |
| `probe` | 3 pass | `probe_midnight_switchboard` now reports success for combined frame/lifecycle analysis, with configured style/shader effects observed. |
| `offscreen` | 1 pass, 2 fail | `offscreen_follow_lag` now has passing headless probe evidence, but it and `offscreen_scene_full_stack` remain blocked on required `render_capture_png`. |
| `role_aware_lowering` | 1 pass, 2 fail | `role_scope_scene_pipeline` and `role_scope_border_style` now have passing headless lowering/trace evidence where required, but remain blocked on required `render_capture_png`. |
| `gtd_integration` | 4 owner/GTD-dependent missing records | These require owner-approved GTD representative surface capture and are not invented from library/tooling fixtures. |

For records whose only missing evidence is `render_capture_png`, classify the
remaining gap as `blocked-on-explicit-owner-visual-capture` until an owner
explicitly approves a visual capture workflow. Do not open X11, Zutty, Xvfb, or
demo-window capture surfaces during headless evidence packets.

## Evidence record shape

Evidence records are small JSON sidecars keyed by `fixture_id`. They reference
artifacts produced by existing tools instead of embedding reports or inventing a
new diff format.

Minimum record:

```json
{
  "schema": "tui_vfx_v3_release_gate_evidence.v1",
  "fixture_id": "probe_alarm_lighthouse",
  "gate": "probe",
  "status": "pass",
  "captured_at": "2026-04-24T00:00:00Z",
  "recipe": {
    "repo": "tui-vfx-recipes",
    "path": "recipes/vfx-probe-validation/alarm_lighthouse.json"
  },
  "tool_runs": [
    {
      "evidence_type": "probe_report_json",
      "command": "cargo run -q -p recipe-probe -- recipes/vfx-probe-validation/alarm_lighthouse.json --format json --phase dwelling --sample-t 1.0 --with-causation",
      "artifact": "artifacts/release-gates/probe_alarm_lighthouse/probe_report.json",
      "status": "pass"
    }
  ],
  "review": {
    "owner": "tui-vfx tooling lead",
    "approval_required": false,
    "notes": "Probe report emitted lifecycle and stage analysis without diagnostics."
  }
}
```

Rules:

- `fixture_id`, `gate`, `recipe`, and required `evidence_type` values must match
  the seed manifest.
- `status` must be one of the manifest's allowed outcome states.
- `accepted_change` records must include a rationale and the approving owner in
  `review.notes` / `review.owner`.
- `stale_fixture` records must not silently recapture. They mark that explicit
  recapture approval is needed.
- Large reports stay as artifacts. The record only names the command and artifact
  path. Generated artifacts should live under
  `artifacts/release-gates/<fixture_id>/...` and are treated as local/generated
  evidence unless a release lane explicitly promotes selected records into
  checked-in fixtures.

## Command map by evidence type

Run commands from `/usr/projects/tui-vfx-recipes` unless noted.

| Evidence type | Existing command surface | Artifact to store | Notes |
|---|---|---|---|
| `probe_report_json` | `cargo run -q -p recipe-probe -- <recipe> --format json --phase dwelling --sample-t <t> --with-causation` | `probe_report.json` | Preferred recipe-backed probe path. Add `--diff-to <t>` for frame diff evidence or `--sqlite-query <sql>` for focused database checks. |
| `trace_report_json` | `cargo run -q -p tui-vfx-trace -- --recipe <recipe> --format report` | `trace_report.json` | Use for lifecycle / resolution / composition / pipeline trace evidence. If the trace CLI changes flags, keep this map current rather than creating a new trace wrapper. |
| `lowering_report_json` | `cargo run -q -p pipeline-validator -- --lowering-report --format json <recipe>` | `lowering_report.json` | Canonical lowering / human-review-needed surface for role-aware checks. |
| `render_capture_png` | `cargo run -q --example play_recipe -- <recipe>` or the existing demo capture workflow | `render_capture.png` | Human visual sign-off. Pair with probe evidence for machine-readable comparison. |
| `gtd_surface_capture` | GT-Design representative preview / capture flow | `gtd_surface_capture.png` | Owner-approved downstream capture. Do not classify product-visible drift without GTD lead approval. |

## First smoke target

For a low-risk first D6 slice, run the checked smoke target or capture `probe_alarm_lighthouse` directly:

```bash
cd /usr/projects/tui-vfx-recipes
just v3-release-gate-probe-smoke
```

```bash
cd /usr/projects/tui-vfx-recipes
mkdir -p artifacts/release-gates/probe_alarm_lighthouse
cargo run -q -p recipe-probe -- \
  recipes/vfx-probe-validation/alarm_lighthouse.json \
  --format json \
  --phase dwelling \
  --sample-t 1.0 \
  --with-causation \
  > artifacts/release-gates/probe_alarm_lighthouse/probe_report.json
```

Optional database check using the same probe surface:

```bash
cargo run -q -p recipe-probe -- \
  recipes/vfx-probe-validation/alarm_lighthouse.json \
  --phase dwelling \
  --sample-t 1.0 \
  --with-causation \
  --sqlite-query "select scope, stage, status, observed_event_count from probe_analysis_stages order by scope, stage"
```

This is intentionally only probe evidence. Shadow, offscreen, trace,
GT-Design, and role-aware lowering gates use the same record shape with their
own required evidence types from the manifest.

## First smoke record plan

Use `probe_alarm_lighthouse` as the first actual command-backed smoke record.
Keep the generated probe report local under
`artifacts/release-gates/probe_alarm_lighthouse/` and pair it with a small
sidecar JSON record that follows the minimum record shape above.

Suggested sequence:

1. Run the command in [First smoke target](#first-smoke-target) from
   `/usr/projects/tui-vfx-recipes`.
2. Copy the checked template from
   `docs/tooling/probe_alarm_lighthouse.evidence.record.template.json` to
   `artifacts/release-gates/probe_alarm_lighthouse/evidence.record.json`.
3. Keep the sidecar tiny; do not embed the full report payload.

The checked-in template is the minimum JSON example above. Treat it as the
shape to copy when D6 needs a command-backed smoke record without promoting a
large generated artifact.

Template file:

- [`probe_alarm_lighthouse.evidence.record.template.json`](probe_alarm_lighthouse.evidence.record.template.json)

<!-- <FILE>docs/tooling/release-gate-evidence.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.6</VERS> -->
