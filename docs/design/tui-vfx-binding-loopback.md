<!-- <FILE>docs/design/tui-vfx-binding-loopback.md</FILE> - <DESC>Design proposal: binding loopback — recipe-author-declared fallback values for runtime-bound parameters so debug/preview contexts can play recipes that production hosts would normally drive externally</DESC> -->
<!-- <VERS>VERSION: 0.4.0</VERS> -->
<!-- L1+L2 shipped — see implementation pointers in the status banner below. Authoring shape reconciled with the existing root-level `requires_bindings` block; all five BindingKinds (U16/F32/Bool/String/Color) ship in L2; Intention 37 supersedes the "loopback is optional, omit when production-only" guidance — every declaration MUST yield an effective loopback at the strict-contracts gate. -->
<!-- <WCTX>Pull the `extends`-based tiered base recipes proposal out of the design — for the demo-recipe use case the indirection cost outweighs the DRY win, and dropping it removes the deep-merge-vs-replace open question entirely. Replace the `⚠` glyph in the badge spec with a Nerd Font primary + ASCII fallback decision since the emoji-presentation `⚠` formats inconsistently next to monospace text.</WCTX>
<!-- <CLOG>Demote section 10 (tiered base recipes via extends) to a one-paragraph "Considered alternatives" note with the reasoning preserved so future-me doesn't reinvent it. Update badge glyph spec to Nerd Font `nf-fa-warning` () primary with ASCII `!` fallback, configurable via LoopbackBadgeStyle; drop emoji-presentation `⚠`.</CLOG> -->

# Binding Loopback

> **Status:** L1 + L2 shipped (2026-04-26). L3 (visibility badge), L4
> (strictness modes), and L5 (probe + browser + demo recipes) remain.
>
> **Implementation pointers** for the L1+L2 surface:
>
> - L1 engine layer: `tui-vfx-recipes/src/loopback/` —
>   `BindingKind`, `LoopbackValue`, `LoopbackDeclaration`,
>   `LoopbackDeclarations`, `evaluate_loopback`, `merge_loopback_params`.
> - L2 authoring shape: `tui-vfx-recipes/src/v3/authoring/` —
>   `V3BindingDeclaration`, `V3BindingDeclarationKind`, `V3LoopbackValue`.
>   The block is the existing root-level `requires_bindings` (not a new
>   `config.bindings` block as the original v0.1 draft proposed; the
>   corpus already authored at the root, so the typed shape was layered
>   there).
> - L2 strict-contracts gate: `tui-vfx-recipes/src/v3/validate/col_validate_contracts.rs` —
>   rejects any `requires_bindings` entry where `effective_loopback().is_none()`.
> - L2 compile / render wiring: `tui-vfx-recipes/src/v3/compile/fnc_compile_loopback_declarations.rs`
>   (V3 → L1 lowering) and the `with_loopback_applied` method on
>   `CompiledV3RuntimeOverrides` (per-frame merge before the engine sees
>   `runtime_params`).
>
> **Behavioural notes**:
>
> - **Intention 37 supersedes section 3's "omit when production-only"
>   guidance.** Every declaration MUST yield an effective loopback. The
>   strict-contracts gate fails the recipe at validation time when it
>   doesn't. Recipes that wire to a live host still author a synthetic
>   loopback (a static literal, a slow ramp) so the preview tile renders
>   meaningfully; document the production intent in `description`, not
>   by omitting the loopback.
> - **All five BindingKinds ship in L2.** Numeric kinds (`U16`, `F32`)
>   accept signal-driven loopback (static or `SignalSpec`); non-numeric
>   kinds (`Bool`, `String`, `Color`) accept literal-only loopback in v1
>   via the legacy `default:` field — engine-side string/color signal
>   vocabularies are future work and would extend `LoopbackValue`.
> - **L1 SignalSpec field names**: the implementation uses the
>   `mixed_signals` crate's existing `SignalSpec::Ramp { start, end,
>   duration }` shape. The original v0.1 draft used `from / to /
>   period_ms`; those names were aspirational and never landed.
>
> **Audience:** Anyone touching `BindableU16`, `BindableValue`,
> `BindableString`, procedural sources, the recipe runtime, the recipe
> browser, the demo player, or the pipeline-validator probe.

---

## 0. Canonical framing: tokens vs. bindings

> **Tokens rewrite the recipe before it enters the pipeline. Deterministic
> and stable. Bindings populate the pipeline dynamically as the recipe
> executes and make it dynamic.**

This one-line distinction is load-bearing for everything below. Internalise
it before reading the rest:

- **Tokens** are template substitution. Resolved once at recipe-load
  time, before the pipeline ever runs. A token reference is gone by the
  time the pipeline executes — it leaves no trace beyond the substituted
  JSON. They are an authoring/theming concern. They have their own
  contract layer (`UndeclaredTemplateContract`) and their own
  resolution pass.
- **Bindings** are runtime parameters. Looked up every frame against
  `ShaderRuntimeParams`. A binding reference is the live wire — it
  exists in the data path for the entire lifetime of the recipe, not
  just at load time. They have their own contract layer
  (`UndeclaredBindingContract`) and the per-frame lookup is the
  resolution.

The corollary that drives this whole document: **the loopback only
makes sense for bindings.** Tokens cannot fall back because by the
time anything could fall back, the token is already a literal value
baked into the recipe. Bindings can fall back precisely because they
are the only thing still being looked up at frame time.

Everything in this proposal lives on the bindings side of that line.

---

## 1. Problem statement

Several authoring shapes in tui-vfx end in a per-frame lookup against
a host-supplied parameter map (`ShaderRuntimeParams`):

| Shape | Where it lives | Lookup signature |
| --- | --- | --- |
| `BindableU16::Binding(name)` | `tui-vfx-style::cls_bindable_u16` | `params.get_u16(name)` |
| `BindableValue::Binding(name)` | `tui-vfx-compositor::cls_bindable_value` | `params.get_f32(name)` |
| Future bindable types | TBD | TBD |

These are **the right shape for production**: an interactive UI's hover
row, a scrubber's progress, a stage's expand/collapse position — all of
those values originate outside the recipe and must be injected per
frame by host code.

But the same shape **breaks debug and preview contexts**. The recipe
browser, the standalone demo player, `pipeline-validator --probe`, and
any "render this fixture for review" workflow have no host code wired
up — they don't know what value to push for `synth_grid_end_row` or
`hovered_button_x`. Today the workarounds are:

1. **Author with a literal in the demo recipe.** The binding path
   never runs. Bugs in lowering, resolution, or contract enforcement
   stay hidden until production hosts wire up the live data — which is
   the worst possible time to discover them.
2. **Leave the binding unresolved.** The predicate silently mismatches
   (per the documented `Cell` / `RowRange` / etc. contract), so the
   recipe renders as if the styled region were empty. To a reviewer
   this looks like a broken recipe, not a missing wiring.
3. **Push fake data per recipe from the demo player.** Requires
   bespoke per-recipe code in the player; doesn't scale and ages
   poorly as new bindable types land.

None of these is acceptable as the primary story for a library whose
recipe browser is the dominant author/review surface.

## 2. The loopback metaphor

In telecom, a **loopback** wires a transmitter's output back to its
own receiver so the local equipment can be tested without the remote
end being present. The data path under test runs unchanged; the
loopback just supplies the input the remote side normally would. T1
lines have hardware loopbacks, modems have local/remote loopback
modes, every NIC supports `lo`.

We want the same engineering payoff for recipes: the binding lookup
path runs **unchanged**, but in debug/preview contexts the value
comes from a recipe-author-declared signal source instead of from
host injection. In production the host's live value takes precedence
and the loopback steps aside.

Critically: the loopback is not a "demo mode" parallel code path. It
is the same path. The recipe is one source-of-truth artifact that
plays correctly in the browser, in the probe, in the demo player, and
in production hosts — with the only difference being who supplied the
binding's value at frame time.

## 3. Authoring shape — the `bindings:` declaration block

A recipe declares its expected interface to the host via a new
top-level `bindings:` block. Each entry names one binding the recipe
will look up at frame time, declares its kind (destination type),
optionally documents it, and optionally supplies a loopback value:

```json
{
  "schema_version": 3,
  "id": "demo.synth_grid.expand_collapse",
  "config": {
    "bindings": {
      "synth_grid_end_row": {
        "kind": "u16",
        "description": "End row of the expand animation, 0..height.",
        "loopback": { "signal": { "ramp": { "from": 0, "to": 24, "period_ms": 2400 } } }
      },
      "synth_grid_start_row": {
        "kind": "u16",
        "description": "Start row of the expand animation. Stays at 0 in this demo.",
        "loopback": { "literal": 0 }
      },
      "hover_progress": {
        "kind": "f32",
        "description": "0..1 progress of the active hover.",
        "loopback": { "literal": 1.0 }
      }
    },
    "pipeline": { /* … uses {"binding": "synth_grid_end_row"} as before … */ }
  }
}
```

The `bindings:` block is an **interface declaration**, not just a
loopback bag. Every binding the pipeline references must be declared
here (see section 6 — strict-contracts gate). The loopback is one
field on each declaration; declarations also carry kind and
description, both of which are useful in production where loopback
never fires.

### Field reference per entry

| Field | Required | Purpose |
| --- | --- | --- |
| `kind` | yes | Destination type. Today: `u16`, `f32`. Future: `string`, `seed`, … one new variant per new bindable type. Drives the loopback boundary conversion (e.g. `f32` signal evaluated to `f32`, then `round+clamp` for `u16` destinations). |
| `description` | recommended | Free-text contract documentation. Surfaced in the recipe browser, in generated host-binding docs, and in probe diagnostics. Authors should describe what the value means, its expected range, and what visual outcome the host should expect. |
| `loopback` | optional | Fallback value used in debug/preview contexts when the host hasn't supplied this binding (see section 4). Omit when the recipe has no meaningful preview value (rare; usually means the recipe is production-only). |
| Future fields | — | Reserve for `units`, `valid_range`, `severity_on_missing`, etc., as real use cases surface. |

### Why this shape (vs. a parallel `preview_runtime_params` block)

An earlier draft used a separate `preview_runtime_params` block keyed
by binding name. That works mechanically but is the wrong layering:

- It buries the recipe's host-interface contract inside what reads as
  a debug-mode-only convenience.
- It requires the strict-contracts validator to track two separate
  declaration sources.
- Adding kind / description / future per-binding metadata would either
  bloat the preview block (mixing concerns) or require yet another
  parallel block.

`bindings:` puts the contract in one place. Loopback is one field on
that contract, alongside the contract's other metadata. That keeps
debug, preview, validator, and production all reading the same record.

### Composition and the lenient deserializer

The `bindings:` block is **strictly additive** in JSON: recipes that
don't have one keep behaving exactly as today (no declared bindings,
no loopback layer, the existing "binding missing → predicate silently
mismatches" contract fires). Any binding referenced in the pipeline
without a matching declaration fails strict-contracts (see section 6),
the same way undeclared template tokens fail
`UndeclaredTemplateContract` today.

## 4. Fallback semantics

The loopback is a **fallback**, not an override. The lookup precedence
is:

1. Host-supplied `ShaderRuntimeParams` value for that key.
2. If absent, the loopback evaluation from the recipe's `bindings:`
   block (the named entry's `loopback` field, if present).
3. If still absent (no loopback declared, no host value), the existing
   "binding missing → predicate silently mismatches" contract fires
   exactly as today.

This composes cleanly with production wiring: a host that knows about
`synth_grid_end_row` injects it as today and the loopback never fires.
A debug player that doesn't inject anything sees the loopback supply
the value and the recipe plays. Same recipe, both contexts.

## 5. Visibility — the ⚠LB badge

Silent fallbacks are how production incidents start. A recipe playing
with one or more loopbacks active in a context the author didn't
expect must be **visibly distinguishable** from one playing on real
data.

### Badge spec

| Property | Value |
| --- | --- |
| Width | 4 cells (ASCII) or 5 cells (Nerd Font) — see `Glyphs` |
| Glyphs | **Nerd Font (default):** `` (`nf-fa-warning`, single-cell filled triangle with `!`) followed by `LB` and trailing pad → 5 cells. **ASCII fallback:** `!LB ` or `[LB]` — 4 cells, no font dependency. The `⚠` (U+26A0) emoji-presentation glyph is **explicitly rejected** — it gets variant-selector emoji rendering in most modern terminals (often 2 cells wide, inconsistent formatting next to monospace text). Host picks via `LoopbackBadgeStyle::{Auto, NerdFont, Ascii}`; `Auto` defaults to Nerd Font on the assumption that tui-vfx's terminal-rendering audience already runs one (same baseline as the existing powerline detection in the codebase). |
| Foreground | Orange (matches `Warning` probe severity tier — yellow reads as info, red as error) |
| Background | Recipe's base bg (transparent over content) |
| Position | Upper-right of the rendered surface, inside the border if present |
| Animation | Slow pulse (~0.6 Hz) for the first ~3 seconds of playback, then settle to solid orange |
| Edge fade | Sub-cell alpha falloff at the badge's outer edges (1-cell ramp into the surrounding surface) so the orange feels integrated rather than pasted-on. Costs nothing rendering-wise — we already have the alpha pipeline. |
| Trigger | Any loopback fired during the current frame |

The "pulse, then settle" cadence is deliberate: the pulse does the
"hey, attention" job up front, then the badge stays solid so a recipe
browser tiling 30 cards doesn't become a sea of motion. The badge
remains visible (not animated) for the rest of playback so the
loopback-active state never slips out of view.

### What the badge means in each context

| Context | Badge meaning |
| --- | --- |
| Recipe browser | Expected — confirms the loopback is wired and the recipe is in preview-mode rendering. |
| Standalone demo player | Expected — same as browser. |
| `pipeline-validator --probe` | Recorded as a `Warning` diagnostic (one per binding that fired loopback). Probe report lists the keys for review. |
| Production host | **Not expected.** The host is supposed to inject the live value; if the badge appears, host wiring is missing. The strictness mode (section 6) decides whether this is also an error. |

### Detail labels (optional)

Hovering / focusing the badge in the recipe browser should reveal the
list of binding keys that fell through to loopback this frame. The
badge itself stays compact; the detail is one click away.

## 6. Strictness modes

The host configures how loopback firings are treated:

| Mode | Behavior when loopback fires |
| --- | --- |
| `permissive` (default in browser, demo player, probe) | Loopback supplies the value, badge appears, render continues. |
| `warn` | Same as permissive, plus a structured log/event each frame any loopback fires. Useful for staging hosts. |
| `strict` | Loopback is disabled entirely. A missing host value falls through to the pre-loopback "binding missing" contract (predicate silently mismatches as today). The badge **always** appears whenever the predicate would have used a loopback in permissive mode, so missing host wiring is loud even though the recipe still renders. |
| `error` | Loopback is disabled and a missing host value is a hard render error (panic / `Result::Err` depending on the host's tolerance). Suitable for CI gates that should never let a recipe ship with un-wired bindings. |

Strictness is a **runtime** setting (host configuration), not a
recipe-author setting. Authors declare loopbacks; hosts decide
whether to honor them. This keeps recipes portable across debug /
staging / production without per-environment forks.

## 7. Host API surface

The host needs a small, learnable set of knobs. Working list (subject
to refinement during implementation):

```rust
// On the renderer / animation manager builder.
.with_loopback_strictness(LoopbackStrictness::Permissive)
.with_loopback_badge_visibility(LoopbackBadgeVisibility::Auto)   // Auto / AlwaysShow / AlwaysHide
.with_loopback_observer(|event| { /* … */ })                     // structured per-fire callback
```

Plus a per-recipe-item override for the rare cases (e.g. "this one
recipe in the browser should be played strict to validate it is fully
wired"):

```rust
item.with_loopback_strictness(LoopbackStrictness::Strict)
```

And a query hook for the recipe browser UI to enumerate the recipe's
binding contract (kind, description, loopback presence) without
playing it:

```rust
recipe.bindings_summary() -> Vec<BindingDeclaration>
```

The defaults must be such that **a developer who doesn't engage with
any of this gets sensible behavior**: browser/preview contexts default
permissive (so demos play), production-style hosts that omit the
configuration default permissive too (badge appears so missing wiring
is at least visible) — flipping to `strict` or `error` is an explicit,
documented opt-in for hardening.

## 8. Composition with existing bindable types

The loopback layer sits in the lookup path, not in any individual
bindable type. It applies uniformly:

- `BindableU16::evaluate(&runtime_params)` consults the same
  `runtime_params` as today — that map is the one the host populates,
  and the loopback layer pre-fills any keys the recipe declared
  before the lookup runs.
- `BindableValue::evaluate(t, signal_ctx, &runtime_params)` —
  identical mechanism. The `Signal` arm of `BindableValue` is
  unaffected (it's already self-contained); only the `Binding` arm
  benefits from loopback fallback.
- Future bindable types (e.g. `BindableColor`, `BindableString`)
  inherit loopback support as soon as they thread through
  `runtime_params`. The `bindings:` declaration's `kind` field grows
  by one variant per new bindable type.

No per-type code changes are required when adding a new bindable
type, beyond the `kind` enum extension.

## 9. Procedural pathway

Procedurals warrant a dedicated section because their relationship
to host-injected data is **different from bindables today** but
converging. Capturing the current state and the convergence path
prevents a future fork into "loopback for bindables, separate
mechanism for procedurals."

### Current state (audited 2026-04-26)

`ProceduralCtx` (at `src/scene/procedural/types/cls_procedural_ctx.rs`)
carries `area`, `phase_t`, `loop_t`, `signal_ctx`, and a
recipe-author-supplied opaque `params: &Value`. It does **not**
carry `&ShaderRuntimeParams`. Procedural sources today consume only:

1. Static params from the recipe JSON (set at load time, never
   changes during playback).
2. The recipe clock (phase_t / loop_t).
3. A signal context for deterministic spatial waveforms.

That means **procedurals don't currently have the gap** that bindables
have. They can't suffer from "missing host data" because they don't
ask for any. A procedural source is already self-contained and
deterministic given the recipe + clock.

### Why the loopback design still applies

Procedurals are one architectural step away from the gap. As soon as
a procedural source wants a host-injected parameter — a marquee whose
text comes from a host counter, a generated grid keyed off a
user-action seed, a numeric display sourced from a live metric — it
will need `ShaderRuntimeParams` threaded into `ProceduralCtx`. The
moment that happens, the loopback design applies unchanged:

- Same lookup precedence (host → loopback → missing).
- Same `bindings:` declaration block (the procedural's parameters
  appear as named bindings of kind `string`, `seed`, etc.).
- Same ⚠LB badge.
- Same strictness modes.

This validates the loopback design rather than complicating it.
Building loopback on the `ShaderRuntimeParams` substrate means any
future runtime-data consumer — bindable types, procedurals, anything
else — inherits loopback support automatically. The alternative —
per-feature loopback layers — would compound technical debt with
every new consumer.

### Concrete phasing

When a procedural source first needs host-injection (call it
"procedural Phase P0"), the work is:

1. Add `runtime_params: &ShaderRuntimeParams` to `ProceduralCtx`.
2. Have the source impl read its parameters from a small
   `BindableString` / `BindableSeed` / etc. wrapper that consults
   `runtime_params`.
3. Author the procedural's parameters as `bindings:` declarations
   with `kind: "string"` / `"seed"` / etc., declaring loopback values
   for the debug/preview path.

Steps 1–2 are independent of this loopback proposal and unblock
host-driven procedurals on their own. Step 3 is free if the loopback
layer (Phase L1+) is already shipped — same `bindings:` block, same
mechanism, no per-type plumbing.

### What stays out of scope for v1

The opaque `params: &Value` payload that procedurals currently consume
is **not** subject to loopback. It is a recipe-author-supplied static
configuration, not host-injected runtime data. Loopback only applies
to data that crosses the recipe/host boundary at frame time.

## 10. Considered alternatives — why each recipe declares its own `bindings:` inline

**Inline declarations win for the demo-recipe use case.** Each demo
recipe is its own self-contained example, and a developer reading
one should see the full binding contract without chasing an
`extends` chain. Repetition across N small demos is documentation
through repetition, not a maintenance hazard, when each declaration
is 4-6 lines of JSON with a simple signal expression.

### Rejected: tiered base recipes via `extends`

An earlier draft proposed a `recipes/_bases/loopback_*.json` family
of base recipes carrying canonical `bindings:` declarations, with
demos `extends`-ing a base to inherit them. The case for it: DRY
across many recipes sharing the same binding contract.

The case against it (which won):

1. **Indirection cost dominates the DRY win at the demo scale.** Most
   demo recipes will declare 1-3 bindings with simple signal
   expressions. Copy-paste across them isn't painful enough to
   justify chasing an `extends` chain to read what a recipe consumes.
2. **Self-documentation matters more for demos than DRY.** Demos
   *are* the documentation. Inline declarations make every demo
   recipe a complete reference for its own bindings.
3. **Removes a load-bearing open question.** The `extends` path needed
   deep-merge-vs-replace semantics resolved against the existing
   resolver's behaviour for other additive blocks. That's a real
   risk of subtle bugs at recipe-load time; dropping the path drops
   the question.
4. **Production recipes can revisit if a real pain point appears.**
   YAGNI for v1. If a specific recipe family ends up with 20
   demos all declaring the same 6 bindings, an `extends`-based
   refactor at that point is mechanical.

The `extends` mechanism remains the right tool for sharing pipeline
shape, theme styles, base layouts, etc. It just isn't the right tool
for binding declarations.

## 11. Open questions

These need resolving before implementation, captured here so they
don't get lost:

1. **Signal vocabulary for u16 destinations.** Reuse `SignalOrFloat`
   with a boundary-conversion (round+clamp), or introduce a sibling
   `SignalOrU16`? Reusing `SignalOrFloat` keeps the surface small but
   bakes float→u16 semantics into one place that may need
   per-destination control later.
2. **Where the badge is composited.** As a final post-process
   shader on the rendered surface (engine concern), or as a recipe
   browser overlay (player concern)? The probe-time use case argues
   for engine-side composition so probe outputs carry the badge too;
   the player-overlay path is simpler. Likely answer: engine-side,
   gated by a flag the host can disable.
3. **Probe diagnostic shape.** Existing
   `ProbeOperationalStatus::Warning` is the right tier; need to
   decide whether each loopback fire is one diagnostic or whether
   they aggregate per-recipe-per-key.
4. **Per-binding strictness.** Do hosts need to express "strict for
   this key, permissive for that key"? Probably YAGNI for v1; revisit
   if a real use case appears.
5. **Loopback declarations on scene layers vs root.** A V3 recipe with
   scene layers may want per-layer loopback scopes. Initial proposal:
   single root-level block, with per-layer overrides as a follow-on
   if demand surfaces.
6. **Interaction with `--strict-contracts` validation.** Should the
   validator gate that any binding referenced in the pipeline has
   either a loopback declaration OR a documented host-injection
   expectation? Strong yes for v2; v1 can ship without it.

## 12. Phased implementation outline

Suggested slice order (each slice is independently shippable):

1. **Phase L1 — engine fallback layer.** Pre-fill `runtime_params`
   from a recipe-supplied map before the lookup runs. No badge, no
   strictness, no JSON authoring yet — just the lookup-precedence
   plumbing and a Rust-level API for hosts to supply the loopback
   map programmatically. Unblocks internal testing.
2. **Phase L2 — `bindings:` block authoring.** Add the `bindings:`
   block to the V3 schema, parse + normalize + compile it, and feed
   the loopback values into the L1 layer. The strict-contracts
   validator starts requiring every pipeline-referenced binding to
   be declared here. At this point the debug player can render
   bindable recipes from the JSON alone.
3. **Phase L3 — visibility badge.** Engine-side composition of the
   `LB` badge per the spec in section 5 (including the soft edge
   fade and the Nerd Font / ASCII glyph choice), gated by the host's
   visibility configuration.
4. **Phase L4 — strictness modes.** Add the `LoopbackStrictness` enum
   and host configuration surface per section 6.
5. **Phase L5 — probe + browser integration.** Probe diagnostic emit,
   recipe browser detail-on-focus, strict-contracts validator full
   integration with `bindings:` declarations.
6. **Phase P0 — procedural host-injection.** Thread
   `&ShaderRuntimeParams` into `ProceduralCtx` so procedurals can
   consume host-injected runtime data (live progress meters,
   counters, scrubber positions, etc.) on the same substrate
   bindables already use. Once shipped, procedurals get loopback
   support automatically by going through the L1–L2 layer (section 9).
   **Release-blocking** — host-driven progress-meter procedurals
   depend on this; not a follow-on to L1–L5.

Each phase is end-to-end testable on its own. Phases L1–L3 are the
critical path for "demo recipes that exercise bindings actually work
in the browser." **P0 runs in parallel with L1–L5 because it has its
own release-blocking driver** (live progress meters), and lands the
plumbing the loopback layer needs to be useful to procedurals.

## 13. References

- `crates/tui-vfx-style/src/models/cls_bindable_u16.rs` — current
  BindableU16 implementation; the type the loopback first targets.
- `crates/tui-vfx-compositor/src/types/cls_bindable_value.rs` — sibling
  BindableValue with the existing `Signal` variant we mirror for u16.
- `crates/tui-vfx-style/src/traits/` — `ShaderRuntimeParams` trait;
  the lookup surface the loopback layer pre-fills.
- `tui-vfx-recipes/src/v3/validate/enum_validate_error.rs` —
  `UndeclaredBindingContract` and `UndeclaredTemplateContract` variants
  that already separate the binding and token contract layers
  (section 0's tokens-vs-bindings distinction is encoded here).
- `tui-vfx-recipes/src/scene/procedural/types/cls_procedural_ctx.rs` —
  current ProceduralCtx surface; the file Phase P0 extends to thread
  runtime_params (section 9).
- `tui-vfx-recipes/src/recipe_schema/functions/fnc_resolve_recipe_template.rs` —
  the `extends` resolver. Section 10 documents why we explicitly do
  *not* use it for binding declarations.
- `docs/CAPABILITIES_REFERENCE.md` — `StyleRegion::Cell/RowRange/
  ColumnRange/Modulo` sections that document the
  binding-missing-silently-mismatches contract this design preserves.
- `docs/design/tui-vfx-v3-recipe-vocabulary.md` —
  "Runtime-binding support today" subsection that this document
  effectively obsoletes once shipped.

<!-- <FILE>docs/design/tui-vfx-binding-loopback.md</FILE> - <DESC>Design proposal: binding loopback</DESC> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
