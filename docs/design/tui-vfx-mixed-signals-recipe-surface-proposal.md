<!-- <FILE>docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md</FILE> - <DESC>Proposal: ergonomic, normalized signal-source surface for recipe authors</DESC> -->
<!-- <VERS>VERSION: 0.4.0</VERS> -->
<!-- <WCTX>2026-04-27 audit + revised completion plan: Phases α and β shipped; γ partial at 43/58 facade variants. Capture missing-signal audit, set Phase 1 (close 15-variant gap), Phase 2 (recipe-side consolidation onto facade), Phase 3 (engine-vs-recipe-player delineation in docs).</WCTX> -->
<!-- <CLOG>0.4.0: mark α and β complete; mark γ partial (43/58 variants); add 2026-04-27 status snapshot with missing-variant audit; add Phase 1/2/3 completion plan; record architectural framing (facade is recipe-JSON deserialization seam, not a substitute for mixed-signals).</CLOG>

# tui-vfx ⇄ mixed-signals: recipe-author signal surface

> **Status:** proposal, not a plan. Submitted 2026-04-26 by request — articulate the existing wiring and propose an ergonomic + maintainable surface for recipe authors to reach mixed-signals primitives without source-code changes. Companion to `docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md`; this is sweep §1.X-class scope.
>
> **Read first:** `docs/design/tui-vfx-effect-composition-model.md` — the broader framing. The signal facade described here is the *Model B* shape (fixed-stage pipeline; signals fill parameter slots). The composition-model doc explains why Model B is the right call given terminal constraints and what that implies for the facade's scope. If you're reading the facade proposal cold, the composition-model doc is the load-bearing context.

## TL;DR

We have a **rich** mixed-signals palette already wired (~40 SignalSpec discriminants spanning oscillators, noise, spatial coordinates, envelopes, composition, and processing) plus **physics primitives** (spring, bounce, pendulum, orbit, projectile) reachable from recipes but routed through a *parallel channel* outside `SignalSpec`. The capability is real and demonstrably used (`spring` in 9 recipes, `bounce` in 7, `sine`/`triangle` in 20–50+). What it lacks is a **single normalized surface** an author can browse: today's mental model is "go grep recipes for examples."

Four moves to fix it without over-baking:

1. **Promote physics into `SignalSpec`** — collapse the parallel channel; one SSOT.
2. **Curate a "Core 12" cheatsheet** — published list of the dozen most-useful discriminants with worked examples; the other ~28 stay available, marked "advanced."
3. **Symmetric `Bindable*` family + sibling `signal | binding` fields** — `BindableF32`, `BindableColor` exist; everywhere a literal goes, a signal or a runtime binding may go too.
4. **Autogen `SIGNALS_REFERENCE.md`** from `SignalSpec` rustdoc + a `signals.toml` authoring overlay — same pipeline that already generates `CAPABILITIES_REFERENCE.md`.

None of these require breaking existing recipes. All are additive or consolidating.

---

## Status snapshot — 2026-04-27

| Phase | Scope | Status | Evidence |
|---|---|---|---|
| α | Autogen `SIGNALS_REFERENCE.md` from rustdoc + `signals.toml` overlay | ✅ Complete | `docs/generated/SIGNALS_REFERENCE.md` + `docs/templates/signals.toml` ship; xtask handlers in `xtask/src/docs/{validate,merge,parse,extract,gen}_signals*.rs` |
| β | Curated "Core 12" cheatsheet | ✅ Complete | 13 catalog entries flagged `in_core_12: true` in `tui_vfx_recipes::signals::vfx_recipe_signal_catalog`; reference doc surfaces a Core section |
| γ | In-crate facade `tui_vfx_recipes::signals::*` (Option A) | ⚠️ Partial — 43/58 variants | Module ships `VfxRecipeSignalSpec`, `VfxIntoRecipeSignal`, `VfxRecipeSignalMeta`, `vfx_recipe_signal_catalog`. One deserialization site uses it (`v3/compile/fnc_build_composition_spec_from_compiled_plan.rs:635`). 15 mixed-signals primitives missing per audit below. |
| δ | Symmetric Bindable family + `signal | binding` polymorphism | ⏸ Deferred → becomes Phase 2 of revised plan | Hard-gated on 1.2.A `VfxBindable<T, S>` (shipped in tui-vfx-core). |

### Audit (2026-04-27): missing variants

Cross-checked facade against `mixed_signals` public exports + `mixed_signals::SignalSpec`. **Zero hallucinations** — every primitive named in §1.1 and §3.1 exists in the library today.

**15 missing variants; all are real upstream exports:**

- **Random / RNG noise (9)** — `seeded_random`, `spatial_noise`, `gaussian_noise`, `poisson_noise`, `correlated_noise`, `pink_noise`, `per_character_noise`, `student_t_noise`, `impulse_noise`. All in `mixed_signals::random::*`. All in upstream `SignalSpec`.
- **Envelopes (3)** — `linear_envelope`, `linear_decay`, `exponential_decay`. All in `mixed_signals::envelopes::*`. Only `linear_envelope` is in upstream `SignalSpec`; the facade can wrap the upstream Rust types directly without needing `SignalSpec` to grow.
- **Composition (3)** — `vca_centered`, `phase_accumulator`, `phase_sine`. All in `mixed_signals::{composition,generators}::*`. All in upstream `SignalSpec`.

**Wire-format compatibility risks identified:**

- **`impulse_noise`** — upstream `ImpulseNoise` struct has 6 fields; `SignalSpec::ImpulseNoise` exposes 3 (`{seed, rate_hz, impulse_width}`). A `#[serde(transparent)]` wrapper would break SignalSpec-compatible JSON. Mitigation: inline struct mirroring the 3-field SignalSpec shape; build via `ImpulseNoise::with_width(rate_hz, seed, impulse_width)` in `into_recipe_signal()`.

**Deferred (out of scope for this audit):**

- **`weighted_mix`** — exists in `mixed_signals::composition` but is not Serialize/Deserialize (stores `Vec<(Box<dyn Signal>, f32)>`). Exposing it requires custom serde with `Vec<(VfxRecipeSignalSpec, f32)>` lowering at build time. Document in progress.txt; revisit when a recipe needs it.

### Recipe corpus impact

`grep -l '"type": "<discriminant>"' recipes/ debug_recipes/` for each of the 15 missing discriminants returned **0 hits in 14 cases and 1 hit for `spatial_noise`** (in `tui-vfx-recipes`). Closing the gap is purely additive; no in-tree recipe breaks.

---

## 1. Current state

### 1.1 What's wired today (confirmed via ofpf-* across all four repos)

**`/usr/projects/mixed-signals/src/`** — primitives:

| Category | Files | Serializable into `SignalSpec` |
|---|---|---|
| `generators/` (15) | sine, triangle, square, sawtooth, ramp, step, pulse, constant, keyframes, phase_sine, phase_accumulator, cell_distance, spatial_coordinate, surface_angle, surface_distance | ✅ all |
| `envelopes/` (5) | adsr, impact, linear, linear_decay, exponential_decay | ⚠️ only adsr, impact, linear |
| `physics/` (7) | spring, bounce, decay, pendulum, orbit, projectile, attractor | ❌ none in `SignalSpec` (parallel channel) |
| `composition/` (6) | add, multiply, mix, frequency_mod, vca_centered, weighted_mix | ✅ all + phase_sine, phase_accumulator |
| `noise/` (~9) | white, perlin, seeded_random, spatial, gaussian, poisson, correlated, pink, per_character, student_t, impulse | ✅ all |
| `processing/` (5) | clamp, quantize, remap, invert, abs | ✅ all |

**`SignalContext`** (the ambient bag of carry bits a Signal evaluator can read):

| Field | Type | Purpose |
|---|---|---|
| `frame` | `u64` | monotonic frame counter |
| `seed` | `u64` | reproducibility for stochastic signals |
| `width`, `height` | `u16` | render area dimensions |
| `phase` | `Option<Phase>` | lifecycle (Start / Active / End / Done) |
| `phase_t` | `Option<SignalTime>` | progress within current phase 0..1 |
| `loop_t` | `Option<SignalTime>` | cyclic loop time 0..1 |
| `absolute_t` | `Option<SignalTime>` | monotonic elapsed time |
| `char_index` | `Option<usize>` | per-character signal evaluation |
| `cell_x`, `cell_y` | `Option<u16>` | per-sample cell coords |
| `subcell_offset` | `Option<(f32, f32)>` | sub-cell fractional offset |

**Recipe-author entry points today** (where signals appear in JSON):

```
{"signal": {"type": "ramp", "start": 0, "end": 8, "duration": 5.0}}
{"signal": {"type": "sample_norm_x"}}
{"signal": {"type": "spring", "stiffness": 8, "damping": 0.6, "target": 1}}   // ← physics, parallel channel
{"binding": "drum_font"}                                                       // BindableString
"line-3x3"                                                                     // also valid for BindableString (lenient)
0.5                                                                            // BindableValue accepts bare literal
```

### 1.2 ASCII block diagram — today's flow

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ /usr/projects/mixed-signals  (SSOT for primitive math + spatial functions)   │
│                                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ generators/  │  │  envelopes/  │  │   physics/   │  │ composition/ │      │
│  │  sine,tri,   │  │  adsr,impact │  │ spring,bounce│  │  add,mul,mix │      │
│  │  ramp,step,  │  │  linear,...  │  │ pendulum,... │  │ freq_mod,... │      │
│  │  keyframes...│  │              │  │              │  │              │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                  │                  │                  │             │
│         └──────┬───────────┴────────┐         │                  │             │
│                │                    │         │                  │             │
│                ▼                    ▼         │                  ▼             │
│       ┌─────────────────────────────────┐    │       ┌──────────────────┐    │
│       │  SignalSpec enum (~40 variants) │    │       │ Signal trait     │    │
│       │  #[serde(tag="type")]           │    │       │ sample()         │    │
│       │  ✅ oscillators                 │    │       │ sample_with_ctx()│    │
│       │  ✅ noise                       │    │       │ sample_w_slope() │    │
│       │  ✅ spatial leaves              │    │       └──────────────────┘    │
│       │  ⚠️  envelopes (only 3 of 5)    │    │                                │
│       │  ✅ composition + processing    │    │                                │
│       │  ❌ physics (NOT exposed here!) │    │                                │
│       └────────────┬────────────────────┘    │                                │
└────────────────────┼─────────────────────────┼────────────────────────────────┘
                     │                         │
                     │ JSON deserialize        │ Rust struct deserialize
                     │ (recipes/*.json)        │ (parallel channel)
                     ▼                         ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│ /usr/projects/tui-vfx-recipes  (recipe schema + compiler)                    │
│                                                                              │
│  ┌──────────────────────┐                ┌──────────────────────────────┐    │
│  │ {"signal": {...}}    │                │ effect specs that wrap       │    │
│  │ field on many        │                │ physics types via separate   │    │
│  │ effect parameters    │                │ deserialization (opaque)     │    │
│  └──────────┬───────────┘                └──────────┬───────────────────┘    │
│             │                                        │                        │
│             ▼                                        ▼                        │
│  ┌─────────────────────────────────────────────────────────────────┐         │
│  │ fnc_build_composition_spec_from_compiled_plan / preview / probe │         │
│  └────────────────────────────────┬────────────────────────────────┘         │
└───────────────────────────────────┼──────────────────────────────────────────┘
                                    │
                                    ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│ /usr/projects/tui-vfx          (consumers — effects)                         │
│                                                                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐               │
│  │ BindableValue   │  │ BindableU16     │  │ BindableString  │               │
│  │ Signal(SoF) |   │  │ Literal(u16) |  │ Literal(s) |    │               │
│  │ Binding(name)   │  │ Binding(name)   │  │ Binding(name)   │               │
│  │ ✅ signal accept│  │ ❌ no signal    │  │ ❌ no signal    │               │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘               │
│                                                                              │
│  ❌ no BindableF32, no BindableColor as named types                          │
│                                                                              │
│  Effects (Filter, Mask, Sampler, StyleShader) read these in their            │
│  trait methods + call .sample_with_context(&signal_ctx) at evaluation time.  │
└──────────────────────────────────────────────────────────────────────────────┘
```

### 1.3 What's good

1. **SignalSpec is the right abstraction.** A tagged enum with `#[serde(tag = "type", rename_all = "snake_case")]` is exactly what JSON-driven recipes want. Author writes `{"type": "ramp", ...}`, gets a Rust value with no glue code.
2. **`SignalContext` carries rich semantics.** Phase, loop time, char index, cell coords, subcell offset — most things a per-cell evaluator wants are already in the bag.
3. **First-class composition operators** (`add`, `multiply`, `mix`, `frequency_mod`) let recipes build expressions: `mix(sine, sine, weight=ramp)` is one of the more interesting things in the wild.
4. **Spatial leaves are recipe-reachable** (`sample_norm_x`, `sample_radius`, etc.). Recipe authors write shader-style effects without touching Rust.
5. **`BindableValue` does the right thing for floats**: a single field accepts bare literal OR signal expression OR runtime-binding key. The asymmetry only bites for the U16 / String / Color cases.

### 1.4 What's bad / missing

1. **Physics is in a parallel channel.** `spring`, `bounce`, `pendulum`, etc. are referenced in 6–9 recipes each but the deserialization path is not via SignalSpec. Recipe authors who learn the SignalSpec mental model don't naturally find these; they learn them by grepping production recipes. The two routes also mean the autogen pipeline can document one but not the other.
2. **Discoverability is by example, not by reference.** `sample_norm_y` exists, `sample_norm_x` is used in 12 recipes — `sample_norm_y` is in **zero**. Surface variants (`sample_surface_radius`, `sample_surface_angle_from`) are in zero or one recipe. Either authors don't know they exist, or there's no clear hint when to reach for them. There is no `SIGNALS_REFERENCE.md` analogous to `CAPABILITIES_REFERENCE.md`.
3. **Bindable family is asymmetric.**
   - `BindableValue` accepts signals.
   - `BindableU16` accepts literal-or-binding only — **no signal**. So a coordinate field cannot be signal-driven at all from a recipe; only host-bound.
   - `BindableString` similarly literal-or-binding.
   - There is no `BindableF32` or `BindableColor`. Float fields use `BindableValue` (works), color fields don't have a Bindable form at all (resolved through theme).
4. **Envelope coverage is partial.** SignalSpec exposes `adsr`, `impact`, `linear` but not `exponential_decay` or `linear_decay` — yet both are `#[derive(Serialize, Deserialize)]` Rust structs. This is the smallest gap (probably one PR), but it's the kind of thing that erodes author trust ("why this one and not the other? is that intentional?").

---

## 2. Proposal

Four deliberate moves. None are speculative; each addresses a gap in §1.4 and earns its place per Intentions 23 (rule of three) and 24 (library changes earn their place).

### 2.1 Move 1 — Promote physics into `SignalSpec`

Add the seven physics primitives as first-class `SignalSpec` discriminants:

```
spring, bounce, decay, pendulum, orbit, projectile, attractor
```

They are already serializable Rust types. The change is plumbing: add the variants to the enum, route to the existing struct constructors, expose the `#[serde]` shape that recipes currently use through the parallel channel.

**Why.** Today's bifurcation is real cognitive cost: an author who has internalized "every signal is `{"type": "X"}`" gets surprised that `spring` works *somewhere* but not in arbitrary signal fields. After this change, **every signal-producing primitive in mixed-signals is reachable through the same SignalSpec gate**, the autogen pipeline can document them all uniformly, and the existing recipes that use spring/bounce/pendulum continue to work without re-authoring.

**Risk.** Migration of the parallel channel — if there's a downstream consumer that relies on the current opaque deserialization, the change must include a back-compat shim. F.6-style cross-repo audit before landing.

### 2.2 Move 2 — Curate the "Core 12" cheatsheet

Pick twelve discriminants representing the highest-leverage signal types and publish them prominently. The other ~30 stay available but bracketed as "advanced." Suggested core, one per behavioral category:

| Category | Core pick | Why |
|---|---|---|
| Oscillators | `sine`, `triangle` | musical, visually distinct |
| Utility | `ramp`, `keyframes` | most-directly authorable shapes |
| Envelopes | `adsr`, `impact` | the two canonical attack profiles |
| Physics | `spring`, `bounce` | tactile feel; both popular in current recipes |
| Spatial | `sample_norm_x`, `sample_radius` | the two coordinate axes most-reached-for |
| Composition | `add`, `multiply` | the two arithmetic primitives |
| Noise | `perlin` | one canonical organic noise |

This is **documentation and authoring affordance**, not a code change. It's the smallest move with the largest discoverability dividend. The list is opinionated on purpose; the goal is to give a new author a place to start, not to deprecate variety.

**Why this set, not another.** The picks are evidence-driven from the audit:
- `sine`, `triangle`, `ramp`: each appears in 20–50 recipes (verified by ofpf-content "type": "X").
- `spring`, `bounce`: 9 / 7 recipes — physics is being used and needs first-class doc.
- `sample_norm_x`: 12 recipes; the only spatial coordinate authors actually know about.
- `adsr`: 1 recipe today, but it's the canonical envelope shape that every demo will reach for once visible.
- `add`, `multiply`: irreplaceable composition primitives.

### 2.3 Move 3 — Symmetric Bindable family + `signal | binding` polymorphism

Add `BindableF32` and `BindableColor` to mirror the existing `BindableU16` / `BindableString` shape. Then make every Bindable variant accept three forms in JSON:

```jsonc
// All four Bindable* types should accept these shapes uniformly:
{"literal": <T>}                            // explicit literal
<T>                                          // bare literal (lenient, where unambiguous)
{"binding": "name"}                         // host-supplied runtime parameter
{"signal": {"type": "...", ...}}            // signal expression (NEW for U16/String/Color)
```

For `BindableU16` this means coordinates can be signal-driven (e.g. `"x": {"signal": {"type": "sine", "frequency": 0.5, "amplitude": 8}}` for an oscillating x-coord). For `BindableColor`, color fields can pull from `runtime_params` *or* be signal-evaluated against a `GradientLut` in the same field shape.

**Why.** This is the Intention 24 ("library changes earn their place") payoff for the Slice 6.6 + Phase F infrastructure. We just spent six commits making `ShaderRuntimeParams` reachable inside transformers and the per-cell context bundle uniform across Filter/Mask/Sampler/Shader. The capability surface should reflect that uniformity at the recipe layer too.

**Scope discipline.** This is the **largest** of the four moves and the easiest place to over-bake. The proposal is to add the symmetric variants and the `signal` form everywhere — *not* to invent a new evaluation engine, *not* to add cross-form polymorphism (e.g. signal-driven binding key resolution), *not* to add type coercion (e.g. evaluating a float signal into a u16). One field accepts one form per recipe declaration; the four forms are sibling discriminants.

### 2.4 Move 4 — Autogen `docs/SIGNALS_REFERENCE.md`

Reuse the same xtask pipeline that generates `docs/CAPABILITIES_REFERENCE.md`. Add:
- `docs/templates/signals.toml` with per-discriminant authoring hints (one-line summary, recommended usage, common gotchas).
- xtask handler that reads `SignalSpec` rustdoc + the toml overlay and writes `docs/SIGNALS_REFERENCE.md`.
- `cargo xtask docs generate` covers it.

The output is one canonical, always-current cheatsheet a recipe author opens before writing a new effect. It eliminates the "go grep recipes" workflow.

**Why.** This is the smallest move with the highest *long-term* maintainability dividend: as long as authors keep `SignalSpec` rustdoc accurate (already a standing rule per `feedback_rustdoc_when_editing.md`), the reference stays accurate. No second source of truth to drift.

---

## 3. Proposed state

### 3.1 ASCII block diagram — after the four moves

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ /usr/projects/mixed-signals  (unchanged primitive set, one extra variant     │
│                                arm added per physics type — purely additive) │
│                                                                              │
│  generators/  envelopes/  physics/  composition/  noise/  processing/        │
│       │           │          │           │          │          │             │
│       └───────────┴──────────┴───────────┴──────────┴──────────┘             │
│                                  │                                           │
│                                  ▼                                           │
│       ┌─────────────────────────────────────────────────────────┐            │
│       │  SignalSpec enum  (~47 variants, ALL primitives)        │            │
│       │  #[serde(tag="type", rename_all="snake_case")]          │            │
│       │  ✅ oscillators                                         │            │
│       │  ✅ noise (full)                                        │            │
│       │  ✅ spatial leaves (full)                               │            │
│       │  ✅ envelopes (NOW: adsr, impact, linear,               │            │
│       │                     exp_decay, linear_decay)            │            │
│       │  ✅ composition + processing                            │            │
│       │  ✅ physics (NEW: spring, bounce, decay, pendulum,      │            │
│       │                  orbit, projectile, attractor)          │            │
│       └────────────┬────────────────────────────────────────────┘            │
└────────────────────┼─────────────────────────────────────────────────────────┘
                     │
                     │ ONE deserialization path; parallel channel collapsed
                     ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│ /usr/projects/tui-vfx                                                        │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────┐            │
│  │ Symmetric Bindable family — all accept the same three forms: │            │
│  │                                                              │            │
│  │  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐    │            │
│  │  │ BindableValue  │ │  BindableU16   │ │ BindableString │    │            │
│  │  └────────────────┘ └────────────────┘ └────────────────┘    │            │
│  │  ┌────────────────┐ ┌────────────────┐                       │            │
│  │  │  BindableF32   │ │  BindableColor │   ← NEW               │            │
│  │  └────────────────┘ └────────────────┘                       │            │
│  │                                                              │            │
│  │  Each accepts:                                               │            │
│  │   • bare literal (lenient where unambiguous)                 │            │
│  │   • {"literal": <T>}                                         │            │
│  │   • {"binding": "name"}                                      │            │
│  │   • {"signal": {"type": "...", ...}}                         │            │
│  └──────────────────────────────────────────────────────────────┘            │
│                                                                              │
│  Effects (Filter, Mask, Sampler, StyleShader) — UNCHANGED CONSUMPTION:       │
│  read Bindable*, call .sample_with_context(&signal_ctx) at eval time.        │
│  The VfxCellContext bundle from Phase F gives them the per-cell carry.       │
└──────────────────────────────────────────────────────────────────────────────┘
                                ▲
                                │
                                │ documented from one source of truth
                                │
┌──────────────────────────────────────────────────────────────────────────────┐
│ docs/  (autogen pipeline — same one as CAPABILITIES_REFERENCE.md)            │
│                                                                              │
│  ┌────────────────────────────┐    ┌────────────────────────────┐            │
│  │ SignalSpec rustdoc         │ +  │ docs/templates/signals.toml│            │
│  │ (in mixed-signals/src/)    │    │ (authoring hints overlay)  │            │
│  └────────────┬───────────────┘    └────────────┬───────────────┘            │
│               │                                  │                            │
│               └──────────────┬───────────────────┘                            │
│                              ▼                                                │
│             ┌────────────────────────────────┐                                │
│             │ docs/SIGNALS_REFERENCE.md      │  ← canonical author cheatsheet│
│             │  Core 12 (top of doc)          │                                │
│             │  All ~47 (advanced section)    │                                │
│             │  Examples per type             │                                │
│             │  cargo xtask docs generate     │                                │
│             └────────────────────────────────┘                                │
└──────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Recipe-author experience after

```jsonc
// Same shape, more reach. An author who learned SignalSpec for ramp/sine
// now naturally finds spring/bounce/perlin without changing mental model.

"width": 8,                                        // bare literal (BindableU16)
"width": {"binding": "panel_width"},               // host-bound
"width": {"signal": {"type": "sine", "frequency": 0.5, "amplitude": 4, "offset": 8}},

"strength": 0.6,                                   // bare literal (BindableValue)
"strength": {"signal": {"type": "spring", "stiffness": 12, "damping": 0.4, "target": 1}},
"strength": {"signal": {"type": "mix",
                         "a": {"type": "sine", "frequency": 1},
                         "b": {"type": "perlin", "scale": 0.3},
                         "weight": {"type": "ramp", "start": 0, "end": 1, "duration": 2}}},

"font": {"binding": "drum_font"},                  // BindableString — unchanged

"color": {"signal": {"type": "lookup",
                       "gradient": "ember",
                       "t": {"type": "sample_norm_x"}}},   // BindableColor — NEW
```

---

## 4. Out of scope (explicit, to prevent over-baking)

These are *real* improvements but explicitly NOT part of this proposal:

- **Type coercion** (e.g. evaluating a float signal into a u16 with rounding). Each Bindable*'s signal form returns its own type; signals don't cross.
- **Cross-form polymorphism** (e.g. binding key resolved by a signal). One discriminant per field.
- **A new signal evaluation engine.** All evaluation continues through `Signal::sample_with_context`; this proposal only widens the *recipe-authoring surface*.
- **Theme-driven color signals** beyond the simple `lookup` shape suggested above. Theme system stays in the gt-design / style boundary.
- **Recipe-level signal naming/aliasing** (e.g. `"my_oscillator": {...}` for reuse). Recipes can already inline; aliasing is a separate authoring-ergonomics question.
- **Signal-driven structural changes** (e.g. variable layer count). Recipes stay structurally static; signals only drive parameter values.

---

## 5. Phasing if accepted

Order matters. None of these are forced; each lands behind its own gate.

| Phase | Move | Cost | Earns its place via |
|---|---|---|---|
| α | **Move 4** (autogen `SIGNALS_REFERENCE.md`) | smallest; one xtask handler + one toml | discoverability for the surface that already exists; no semantic change |
| β | **Move 2** (Core 12 cheatsheet) | doc-only; depends on α | the cheatsheet is one section of the autogen output |
| γ | **Move 1** (physics into SignalSpec) | medium; enum additions + parallel-channel cleanup + cross-repo F.6-style audit | the bifurcation is the most concrete cognitive cost in §1.4 |
| δ | **Move 3** (symmetric Bindable family + signal form) | largest; new types + serde changes + audit | post-Phase F is the natural moment — VfxCellContext + ShaderRuntimeParams are uniform; this completes the recipe-side mirror |

α + β can ship as a single packet with no source change beyond the xtask additions. γ is a real Slice. δ is a real Slice and should not start until γ has shipped and the parallel channel is gone (otherwise δ is implementing two different deserialization paths simultaneously).

---

## 6. Open questions before any of this becomes a plan

1. **Is the current bifurcation intentional?** Are physics primitives kept out of SignalSpec for a reason (perf? init cost? state semantics?)? If yes, Move 1 needs to address that first.
2. **Do we want bare-literal acceptance everywhere** or only where unambiguous? `BindableU16` taking bare `8` is fine; `BindableColor` taking bare `"#ff0000"` competes with `{"binding": "color_name"}`. Likely needs a small disambiguation table.
3. **Cap on signal-expression depth** in recipes? `mix(mix(mix(...), ...), ...)` is technically allowed by SignalSpec; do we want a recipe-side validator gate on nesting depth to keep recipes readable?
4. **Naming for Move 3's discriminants.** Currently we have `{"signal": {"type": "..."}}`. For a `BindableU16` accepting a signal, do we keep `{"signal": ...}` or normalize to `{"signal_expression": ...}` to distinguish from the SignalSpec-internal `{"signal": ...}` field name in some operators (e.g. `weighted_mix.signals: [...]`)? Probably the former for consistency, but flag for review.

---

## 7. Appendix — exact ofpf-* commands for the audit

For reproducibility / future re-audits:

```bash
# Mixed-signals primitive inventory
ls /usr/projects/mixed-signals/src/{generators,envelopes,physics,composition,noise,processing}

# SignalSpec discriminants
ofpf-content "#\\[serde\\(rename" --regex --root /usr/projects/mixed-signals \
  --glob "**/types/signal_spec.rs"

# Signal trait surface
ofpf-defs Signal --root /usr/projects/mixed-signals
ofpf-defs SignalContext --root /usr/projects/mixed-signals

# Recipe-side usage of signal types (per discriminant)
ofpf-content '"type": "sine"'    --regex --glob "**/*.json" --root /usr/projects/tui-vfx-recipes
ofpf-content '"type": "spring"'  --regex --glob "**/*.json" --root /usr/projects/tui-vfx-recipes
# ...etc per discriminant of interest

# Bindable family
ofpf-defs Bindable --root /usr/projects/tui-vfx
```

---

## 8. Addendum (2026-04-26 — user response): a `tui-vfx-recipes-signals` recipe facade crate

> **Scope amendment (user direction):** "I don't want to change everything, I'm thinking specifically for recipe authoring so maybe we should keep the scope narrow in that regard, I want an interface point for recipes to get signals. I don't want to sweep the code and make the binaries all use a recipe interface facade."

This amendment **replaces Move 1** (promote physics into SignalSpec, which would have rippled through mixed-signals upstream and through every production binary that imports `SignalSpec`). The replacement is smaller, narrower, and respects the constraint that production code keeps importing `mixed_signals` directly.

### 8.1 Recommendation: yes, build it. Here is the shape.

A new module at `/usr/projects/tui-vfx-recipes/src/signals/` reachable as `tui_vfx_recipes::signals::*` (Option A in §8.7 — in-crate module within the existing `tui-vfx-recipes` crate; not a standalone crate, so it cannot be published independently and its scope-to-recipes intent is self-evident at every import site). It is a **recipe-facing facade** with a single declared responsibility: be the deserialization seam between recipe JSON and the runtime signal value the effect machinery consumes. Nothing else.

**Public surface — exactly four exports** (re-exported from `tui_vfx_recipes::signals::*`):

```rust
// tui-vfx-recipes/src/signals/mod.rs

/// Recipe-facing signal expression. Tagged enum mirroring the curated
/// surface a recipe author can reach for. Wraps mixed-signals primitives
/// and any tui-vfx-only signals the project decides to ship.
pub enum VfxRecipeSignalSpec { /* see §8.3 */ }

/// Trait the recipe loader calls to turn a VfxRecipeSignalSpec into a
/// boxed Signal at compile time.
pub trait VfxIntoRecipeSignal {
    fn into_recipe_signal(self) -> Box<dyn mixed_signals::Signal>;
}

/// Authoring-overlay metadata used by the docs autogen (one entry per
/// discriminant). Lives in code (not toml) so rustdoc + the metadata
/// stay coherent on every edit.
pub struct VfxRecipeSignalMeta { /* one_line_summary, recommended_use, ... */ }

/// The autogen index — every discriminant + its metadata, iterable.
pub fn vfx_recipe_signal_catalog() -> &'static [VfxRecipeSignalMeta];
```

That is **the entire module's public surface.** No re-exports of `Sine`/`Spring`/etc. — recipes never touch the primitive types directly; they write JSON, which deserializes to `VfxRecipeSignalSpec`, which compiles to a `Box<dyn Signal>`. Production code that wants `Sine` continues `use mixed_signals::Sine` directly. The facade does **not** intercept production code paths and is unreachable from any binary that doesn't already depend on `tui-vfx-recipes`.

### 8.2 Why this is the right shape

> **The headline argument (user articulation, 2026-04-26):** *"This way we have a locally named/scoped interface point to drive inputs when authoring recipes and we can change how the signals are generated in future or plugin new signals or limit what is exposed or rename/remap etc as required for maintenance."*

That single sentence captures the maintenance lever the facade gives us that we do not currently have:

- **Locally named, locally scoped** — the recipe-author surface lives at `tui_vfx_recipes::signals` and nowhere else. There is one place to look, one place to grep, one place to document.
- **Change how the signals are generated** — if we ever want to swap a mixed-signals primitive for a project-tuned alternative (better performance, different defaults, project-flavored behavior), the swap happens inside `into_recipe_signal()` and zero recipes need to change.
- **Plug in new signals** — a future signal type (something derived from `VfxCellContext`, theme-aware, recipe-author-defined) gets a new `VfxRecipeSignalSpec` variant. mixed-signals stays untouched.
- **Limit what is exposed** — the curated subset (the Core 12 plus advanced) is *the* public surface. New mixed-signals primitives don't auto-leak; each one is an explicit decision.
- **Rename/remap as required for maintenance** — if mixed-signals renames `Sine` to `SineWave` upstream, recipes don't break; the facade absorbs the rename. If the project decides `sine` should mean something subtly different (e.g. always normalized to 0..1), that policy lives in the facade, not in 50 recipes.

Below: the original supporting arguments still hold, restated in support of the headline.

1. **Narrow scope, stable surface.** One enum + one trait + one metadata table. No churn on production binaries, no upstream changes to mixed-signals (Intention 9 stays clean — we extend mixed-signals when we need a *new primitive*, not when we want a recipe-facing reorganization of *existing* primitives).
2. **Collapses the parallel channel without rewriting it.** Today's recipe machinery has SignalSpec deserialization in one path and physics-via-effect-spec in another. Both paths terminate in a Rust signal type. The facade turns them into one path: every recipe-side signal field deserializes to `RecipeSignalSpec`, period. The bifurcation goes away from the *recipe author's* perspective; the underlying Rust types stay where they are.
3. **Becomes the natural home for project-flavored composites.** The example I called out earlier ("vfx_spring_glow = spring + sample_radius") is exactly the kind of thing that doesn't belong in `mixed-signals` (too project-specific) and shouldn't land as raw boilerplate in every recipe. `RecipeSignalSpec` can include it as a discriminant; mixed-signals stays pure-primitive, recipes stay JSON-ergonomic.
4. **Hosts the autogen target.** The catalog returned by `recipe_signal_catalog()` is what `cargo xtask docs generate` reads to emit `SIGNALS_REFERENCE.md`. One source of truth for the cheatsheet.
5. **Future-proofs "other sources."** Your phrasing — "potentially other sources" — is the open hand. If a future signal source isn't a mixed-signals primitive (e.g. signals derived from `VfxCellContext`, theme-resolved gradients, recipe-author-defined keyframe shorthand), the facade absorbs it as another `RecipeSignalSpec` variant without leaking into mixed-signals.

### 8.3 `RecipeSignalSpec` shape sketch

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RecipeSignalSpec {
    // Curated direct passthroughs (recipe writes the same JSON it would
    // for SignalSpec; facade re-exports the deserialized struct as-is).
    Sine(mixed_signals::Sine),
    Triangle(mixed_signals::Triangle),
    Ramp(mixed_signals::Ramp),
    Keyframes(mixed_signals::Keyframes),
    Adsr(mixed_signals::Adsr),
    Impact(mixed_signals::Impact),
    Perlin(mixed_signals::Perlin),
    SampleNormX,                      // unit variants for nullary spatial leaves
    SampleNormY,
    SampleRadius,
    // ... the curated Core 12 + advanced (one variant per primitive we expose)

    // Physics variants — collapse the parallel channel into the same enum.
    Spring(mixed_signals::Spring),
    Bounce(mixed_signals::Bounce),
    Pendulum(mixed_signals::Pendulum),
    Orbit(mixed_signals::Orbit),
    Projectile(mixed_signals::Projectile),
    Decay(mixed_signals::Decay),
    Attractor(mixed_signals::Attractor),

    // Composition operators (recursively contain RecipeSignalSpec, NOT
    // SignalSpec — the recipe author stays in this surface).
    Add { a: Box<RecipeSignalSpec>, b: Box<RecipeSignalSpec> },
    Multiply { a: Box<RecipeSignalSpec>, b: Box<RecipeSignalSpec> },
    Mix { a: Box<RecipeSignalSpec>, b: Box<RecipeSignalSpec>, weight: Box<RecipeSignalSpec> },

    // Project-flavored composites — start empty; add only with rule-of-three justification.
    // Example future variant:
    //   VfxSpringGlow { stiffness: f32, damping: f32, target_color: Color },
}
```

Each variant either wraps a mixed-signals primitive directly (cheapest), is a unit variant for a nullary spatial leaf, recurses into `RecipeSignalSpec` for composition, or is a project-flavored composite. The recipe author writes one JSON shape; deserialization is one path.

### 8.4 What about the `Bindable*` family?

Move 3 from §2.3 (symmetric `BindableF32` / `BindableColor` + signal form everywhere) **stays as proposed** but the signal form in those Bindable* types is `RecipeSignalSpec`, not `SignalSpec`. So:

```rust
pub enum BindableU16 {
    Literal(u16),
    Binding(String),
    Signal(RecipeSignalSpec),    // ← the addition uses the facade, not raw mixed-signals
}
```

The recipe-author surface and the type system end up consistent.

### 8.5 What does NOT change

- Production effect code. Filters, masks, samplers, shaders continue importing `mixed_signals::Signal` and calling `.sample_with_context(&signal_ctx)`. They don't know `tui-vfx-recipes-signals` exists.
- gt-design. Same deal — the only consumer of the facade is `tui-vfx-recipes` (deserialization) and the docs autogen.
- mixed-signals. Zero changes upstream. If we ever need to add a primitive there (Intention 9), that's a separate decision driven by mixed-signals' own scope.

### 8.6 Phasing (replaces §5 — strictly tighter)

| Phase | Move | Cost | Earns its place via |
|---|---|---|---|
| α | **Move 4** (autogen `SIGNALS_REFERENCE.md`) | small | discoverability for the surface that exists |
| β | **Move 2** (Core 12 cheatsheet) | doc-only | curated entry-point for new authors |
| γ | **NEW: build the `recipe_signals` module inside `tui-vfx-recipes`** (Option A in §8.7) | small-medium; one new module subtree (~6 files under OFPF prefixes), one new enum, recipe-side deserialization redirected through it | replaces old Move 1; collapses the parallel channel without upstream churn AND without adding a workspace-level crate |
| δ | **Move 3** (symmetric Bindable family + signal form) | medium-large | the Bindable signal-form variants point at `VfxRecipeSignalSpec` (γ), so γ is the gate for δ |

α and β can land as one packet. γ is one Slice. δ is the next Slice and only starts after γ ships.

### 8.7 Open questions specific to the facade

1. **Naming — DECIDED 2026-04-26 (user direction):**
   - **Crate name (if/when promoted to Option B or C):** `tui-vfx-recipes-signals` — literally `<parent crate name>-signals`, mirroring the parent's plural `recipes`. Consistent and self-documenting in `Cargo.toml` and in `use` statements.
   - **Module path under the recommended Option A:** `tui_vfx_recipes::signals` — since the parent crate name already carries "recipes", the inner module is just `signals` (avoids the awkward `tui_vfx_recipes::recipe_signals` doubling).
   - The longer name keeps the recipe-only scope explicit at every import site and discourages production code from reaching for it ("I'm not writing recipes — why am I touching this?"). Aligns with the project's strong instinct for narrow scoping.
2. **Placement — three options, narrowest preferred.** User direction (2026-04-26): "it could be a sub-crate within tui-vfx-recipes even, so it doesn't get published on it's own". Three concrete shapes; all of them prevent independent publication:

   | Option | Shape | Cost to set up | Publication risk | Recommended? |
   |---|---|---|---|---|
   | **A — in-crate module** | `tui_vfx_recipes::signals::*` (just `pub mod recipe_signals` inside the existing recipes crate) | smallest — one new module | zero (ships only as part of `tui-vfx-recipes`) | **✅ recommended** |
   | B — sub-crate in a recipes workspace | `tui-vfx-recipes/crates/tui-vfx-recipes-signals/` after converting recipes from single-crate to a Cargo workspace | medium — workspace conversion + new Cargo.toml | low (`publish = false`) | only if Option A grows past `cls_*` size limits |
   | C — standalone crate in tui-vfx workspace | `tui-vfx/crates/tui-vfx-recipes-signals/` | medium — new crate, cross-repo dep wiring | low (`publish = false`) | rejected — misplaces the surface in the wrong repo |

   **Why Option A wins.** It's the smallest change (one `pub mod` declaration), it captures the user's "don't publish it independently" intent absolutely (it isn't a crate at all), and the import shape `tui_vfx_recipes::signals::VfxRecipeSignalSpec` is self-documenting. If the module ever outgrows reasonable size (call it >800 LOC across the module subtree), promoting it to Option B is mechanical: convert recipes to a workspace, move the module to a sub-crate, add a re-export from `tui-vfx-recipes` for back-compat. So Option A is also the cheapest reversible decision.

   **Wire-format prefix.** Per Intention 8, public types crossing crate boundaries use the `Vfx*` prefix. Since `recipe_signals` is a module inside `tui-vfx-recipes`, *all* its public types still cross the recipes-crate boundary when downstream consumers (gt-design, the docs autogen) import them. So the prefix still applies: `VfxRecipeSignalSpec`, `VfxIntoRecipeSignal`, `VfxRecipeSignalMeta`, `vfx_recipe_signal_catalog()`.

   **Layout sketch under Option A:**
   ```
   tui-vfx-recipes/src/
     signals/
       mod.rs                            # pub use, module docs
       cls_vfx_recipe_signal_spec.rs     # the enum
       fnc_into_recipe_signal.rs         # the trait + impls
       cls_vfx_recipe_signal_meta.rs     # autogen metadata struct
       fnc_vfx_recipe_signal_catalog.rs  # the static catalog
       test_*.rs                         # round-trip serde tests, catalog completeness check
   ```
   OFPF prefixes apply per the per-prefix size limits (`cls_` 150/200, `fnc_` 75/120). The enum file will be large because of `Serialize/Deserialize` derives + per-variant doc comments — likely the first to bump against the `cls_` hard limit. If so, split the variants by family (oscillators, physics, spatial, composition) into separate files and `pub use` them up through `mod.rs`.
3. **Direct passthrough vs re-wrapping.** The sketch in §8.3 wraps `Sine(mixed_signals::Sine)`. An alternative is to define the recipe shape in `tui-vfx-recipes-signals` as a transparent wrapper that deserializes via `mixed_signals::Sine`'s own serde derive (using `#[serde(transparent)]` or similar). The wrapper has zero behavioral cost and gives us a place to attach recipe-only authoring metadata. Probably the wrapper.
4. **Versioning relationship to mixed-signals.** When mixed-signals adds a primitive, the facade doesn't *automatically* expose it. Adding a primitive to `RecipeSignalSpec` is a deliberate decision (each variant earns its place). This is a feature, not a bug — it keeps the recipe surface curated.

## Decision (2026-04-26)

**Accepted:** Option A — in-crate facade at `tui_vfx_recipes::signals::*`. Narrow scope: recipe-deserialization only; production code keeps importing `mixed_signals::*` directly. Promotion to a sub-crate (`tui-vfx-recipes-signals`, the proposal's Option B) stays on the table as a mechanical conversion if the module ever outgrows reasonable size.

**Phase ordering accepted from §8.6:**
- **α (green-lit, no further decision):** autogen `SIGNALS_REFERENCE.md` from `SignalSpec` rustdoc + `signals.toml` overlay. Doc-only.
- **β (green-lit, no further decision):** curated "Core 12" cheatsheet (one section of α's autogen output). Doc-only.
- **γ (deferred):** build the `signals` module + collapse the parallel physics channel. Wait until 1.2.A `VfxBindable<T>` lands so the symmetric Bindable family the module references is available.
- **δ (deferred):** symmetric `BindableF32` / `BindableColor` family with signal-form variants pointing at `VfxRecipeSignalSpec`. Hard-gated on 1.2.A.

**Acceptance also locks the headline maintenance lever:** locally-named/scoped interface point to drive recipe inputs; future swaps, plug-ins, exposure-limiting, and rename/remap stay in one place.

---

## 9. Revision (2026-04-27): completion plan after audit

### 9.1 Architectural framing

Two consumers, two paths — already separable in the codebase, but the delineation was undocumented.

- **Engine API (direct consumers)** in `tui-vfx::*` crates uses `mixed_signals::*` directly. Field types like `factor: SignalOrFloat` stay engine-native. Direct-API consumers (gt-design factory integration, `pipeline_effects_showcase.rs`, future widget consumers) keep the full upstream signal palette and IDE completion. **No change to direct API.**
- **Recipe player** in `tui-vfx-recipes` deserializes JSON through the facade and produces engine-native types at the seam. The facade's curation policy applies to JSON authoring only.
- **`Binding(String)` (host-supplied runtime values)** is orthogonal. Apps drive per-frame values through `RuntimeBindings` regardless of which signal-expression authoring path produced the field's default.

The facade is **a recipe-JSON deserialization seam, not a substitute for mixed-signals**. It lives one layer above the engine; the engine's contract with mixed-signals does not move.

### 9.2 Phase 1 — close the 15-variant gap in the facade

**Scope:** `VfxRecipeSignalSpec` reaches every Serialize/Deserialize-able primitive in `mixed_signals`. Catalog grows 43 → 58. No call-site migration.

**Work packet:** `steering/work-packets/64-recipe-signal-facade-completion-phase1.md`.

| Story | Pattern | Variants |
|---|---|---|
| US-1.1 | 8 transparent wrappers under `signals/random/` | `seeded_random`, `spatial_noise`, `gaussian_noise`, `poisson_noise`, `correlated_noise`, `pink_noise`, `per_character_noise`, `student_t_noise` |
| US-1.2 | inline struct mirroring SignalSpec shape; build via `ImpulseNoise::with_width(...)` | `impulse_noise` |
| US-1.3 | 3 transparent wrappers under `signals/envelopes/` (new family directory) | `linear_envelope`, `linear_decay`, `exponential_decay` |
| US-1.4 | 3 inline structs with `Box<VfxRecipeSignalSpec>` recursion (mirrors existing `Add` / `Mix`) | `vca_centered`, `phase_accumulator`, `phase_sine` |
| US-1.5 | wire enum + dispatch + catalog: 15 enum variants, 15 `into_recipe_signal()` arms, 15 catalog entries (`in_core_12: false`) | — |
| US-1.6 | round-trip serde tests + bump `catalog_completeness` assertion 43 → 58 | — |
| US-1.7 | rustdoc on every public item; metadata envelopes; one-line CLOG bumps | — |
| US-1.8 | verification + cross-repo audit per Intention 41: all four repos `cargo build` clean; rg counts in progress.txt | — |

**Phase 1 exit gates:** all 15 variants ship; catalog at 58; round-trip tests green; cross-repo `cargo build --workspace` clean; zero clippy warnings on `--all-targets -- -D warnings`; metadata envelopes complete; no `#[allow]` suppressions; progress.txt records audit evidence.

### 9.3 Phase 2 — consolidate recipe-side signal access onto the facade

**Scope:** every recipe-JSON deserialization site that accepts a signal expression routes through `VfxRecipeSignalSpec`. Engine field types (`SignalOrFloat`, `mixed_signals::*`) do not change. Direct-API consumers do not change.

**Gates:** Phase 1 must ship green.

**Work packet:** `steering/work-packets/65-recipe-signal-facade-consolidation-phase2.md`.

**Migration sites identified by `ofpf-refs SignalSpec` against `tui-vfx-recipes`:**

| Site | Current | Target |
|---|---|---|
| `src/v3/authoring/enum_v3_loopback_value.rs:35,71` | `V3LoopbackValue::Signal(SignalSpec)` | `V3LoopbackValue::Signal(VfxRecipeSignalSpec)`; lower at `to_signal_or_float()` via `into_recipe_signal()` |
| `src/v3/compile/fnc_build_composition_spec_from_compiled_plan.rs:635` | already on facade | unchanged |
| Recipe-deserialized `SignalOrFloat` fields (`fnc_derive_cursor_paint_ops_from_progress.rs`, `fnc_populate_effects.rs`, `cls_loopback_declaration.rs`, …) | engine-native `SignalOrFloat` directly from JSON | recipe-side adapter: accept `<number>` or `{"signal": <VfxRecipeSignalSpec>}`; lower to `SignalOrFloat` at the seam |
| `BindableValue::Signal(SignalOrFloat)` | engine-native | **Decision 2A (confirmed):** keep engine-native; lower at the recipe seam. Smaller blast radius; preserves Intention 24. Promote to 2B (`RecipeBindableValue` with facade-typed signal) only if leakage is later discovered. |

**Stories:**

| Story | Description |
|---|---|
| US-2.1 | Migrate `V3LoopbackValue::Signal` to facade. Update lowering. Round-trip tests. |
| US-2.2 | Audit recipe-side `SignalOrFloat`-typed fields; identify which are populated by JSON deserialization (recipe seam) vs. constructed in code. Recorded in progress.txt. |
| US-2.3 | Implement a recipe-deserialization adapter (single helper) that accepts `<number>` or `{"signal": <VfxRecipeSignalSpec>}` and lowers to `SignalOrFloat`. |
| US-2.4 | Apply the adapter at every recipe-deserialization site found in US-2.2. |
| US-2.5 | Strict-contracts validator gate (Intention 25): a recipe authoring an upstream-only `SignalSpec` discriminant fails strict validation. |
| US-2.6 | Cross-repo audit per Intention 41 across all four repos. |
| US-2.7 | Verification: full workspace build + test green; recipe corpus validates clean; gt-design integration tests pass; pipeline_effects_showcase example unchanged. |

**Phase 2 exit gates:** every recipe-JSON deserialization site that accepts a signal routes through `VfxRecipeSignalSpec`; strict-contracts validator rejects non-facade signal discriminants; cross-repo build green; zero behavior change for direct-API consumers.

### 9.4 Phase 3 — engine-vs-recipe-player delineation in docs

**Scope:** make the engine vs recipe-player split explicit in code-level rustdoc, examples, and steering. Closes the silent-delineation gap surfaced 2026-04-27.

**Work packet:** `steering/work-packets/66-engine-vs-recipe-player-delineation-phase3.md`.

| Story | Description |
|---|---|
| US-3.1 | `crates/tui-vfx/src/lib.rs` rustdoc: state that the engine API (`render_pipeline`, `CompositionSpec`, `MaskSpec`, `FilterSpec`, `SamplerSpec`, `ShadowSpec`) is a public, fully-supported direct-consumption surface. Recipes are a peer authoring layer in `tui-vfx-recipes`, optional for direct-API consumers. mixed-signals is the substrate; the engine consumes it directly. |
| US-3.2 | `tui-vfx-recipes/src/lib.rs` and `src/signals/mod.rs` rustdoc: the facade is a recipe-JSON deserialization seam, recipe-only; direct-API consumers should `use mixed_signals::*` instead. |
| US-3.3 | Add `examples/direct_api_signal_strength.rs` exercising `FilterSpec::Vignette { strength: SignalOrFloat::Signal(SignalSpec::Sine { ... }), ... }` constructed in Rust and passed to `render_pipeline()`. |
| US-3.4 | Update this proposal doc to reflect Phase 1+2+3 ship state (final closeout). |
| US-3.5 | Add Intention 44 to `steering/INTENTIONS.md`: "Recipe-JSON signal authoring goes through `VfxRecipeSignalSpec`; engine direct-API consumers use `mixed_signals::*` directly. The two surfaces are intentional and meet at `SignalOrFloat`-typed engine fields." Durable counter-force against future drift. |

**Phase 3 exit gates:** delineation appears in `tui-vfx::lib.rs` rustdoc; one direct-API signal example ships; this doc updated; Intention 44 added to steering.

### 9.5 Sequencing

```
Phase 1 (this packet)
  ├── US-1.1 .. US-1.4 (variant additions, parallelizable per family)
  ├── US-1.5 (wire enum + dispatch + catalog)        ← gates on US-1.1..1.4
  ├── US-1.6 (tests)                                 ← gates on US-1.5
  ├── US-1.7 (rustdoc hygiene)                       ← runs alongside
  └── US-1.8 (verify + audit)                        ← gates on US-1.6
                │
                ▼
Phase 2 (next packet; gates on Phase 1 green)
  ├── US-2.1 (V3LoopbackValue migration)
  ├── US-2.2 .. US-2.4 (audit + adapter + apply)
  ├── US-2.5 (validator gate)
  ├── US-2.6 (cross-repo audit)
  └── US-2.7 (verify)
                │
                ▼
Phase 3 (next packet; can overlap with Phase 2)
  └── US-3.1 .. US-3.5 (docs + Intention 44)
```

### 9.6 Out of scope (across all three phases)

- New mixed-signals primitives — Intention 9 (separate decision).
- Replacing `SignalOrFloat` in engine field types — would break direct-API consumers.
- App-driven SignalSpec injection at frame time — not used today; if needed later, an explicit `RecipeSignalRef` variant on the facade can absorb it.
- `WeightedMix` exposure — non-Serialize upstream type; defer until a recipe needs it.
- Replacement of the V3 binding layer (`Binding(String)`) — orthogonal; works with both paths.

### 9.7 Decisions confirmed by user (2026-04-27)

1. ✅ **Phase 1 only this session.** Phase 2 and 3 follow as separate packets.
2. ✅ **Option 2A** for `BindableValue::Signal`: keep engine-native; lower at the recipe seam.
3. ✅ **`WeightedMix` deferred.** Documented; revisit when a recipe needs it.
4. ✅ **Intention 44** added in Phase 3 as the durable counter-force.
5. ✅ **Spec audit:** zero hallucinations; all 15 missing variants are real upstream types.

<!-- <FILE>docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.4.0</VERS> -->
