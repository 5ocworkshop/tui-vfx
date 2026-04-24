<!-- <FILE>docs/tooling/release-gate-evidence.md</FILE> - <DESC>Executable evidence-record contract and command map for V3 Chapter 60 release gates.</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>Make the seeded Chapter 60 release-gate manifest actionable by mapping each evidence type to existing tooling and a small checked record shape.</WCTX> -->
<!-- <CLOG>0.1.1: add the probe_alarm_lighthouse smoke plan and clarify the small local sidecar record handoff.</CLOG> -->

# Release-gate evidence records

This guide turns the V3 release-gate manifest seed into an executable evidence
capture contract. It does not add a second frame-diff system. Probe and frame
comparison evidence continues to use `recipe-probe` / `pipeline-validator --probe`
and the existing SQLite xray surface documented in
[Probe database and frame diffs](probe-database-and-frame-diffs.md).

Use this guide for Track D / `D6-RELEASE-EVIDENCE` after selecting a fixture from
[`tui-vfx-v3-release-gate-manifest.seed.json`](../design/tui-vfx-v3-release-gate-manifest.seed.json).

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

For a low-risk first D6 slice, capture `probe_alarm_lighthouse`:

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
2. Write the sidecar record next to the report as
   `artifacts/release-gates/probe_alarm_lighthouse/evidence.record.json`.
3. Keep the sidecar tiny; do not embed the full report payload.

The checked-in template is the minimum JSON example above. Treat it as the
shape to copy when D6 needs a command-backed smoke record without promoting a
large generated artifact.

<!-- <FILE>docs/tooling/release-gate-evidence.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.1</VERS> -->
