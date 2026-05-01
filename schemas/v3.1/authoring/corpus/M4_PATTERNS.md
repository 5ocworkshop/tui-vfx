# M4 — Patterns extracted from the corpus

39 recipes read across V2, V3, V3.1 schema versions. 10 paired files written to
`canonical/` and `shorthand/`. Patterns below are evidence-backed; the count after
each pattern names how many of the 39 source recipes confirm it.

The convention for advancing a pattern from "noted" to "rule":
**rule of three** — three or more recipes must demand the pattern before it earns
its place in the alias or expansion table. Patterns with fewer than three
demonstrators are filed under "Single-witness — push back into canonical."

---

## A. Wrapper-stripping patterns (canonicalizer expands literal envelopes)

These are the highest-leverage patterns. They affect every recipe, regardless of
content. Without them, shorthand cannot be measurably terser than canonical.

### A1. Bare literal in a typed context (39/39)

Canonical:
```json
"width": { "kind": "literal", "value": { "kind": "integer", "value": 35 } }
```
Shorthand:
```json
"size": [35, 3]
```
Every recipe in the corpus exercises this. The `inputs` map's value type is
known from the descriptor, so the `{kind:literal,value:{kind:T,value:V}}`
wrapper is the canonicalizer's job, not the author's.

**Rule:** for any descriptor input typed as a primitive (integer, number,
boolean, text, string, color, enum), accept the bare value at the shorthand
layer and lift it into the canonical envelope at canonicalize time.

### A2. Color triple → hex string or RGB array (38/39)

Canonical (six lines):
```json
"foreground": { "kind": "literal", "value": { "kind": "color", "value": { "r": 255, "g": 255, "b": 255, "a": 255 } } }
```
Shorthand (one line):
```json
"fg": "#ffffff"
```
Both forms accepted: `"#rrggbb"`, `"#rrggbbaa"`, `[r, g, b]`, `[r, g, b, a]`.
Named colors (`"white"`, `"cyan"`, `"transparent"`) also accepted; `"transparent"`
canonicalizes to `{r:0,g:0,b:0,a:0}`.

The only recipe in the corpus that doesn't trip this is `bool_binding_demo`
when read in a hypothetical "no-color" form; in practice it sets fg/bg too.

### A3. Duration → string with unit (39/39)

Canonical:
```json
{ "kind": "milliseconds", "value": 60000 }
```
Shorthand:
```json
"60s"   // or "60000ms", or "1m"
```
Every duration occurrence in the corpus uses the milliseconds variant. Other
duration kinds (frames, beats) exist in the schema but are not used by any
corpus recipe. The string form `"<n><unit>"` covers ms/s/m and is unambiguous.

### A4. Easing — bare named string OR bezier object (12/39)

Canonical:
```json
"easing": { "kind": "named", "value": "quadOut" }
```
Shorthand:
```json
"easing": "quadOut"
```
Custom bezier:
```json
"easing": { "x1": 0.18, "y1": 0.88, "x2": 0.32, "y2": 1.24 }
```
Eight named easings appear across the corpus (`linear`, `quadOut`, `quadIn`,
`cubicOut`, `cubicIn`, `sineInOut`, `expoOut`, `backOut`). One custom bezier.
Canonicalizer routes bare strings via the named-easing table; routes 4-tuple
objects to bezier form.

---

## B. Default-omission patterns (canonicalizer fills in defaults)

These leverage the M1 schema relaxation directly. Authors don't write fields
that already have sensible defaults.

### B1. Empty graph collections omitted (39/39 post-relaxation)

After M1, GraphSpec's `parameters`, `signals`, `bindings`, `effects`, `nodes`,
`order` all default. A recipe with no graph contents writes only:
```json
"graph": { "id": "mainGraph", "version": "3.1" }
```
Six fewer lines per simple recipe. The shorthand can omit `graph` entirely
when there are no effects — the canonicalizer inserts the empty stub.

### B2. Standard lifecycle defaulted (28/39)

The pattern `1s/5s/1s` (enter / dwell / exit, all `fixed`) appears verbatim in
28 recipes. Shorthand allows omission entirely:
```json
// no lifecycle field → canonicalizer inserts default 1s/5s/1s
```
Or string-shorthand override:
```json
"lifecycle": "1s/5s/1s"
```
Or per-phase object override (covers loops, dwell-fallback, motion routes):
```json
"lifecycle": { "enter": "1.5s", "dwell": "5s", "exit": "800ms" }
```
The `until + fallback` form (event-driven dwell, B7 below) extends the same
shape.

### B3. `pipeline: null` omitted (relaxed by M1) (16/39)

Scene elements without an attached graph drop the `pipeline: null` field.
Already accepted by the relaxed schema; shorthand inherits the omission.

### B4. Scene-element write policies omitted (39/39 once defaulted)

`clipPolicy: "clip"`, `cellWritePolicy: "writeCell"`, `roleWritePolicy:
{"kind":"preserveDestination"}` — every recipe writes them. Per the Pattern-5
deferral, they aren't serde-defaulted yet, so canonical recipes still spell
them out. Shorthand omits them; the canonicalizer fills them in. **When the
substrate work lands and the M1-deferred policy defaults are added, the
canonicalizer rule simplifies to "M1 already does this."**

### B5. `placement` derived from `placementRule` when absolute (8/39)

`placement: {x, y}` and `placementRule: {kind: "absolute", rect: {x, y, w, h}}`
encode overlapping facts. Shorthand writes one (`at: [x, y]` or `at: "center"`),
the canonicalizer fills both.

### B6. Single-source single-element scene → implicit (24/39)

When a recipe has one source and one scene element placing it, both are
inferable from a top-level `card` or `text` block:
```json
"card": { "message": "...", "size": [35, 3], "fg": "...", "bg": "...", "border": "rounded" }
```
canonicalizes to a `mainCard` source + `mainScene` with one element. 24 of 39
corpus recipes have this shape.

For multi-source recipes the explicit `sources` + `scenes` blocks remain. A
single shorthand recipe must pick one form.

### B7. Event-driven dwell shorthand (2/39 + 1 from V3 corpus)

```json
"lifecycle": { "dwell": { "until": "$bind:userDismissed", "fallback": "5s" } }
```
canonicalizes to:
```json
"timing": { "kind": "dwell", "policy": { "kind": "untilTruthy", "binding": "userDismissed", "fallback": { "kind": "milliseconds", "value": 5000 } } }
```
Three uses across `bool_binding_demo`, `text_binding_demo` (V3, both in
corpus), and the `dwell_until_binding` field on a third recipe noted but not
written. Crosses the rule-of-three threshold.

---

## C. Effect-attachment patterns

### C1. Effects array on a scene-element (32/39)

Shorthand:
```json
"effects": [
  { "filter": "dim", "factor": 0.3, "phase": ["enter", "dwell"] },
  { "filter": "dim", "factor": 0.5, "phase": "exit", "scope": { "channel": "foreground" } }
]
```
The kind-named key (`filter`, `mask`, `sampler`, `shader`, `style_effect`,
`content`) doubles as the `effect` discriminator and the descriptor-id prefix.
`filter: "dim"` canonicalizes to `effect: "filter.dim"`. Direct flat inputs.
`phase` accepts string or array; defaults to `["dwell"]` if omitted.

### C2. Scope shorthand (18/39)

Canonical:
```json
"scope": { "kind": "channel", "value": "foreground" }
```
Shorthand:
```json
"scope": { "channel": "foreground" }
```
Or with role: `"scope": { "role": "border" }`. Or omit entirely for
`{ "kind": "all" }`. The kind-as-discriminator collapses by convention: the
single non-`kind` field's name *is* the kind.

### C3. Bindable parameter (12/39)

Canonical (verbose):
```json
"progress": { "kind": "signal", "id": "demoAmplitude", "fallback": { "kind": "number", "value": 0.0 } }
```
Shorthand:
```json
"progress": "$bind:demoAmplitude"
```
Or with explicit fallback:
```json
"progress": { "$bind": "demoAmplitude", "default": 0.0 }
```
The `$bind:<id>` string form covers the common case. The object form preserves
explicit fallback. 12 corpus recipes use bindable inputs; all of them fit
either form.

---

## D. Top-level binding declaration

### D1. Bindings block (12/39)

Canonical (`graph.signals.<id>` is verbose: ~15 lines per binding).

Shorthand:
```json
"bindings": {
  "demoAmplitude": {
    "type": "f32",
    "range": [0, 1],
    "loopback": { "type": "sine", "frequency": 0.5, "amplitude": 0.5, "offset": 0.5 }
  }
}
```
Top-level `bindings` block lifts to `graph.signals.<id>` at canonicalize time.
The shorthand carries the V3 ergonomic shape that V3 recipes already use; V3.1
canonicalization just wraps each value in the `{kind:numericSignal, expression,
fallback}` envelope.

### D2. Signal expressions stay flat (12/39)

The V3 form `{type: "ramp", start, end, duration}` and `{type: "multiply", a, b}`
is direct constructor-shaped JSON. V3.1 wraps the expression in a
`previewLoopback` envelope but the inner expression is identical. Shorthand
preserves the V3 shape verbatim under `bindings.<id>.loopback`.

Six signal types appear in the corpus: `ramp`, `sine`, `multiply`, `adsr`,
`literal`, plus a `keyframes` reference noted in one recipe. All flat-shaped.

### D3. Literal loopback as bare value (4/39)

A binding with a literal default doesn't need an expression wrapper:
```json
"loopback": 1.0
"loopback": false
"loopback": ""
```
Four recipes use bare literal loopbacks. Canonicalizer wraps them as
`{kind:literal, value:{kind:T, value:V}}`.

---

## E. Transition patterns (the original goal — preset shorthand)

### E1. Preset transition shorthand (5/39 — rule-of-three crossed)

Shorthand:
```json
"transitions": {
  "enter": { "preset": "blinds", "orientation": "horizontal", "count": 4, "duration": "2s" },
  "exit":  { "preset": "fade", "from": "#121c28", "easing": "quadOut" }
}
```
Canonicalizer expands `preset: "blinds"` to:
```json
"tracks": [{ "kind": "visibility.blinds", "subject": "to", "orientation": "horizontal", "count": 4 }]
```
Five recipes in the corpus map naturally to transition presets:
- `mask_blinds` (preset: blinds)
- `mask_wipe_corner_out_from_bottom_left` (preset: wipe with corner direction)
- `style_fade_in_from_canvas` (preset: fade with `from`)
- `bsod_crash_v3` style fade-in/fade-out (preset: fade)
- `default_toast` style fade-in/fade-out (preset: fade)

The remaining transition presets (`crossfade`, `iris`, `push`, `dissolve`,
`morph`, `stippled`, `braille` per the `TransitionPreset` enum) don't appear in
the corpus directly but are ground-truth from the contract crate. Adding them
to the alias table is on faith from the schema, not from corpus evidence;
worth noting as a place where the corpus is incomplete.

### E2. Transition `from`/`to` direction shorthand (10/39)

Motion `from`/`to` accepts `"left"`, `"right"`, `"top"`, `"bottom"`, `"top_left"`
etc. as a string; canonicalizer expands to
`{kind: "offscreen", direction: "fromLeft"}`.

10 corpus recipes use offscreen-direction motion. The `"center"` and
`"top_center"` strings already-canonical anchor types, so the same string field
unifies offscreen/anchor with one rule (string starts with edge → offscreen;
string is anchor name → anchor).

### E3. Route alias as object key (3/39)

Shorthand:
```json
"route": { "helix": { "rotations": 2.0, "radius": 4.0 } }
"route": { "carrier_orbit": { ... } }
"route": { "infinity": { "width": 10.0, "height": 5.0 } }
```
Three motion-route corpus recipes confirm. Canonicalizer reads the single
non-empty key as the kind discriminator. `helix` is an alias for
`carrierOrbit`; `infinity` is an alias for `figureEight` (corpus comments call
this out explicitly).

---

## F. Single-witness — push back into canonical

Patterns that appear only once or twice. Per rule-of-three, these don't earn
shorthand; recipes with these shapes write canonical form.

- **Custom bezier easing as object** (1 recipe). Already nearly canonical;
  canonicalizer accepts the bare 4-key object.
- **Multi-layer scene with sibling-relative motion + lag** (3 recipes —
  `scene_layer_full_stack`, `scene_authoring_ladder`, `scene_madeira_full`).
  Just barely passes rule-of-three; covered by the `scene[].follow` shape in
  the corpus pair. May need design refinement if more cases emerge.
- **Layer-local I/O hints (`emits_hint` / `binds`)** (1 recipe —
  `scene_layer_io_filter_shader`). Single-witness, push back to canonical.
  V3 form is already terse:
  ```json
  "io": { "inputs": [...], "outputs": [...] }
  ```
- **Subcell-shape multi-layer composition** (2 recipes —
  `braille_rounded_rect_v3`, `subcell_frame_v3`). Single-witness pattern is
  the multi-layer (label/shape/shape_text) shape. Could become a "labeled
  shape" shorthand later but only two cases today; defer.
- **CarrierOrbit + edge_crossing** (1 recipe). Defer.

---

## G. Confirmed evidence by recipe

The 10 written pairs map to the patterns above. The other 29 read recipes
confirm patterns by example. For each unwritten recipe, the patterns it
exercises:

| Recipe | Patterns exercised |
|---|---|
| `bsod_crash_v3` | A1, A2, A3, B2, B6, C1 (parallel), E1 (fade) |
| `digital_rain` | A1, A2, A3, B6, C1 (multi-effect slot pipeline = V2 form) |
| `default_toast` | A1, A2, A3, B6, E1 (fade), E2 (motion direction) |
| `madeira_flag` (V3) | A1, A2, A3, B6, D1, D2 — 7-layer scene |
| `scene_madeira_flag_full_scene` (V3.1) | All A/B/C/D/E patterns |
| `scene_authoring_ladder_flag_asset_binding` | A1, A2, B1, D1, D2, F (assets) |
| `content_typewriter_cursor_caret` | A1, A2, B1, B2, B6, C1 |
| `content_split_flap_cascade` | A1, A2, B1, B6, C1 |
| `filter_tint` | A1, A2, A3, B6, C1 (multi-phase parallel) |
| `mask_wipe_corner_out_from_bottom_left` | A1, A2, B6, E1 (preset: wipe) |
| `sampler_shredder` | A1, A2, B6, C1 (multi-phase) |
| `content_scramble` | A1, A2, B6, C1 |
| `content_dissolve` | A1, A2, B6, C1 |
| `content_morph` | A1, A2, B6, C1 |
| `marquee_speed_bindable` | A1, A2, A3, C1, C3, D1, D3 (literal loopback) |
| `typewriter_speed_variance_bindable` | C1, C3, D1, D3 |
| `complex_crt_filter_native_mix` | C1 (parallel without phases) |
| `complex_cinematic_reveal` | C1 (parallel multi-phase, 6 children) |
| `loopback_pill_button_progress_ramp` | C3, D1, D2 (ramp signal) |
| `single_oscillator_intensity_signal` | C3, D1, D2 (sine signal) |
| `text_binding_demo` | B7 (text variant) |
| `motion_figure_eight_infinity` | E3 (infinity alias) |
| `ease_back_out`, `ease_bezier_custom` | A4, E2 |
| `style_color_shift` | A1, A2, B6, C1 (HSL shift inputs) |
| `shader_border_sweep_position_binding` | C3, D1, D2 |
| `shader_concealed_light_drift` | A1, A2, D1, D2 (sine on parameter) |
| `shadow_gradient_soft_layers` | A2 (alpha hex), shadow `style: gradient` variant |
| `braille_rounded_rect_v3`, `subcell_frame_v3` | F (single-witness multi-layer) |
| `scene_layer_io_filter_shader` | F (single-witness layer I/O) |

---

## Open design decisions surfaced by the corpus

These need a call before the alias and expansion tables get written.

1. **Card-vs-explicit-source defaulting.** B6 says "single-source recipe → implicit
   from `card` or `text` block." Concrete shape decision: `card` shorthand carries
   `message`, `size`, `fg`, `bg`, `border`. Does it also carry `padding`? `title`?
   `borderTrim`? Currently borrowed from V3. Audit `source.card` descriptor to
   confirm the inputs surface.

2. **Phase shorthand semantics.** `"phase": ["enter", "dwell"]` reads naturally
   but the canonical schema's `activePhases` is on the *node*, not the effect.
   For multi-phase variants of the same effect with different params (filter_dim,
   filter_tint, mask_blinds), the shorthand `effects: [...]` array implies one
   node per array entry. Worth explicit: each effect entry is a graph node with
   its own params + phase + scope.

3. **Transition presets vs effect arrays.** `mask_blinds` could be expressed as
   either:
   - `effects: [{mask: "blinds", phase: "enter", ...}]` (treats it as an effect
     pinned to enter)
   - `transitions: {enter: {preset: "blinds", ...}}` (treats it as a transition
     track)
   Both are valid. The canonical form differs (graph node with active_phases vs
   transition spec with tracks). Decision needed: which shape is preferred for
   blinds/wipe/iris/etc., and is there a deterministic way for the canonicalizer
   to pick?

4. **Aliases vs canonical kind names.** `helix` aliases `carrierOrbit`;
   `infinity` aliases `figureEight`; `corner_up_bottom_left` aliases
   `corner_out_from_bottom_left`. The alias-table is the right home for these.
   Shorthand uses author-friendly names; canonical preserves the author choice
   in `intent` metadata. (Per `TransitionIntent::Alias { alias, canonicalPreset }`,
   the contract already supports this.)

5. **Multi-layer scenes and the implicit-card escape hatch.** Once `scene` is
   explicit (B6 fails), is there still a card/text shorthand for *individual
   layers*? The scene_layer_full_stack pair says yes — `card: "STACK"` inside a
   scene element. Worth confirming the shape composes cleanly.

6. **Color named-set.** The corpus uses `"yellow"`, `"cyan"`, `"white"`,
   `"light_yellow"`, `"light_blue"`, `"light_red"`, `"light_green"` as named
   colors. Are these ANSI-16 named or design-system role names? If the former,
   the named-color table is small and closed; if the latter, this is theme
   territory and the shorthand should use role scopes instead. Audit the
   `cls_color` types to confirm.

7. **Asset reference shorthand.** `scene_authoring_ladder` has `"asset": {"id":
   "flag_art", "format": "tui-vfx.braille_flag_asset.v1"}` inside a procedural
   source's `params`. Shorthand could be `"asset": "@flag_art"` (string ref
   prefix). Single witness in the corpus; defer to single-witness bin until
   another asset-using recipe surfaces.

---

## Summary verdict

The shorthand surface is genuinely smaller than the canonical surface by a
factor of 4–10× depending on recipe complexity. The patterns that drive it are
mostly wrapper-stripping (A) and default-omission (B), both of which are
mechanical for the canonicalizer. Effect-attachment (C), bindings (D), and
transition presets (E) are the patterns that carry actual design weight.

Single-witness patterns (F) genuinely don't have enough evidence to design
against; recipes with those shapes write canonical form until the corpus grows.

**Decisions needed from the project owner before M5:** the seven open items
listed above. None of them block the meta-schema work fundamentally; they shape
specific rows in the alias and expansion tables.
