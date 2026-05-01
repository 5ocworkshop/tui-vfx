# M4 — Round 7 (ripgrep sweep — Q12 and Q13 resolved)

After the saturation declaration in v5 and the late-found patterns in v6, a
ripgrep sweep across `recipes/` for `trace_path`, `paths:`, `composed`,
`dynamics:`, and the dynamic-name catalog (`spring`, `pendulum`, `friction`,
`gravity`) surfaced substantial new evidence. Three more paired files written;
two of the 16 open design questions resolve.

## Q12 — Polyline as shared geometry primitive: **NO** (resolved)

Three witnesses for `shader.trace_path` confirm the `paths: [{points: [{x,y}],
delay}]` shape:

- `gt-design/mid-range/M11_blueprint_circuit_pulse_info` (round 4)
- `gt-design-codex/blueprint_inspection_gate_modal` (round 4)
- `debug_recipes/shaders/primitives/shader_trace_path` (newly found — primitive
  reference for the shader)

`shader.wayfinding_node` uses a structurally similar but distinct shape:
`nodes: [{x, y}, ...]` (point set, no polyline structure, no delay) — three
witnesses (L05 round 4, sonar_popover round 3,
`v3.1/.../shader_wayfinding_node_current_index_binding`).

`mask.path_reveal` and motion routes use a **third shape**: `path: { type:
"spiral", rotations, direction }` — parametric curves rather than authored
points.

The shader_trace_path primitive's authoring notes anticipate exactly this
question:

> "this is the earliest hint of Decision 5's scene-layer thinking applied at
> the pixel-shader level. Surfaces Schema Q18 (how do we handle primitives
> that carry internally-composed sub-animations?)."

Three semantically distinct shapes for three different intents (animated
polylines with timing, static highlighted point set, parametric curves).
**Don't unify into a shared `Polyline` primitive.** Each shader keeps its
geometry private.

The alias and expansion table consequence: `shader.trace_path` and
`shader.wayfinding_node` get full per-shader entries; no shared geometry
sub-schema is needed at this stage. If a fourth shader using polyline
geometry surfaces later, revisit.

## Q13 — Motion dynamics: **multi-architecture, not just composed-private** (resolved)

The dynamics word ("spring", "gravity", "pendulum", "friction") shows up in
**four architectural slots**:

### Slot A — Route as primitive type

`motion_path: { type: "<dynamic>", ...params }` is the dominant pattern.
Witnesses:

| Type | Recipes | Params |
|---|---|---|
| `spring` | `physics_spring_elastic`, `slingshot_elastic`, `taliesin_hearthline`, `content_odometer_slot_reel`, `heartbeat_critical`, `content_odometer_decimal_preset_carry`, `content_odometer_3x3_count_bindable` | `stiffness`, `damping` |
| `pendulum` | `physics_pendulum_swing`, `torch_flame` | route-style |
| `friction` | `physics_friction_slide` | `drag` |

Spring is a **core route type** alongside linear/helix/infinity/bounce/etc.
**Eight-plus witnesses** strongly cross rule-of-three.

### Slot B — Route modifier (composed route)

`route: { type: "composed", route: <base>, dynamics: [{ type: "pendulum", ... }] }`.
Single witness: `5.5-suggestions/16_newtons_cradle`.

This is **architecturally distinct**: the dynamic *modifies* a base route
rather than *being* a route. Pendulum-as-modifier is the corpus's only
witness.

### Slot C — Sampler / per-cell pipeline primitive

`sampler: { type: "gravity", axis, acceleration, terminal_velocity }` —
gravity as per-cell displacement, not motion. Witness:
`debug_recipes/complex/complex_gravity_filter_native_mix`.

Conceptually distinct: this isn't motion of a recipe element; it's per-cell
falling within the rendered grid. Gravity here means "make cells fall through
the canvas" not "make my panel drop in from above."

### Slot D — Filter / per-cell oscillation

`filter: { type: "rigid_shake", ... }` — already cataloged. Same conceptual
family (physical oscillation modeled per-cell), filter slot.

### Resolution

Don't unify dynamics into a single primitive. The shorthand vocabulary
disambiguates by author intent:

- "make my panel spring into place" → `lifecycle.enter.route: { spring: {...} }`
- "make my ball oscillate at rest along an arc" →
  `lifecycle.enter.route: { composed: { route: { arc: {...} }, dynamics: [{ pendulum: {...} }] } }`
- "make my cells fall like rain" → `effects: [{ sampler: "gravity", ... }]`

The alias table can route the same author word ("spring", "pendulum") to the
correct slot based on container — `lifecycle.<phase>.route.<word>` is a route
primitive; `lifecycle.<phase>.route.composed.dynamics[].<word>` is a route
modifier; `effects[].sampler` is a per-cell primitive.

## New paired files written for Q13 evidence

Three pairs covering the three slots:

1. **`physics_spring_elastic`** — Slot A, route-as-primitive (8+ witnesses).
2. **`physics_friction_slide`** — Slot A, friction route (1 witness, but
   parallel shape to spring).
3. **`sampler_gravity`** — Slot C, sampler-named-after-dynamic.

Composed-route + pendulum was already covered by `newtons_cradle` (round 4).

## Updated catalog after round 7

**Routes** (now 11):
linear, helix/carrierOrbit, infinity/figureEight, bounce, rectilinear, spiral,
arc, **composed** (with pendulum modifier), **spring** (8+), **friction**
(1), **hover** (1).

Pendulum can be either a route (`type: "pendulum"`, V2/V3 form) or a
composed-route modifier (`dynamics: [{type: "pendulum"}]`, V3.1 form
established by Newton's cradle). Both shapes coexist in the corpus.

**Samplers** add `gravity` to the catalog. Now 11 sampler types witnessed:
crt, crt_jitter, sine_wave, ripple, fault_line, shredder, radial_twist,
spatial_signal, terminal_fire, gravity, plus content-mutation samplers.

## Final state after round 7

- **180 recipes read** + 5 schema-faith demonstrators authored = 185 design
  evidence units (added 4 new corpus reads in round 7).
- **47 paired files** (42 corpus-derived + 5 schema-faith).
- **7 M4 docs** (`M4_PATTERNS.md` + `_v2`–`_v7`, plus `_v6_addendum`).
- **14 open design questions** (was 16; **Q12 and Q13 now resolved**).
- **Pattern catalog complete and saturated.**

## Remaining 14 open design questions

The questions that *still* need design calls (not corpus evidence):

- **Q1**: card-shorthand fields — needs `source.card` descriptor audit.
- **Q2**: phase shorthand semantics — design call.
- **Q3**: mask-as-effect vs mask-as-transition — design call.
- **Q4**: alias/canonical naming strategy — design call.
- **Q5**: multi-layer scene shorthand composition — design call.
- **Q6**: color named-set scope — design call.
- **Q7**: asset reference shorthand — promoted by Canada flag, design call to lock.
- **Q8**: `extends` template support — schema-side or canonicalization-only?
- **Q9**: `filter.glyph_timeline` trigger discriminator design.
- **Q10**: shader-on-style sugar — design call.
- **Q11**: theme/template directory layout — depends on Q8.
- **Q14**: region/scope vocabulary unification — design call.
- **Q15**: multi-track transition shorthand naming (`compose` vs
  `combineMode`) — minor.
- **Q16**: two-subject relation transition shorthand (raised in v6 by the
  crossfade demonstrators) — design call, recommendation already noted.

**~~Q12~~ resolved (no shared Polyline).**
**~~Q13~~ resolved (multi-architecture dynamics).**

## Recommendation

Move to M5. The corpus is fully saturated; the 14 remaining questions are
design calls that more recipes won't answer. The alias and expansion tables
have evidence-backed entries for every promoted pattern. The schema-faith
demonstrators give the missing transition presets their first rows.

The only meaningful work the corpus could still do is **scratch-authored
demonstrators** for combinations the corpus never witnessed — but those need
the meta-schema to exist first to validate against.
