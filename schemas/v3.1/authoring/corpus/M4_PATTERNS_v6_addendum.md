# Addendum to M4 v6 — late-found patterns

After the round-5 saturation declaration, two more recipes were checked at the
project owner's prompt: btop's `examples/btop_focused_row_live_list.json` and
the V3.1 Canada flag scene.

## K1. Separate-field binding form (V2-flavored — 2 demonstrators)

`btop_focused_row_live_list.json` uses a different authoring shape for the
same canonical destination as the V2 inline `{binding, default}` form:

```json
"spatial_shader": {
  "type": "focused_row_gradient",
  "selected_row_binding": "selected_row",   ← parallel field naming the binding
  "selected_row": 4,                         ← original field carrying the default
  "falloff_distance": 16
}
```

Compare to the more-common V2 inline form (~12 corpus witnesses):

```json
"selected_row": { "binding": "selected_row", "default": 4 }
```

Both canonicalize to the same V3.1:

```json
"selectedRow": { "kind": "signal", "id": "selectedRow", "fallback": { "kind": "number", "value": 4 } }
```

Second demonstrator: `scene_layer_full_stack` (round 1 corpus) uses
`phase_offset_ms` as a similar parallel-field shape inside its motion config.

**Three input shapes, one canonical output.** The shorthand should accept all
three; the canonicalizer normalizes:

1. V2 inline: `param: { binding: "name", default: V }`
2. V2 separate-field: `param_binding: "name"` + `param: V`
3. V3 graph reference: `param: { binding: "name" }` (loose default)

Plus the shorthand's own form: `param: "$bind:name"`.

The alias-table format already supports parallel-field aliasing — entries like
`"<name>_binding"` route to the canonical signal-reference path. No new
schema-shape question raised.

## K2. Canada flag — confirms Madeira asset-bound pattern (no new shape)

`scene_canada_flag_runtime_wave.json` uses the same architecture as
`scene_madeira_flag_full_scene` (round 3) and `scene_authoring_ladder_flag_asset_binding`
(round 1):

- Procedural source `braille_flag_field`.
- Asset-backed dotfield via `requires_assets`-equivalent (V3.1: top-level
  `assets: {}` block).
- `wave_speed` runtime binding with ramp loopback.

Distinguishing details: smaller scene (40x14 vs Madeira's 80x24), 2:1 aspect
asset, 2x oversampled (160x80 dotfield underneath). All visual; no
architectural novelty.

Confirms the **asset-bound waving-flag pattern** as a third witness, which
crosses rule of three and promotes the asset-shorthand syntax (`$asset:<id>`)
proposed in M4_PATTERNS.md Q7. The Canada flag pair shows the proposed
shorthand form for asset references inline.

## Updated transition-preset and pattern coverage

No change to the 16 transition presets table — all already covered.

Updated total:

- **176 recipes read + 5 schema-faith demonstrators authored** = 181
  evidence units.
- **44 paired files** (39 corpus-derived + 5 schema-faith).
- **6 M4 docs + 1 addendum**.
- **16 open design questions** (unchanged — K1 and K2 don't raise new ones).

## Recommendation unchanged

Move to M5. The two late-found patterns confirm what the corpus already
implied: vocabulary continues to extend, architecture stays stable. The
shorthand mechanism handles K1 (parallel-field binding) via alias-table
entries that name the parallel field as a binding-routing alias. The corpus
work is genuinely complete for design purposes.
