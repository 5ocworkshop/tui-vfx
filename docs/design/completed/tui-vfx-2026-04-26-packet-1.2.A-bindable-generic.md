<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-1.2.A-bindable-generic.md</FILE> - <DESC>Implementation packet for buy-once sweep finding 1.2.A (generalize the Bindable* family into one Bindable<T>). Bundles finding 1.7.A (BindableValue cross-crate home decision). Self-contained execution brief: pre-flight, current-state audit of all three concrete Bindable types and their consumers, open architectural decisions with recommended defaults, step-by-step plan, code snippets for two competing generic shapes (Bindable<T, S=()> vs Bindable<T, S=Never>), test plan, acceptance criteria, verification commands, rollback plan.</DESC> -->
<!-- <VERS>VERSION: 1.1.0</VERS> -->
<!-- <WCTX>Mark packet DONE 2026-04-26 — all three Q1/Q2/Q3 leader calls accepted with recommended defaults (home: tui-vfx-core, prefix: VfxBindable, signal-arm shape: S = Never with three-variant Literal | Binding | Signal enum). Implementation landed.</WCTX> -->
<!-- <CLOG>1.1.0: prepend implementation-status banner — packet is DONE. Implementation landed as VfxBindable<T, S = Never> in tui-vfx-core::bindable; three legacy concrete types are thin re-export aliases; originals recyclebinned; sweep findings 1.2.A and 1.7.A both marked DONE in tui-vfx-buy-once-architecture-sweep.md v1.4.0. 1.0.0: initial packet (pre-flight, current-state audit, open questions, plan, code snippets, acceptance criteria).</CLOG> -->

# Packet 1.2.A — Bindable<T> generalization (bundles 1.7.A cross-crate home)

> **Status: DONE 2026-04-26.** Q1/Q2/Q3 leader calls accepted with recommended defaults — home crate `tui-vfx-core`, prefix `VfxBindable`, signal-arm shape `S = Never` (three-variant `Literal | Binding | Signal` enum, Signal arm uninhabited for non-signal instantiations). Implementation landed:
>
> - `tui_vfx_core::bindable::VfxBindable<T, S>` plus the project-local `Never` uninhabited type (orphan rules forbid using `std::convert::Infallible` because serde lacks impls for it).
> - `RuntimeParamsRead` trait in `tui-vfx-core`, implemented by `tui-vfx-style::ShaderRuntimeParams`. Keeps `evaluate` methods on the generic without forcing `tui-vfx-core` to depend on style-side types.
> - `BindableSignal` helper trait omits the phantom `Signal` arm from generated schemas for non-signal `S`.
> - Three type aliases: `VfxBindableU16 = VfxBindable<u16>`, `VfxBindableString = VfxBindable<String>`, `VfxBindableValue = VfxBindable<f32, SignalOrFloat>`. Per-instantiation inherent impls preserve the three legacy `evaluate` signatures verbatim.
> - Wire format: bare `T` → `Literal(T)`, tagged `{"literal": ...}` / `{"binding": ...}` / `{"signal": ...}` work universally, plus a `BareSignal(S)` lenient fallback for inline signal payloads. Static values now route canonically through `Literal` (e.g. `BindableValue::default()` is `Literal(0.0)`, `static_f32(v)` produces `Literal(v)`). Evaluator outputs unchanged.
> - The three legacy modules (`crates/tui-vfx-style/src/models/cls_bindable_u16.rs`, `cls_bindable_string.rs`; `crates/tui-vfx-compositor/src/types/cls_bindable_value.rs`) are now thin `pub use` re-exports under their historical names. Original 250 / 322 / 116 LOC bodies retired to `recyclebin/` mirroring the source paths.
> - Tests: 35 new peer tests (`crates/tui-vfx-core/src/bindable/test_cls_bindable.rs`) + 75/75 BindableValue regression tests (`crates/tui-vfx-compositor/tests/types/test_bindable_value.rs`, assertions updated for new variant routing) green. In-scope clippy clean (`cargo clippy -p tui-vfx-core --all-targets -- -D warnings`).
> - Sweep findings 1.2.A and 1.7.A marked DONE in `docs/design/tui-vfx-buy-once-architecture-sweep.md` v1.4.0; handoff doc `tui-vfx-2026-04-26-handoff-outstanding.md` v1.3.0.
> - Workspace-wide verification gate (`cargo test --workspace`, `cargo clippy --workspace`, `cargo xtask docs generate`) is owned by the parallel pipeline-observability slice that runs concurrently and currently has the workspace temporarily red on `tui-vfx-debug`. Bindable scope is fully green.
>
> The rest of this packet is preserved as the historical brief used to drive the implementation. Open questions, code snippets, and rollback plans below describe the design as it was decided, not active work.

---


> **Source findings.** `docs/design/tui-vfx-buy-once-architecture-sweep.md` §1.2.A (lines 140–186), §1.7.A (lines 388–404, "bundle into 1.2.A"), §6.2 (lines 617–642).
>
> **Status note (2026-04-26).** Genuinely queued. OFPF audit confirms three concrete Bindable types still live in their original homes (`crates/tui-vfx-style/src/models/cls_bindable_u16.rs`, `cls_bindable_string.rs`; `crates/tui-vfx-compositor/src/types/cls_bindable_value.rs`). The handoff doc `tui-vfx-2026-04-26-handoff-outstanding.md:20` lists 1.2.A as queued and that is accurate.
>
> **Risk tier (per sweep).** L — broad ripple across `tui-vfx-style/src/models/`, `tui-vfx-compositor/src/types/`, and every consumer of the three concrete types.
>
> **Sequencing.** Land **before** signal-facade Move 3 ("symmetric Bindable family + sibling `signal | binding` fields" — `docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md:195–211`). The facade's phase δ adds `BindableF32` and `BindableColor` and adds a `Signal` arm to every Bindable; doing that on three parallel hand-rolled types is five files of churn versus one type. This packet is the prerequisite.

---

## Goal & motivation

Three parallel hand-rolled Bindable types share an identical structural pattern:

| Type | File | LOC | Signal arm? | ConfigSchema impl |
|---|---|---|---|---|
| `BindableU16` | `crates/tui-vfx-style/src/models/cls_bindable_u16.rs` | 251 | No | hand-written (line 107) |
| `BindableString` | `crates/tui-vfx-style/src/models/cls_bindable_string.rs` | 322 | No | hand-written (line 138) |
| `BindableValue` | `crates/tui-vfx-compositor/src/types/cls_bindable_value.rs` | 116 | Yes (`SignalOrFloat`) | `#[derive(ConfigSchema)]` |

All three have:

- `Literal(T) | Binding(String)` envelope (`BindableValue::Signal` is the same shape with an embedded `SignalOrFloat`).
- `evaluate(...) -> Option<T>` accessor (signature varies — see §Risks & gotchas).
- `From<T>` conversions for ergonomic literal construction.
- `Default` impl.
- Lenient `Repr` enum for bare-value deserialization fallback.

`BindableColor` and `BindableF32` are named in two design docs as the next additions (`docs/design/tui-vfx-binding-loopback.md:344`, `docs/design/tui-vfx-transform-context-implementation-plan.md:294`, and the signal-facade proposal). Each new Bindable currently re-pays ~250 LOC of identical scaffolding. The consolidation defines one `Bindable<T>` generic, retains the existing concrete names as type aliases, and lets the next two additions land as `pub type VfxBindableColor = VfxBindable<Color>;` one-line declarations.

This packet also resolves bundled finding **1.7.A** (`BindableValue` lives in `tui-vfx-compositor` while its siblings live in `tui-vfx-style` — the home-crate question must be answered before the consolidation can land).

## Scope

**In scope.**

- Define `VfxBindable<T, S>` (or `Bindable<T, S>`) in a single home crate. See §Open architectural questions for the home-crate and prefix decisions.
- Migrate the three concrete types to type aliases over the new generic, keeping the existing names so consumers do not edit imports.
- Move `BindableValue` from `tui-vfx-compositor` to the consolidated home crate (resolves 1.7.A).
- Preserve serde wire format on all three: bare-string for BindableString, bare-integer for BindableU16, bare-number for BindableValue, plus the tagged `{"literal": ...}` and `{"binding": ...}` shapes.
- Hand-written `ConfigSchema` impl on the generic with appropriate generic bounds (`T: ConfigSchema`, plus `S: ConfigSchema` if the Signal arm is generic).
- Keep BindableU16 and BindableString **without** the `Signal` arm in this packet (per Intention 23 rule 5 — three signatures is the minimum for confident consolidation; the Signal arm lands in the signal-facade phase δ that follows this packet).

**Out of scope.**

- The signal-facade Move 3 itself (adding `Signal` arm to BindableU16/String, adding `BindableF32` / `BindableColor`). That work is the immediate follow-on packet that depends on this one.
- Sweep finding 1.1.B (`Bindable*::evaluate` signature unification into a single `BindingContext` bundle). Per the sweep, that consolidation should bundle with the slice that lands `BindableColor` — i.e. it follows the signal-facade phase δ, not this packet. This packet keeps each `evaluate` signature unchanged so consumer call sites do not migrate.
- The `ConfigSchema` derive macro learning to emit generic bounds (sweep finding 1.9.A). The hand-written impls suffice for now.
- Any `tui-vfx-recipes` or `gt-design` consumer changes. Aliases preserve the names; downstream `use tui_vfx_style::models::BindableString;` continues to compile.

**Crates touched.**

- **Home crate (TBD per Open questions, recommended `tui-vfx-core`):** new `Bindable<T, S>` definition.
- **`tui-vfx-style`:** `BindableU16` and `BindableString` become aliases; the existing files move to `recyclebin/`.
- **`tui-vfx-compositor`:** `BindableValue` becomes an alias; the existing file moves to `recyclebin/`.
- **`tui-vfx-content`:** consumers (`cls_transform_context.rs`, `cls_mechanical_cycle_source.rs`, `fnc_apply_content_effect.rs`, `fnc_resolve_mechanical_cycle.rs`, `cls_image_layer.rs`) use the alias names — no edits required.
- **`tui-vfx-recipes`:** all consumers use the alias names — no edits required.

## Pre-work checklist

```bash
# Daemon health.
ofpf-status
ofpf-stats

# Read the source findings.
sed -n '140,186p' /usr/projects/tui-vfx/docs/design/tui-vfx-buy-once-architecture-sweep.md
sed -n '388,404p' /usr/projects/tui-vfx/docs/design/tui-vfx-buy-once-architecture-sweep.md
sed -n '617,642p' /usr/projects/tui-vfx/docs/design/tui-vfx-buy-once-architecture-sweep.md

# Read the signal-facade proposal — phase δ depends on this packet.
sed -n '195,225p' /usr/projects/tui-vfx/docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md

# Read related design docs.
grep -n "BindableColor\|BindableF32" /usr/projects/tui-vfx/docs/design/tui-vfx-binding-loopback.md
grep -n "BindableColor\|BindableF32" /usr/projects/tui-vfx/docs/design/tui-vfx-transform-context-implementation-plan.md

# Inspect every file the packet touches. ofpf-inspect mandatory before edits.
ofpf-inspect crates/tui-vfx-style/src/models/cls_bindable_u16.rs
ofpf-inspect crates/tui-vfx-style/src/models/cls_bindable_string.rs
ofpf-inspect crates/tui-vfx-compositor/src/types/cls_bindable_value.rs

# Check the ConfigSchema macro to confirm it does NOT yet emit generic bounds (informs Risks).
ofpf-extract crates/tui-vfx-core-macros/src/fnc_impl_config_schema.rs impl_config_schema

# Find all consumer call sites (large blast — narrow with the symbol-level refs).
ofpf-refs BindableU16
ofpf-refs BindableString
ofpf-refs BindableValue

# Confirm crate dependency graph permits the proposed home crate.
cat crates/tui-vfx-core/Cargo.toml | grep -A 10 "\[dependencies\]"
cat crates/tui-vfx-style/Cargo.toml | grep -A 10 "\[dependencies\]"
cat crates/tui-vfx-compositor/Cargo.toml | grep -A 10 "\[dependencies\]"
```

## Current-state audit

Captured 2026-04-26 from the librarian.

### The three concrete Bindable types

| Path | Role | LOC | Fan-in | Fan-out | Key callees |
|---|---|---|---|---|---|
| `crates/tui-vfx-style/src/models/cls_bindable_u16.rs` | unit | 251 | 1 (`models/mod.rs`) | 2 | `tui-vfx-core::schema`, `crate::traits::ShaderRuntimeParams` |
| `crates/tui-vfx-style/src/models/cls_bindable_string.rs` | unit | 322 | 1 (`models/mod.rs`) | 2 | `tui-vfx-core::schema`, `crate::traits::ShaderRuntimeParams` |
| `crates/tui-vfx-compositor/src/types/cls_bindable_value.rs` | unit | 116 | 1 (`types/mod.rs`) | 3 | `mixed_signals::{traits::SignalContext, types::SignalOrFloat}`, `tui_vfx_style::traits::ShaderRuntimeParams` |

### Three `evaluate` signatures (the asymmetry the consolidation preserves verbatim)

```rust
// crates/tui-vfx-style/src/models/cls_bindable_u16.rs:52
pub fn evaluate(&self, runtime_params: &ShaderRuntimeParams) -> Option<u16>;

// crates/tui-vfx-style/src/models/cls_bindable_string.rs:63
pub fn evaluate<'a>(&'a self, runtime_params: &'a ShaderRuntimeParams) -> Option<&'a str>;

// crates/tui-vfx-compositor/src/types/cls_bindable_value.rs:79
pub fn evaluate(
    &self,
    loop_t: f64,
    signal_ctx: &SignalContext,
    runtime_params: &ShaderRuntimeParams,
) -> Option<f32>;
```

This packet does **not** unify these signatures. Sweep finding 1.1.B (`BindingContext` bundle) is the consolidation that takes that on, and it's gated behind `BindableColor` landing per the sweep. Per Intention 23 rule 5 ("three is the threshold, not two"), the third concrete signature has not appeared.

### Consumer call-site counts

Real call-site evidence from `grep -rln <Type> crates/ --include="*.rs"` (excluding `tests/`):

| Type | Source files outside the type's own home | Largest in-file count | Notable consumers |
|---|---|---|---|
| `BindableU16` | 7 (`fnc_style_region_should_style.rs`, `fnc_style_region_deserialize.rs`, `fnc_style_region_schema.rs`, `fnc_style_region_resolved.rs`, `fnc_style_region_bounding_rect.rs`, `cls_style_region.rs`, `cls_transform_context.rs`) | several refs each in `cls_style_region.rs` | `StyleRegion::{Cell, RowRange, ColumnRange, Modulo}` — Phase 3b lift. |
| `BindableString` | 9 across `tui-vfx-style`, `tui-vfx-content`, `tui-vfx-recipes` | `cls_image_layer.rs`, `cls_mechanical_cycle_source.rs` | Font binding (Phase 6); image-name binding (1.3.A); odometer cycle source. |
| `BindableValue` | 4 in-crate (`cls_filter_spec.rs`, `cls_prepared_filter.rs`, `types/mod.rs`, `lib.rs`) plus inline counts: `cls_filter_spec.rs` uses it 24 times, `cls_prepared_filter.rs` uses it 22 times. | 24 (`cls_filter_spec.rs`) | Every filter-spec field that accepts a runtime-bound parameter. |

`ofpf-blast` on each Bindable type returns 168–181 dependents (guarded — most are inline references inside the trait-impl files). The aliases preserve all 175+ import paths; consumer migration is zero.

### The ConfigSchema macro reality

`crates/tui-vfx-core-macros/src/fnc_impl_config_schema.rs:14–42` (the entire macro body):

```rust
pub(crate) fn impl_config_schema(input: &DeriveInput) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    // ... emit `impl #impl_generics ConfigSchema for #ident #ty_generics #where_clause`
}
```

The macro **does** carry generic params from the type definition through to the impl block, but it does **not** add a `where T: ConfigSchema` bound. So `#[derive(ConfigSchema)]` on `pub enum Bindable<T> { Literal(T), Binding(String) }` generates `impl<T> ConfigSchema for Bindable<T>` with no `T: ConfigSchema` bound — the body would call `T::schema()` and fail to compile.

**Implication for this packet:** the generic Bindable cannot use `#[derive(ConfigSchema)]` directly. We must hand-write the impl with the explicit `T: ConfigSchema` bound (and `S: ConfigSchema` if the Signal arm carries a generic). This is the same pattern `Pool<T>` uses at `crates/tui-vfx-content/src/pool/cls_pool.rs:68` (sweep finding 1.2.B's parallel exception).

### Crate dependency graph (informs the home-crate decision)

- `tui-vfx-core` depends on: `tui-vfx-core-macros`, `mixed-signals`, `serde`, `serde_json`. **Already pulls `mixed-signals`.**
- `tui-vfx-style` depends on: `tui-vfx-types`, `tui-vfx-core`, `mixed-signals`, `tui-vfx-geometry`, `mcu-hct`, plus serde.
- `tui-vfx-compositor` depends on: `tui-vfx-style` (transitively `tui-vfx-core`).

Critical: **`tui-vfx-core` already depends on `mixed-signals`** (for the `mixed_signals_schema.rs` bridge that hosts `SignalOrFloat` / `SignalSpec` / `EasingType` schemas). This makes `tui-vfx-core` the cleanest home for the consolidated `Bindable<T>` — both `tui-vfx-style` (which currently houses BindableU16/String) and `tui-vfx-compositor` (which currently houses BindableValue) already depend on it. No new crate edges needed.

### `BindableValue` cross-crate home (1.7.A) — re-grounding

Per Intention 8 the `Vfx*` prefix tests are: (a) wire-format data, (b) errors from public APIs, (c) contract-producing traits. All three Bindables (1) deserialize from recipe JSON, (2) flow as field types across crate boundaries, (3) appear in `ConfigSchema` output. Test (a) is satisfied unambiguously. Per Intention 8, the consolidated type **gets the `Vfx` prefix**. The current names (`BindableU16`, `BindableString`, `BindableValue`) are out of compliance with Intention 8 today; the V3 cutover (Intention 10) is the right time to correct.

## Open architectural questions

These are the §5 questions and additional decisions a leader call gates this packet on. Each carries a recommended default the junior can apply if no other guidance arrives.

### Q1 (sweep §5 #2) — Home crate

Where does the consolidated generic live?

| Option | Trade-off |
|---|---|
| A — `tui-vfx-style` | One home for the family. Style would import `SignalContext`/`SignalOrFloat` (today only the compositor pulls them transitively). One extra import edge per `cargo build`. |
| B — new `tui-vfx-bindable` micro-crate | SSOT in a leaf crate both downstream depend on. New crate to maintain — Intention 24 rule 1 cost. Three concrete + two named is exactly the rule-of-three threshold; new crate is heavyweight. |
| C — `tui-vfx-core` | Both downstream crates already depend. `mixed-signals` already in core's deps. Bindable composes with the existing `SignalOrFloat` bridge schema in `mixed_signals_schema.rs`. |

**Recommended default: Option C (`tui-vfx-core`).** Per Intention 26 (SSOT) the consolidated home is the crate both downstream consumers already depend on. Per Intention 24 rule 6 ("watch for the rationalization chain") the type fits core's existing role — core already owns the schema-bridge types Bindable composes with. No new crate edge.

### Q2 (sweep §5 #2 sub-decision) — Vfx* prefix

Prefixed `VfxBindable<T>` versus unprefixed `Bindable<T>`?

**Recommended default: prefixed `VfxBindable<T>`.** Per Intention 8 three-test criterion the type is wire-format data crossing crate boundaries; it goes on the prefixed side. The current names violate Intention 8 today. The consolidation is the V3 cutover moment to correct (Intention 10).

The aliases preserve the legacy names: `pub type VfxBindableString = VfxBindable<String>;` plus `#[doc(hidden)] pub use VfxBindableString as BindableString;` (the same alias-migration mechanic V3's `Ra*` → `Vfx*` cutover already uses at `cls_ra_image_source.rs:55`).

### Q3 (sweep §6.2 follow-on) — Signal-arm shape

`BindableValue` has a Signal arm (`SignalOrFloat`); BindableU16 and BindableString do not (today). The signal-facade phase δ (`docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md:195`) will add Signal arms to all Bindables. Two candidate shapes:

| Option | Sketch | Trade-off |
|---|---|---|
| A — `Bindable<T, S = ()>` with phantom signal | `enum Bindable<T, S = ()> { Literal(T), Binding(String), Signal(S) }` — non-signal Bindables instantiate the unit-typed Signal arm but never construct it. | Single shape across all uses. The `Signal(())` arm is constructable in non-signal contexts (footgun). Default `S = ()` lets existing call sites omit the second param. |
| B — `Bindable<T, S = Never>` with uninhabited signal | Same shape, but `S` defaults to a `Never` (uninhabited) type from `std::convert::Infallible` or a project-defined zero-variant enum. The Signal arm is provably unconstructable when `S = Never`. | Type-system enforces "no Signal arm" for non-signal types. Slightly heavier cognitive cost (reader must understand `Never`). Match arms still need `Signal(_) => unreachable!()` (or use `match exhaustiveness with !` if stable). |

**Recommended default: Option B (`S = Never` via `std::convert::Infallible`).** Rationale per Intention 24 rule 4 (improve readability for the reader): unconstructable Signal-arm-when-not-applicable is a **provable** invariant via the type system rather than a footgun-by-discipline. The cognitive cost of `Infallible` is a one-line `where` clause in the type alias; the payoff is that downstream code does not need to defensively handle `Signal` for types that don't have signals. **Stop-and-ask trigger: this is the single most impactful design choice in the packet — surface to the user before committing if there's any uncertainty.**

If Option B is rejected, default to Option A (`S = ()`) with a runtime `unreachable!()` arm in the non-signal types' evaluators and a clippy-acknowledged comment per `feedback_no_landmines` (no `#[allow]` — fix the design or accept the `unreachable!`).

### Q4 — Migration order: additive aliases first vs. flag-day rename

Per Intention 23 rule 4 (additive migration, never breaking churn): introduce `VfxBindable<T>` alongside the three concrete types, alias the names, then retire the hand-rolled files in a single follow-on commit once the alias build is green.

**Recommended default: additive.** Sequence in §Step-by-step.

### Q5 — Hand-written ConfigSchema generic-bound expression

Hand-write the impl with explicit `T: ConfigSchema` bound (and `S: ConfigSchema` if Option A is chosen for Q3). The macro does not yet emit generic bounds (sweep 1.9.A). Use the same pattern as `cls_pool.rs:68` (the parallel `Pool<T>` exception).

**Recommended default: hand-written.** No macro change in this packet's scope.

### Stop-and-ask triggers

If the user has not pre-decided Q1, Q2, or Q3, **stop after pre-flight and surface to the user**. These are leader calls per the sweep §5. The other questions have defensible defaults the junior can apply.

## Step-by-step implementation plan

OFPF discipline: edit one file at a time, write tests first (red), implement (green), confirm clippy clean, commit interim work between phases.

### Phase 1 — Define `VfxBindable<T, S>` in the home crate

**Step 1.1.** Pre-edit: `ofpf-inspect crates/tui-vfx-core/src/lib.rs` to confirm module structure. `ofpf-inspect crates/tui-vfx-core/src/mixed_signals_schema.rs` to see the existing schema-bridge pattern.

**Step 1.2.** Write the failing test first. New file `crates/tui-vfx-core/src/bindable/test_cls_bindable.rs` (peer test per OFPF). Cover:

- `VfxBindable::Literal(42_u16)` constructs.
- `VfxBindable::Binding("key".into())` constructs.
- Default for `VfxBindable<u16, Never>` is `Literal(0)` (or document why default cannot be implemented generically and require type aliases to provide it).
- serde roundtrip: `{"literal": 42}` → `Literal(42)` → `{"literal": 42}`.
- serde lenient: bare `42` → `Literal(42)`.
- serde tagged binding: `{"binding": "x"}` → `Binding("x".to_string())`.
- ConfigSchema returns `SchemaNode::Enum` with two (or three, if Signal arm present) variants.
- `T = String` lifetime: the `From<&str>` and `From<String>` impls compile.

Run `cargo test -p tui-vfx-core bindable` — fails (file does not exist).

**Step 1.3.** Create `crates/tui-vfx-core/src/bindable/cls_bindable.rs` per the §Code snippets `VfxBindable` block. Include:

- The `enum VfxBindable<T, S>` definition.
- The `VfxBindableRepr<T, S>` lenient deserialization shape.
- `From<VfxBindableRepr<T, S>>` conversion.
- `From<T>` literal constructor (where `T: Sized`).
- `Default` for `VfxBindable<T, S> where T: Default` (Literal arm gets `T::default()`).
- Hand-written `ConfigSchema` impl gated on `T: ConfigSchema` (and `S: ConfigSchema` if Option A from Q3).
- Inline rustdoc on every public item.
- Public type aliases `VfxBindableU16`, `VfxBindableString`, `VfxBindableValue` defined in the same file or a sibling `mod.rs`.

Metadata envelope:

- `<DESC>Generic Bindable&lt;T, S&gt; for runtime-parameter and signal-driven recipe values. Three sibling concrete types (BindableU16, BindableString, BindableValue) collapsed into one shape; concrete types are aliases that downstream re-exports keep stable.</DESC>`
- `<VERS>VERSION: 0.1.0</VERS>`
- `<WCTX>Buy-once sweep finding 1.2.A — generalize the three Bindable* siblings into one VfxBindable&lt;T, S&gt;. Bundles 1.7.A by hosting the consolidated type in tui-vfx-core (both tui-vfx-style and tui-vfx-compositor already depend). Vfx* prefix per Intention 8.</WCTX>`
- `<CLOG>0.1.0: introduce VfxBindable&lt;T, S = Never&gt; (or S = ()), VfxBindableRepr lenient deserialization, hand-written ConfigSchema gated on T: ConfigSchema, Default gated on T: Default, From&lt;T&gt; literal constructor. Three type aliases: VfxBindableU16, VfxBindableString, VfxBindableValue.</CLOG>`

**Step 1.4.** Run `cargo test -p tui-vfx-core bindable` — green. Run `cargo build --workspace` — confirm the new type compiles in isolation. Existing types untouched at this point.

**Step 1.5.** Commit interim: `Add VfxBindable<T, S> to tui-vfx-core (1.2.A phase 1)`.

### Phase 2 — Replace `BindableU16` with an alias

**Step 2.1.** Pre-edit: `ofpf-inspect crates/tui-vfx-style/src/models/cls_bindable_u16.rs`. Confirm the existing tests inventory (10 tests at `cls_bindable_u16.rs:163–247`).

**Step 2.2.** New file `crates/tui-vfx-style/src/models/cls_bindable_u16.rs` (overwrite). Body:

```rust
// Module is now a thin alias surface; the canonical type lives in tui-vfx-core.
pub use tui_vfx_core::bindable::VfxBindableU16 as BindableU16;
```

Bump VERS to 0.2.0; CLOG entry: `0.2.0: BindableU16 becomes a re-export of tui_vfx_core::bindable::VfxBindableU16. Hand-rolled body retired to recyclebin.`

**Step 2.3.** Move the original (v0.1.0) body to `recyclebin/crates/tui-vfx-style/src/models/cls_bindable_u16.rs` per recyclebin protocol. Move the inline tests too (they're now redundant — the canonical tests live in `tui-vfx-core/src/bindable/test_cls_bindable.rs`).

**Step 2.4.** Run `cargo test -p tui-vfx-style` — every existing call site that imports `BindableU16` continues to compile (the alias is name-equivalent). Run `cargo build --workspace` — confirm no consumer broke.

**Step 2.5.** Commit interim: `Alias BindableU16 to VfxBindableU16 (1.2.A phase 2)`.

### Phase 3 — Replace `BindableString` with an alias

**Step 3.1–3.5.** Same pattern as Phase 2, applied to `cls_bindable_string.rs`. Lifetime detail: `BindableString::evaluate<'a>(&'a self, runtime_params: &'a ShaderRuntimeParams) -> Option<&'a str>` — the existing lifetime asymmetry must survive the move. Verify by running the existing 14 inline tests against the alias before retiring them.

### Phase 4 — Move `BindableValue` to the home crate, replace with alias

**Step 4.1.** Pre-edit: `ofpf-inspect crates/tui-vfx-compositor/src/types/cls_bindable_value.rs`. Confirm the in-crate test file at `crates/tui-vfx-compositor/tests/types/test_bindable_value.rs` (13 tests).

**Step 4.2.** Verify `VfxBindableValue` in `tui-vfx-core/src/bindable/` carries the Signal arm. Per Q3 default (Option B), `VfxBindableValue = VfxBindable<f32, SignalOrFloat>` — `S` is the Signal payload type, not Never.

**Step 4.3.** Replace `crates/tui-vfx-compositor/src/types/cls_bindable_value.rs` body with:

```rust
pub use tui_vfx_core::bindable::VfxBindableValue as BindableValue;
```

Bump VERS to 0.3.0; CLOG entry: `0.3.0: BindableValue becomes a re-export of tui_vfx_core::bindable::VfxBindableValue. Cross-crate home migrated per sweep 1.7.A. Hand-rolled body retired to recyclebin.`

**Step 4.4.** Move the original body to `recyclebin/crates/tui-vfx-compositor/src/types/cls_bindable_value.rs`. **Do not** move `tests/types/test_bindable_value.rs` — those tests stay as a regression surface against the alias (they exercise the public API that consumers depend on).

**Step 4.5.** Run `cargo test -p tui-vfx-compositor`, `cargo test -p tui-vfx-content`, `cargo test -p tui-vfx-recipes` — every consumer's tests must pass with no edits.

**Step 4.6.** Commit interim: `Move BindableValue to tui-vfx-core, alias from compositor (1.2.A phase 4 / 1.7.A)`.

### Phase 5 — Add `VfxBindableColor` and `VfxBindableF32` aliases (optional, prepares signal-facade phase δ)

**Step 5.1.** If the Q3 decision is Option A (`S = ()`) or B (`S = Never`), declare the next two named types as aliases now:

```rust
pub type VfxBindableF32 = VfxBindable<f32, Never>;     // No signal arm yet — phase δ adds it.
pub type VfxBindableColor = VfxBindable<Color, Never>; // Same — phase δ adds Signal(GradientLut).
```

**Step 5.2.** This step is **optional** — only land it if the user explicitly wants the named types reserved. Otherwise leave for the signal-facade phase δ packet.

### Phase 6 — Workspace verification + documentation

**Step 6.1.** Run the full §Verification commands block.

**Step 6.2.** Update `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md:20` to mark 1.2.A as done.

**Step 6.3.** If the consolidation lands, check whether `BindableU16`, `BindableString`, or `BindableValue` appear in `docs/templates/capabilities.toml`. Run `cargo xtask docs generate` and commit the regenerated `docs/CAPABILITIES_REFERENCE.md`.

**Step 6.4.** Final commit: `Phase 6: workspace clean (1.2.A complete)`.

## Code snippets

Two competing shapes; pick per Q3.

### Shape A — `VfxBindable<T, S = ()>`

```rust
use serde::{Deserialize, Serialize};
use tui_vfx_core::schema::{ConfigSchema, FieldMeta, SchemaField, SchemaNode, SchemaVariant};

/// A value that resolves either to a literal of type `T`, a named runtime
/// parameter (`Binding`), or a signal expression of type `S`. The `S`
/// type defaults to `()` for Bindables that do not carry signals.
///
/// # Type aliases
///
/// - [`VfxBindableU16`] — `VfxBindable<u16, ()>` for cell coordinates.
/// - [`VfxBindableString`] — `VfxBindable<String, ()>` for asset / font / locale names.
/// - [`VfxBindableValue`] — `VfxBindable<f32, SignalOrFloat>` for filter parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", from = "VfxBindableRepr<T, S>")]
pub enum VfxBindable<T, S = ()>
where
    T: Clone + PartialEq,
    S: Clone + PartialEq,
{
    /// A concrete literal value.
    Literal(T),
    /// A named runtime parameter looked up at evaluation time.
    Binding(String),
    /// A signal expression. For Bindables with `S = ()` this arm is
    /// constructable but should not be constructed; the type-aliased
    /// concrete types document whether their `S` is meaningful.
    Signal(S),
}

#[derive(Debug, Deserialize)]
#[serde(untagged, bound(deserialize = "T: Deserialize<'de>, S: Deserialize<'de>"))]
enum VfxBindableRepr<T, S> {
    Binding { binding: String },
    Literal { literal: T },
    Signal { signal: S },
    /// Bare value: lenient — only valid where T is unambiguous in JSON
    /// (numbers, strings, but NOT objects that could collide with the
    /// tagged forms).
    Bare(T),
}

impl<T, S> From<VfxBindableRepr<T, S>> for VfxBindable<T, S>
where
    T: Clone + PartialEq,
    S: Clone + PartialEq,
{
    fn from(repr: VfxBindableRepr<T, S>) -> Self {
        match repr {
            VfxBindableRepr::Binding { binding } => VfxBindable::Binding(binding),
            VfxBindableRepr::Literal { literal } => VfxBindable::Literal(literal),
            VfxBindableRepr::Signal { signal } => VfxBindable::Signal(signal),
            VfxBindableRepr::Bare(value) => VfxBindable::Literal(value),
        }
    }
}

impl<T, S> Default for VfxBindable<T, S>
where
    T: Default + Clone + PartialEq,
    S: Clone + PartialEq,
{
    fn default() -> Self { VfxBindable::Literal(T::default()) }
}

impl<T, S> From<T> for VfxBindable<T, S>
where
    T: Clone + PartialEq,
    S: Clone + PartialEq,
{
    fn from(value: T) -> Self { VfxBindable::Literal(value) }
}

// Hand-written ConfigSchema. The derive macro at
// crates/tui-vfx-core-macros/src/fnc_impl_config_schema.rs propagates
// generic params but does NOT emit `where T: ConfigSchema, S: ConfigSchema`
// bounds (sweep finding 1.9.A is the queued macro improvement). Until
// then this hand-written impl provides the bounds explicitly.
impl<T, S> ConfigSchema for VfxBindable<T, S>
where
    T: ConfigSchema + Clone + PartialEq,
    S: ConfigSchema + Clone + PartialEq,
{
    fn schema() -> SchemaNode {
        SchemaNode::Enum {
            name: "VfxBindable".to_string(),
            description: Some("Literal, runtime binding, or signal expression.".to_string()),
            json_name: None,
            tag_field: None,
            variants: vec![
                SchemaVariant::Tuple {
                    name: "Literal".to_string(),
                    description: Some("A concrete literal value".to_string()),
                    json_value: Some("literal".to_string()),
                    items: vec![SchemaField::new(
                        "value", T::schema(), FieldMeta::default(),
                    )],
                },
                SchemaVariant::Tuple {
                    name: "Binding".to_string(),
                    description: Some("A named runtime parameter".to_string()),
                    json_value: Some("binding".to_string()),
                    items: vec![SchemaField::new(
                        "name",
                        SchemaNode::Primitive { type_name: "String".to_string(), range: None },
                        FieldMeta::default(),
                    )],
                },
                SchemaVariant::Tuple {
                    name: "Signal".to_string(),
                    description: Some("A signal expression".to_string()),
                    json_value: Some("signal".to_string()),
                    items: vec![SchemaField::new(
                        "expression", S::schema(), FieldMeta::default(),
                    )],
                },
            ],
        }
    }
}

// Type aliases preserve the consumer-facing names.
pub type VfxBindableU16 = VfxBindable<u16, ()>;
pub type VfxBindableString = VfxBindable<String, ()>;

// VfxBindableValue uses SignalOrFloat as its Signal payload.
use mixed_signals::types::SignalOrFloat;
pub type VfxBindableValue = VfxBindable<f32, SignalOrFloat>;
```

### Shape B — `VfxBindable<T, S = Never>` (RECOMMENDED)

Same shape as Shape A, but `S` defaults to `std::convert::Infallible` (the standard uninhabited type). The Signal arm becomes provably unconstructable when `S = Never`.

```rust
// Replace the default in the enum declaration:
pub enum VfxBindable<T, S = std::convert::Infallible>
where
    T: Clone + PartialEq,
    S: Clone + PartialEq,
{
    Literal(T),
    Binding(String),
    Signal(S),  // For S = Infallible, this arm is unconstructable.
}

// Type aliases — non-signal types omit the second param to use the default:
pub type VfxBindableU16 = VfxBindable<u16>;     // S = Infallible
pub type VfxBindableString = VfxBindable<String>; // S = Infallible
pub type VfxBindableValue = VfxBindable<f32, SignalOrFloat>;

// Match arms on Bindables-without-signal use ! ("never type") exhaustiveness
// where stable, or `Signal(_) => unreachable!()` as the explicit fallback.
```

Per Q3 default the body is the same as Shape A modulo the type parameter default and the match-arm handling.

### Legacy aliases (the migration mechanic)

In `crates/tui-vfx-style/src/models/cls_bindable_u16.rs` after Phase 2:

```rust
//! Re-export of [`tui_vfx_core::bindable::VfxBindableU16`] under the
//! historical `BindableU16` name. The canonical type and tests live in
//! `tui-vfx-core`; this module exists to keep downstream import paths
//! (`use tui_vfx_style::models::BindableU16;`) compiling unchanged
//! during the sweep 1.2.A consolidation.

pub use tui_vfx_core::bindable::VfxBindableU16 as BindableU16;
```

Same pattern in `cls_bindable_string.rs` and `cls_bindable_value.rs`.

## Test plan

### Existing tests that must keep passing unchanged

- `cargo test -p tui-vfx-style cls_bindable_u16::tests` — 10 inline tests in the original file. After Phase 2 these tests are replaced by the canonical tests in `tui-vfx-core/src/bindable/test_cls_bindable.rs`, but every behavioral assertion (lenient bare-integer, tagged-literal, tagged-binding, default, From, evaluate-hit, evaluate-miss) must be preserved verbatim.
- `cargo test -p tui-vfx-style cls_bindable_string::tests` — 14 inline tests. Same preservation requirement.
- `cargo test -p tui-vfx-compositor test_bindable_value` — 13 tests at `crates/tui-vfx-compositor/tests/types/test_bindable_value.rs`. These stay in place; they exercise the public alias. After Phase 4 they pass against `VfxBindableValue` via the alias.
- `cargo test -p tui-vfx-style fnc_style_region` — every StyleRegion test that exercises BindableU16 fields.
- `cargo test -p tui-vfx-content` — every test exercising BindableString in the mechanical-cycle / content effect paths.
- `cargo test -p tui-vfx-recipes` — every recipe-load test that deserializes Bindable fields.

### New tests in `crates/tui-vfx-core/src/bindable/test_cls_bindable.rs`

Per OFPF every new `cls_` file gets a paired `test_*` file. The test file lives at `crates/tui-vfx-core/src/bindable/test_cls_bindable.rs` (or `crates/tui-vfx-core/tests/bindable/test_cls_bindable.rs` if the crate's test layout is segregated — check `ofpf-tests` for the convention).

Coverage:

- Construction: `Literal(T)`, `Binding(String)`, `Signal(S)` for both Shape A's `S = ()` and the SignalOrFloat case.
- Serde tagged shapes: `{"literal": T}`, `{"binding": "k"}`, `{"signal": S}` for each `T`/`S` combination.
- Serde lenient bare value: bare `T` deserializes to `Literal(T)` for `T = u16`, `T = String`, `T = f32`.
- Default: `VfxBindable::<u16, _>::default() == Literal(0)`, `VfxBindable::<String, _>::default() == Literal("".to_string())`.
- ConfigSchema: schema returns `SchemaNode::Enum` with the right variant count.
- Generic bound: `VfxBindable<MyType, ()>` compiles only when `MyType: ConfigSchema + Clone + PartialEq`.
- `Never` (Shape B only): `VfxBindable::<u16, Never>::Signal(_)` does not compile (negative test, asserted via `compile_fail` doctest).
- Lifetime preservation for `T = String`: `VfxBindable::<String>::Literal("x".into())` and the `From<&str>` impl produce the same value.

### TDD red→green

1. Phase 1 red: `cargo test -p tui-vfx-core bindable` fails.
2. Phase 1 green: write `cls_bindable.rs` per §Code snippets, tests pass.
3. Phase 2 red: nothing fails (alias is additive — the original file stays until verified). Move alias in, then move the original to recyclebin in the same step. `cargo test -p tui-vfx-style cls_bindable_u16` would now fail because tests retired with the file. The replacement test surface lives in `tui-vfx-core` already.
4. Phase 2 green: `cargo test -p tui-vfx-style && cargo test -p tui-vfx-core bindable && cargo build --workspace`.
5. Same loop for Phases 3 and 4.

### Per-phase test commands

```bash
# Phase 1
cargo test -p tui-vfx-core bindable

# Phase 2
cargo test -p tui-vfx-style
cargo test -p tui-vfx-core bindable
cargo build --workspace

# Phase 3
cargo test -p tui-vfx-style
cargo test -p tui-vfx-content
cargo test -p tui-vfx-recipes
cargo build --workspace

# Phase 4
cargo test -p tui-vfx-compositor
cargo test -p tui-vfx-content
cargo test -p tui-vfx-recipes
cargo build --workspace

# Phase 6 (final)
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Acceptance criteria

- [ ] `crates/tui-vfx-core/src/bindable/cls_bindable.rs` exists with `VfxBindable<T, S>` per Q3 decision (Shape A or B).
- [ ] `crates/tui-vfx-core/src/bindable/test_cls_bindable.rs` (peer test) covers construction, serde shapes, default, ConfigSchema, lifetime, generic bounds.
- [ ] `BindableU16` is a re-export of `VfxBindableU16` from `tui-vfx-core`. The original hand-rolled body is in `recyclebin/crates/tui-vfx-style/src/models/cls_bindable_u16.rs`.
- [ ] `BindableString` is a re-export of `VfxBindableString`. Original body in `recyclebin/`.
- [ ] `BindableValue` is a re-export of `VfxBindableValue`. Original body in `recyclebin/crates/tui-vfx-compositor/src/types/cls_bindable_value.rs`. Cross-crate home migrated per 1.7.A.
- [ ] **Serde wire format preserved on all three concrete types:**
    - `BindableU16`: bare integer (e.g. `42`), `{"literal": 42}`, `{"binding": "k"}` all parse.
    - `BindableString`: bare string (e.g. `"x"`), `{"literal": "x"}`, `{"binding": "k"}` all parse.
    - `BindableValue`: bare number (e.g. `0.5`), `{"static": 0.5}` (the existing bare SignalOrFloat fallback), `{"signal": {"static": 0.5}}`, `{"binding": "k"}` all parse.
- [ ] `evaluate` signatures **unchanged** on all three types. Sweep 1.1.B (the signature unification) is explicitly out of scope.
- [ ] Public type names `BindableU16`, `BindableString`, `BindableValue` continue to be importable from their original modules. No consumer edits across `tui-vfx-content`, `tui-vfx-recipes`, `gt-design`.
- [ ] `cargo build --workspace` succeeds with zero new warnings.
- [ ] `cargo test --workspace` green.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` clean.
- [ ] No `#[allow]` suppressions added.
- [ ] No inert schema fields. The new generic's ConfigSchema impl emits all three variants (`Literal`, `Binding`, plus `Signal` if S is non-`Never`).
- [ ] `tui-vfx-core` does not gain new external dependencies (already has `mixed-signals`, `serde`, `serde_json`).
- [ ] Rustdoc improved on every public item touched per `feedback_rustdoc_when_editing`. The new `VfxBindable<T, S>` carries the consolidation rationale comment Intention 23 rule 6 requires.
- [ ] `cargo doc --no-deps` succeeds with no broken intra-doc links.
- [ ] If any consolidated type appears in `docs/templates/capabilities.toml`, `cargo xtask docs generate` regenerates the manifest and the regen is committed.
- [ ] **Vfx*-prefix decision (Q2) recorded in the home-crate file's CLOG.** If the user accepted the recommended `VfxBindable<T>` name, the file's CLOG names Intention 8 explicitly.
- [ ] **Home-crate decision (Q1) reflected in `Cargo.toml` deps.** If the user picked C (`tui-vfx-core`), no Cargo.toml edits beyond the metadata bump are needed.

## Verification commands

```bash
# Build clean across the workspace.
cargo build --workspace

# Per-crate tests.
cargo test -p tui-vfx-core
cargo test -p tui-vfx-style
cargo test -p tui-vfx-compositor
cargo test -p tui-vfx-content
cargo test -p tui-vfx-recipes
cargo test --workspace

# Clippy with denied warnings.
cargo clippy --workspace --all-targets -- -D warnings

# Rustdoc clean.
cargo doc --no-deps

# Capability manifest regen.
cargo xtask docs generate

# Recyclebin moves intact.
ls /usr/projects/tui-vfx/recyclebin/crates/tui-vfx-style/src/models/ | grep cls_bindable
ls /usr/projects/tui-vfx/recyclebin/crates/tui-vfx-compositor/src/types/ | grep cls_bindable

# Confirm consumer imports still resolve. Spot-check the largest:
grep -n "use tui_vfx_style::models::Bindable" /usr/projects/tui-vfx/crates/tui-vfx-style/src/models/cls_style_region.rs
grep -n "BindableValue" /usr/projects/tui-vfx/crates/tui-vfx-compositor/src/types/cls_filter_spec.rs | head -5
```

## Rollback plan

The packet is structured as five interim commits (one per phase). If any phase reveals a deal-breaker:

1. Stop. Do not continue to the next phase.
2. `git revert <phase-commit-hash>` to back out the most recent phase. Earlier phases stay landed (they are additive — Phase 1 just adds a new type, Phases 2–4 alias to it).
3. If the deal-breaker is in Phase 1 (the generic itself), `git revert` Phase 1 too. Move the new file to `recyclebin/crates/tui-vfx-core/src/bindable/` per the recyclebin protocol.
4. `cargo build --workspace` to confirm the restored state compiles.
5. File a finding in the sweep doc capturing what blocked the consolidation, then surface to the user. Common blockers to anticipate: (a) a consumer pattern-matches on the variant body in a way that the generic cannot type-check; (b) the ConfigSchema generic-bound expression triggers a recursion in `cargo doc`; (c) the lenient bare-value deserialization for one of the three types collides with a tagged shape in serde's untagged walk.

The recyclebin protocol from `~/.claude/CLAUDE.md` mandates moves over deletes — every retired file goes to `recyclebin/` mirroring its original path, never `rm`.

## Risks & gotchas

- **Serde wire-format preservation is the load-bearing constraint.** Each of the three concrete types deserializes from a non-trivial input shape: BindableU16 accepts bare integers; BindableString accepts bare strings; BindableValue accepts bare numbers AND bare `SignalOrFloat` objects (e.g. `{"static": 0.5}`). The generic `VfxBindableRepr<T, S>` must reproduce all three. The `serde(untagged)` walk visits variants in declaration order — tagged forms (`{"literal": ...}`, `{"binding": ...}`, `{"signal": ...}`) must come before the bare fallback `Bare(T)` so JSON objects with the tag keys hit the right arm. This is the same constraint the existing per-type `Repr` enums satisfy (`cls_bindable_string.rs:113–116` documents the rule). Verify with the existing tests, especially `roundtrip_bare_input_normalizes_to_tagged_output`.

- **ConfigSchema derive-macro generic-specialization risk.** The macro at `crates/tui-vfx-core-macros/src/fnc_impl_config_schema.rs` propagates generic params but does NOT emit `where T: ConfigSchema` bounds. Hand-write the impl on the generic; gate `T: ConfigSchema` (and `S: ConfigSchema` if non-Never). This is the same exception `Pool<T>` uses at `cls_pool.rs:68`. If a future contributor tries `#[derive(ConfigSchema)] on VfxBindable<T, S>`, the build will fail in a confusing way. Document the exception in the file's CLOG.

- **The BindableValue Signal-arm asymmetry is real and intentional.** BindableValue's `Signal` variant carries a `SignalOrFloat`; BindableU16 and BindableString today carry no Signal arm. Per Q3's recommended Option B (`S = Never`), the type system enforces this — a `VfxBindableU16::Signal(_)` does not compile. If Option A (`S = ()`) is chosen instead, the arm is constructable but should not be — flag this in code review and include a runtime `unreachable!()` arm on the non-signal types' evaluators. **Do not silence with `#[allow(unreachable_patterns)]`** per `feedback_no_landmines`.

- **Vfx*-prefix decision per Intention 8.** The current names (`BindableU16`, `BindableString`, `BindableValue`) are out of compliance with Intention 8 (wire-format types crossing crate boundaries get the `Vfx` prefix). The recommended `VfxBindable<T>` corrects this; the legacy aliases keep downstream code compiling. If the leader rejects the prefix change, document in the file's CLOG that the unprefixed name was a deliberate carve-out and update `steering/INTENTIONS.md` to match — do not leave the inconsistency undocumented.

- **`evaluate` signature drift across the family is preserved by this packet.** The three different signatures (no-time, lifetime-borrowing, signal-time-and-context) survive. Sweep finding 1.1.B is the unification packet that follows; it should bundle with the slice that lands BindableColor (per the sweep, three-signature minimum). Do not pre-emptively unify `evaluate` here — the signature change ripples into every call site, which is the opposite of this packet's "additive migration, no consumer edits" contract.

- **The `mod.rs` re-export surface in `tui-vfx-style/src/models/` and `tui-vfx-compositor/src/types/`.** Both modules currently re-export the concrete Bindable types. Confirm the re-export `pub use cls_bindable_u16::BindableU16;` continues to resolve to the alias (which itself resolves to `VfxBindableU16`). `cargo build --workspace` is the verification. If a `pub use` path breaks, the alias declaration in the original file may need to be `pub use ... as BindableU16;` rather than `pub use VfxBindableU16 as BindableU16;` — verify with `cargo expand` if uncertain.

- **`tui-vfx-core` already depends on `mixed-signals`** (per Cargo.toml inspection). This is what makes Option C viable for the home-crate decision. If the dependency were absent, hosting `VfxBindableValue` (which carries `SignalOrFloat`) in `tui-vfx-core` would require adding `mixed-signals` as a new dep. Verify the dependency before committing to Option C — if a future Cargo.toml change drops `mixed-signals` from `tui-vfx-core`, Option C becomes invalid.

## Sequencing note

- This packet **precedes** the signal-facade Move 3 (`docs/design/tui-vfx-mixed-signals-recipe-surface-proposal.md:195`) — phase δ adds Signal arms to BindableU16/String and adds BindableF32/Color. Doing those additions on three parallel hand-rolled types is five files of churn versus one type. This packet is the prerequisite.
- This packet **bundles** sweep finding 1.7.A (BindableValue cross-crate home). 1.7.A is explicitly tagged "Bundle into Finding 1.2.A" in the sweep summary table.
- This packet **does not** unify `evaluate` signatures (sweep 1.1.B). That unification follows the signal-facade phase δ once BindableColor lands.
- This packet **does not** depend on sweep findings 1.1.A (already DONE — VfxCellContext) or 1.2.B (already DONE — Pool<T>). It is independent of both.
- The handoff doc `docs/design/tui-vfx-2026-04-26-handoff-outstanding.md:20` should be updated to mark 1.2.A and 1.7.A done in the same commit that lands Phase 6.

<!-- <FILE>docs/design/tui-vfx-2026-04-26-packet-1.2.A-bindable-generic.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
