<!-- <FILE>docs/design/tui-vfx-buy-once-architecture-sweep.md</FILE> - <DESC>Repository-wide buy-once/cry-once architectural sweep — surfaces conversion points where a single deliberate change today would prevent repeated trait/signature/schema churn later. Modeled after the TransformContext bundle exemplar (Slice 6.6). Findings are evidence-based via ofpf-* tooling and filtered through Intentions 23 (rule of three) and 24 (changes earn their place).</DESC> -->
<!-- <VERS>VERSION: 1.2.0</VERS> -->
<!-- <WCTX>Architectural sweep companion to the TransformContext bundle. Identifies the next round of trait/signature/schema conversion points the leader can triage into work packets. Sweep only; no code changes.</WCTX> -->
<!-- <CLOG>1.2.0: add §8 (impact analysis across three axes: chainable-effects mechanism, V3 feature surfaces, performance) covering all 12 findings; renumber prior §8 Appendix to §9. Verdicts: zero chainable-effects risk across all 12 findings (StepInput/HintRef/ParamValue are V3 design surfaces, not yet code; Top 3 findings touch trait spatial parameters, struct generics, and recipe field types — orthogonal to inter-step hint propagation); Top 3 V3-surface impact is mostly enabling (no capability lost, several gained); performance count Positive=2, Neutral=8, Negligible=2, Negative=0. §1–§7 unchanged.</CLOG> -->

# tui-vfx buy-once architecture sweep

> **Status:** sweep + recommendations only. Implementation-ready for the "do now" tier.
>
> **Scope:** repository-wide across the eleven `tui-vfx-*` crates plus `tui-vfx-recipes` and `mixed-signals` siblings. Surface findings only — no code in this doc changes.
>
> **Filter:** every finding satisfies Intention 23's rule-of-three (≥2 in-tree instances + ≥1 credible third) and Intention 24's "earn their place" gate (real value, visible problem at current scale).

---

## 0. Method

Findings were produced by direct `ofpf-*` queries against the live librarian graph (`graph_loaded: true`, `definition_count: 6356`). The full transcript of probes used:

| Query | Purpose |
|---|---|
| `ofpf-status` | Confirm daemon health |
| `ofpf-orientation --root /usr/projects/tui-vfx` | Hub/core/orchestrator inventory |
| `ofpf-loc 250 --limit 30` | Files near or above OFPF size limits |
| `ofpf-cycles` | Dependency-cycle inventory |
| `ofpf-defs Bindable` | Enumerate Bindable family |
| `ofpf-defs Registry` | Enumerate registry types |
| `ofpf-defs Spec --limit 50` | Enumerate `*Spec` tagged-enum family |
| `ofpf-defs Source --limit 30` | Enumerate `*Source` enums |
| `ofpf-defs ShaderRuntimeParams` | Locate runtime-param type |
| `ofpf-defs ImagePool` | Locate pool primitive |
| `ofpf-content "impl ConfigSchema for"` | Hand-written ConfigSchema impls |
| `ofpf-content "ConfigSchema for"` | All ConfigSchema impls (incl. fully-qualified) |
| `ofpf-inspect crates/tui-vfx-style/src/models/cls_bindable_string.rs` | Bindable neighborhood |
| `ofpf-inspect crates/tui-vfx-compositor/src/types/cls_bindable_value.rs` | Bindable neighborhood |
| `ofpf-inspect crates/tui-vfx-compositor/src/types/cls_filter_spec.rs` | FilterSpec neighborhood (2193 LOC) |
| `ofpf-inspect crates/tui-vfx-compositor/src/pipeline/cls_prepared_filter.rs` | PreparedFilter (1941 LOC) |
| `ofpf-inspect crates/tui-vfx-content/src/transformers/cls_split_flap.rs` | SplitFlap (1642 LOC) |
| `ofpf-inspect crates/tui-vfx-content/src/assets/cls_asset_registry.rs` | AssetRegistry surface |
| `ofpf-blast crates/tui-vfx-style/src/models/cls_bindable_string.rs` | BindableString blast (168 results) |
| `ofpf-blast crates/tui-vfx-style/src/models/cls_bindable_u16.rs` | BindableU16 blast (174 results) |
| `ofpf-blast crates/tui-vfx-compositor/src/types/cls_bindable_value.rs` | BindableValue blast (~50 files) |
| `ofpf-blast crates/tui-vfx-compositor/src/traits/filter.rs` | Filter trait blast |
| `ofpf-blast crates/tui-vfx-compositor/src/traits/mask.rs` | Mask trait blast |
| `ofpf-blast crates/tui-vfx-compositor/src/traits/sampler.rs` | Sampler trait blast |
| `ofpf-blast crates/tui-vfx-style/src/traits/cls_shader_context.rs` | ShaderContext blast (117 results) |
| `ofpf-blast crates/tui-vfx-content/src/pool/cls_image_pool.rs` | ImagePool blast (5 files) |
| `ofpf-blast crates/tui-vfx-content/src/fonts/cls_font_registry.rs` | FontRegistry blast (7 files) |

Plus targeted `grep -rn` against `crates/`, `recyclebin/`, and the recipes / mixed-signals roots for trait, error, and pool patterns.

**Risk-tier scale.** S = single file or small bounded family. M = bounded family across 1–2 crates with mechanical migration. L = trait or schema change with broad ripple across many impl files. XL = cross-repo coordination.

**Recommendation tiers.** *Do now* = pressure is here, blast radius known, action implementation-ready. *Next slice* = pressure mounting, queue behind one current piece. *Wait for third trigger* = at the watch-it threshold, defer until pain becomes acute. *Leave alone* = looks consolidatable but isn't (false positive).

---

## 1. Findings by category

### 1.1 Trait signature churn risk

#### Finding 1.1.A — Filter / Mask / Sampler share an unbundled spatial context

**Category:** trait signature churn (the canonical exemplar pattern).

**What.** Three sibling traits in `tui-vfx-compositor/src/traits/` carry identical or near-identical positional parameters that are conceptually one "spatial context":

```rust
// crates/tui-vfx-compositor/src/traits/filter.rs:93
fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, t: f64);

// crates/tui-vfx-compositor/src/traits/mask.rs:7
fn is_visible(&self, x: u16, y: u16, w: u16, h: u16, progress: f64) -> bool;

// crates/tui-vfx-compositor/src/traits/sampler.rs:26-33
fn sample(&self, dest_x: u16, dest_y: u16, width: u16, height: u16, t: f64) -> Option<(u16, u16)>;
```

The fourth sibling, `StyleShader::style_at(&self, ctx: &ShaderContext, base: Style) -> Style` at `crates/tui-vfx-style/src/traits/tr_style_shader.rs:48`, **already bundles** these into `ShaderContext` (defined at `crates/tui-vfx-style/src/traits/cls_shader_context.rs:289`) with `local_x`, `local_y`, `width`, `height`, `screen_x`, `screen_y`, `t`, `phase`, plus runtime-params and role-map accessors. ShaderContext is the proven pattern; the other three are the unbundled cousins.

**Why now.** Three signals say the pressure is here, not speculative:
1. **The metadata envelopes record prior churn.** `traits/filter.rs` is at v3.0.0 with `<CLOG>BREAKING CHANGE: Added spatial context parameters`. `traits/sampler.rs` is at v2.0.0 with `<CLOG>BREAKING CHANGE: Added width and height spatial parameters`. Each historical revision was a coordinated rewrite of every impl file. The next addition (role-map awareness, runtime-params injection, screen offset, phase information — all already on `ShaderContext`) re-pays the same cost.
2. **The Filter family is the largest impl set in the repo.** `ofpf-blast crates/tui-vfx-compositor/src/traits/filter.rs` returned 168 dependents (the guard fired at 11 KB). Filters: 31 files in `crates/tui-vfx-compositor/src/filters/`. Masks: 13 files in `crates/tui-vfx-compositor/src/masks/`. Samplers: 12 files in `crates/tui-vfx-compositor/src/samplers/`. Total: 56 trait-impl files across 3 traits.
3. **The TransformContext exemplar is mid-flight on a parallel trait.** `TextTransformer::transform` is being bundled in Slice 6.6 because adding `&ShaderRuntimeParams` would be the third positional churn. Filter / Mask / Sampler already absorbed two churns each and have the same forward pressure (runtime params, role-map, screen offset).

**Existing instances (rule-of-three).**
1. `Filter::apply` — 31 impls churned twice already (v3.0.0).
2. `Mask::is_visible` — 13 impls.
3. `Sampler::sample` — 12 impls churned twice already (v2.0.0).
4. (Already-resolved sibling) `StyleShader::style_at(ctx: &ShaderContext, ...)` — the proof-of-pattern.

**Credible third trigger.** Already past three. The next addition is `&ShaderRuntimeParams` for runtime-bound effect parameters (already piped to ShaderContext at line 1.3.0 of `cls_shader_context.rs` `<CLOG>`) — Filter/Mask/Sampler will need it for the same recipe-bindable surface that BindableValue exposes inside FilterSpec.

**Estimated blast radius.** L. ~56 impl files across three traits, plus the pipeline dispatch and the test corpus. Migration is mechanical (replace tuple of params with a single `&ctx`); each impl can ignore the bundle with `_ctx`. Tests are the larger piece because pipeline tests construct positional arguments by hand.

**Recommended action.** **Do now (after Slice 6.6 lands)** — the same author who lands TransformContext is freshly inside the bundle pattern. Sequence: (a) introduce `PipelineCellContext` (or reuse/extend `ShaderContext`) at `tui-vfx-compositor/src/traits/`, (b) migrate Mask first (smallest impl set, no historical churn pressure), (c) Sampler (already at v2.0.0; one bump to v3.0.0), (d) Filter last (largest impl set; gate behind passing CI on the prior two).

**Trade-offs.** Bundling forces all filters to import the context type; today they import `tui_vfx_types::Cell` only. A `&PipelineCellContext` parameter is more bytes per call than positional `u16`s — but Filter is already a `dyn` virtual call, the per-frame cost is dominated by the dispatch. Bundling does **not** introduce vtable indirection that wasn't already there.

**Open question.** Does this bundle reuse `ShaderContext` directly, or define a sibling type (`PipelineCellContext`) in `tui-vfx-compositor/src/traits/`? Reusing leaks `tui-vfx-style` semantics (role-map, runtime-params) into the compositor; defining a sibling duplicates the eight fields. The leader should decide.

---

#### Finding 1.1.B — `Bindable*::evaluate` signatures already drift across the family

**Category:** trait signature churn (cousin to 1.1.A on a smaller surface).

**What.** Three Bindable types ship `evaluate()` with three different signatures:

```rust
// crates/tui-vfx-style/src/models/cls_bindable_u16.rs:52
pub fn evaluate(&self, runtime_params: &ShaderRuntimeParams) -> Option<u16>;

// crates/tui-vfx-style/src/models/cls_bindable_string.rs:63
pub fn evaluate<'a>(&'a self, runtime_params: &'a ShaderRuntimeParams) -> Option<&'a str>;

// crates/tui-vfx-compositor/src/types/cls_bindable_value.rs:79
pub fn evaluate(&self, loop_t: f64, signal_ctx: &SignalContext, runtime_params: &ShaderRuntimeParams) -> Option<f32>;
```

**Why now.** The asymmetry is structural: BindableValue has a `Signal` arm so it needs `(loop_t, signal_ctx)`; BindableU16 and BindableString today only have `Literal | Binding` arms. The binding-loopback design (`docs/design/tui-vfx-binding-loopback.md:336–344`) explicitly anticipates "Future bindable types (e.g. `BindableColor`, `BindableString`)" growing **Signal arms** for animated bindings. When BindableU16 grows a Signal arm — already discussed as the natural next step in `cls_bindable_u16.rs` rustdoc — every caller of `BindableU16::evaluate(&runtime_params)` will need to pass `loop_t` and `signal_ctx` too. That signature change ripples into every FilterSpec/MaskSpec field that uses BindableU16.

**Existing instances.**
1. `BindableU16::evaluate(runtime_params)` — narrow signature.
2. `BindableString::evaluate(runtime_params)` — narrow signature with lifetime.
3. `BindableValue::evaluate(loop_t, signal_ctx, runtime_params)` — wide signature.

**Credible third trigger.** `docs/design/tui-vfx-binding-loopback.md:344` and `transform-context-implementation-plan.md:294` both name `BindableColor` / `BindableF32` as the next types. By Intention 23 the threshold has been crossed; the question is just *when* to consolidate.

**Estimated blast radius.** M. Today: ~30 BindableValue use sites in `cls_filter_spec.rs`, scattered BindableU16 / BindableString sites. After consolidation: every call site passes a single `&BindingContext { loop_t, signal_ctx, runtime_params }` (or whatever the bundle is named) regardless of whether the underlying Bindable variant uses signals.

**Recommended action.** **Wait for third trigger.** This is at the watch-it threshold but moving now means renaming a stable surface (`evaluate(&runtime_params)` is documented in 8 places). Better to do this *together with* Finding 1.2.A (generalize `Bindable<T>`) when BindableColor or BindableU16-with-Signal lands. The same `BindingContext` bundle should be defined as part of that work.

**Trade-offs.** Consolidating today saves one churn but commits to a `BindingContext` name and shape before the parameter set has stabilized. Waiting one more trigger lets the third use case constrain the design.

---

### 1.2 Sibling type proliferation

#### Finding 1.2.A — `Bindable*` family wants `Bindable<T>` generalization

**Category:** sibling type proliferation (rule-of-three already crossed).

**What.** Three parallel hand-rolled Bindable types share an identical structural pattern:

| Type | File | LOC | Signal arm? | ConfigSchema impl |
|---|---|---|---|---|
| `BindableU16` | `crates/tui-vfx-style/src/models/cls_bindable_u16.rs` | 250 | No | hand-written (line 107) |
| `BindableString` | `crates/tui-vfx-style/src/models/cls_bindable_string.rs` | 322 | No | hand-written (line 138) |
| `BindableValue` | `crates/tui-vfx-compositor/src/types/cls_bindable_value.rs` | 116 | Yes (`SignalOrFloat`) | (none — `BindableValue` is unprefixed and lives in compositor) |

All three have:
- `Literal(T) | Binding(String)` envelope (BindableValue's `Signal` is the same shape with an embedded SignalOrFloat).
- `evaluate(...) -> Option<T>` (signature varies — see 1.1.B).
- `From<T>` for ergonomic literal construction.
- `Default` impl.
- Lenient bare-string deserialization for `Binding` keys.
- Hand-written `ConfigSchema` (or schema-less) impl.

**Why now.** Three siblings exist, and `BindableColor` is named in two design docs as the next addition. Each new Bindable copies ~250 LOC of identical structural code. Each lives at a different crate boundary (style vs. compositor) so there's no shared abstraction even available to inherit.

**Existing instances.**
1. `BindableU16` (style)
2. `BindableString` (style)
3. `BindableValue` (compositor)

**Credible third trigger.** `BindableColor` is named in:
- `docs/design/tui-vfx-binding-loopback.md:344`
- `docs/design/tui-vfx-transform-context-implementation-plan.md:294`

`BindableF32` is named in the same Transform Context plan. Two named-but-not-built types.

**Estimated blast radius.** L. The blast covers `tui-vfx-style/src/models/`, `tui-vfx-compositor/src/types/`, and every consumer: `cls_filter_spec.rs` (~30 BindableValue field uses), the StyleEffect family, FilterSpec defaults, plus the Bindable-aware test helpers. `ofpf-blast` on BindableU16 returns 174 dependents (preview only, guard fired); BindableString returns 168.

**Recommended action.** **Next slice.** This is at the rule-of-three threshold and the design docs name the fourth and fifth instances. But the ergonomics of `Bindable<T>` are non-trivial — serde shape preservation is the load-bearing constraint (each variant must keep `{"binding": "name"}` and the lenient bare-string fallback). The `ConfigSchema` derive macro at `crates/tui-vfx-core-macros/src/fnc_impl_config_schema.rs` may need a generic specialization.

Sequence:
1. Land Finding 1.1.A (TransformContext sibling work) so transformer signatures are stable.
2. Define `Bindable<T>` in `tui-vfx-style` (or a new `tui-vfx-bindable` micro-crate to avoid the style-vs-compositor split) with the canonical `Literal(T) | Binding(String) | Signal(...)` shape, gated by trait bounds.
3. Migrate the three concrete types to type aliases over `Bindable<T>` (additive — Intention 23 rule 4: never breaking churn during consolidation).
4. Land `BindableColor` and `BindableF32` as type aliases over the new generic.

**Trade-offs.** Generalization commits to one variant set across all uses. BindableValue's `Signal` arm is heavier than BindableU16/String need today. A `Bindable<T, S = ()>` with a phantom signal-arm parameter handles this but raises the cognitive cost of reading the type. The simpler escape hatch: keep `Signal` always present but allow `S = Never` for types that don't need it. Both options should be vetted against the ConfigSchema derive.

**Risk callout.** `BindableValue` lives in `tui-vfx-compositor` and lacks the `Vfx*` prefix; the others live in `tui-vfx-style`. Per Intention 8, the consolidated type must determine its prefix policy: if it crosses crate boundaries as wire-format data, it gets `Vfx`; if it's an internal helper, it doesn't. The current names violate Intention 8 either way (BindableValue should be VfxBindableValue, or all three should be unprefixed sibling helpers).

---

#### Finding 1.2.B — Five `*Pool` types have identical generic-Pool<T> shape

**Category:** sibling type proliferation (textbook generic candidate).

**What.** `crates/tui-vfx-content/src/pool/` contains five parallel pool types with structurally identical APIs:

```rust
// All five follow this shape exactly:
pub struct XxxPool { pub items: Vec<T>, pub policy: PoolPolicy }
impl XxxPool {
    pub fn new(items: Vec<T>, policy: PoolPolicy) -> Self;
    pub fn pick(&self) -> Option<&T>;
    pub fn is_empty(&self) -> bool;
}
```

| Pool | T | File |
|---|---|---|
| `ImagePool` | `String` | `cls_image_pool.rs` |
| `TextPool` | `String` | `cls_text_pool.rs` |
| `FontPool` | `String` | `cls_font_pool.rs` |
| `EffectPool` | `ContentEffect` | `cls_effect_pool.rs` |
| `PresetPool` | `Preset` | `cls_preset_pool.rs` |

`PoolPolicy` is shared: `crates/tui-vfx-content/src/pool/col_pool_policy.rs:18`, used identically in all five.

**Why now.** Five sibling instances of an identical pattern, four of them already named with the same prefix. Adding a sixth pool type (e.g., `ColorPool`, `ScopePool` for V3 scope variation, `SubstitutionPool`) re-pays the same boilerplate. Three of the five are `Pool<String>` — they're not even distinct in their type parameter today.

**Existing instances.** Five (rule-of-three is comfortably exceeded).

**Credible third trigger.** Already at five. The leader could fairly take the position that "five identical pools" is itself the trigger — Intention 24 says abstractions earn their place when the generalization is genuine, and five-way duplication is genuine.

**Estimated blast radius.** S. Per `ofpf-blast crates/tui-vfx-content/src/pool/cls_image_pool.rs`: 5 dependents — `cls_lib.rs`, `pool/mod.rs`, the workspace re-export, and `xtask`. The other four pools have similarly small dependent sets (each is consumed by a single feature path in `tui-vfx-recipes` or `gt-design`). Migration: define `Pool<T>`, alias the five concrete types, retire the 5 hand-rolled files to `recyclebin/`. ~250 LOC removed across the five files.

**Recommended action.** **Do now.** Smallest blast radius of any L-or-larger consolidation. Concrete sequence:
1. Define `pub struct Pool<T> { pub items: Vec<T>, pub policy: PoolPolicy }` at `crates/tui-vfx-content/src/pool/cls_pool.rs` with the four-method API.
2. Add type aliases: `pub type ImagePool = Pool<String>;` etc.
3. Move the five hand-rolled files to `recyclebin/crates/tui-vfx-content/src/pool/`.
4. The aliases preserve the names consumers import, so no external migration is required.

**Trade-offs.** Aliases mean the rustdoc landing page for `ImagePool` becomes terser (shows up as a re-export of `Pool<String>`). The current per-pool rustdoc carries domain-specific examples that would migrate into `Pool<T>` examples or onto each alias (`#[doc(...)]` on `pub type`). The five concrete docstrings should be reviewed to see whether they carry distinct value worth preserving as alias-level rustdoc.

**Caveat.** `PresetPool` and `EffectPool` may have helper methods I didn't fully enumerate; the consolidation must preserve those. `ofpf-inspect` on each pool file will surface them.

---

#### Finding 1.2.C — `FontRegistry` and `AssetRegistry` share an identical name→bytes resolver shape

**Category:** sibling type proliferation (pre-three; watch-it threshold).

**What.** Two registries in `tui-vfx-content` expose nearly identical APIs:

| Method | `AssetRegistry` (line) | `FontRegistry` (line) |
|---|---|---|
| `new() -> Self` | 46 | 49 |
| `insert(name, payload) -> Option<...>` | 54 | 60 |
| `set_default(name) -> bool` | 63 | 73 |
| `default_name() -> Option<&str>` / `&str` | 77 | 86 |
| `default_bytes()` / `default_table()` | 83 | 91 |
| `resolve(name) -> Option<...>` | 97 | 110 |
| `resolve_or_default(name) -> ...` | 112 | 121 |
| `entries() -> impl Iterator<...>` | 117 | 126 |

Storage is `BTreeMap<String, Vec<u8>>` (assets) vs. `BTreeMap<String, FontGlyphTable>` (fonts). Default-name sentinel pattern is identical (Intention 36: `default_font` for fonts; `DEFAULT_LOGO_SENTINEL` constant for assets at `cls_asset_registry.rs:29`).

**Why now.** Two registries today; the cycle plan v0.7.0 (line 22) explicitly poses the open V3 scene-layer question of whether `AssetRegistry`, `ImagePool`, and the verbal `AssetMap` contract should "consolidate, compose, or leave parallel". A third resolver (rocketsplash bytes? scope→cell list? color palette name→RGB?) is plausible-but-unconcrete — the cycle plan calls this "V3 scene-layer territory and warrants a coordinated session with sibling when scheduled."

**Existing instances.**
1. `AssetRegistry`
2. `FontRegistry`
3. (Schema sidekick) `SchemaRegistry` at `tui-vfx-core/src/schema/cls_schema_registry.rs:12` — same shape but indexed by `TypeId`, different domain.

**Credible third trigger.** Cycle plan v0.7.0 line 22 names this open question. Not yet committed; not yet speculative.

**Estimated blast radius.** S. ~7 dependents per registry. A shared `NamedRegistry<T>` would absorb both with type aliases, but the migration is small enough that consolidation today doesn't pay back unless the third instance lands soon.

**Recommended action.** **Wait for third trigger.** SchemaRegistry uses `TypeId` keys, not names — it's a different shape. Until the V3 scene-layer composition decision in the cycle plan is made, two-of-a-kind doesn't justify abstraction (Intention 23 rule 5: "three is the threshold, not two"). Park this finding in the V3 scene-layer doc.

**Trade-offs.** None today. The cost of waiting is paying the registry-pattern setup cost twice if a third resolver lands quickly.

---

### 1.3 Schema field-by-field bindable lift

#### Finding 1.3.A — `VfxImageSource.image_name: String` should be `BindableString`

**Category:** schema field-by-field bindable lift (already named as actionable in cycle plan).

**What.** The field `image_name: String` on `VfxImageSource` (file path: `tui-vfx-recipes/src/recipe_schema/scene/cls_ra_image_source.rs` per cycle plan v0.7.0 line 22; the file lives in tui-vfx-recipes) is today a plain `String`. Per cycle plan Phase 7.schema (line 22):

> One-line schema change in `tui-vfx-recipes/src/recipe_schema/scene/cls_ra_image_source.rs` plus a thin runtime adjustment to evaluate the `BindableString` in the scene-layer composer. **Independent of L2** (the `{"binding": ...}` reference shape is V3 canon) and **independent of host-resolver consolidation**. Lands the *authoring* surface for asset binding without committing to the runtime end-to-end.

**Why now.** Phase 7.schema is explicitly marked **"Independently actionable"** in the cycle plan v0.7.0 — the schema lift is decoupled from the harder runtime/resolver questions. The author who lands the BindableString migration is freshly in the binding-loopback context; doing the lift later means reloading that context.

**Existing instances.**
1. `MechanicalContentSource::Preset.font: Option<BindableString>` (Phase 6, shipped).
2. The procedural / V3 step payload bindings (V3 canon `{"binding": "name"}` syntax already accepted everywhere).
3. `VfxImageSource.image_name` — the missing instance.

**Credible third trigger.** The instance is already in-flight; this finding is just queueing it as a schedulable work packet rather than letting it sit indefinitely deferred.

**Estimated blast radius.** S. One-line schema change + thin runtime adjustment. Per cycle plan: contained within `tui-vfx-recipes/src/recipe_schema/scene/cls_ra_image_source.rs` and the scene-layer composer.

**Recommended action.** **Do now.** This is the smallest finding in the whole sweep and has a documented action contract in the cycle plan. The `_bindable` filename / metadata-tag convention from Intention 38 applies to any new debug recipe that exercises it.

**Trade-offs.** Lifting `image_name` to `BindableString` without delivering the runtime end-to-end (Phase 7.bytes — deferred per cycle plan) means recipes can *declare* the binding but not consume it dynamically yet. That's acceptable per Intention 28's pattern-compatibility framing — the schema is correctly typed for the validator, the runtime just hasn't grown the resolver path. The literal form (`image_name: "splash_logo"`) keeps working unchanged.

---

#### Finding 1.3.B — `VfxImageSource.aspect: VfxImageAspect` and similar enum-typed-but-bindable fields

**Category:** schema field-by-field bindable lift (speculative — flagged as watch-it).

**What.** Beyond image_name, several recipe-schema enum fields could plausibly want runtime bindings (e.g., `VfxImageAspect`, scope variant tags, easing types). The cycle plan v0.7.0 line 22 mentions `aspect: VfxImageAspect` as part of the deferred Phase 7.bytes work.

**Why now.** Not now. There's no in-tree recipe asking for "bind the aspect at runtime" or "bind the easing at runtime." Out-of-band binding for enum-typed fields would require a new `BindableEnum<T>` shape with discriminator-aware deserialization.

**Existing instances.** Zero in-tree. This is speculation.

**Recommended action.** **Leave alone (speculation).** Per Intention 24 rule 1: "Real value, not abstract principle." No driver, no finding. Surface this as a watch item in V3 cross-family-coverage if the question recurs.

---

### 1.4 Error type proliferation

#### Finding 1.4.A — `tui-vfx-recipes` has 13+ parallel `*Error` enums in the v3 loader pipeline

**Category:** error type consolidation (false-positive caution; see "Things that look like buy-once but aren't").

**What.** `grep -rn "pub enum.*Error" /usr/projects/tui-vfx-recipes/src/v3` returns 13 distinct error enums, each scoped to one stage of the loader pipeline:

| Error | Stage |
|---|---|
| `ParseV3DocumentError` | parse |
| `LoadV3DocumentError` | load (wraps Parse) |
| `NormalizeError` | normalize |
| `LoadV3NormalizedError` | load+normalize (wraps Load + Normalize) |
| `ValidateError` | validate |
| `CompileV3DocumentError` | compile (wraps Load + Normalize + Validate) |
| `LoadV3CompiledError` | load+normalize+validate+compile |
| `BuildCompositionSpecError` | post-compile |
| `RenderCompiledPlanError` | render |
| `TemplateResolutionError` / `TemplatePathError` | template substitution |
| `ExpandVariantsError` | variants |
| `FromValueV3NormalizedError` | from-value |

**Why now.** This *looks* like 13-way proliferation, which would scream "consolidate." But each enum is a thin `#[derive(Error)]` wrapper that uses `#[from]` to compose prior stages. The pattern is idiomatic Rust (per Intention 24 rule 2: "Move toward ecosystem norms — when the idiomatic ratatui, Rust, or serde answer already covers the use case, use it"). Each error preserves stage-precise diagnostic paths for the validator/probe tooling.

**Existing instances.** 13.

**Credible third trigger.** N/A — this is in the false-positive bin.

**Estimated blast radius.** L if pursued; pursuing is not recommended.

**Recommended action.** **Leave alone.** This is the "looks like buy-once but isn't" case (cross-listed in §3 below). Consolidating these would lose stage-precise error provenance that the validator and probe tooling depend on, in exchange for one fewer enum name. Per Intention 24's rule 4: "Improve readability, measured at the reader side." A single mega-enum does not improve readability for stage-error diagnostics.

**Trade-offs.** Listed for completeness. The non-action saves the existing diagnostic precision.

---

### 1.5 Resolver / registry duplication

Already covered: Finding 1.2.C (FontRegistry / AssetRegistry).

---

### 1.6 Tagged-enum size pressure

#### Finding 1.6.A — `cls_filter_spec.rs` is 2193 LOC (10× hard limit)

**Category:** OFPF size violation; tagged-enum accumulation.

**What.** The `FilterSpec` enum file at `crates/tui-vfx-compositor/src/types/cls_filter_spec.rs` is **2193 LOC** — over 10× the OFPF `cls_` hard limit of 200 LOC. The file contains:
- `FilterSpec` enum (line 346): 17 nested enums, 87 functions, 6 methods, 2 structs (per `ofpf-inspect` `def_counts`).
- 168 dependents (per `ofpf-blast`).
- ~30 inline `BindableValue` field uses (the seam exposed in Finding 1.1.B).

**Why now.** Hard limit is 200 LOC; current is 2193. The file aggregates every filter variant as a tagged-enum branch. Each variant is its own struct-like body inline — adding a 32nd filter is a 60-line edit to this file rather than a new file.

**Existing instances.** Three files exhibit the same pattern:

| File | LOC | Hard limit | Multiple |
|---|---|---|---|
| `cls_filter_spec.rs` | 2193 | 200 | 11× |
| `cls_prepared_filter.rs` | 1941 | 200 | 10× |
| `cls_split_flap.rs` | 1642 | 120 (`fnc_`/`cls_` mixed) | 13× |

**Credible third trigger.** Already at three, all in the same crate. Filter family is at 31 variants and growing (V3 plans 5+ new variants per `tui-vfx-v3-cross-family-coverage-plan.md`).

**Estimated blast radius.** L. Splitting `FilterSpec` into per-variant files (one `cls_filter_spec_<name>.rs` per variant) is mechanically straightforward but touches every Filter impl that pattern-matches on `FilterSpec`. Per `ofpf-blast`: 168 dependents. Migration: extract each variant into its own file with a typed inner struct, then re-export through a thin top-level `FilterSpec` enum that holds variant-typed payloads.

**Recommended action.** **Next slice.** The file has been over-limit for releases; the pressure is real but not novel. Plan: schedule a focused split-up phase in the V3 cross-family work, and coordinate with the parallel split for `cls_split_flap.rs` (the cycle plan Phase 4 already names this as separate territory). Don't bundle this into a buy-once finding alongside trait changes — different blast radii, different timing.

**Trade-offs.** Per-variant files add file-tree depth; current single-file pattern is searchable with one grep. The per-variant pattern is what `cls_mask_spec.rs` (781 LOC) and `cls_sampler_spec.rs` (756 LOC) do today and serves them well — they're under the soft limit because they have fewer variants, but the structure scales.

---

### 1.7 Cross-crate seams

#### Finding 1.7.A — `BindableValue` lives in compositor while `BindableU16` / `BindableString` live in style

**Category:** cross-crate seam misalignment (small but Vfx*-prefix-relevant).

**What.** Two of the three Bindables live in `tui-vfx-style/src/models/`; the third (`BindableValue`) lives in `tui-vfx-compositor/src/types/`. Per Intention 8 the wire-format `Vfx*` prefix rule applies; today none of the three carry it, but BindableValue is the only one that lives in a downstream crate. If the consolidation of Finding 1.2.A produces a unified `Bindable<T>` (or rename pass at V3 cutover per Intention 10), the home crate must be decided.

**Why now.** Choosing the home crate is a prerequisite to Finding 1.2.A. Best-fit answer is `tui-vfx-style` because `BindableU16` and `BindableString` already live there and depend only on `tui-vfx-core` for the schema derive. Moving `BindableValue` upstream would let the compositor's FilterSpec consume the unified type without circular dependency.

**Existing instances.** Two-vs-one split (style:2, compositor:1).

**Credible third trigger.** The next BindableX type's home is undecided until this is resolved.

**Estimated blast radius.** S–M. Moving BindableValue from compositor to style requires the style crate to know about `SignalOrFloat` (currently in `tui-vfx-core/src/mixed_signals_schema.rs` — see `impl ConfigSchema for SignalOrFloat` at line 32). Style already depends on core, so the import is legal.

**Recommended action.** **Bundle into Finding 1.2.A** (next slice). Don't move BindableValue as a standalone change — the move only earns its place when consolidating the Bindable family.

**Trade-offs.** None standalone. Bundle implies the trade-offs of Finding 1.2.A.

---

### 1.8 Dependency cycles

#### Finding 1.8.A — `tui-vfx-core/src/schema/` has multiple internal cycles

**Category:** dependency cycle (small, internal, low priority).

**What.** `ofpf-cycles` returns several cycles in `crates/tui-vfx-core/src/schema/`:

```
cls_json_writer.rs → types.rs → fnc_schema_node_to_json_pretty.rs → cls_json_writer.rs
cls_json_writer.rs → types.rs → fnc_schema_node_to_json_pretty.rs → fnc_json_write_schema_node.rs → cls_json_writer.rs
... (longer chains)
```

These are all intra-crate cycles between the schema-writing helpers.

**Why now.** Intra-crate cycles compile fine in Rust (the language allows them at module level) but they're a refactoring smell — the boundaries between `cls_json_writer.rs`, `types.rs`, and the `fnc_*` helpers aren't clean.

**Existing instances.** Multiple cycles, all in the same module (`tui-vfx-core/src/schema/`).

**Credible third trigger.** Internal — the cycles exist; they're already the trigger.

**Estimated blast radius.** S. Localized to one module of one crate.

**Recommended action.** **Wait for third trigger.** No external consumer is broken by these cycles; the schema-writer refactor would be a pure-quality improvement. Park this finding until somebody's already touching the schema-writer code for a feature reason.

**Trade-offs.** None.

---

### 1.9 Dispatcher / hand-written ConfigSchema wins

#### Finding 1.9.A — Hand-written `impl ConfigSchema for X` proliferates parallel to derive

**Category:** boilerplate that the derive already covers (Intention 25 territory).

**What.** `ofpf-content "ConfigSchema for"` returns 8 hand-written `impl ConfigSchema for X` blocks across the workspace, plus the macro itself at `crates/tui-vfx-core-macros/src/fnc_impl_config_schema.rs:29`:

| Type | File:Line | Note |
|---|---|---|
| `String` | `tui-vfx-core/src/schema/mod.rs:69` | primitive — must be hand-written |
| `&str` | `tui-vfx-core/src/schema/mod.rs:77` | primitive — must be hand-written |
| `SignalOrFloat` | `tui-vfx-core/src/mixed_signals_schema.rs:32` | bridge — must be hand-written |
| `SignalSpec` | `tui-vfx-core/src/mixed_signals_schema.rs:82` | bridge — must be hand-written |
| `EasingType` | `tui-vfx-core/src/mixed_signals_schema.rs:467` | bridge |
| `PathType` | `tui-vfx-geometry/src/types/path_type.rs:201` | could be derived if enum shape allows |
| `RoleTag` | `tui-vfx-types/src/role_tag.rs:124` | could be derived |
| `Color` | `tui-vfx-types/src/color.rs:27` | wire-format primitive — must be hand-written |
| `BindableString` | `tui-vfx-style/src/models/cls_bindable_string.rs:138` | could be derived if generic Bindable<T> shipped |
| `BindableU16` | `tui-vfx-style/src/models/cls_bindable_u16.rs:107` | could be derived if generic Bindable<T> shipped |
| `StyleRegion` | `tui-vfx-style/src/models/fnc_style_region_schema.rs:21` | could be derived |
| `CursorShaderPrimary/Trail` | `tui-vfx-style/src/models/cls_cursor_shader.rs:43,81` | bridge variants |
| `VfxCursorPrimary/Trail` | `tui-vfx-style/src/models/v3/enum_vfx_cursor_behavior.rs:37,73` | V3 cursor lowering — could be derived |

**Why now.** Per Intention 12A: V3 schema-bearing types must support `ConfigSchema` derivation or have an explicit reason not to. Of the 13+ hand-written impls, ~6 (PathType, RoleTag, BindableString, BindableU16, StyleRegion, VfxCursor*) plausibly *could* be derived if either the macro or the type shape were adjusted. Doing the audit case-by-case is cheaper than later when V3 grows more types.

**Existing instances.** 13+ hand-written.

**Credible third trigger.** Every new V3 schema-bearing type adds another (or violates Intention 12A).

**Estimated blast radius.** Variable per impl. S each, but Intention 25 says: "build the check now while context is fresh."

**Recommended action.** **Next slice — but as a validation infrastructure win, not a refactor.** Per Intention 25 rule 5: "the smallest intervention that delivers full coverage." The right move is a `cargo xtask` check that diffs hand-written `ConfigSchema` impls against what `#[derive(ConfigSchema)]` would have produced and flags drift. That mechanizes Intention 12A's "explicit reason not to derive" rule. Each remaining hand-written impl gains a one-line comment justifying why it isn't derived.

**Trade-offs.** Building the differ is moderate effort. Cheaper alternative: a one-time audit that ships an inline `// derive-justification: ...` comment on each hand-written impl, plus a clippy/grep gate that fails if a new `impl ConfigSchema for X` appears without that comment.

---

## 2. Top 3 recommendations

Ranked by leverage (blast-radius prevented per unit of work-now):

### Top 1 — Bundle Filter / Mask / Sampler trait spatial-context parameters (Finding 1.1.A)

**Rationale.** Same shape as the in-flight TransformContext exemplar. ~56 impl files across three traits already absorbed two churns each; the next addition (runtime-params / role-map / phase) is queued. The author landing TransformContext is in the freshest possible context for this work.

**Next step.** After Slice 6.6 lands, scope a "PipelineCellContext" slice. Decision: reuse `ShaderContext` directly (cross-crate semantic leak) vs. define a sibling type in `tui-vfx-compositor/src/traits/`. Migrate Mask first (smallest set, no historical churn). Then Sampler. Then Filter.

### Top 2 — Generalize the five `*Pool` types into `Pool<T>` (Finding 1.2.B)

**Rationale.** Five sibling instances of an identical pattern, three of them already `Pool<String>`. Smallest blast radius of any L-or-larger consolidation in the sweep (5 dependents per pool). Type aliases preserve all current import names. ~250 LOC removed.

**Next step.** Define `Pool<T>` at `crates/tui-vfx-content/src/pool/cls_pool.rs`. Add type aliases `pub type ImagePool = Pool<String>;` etc. Move five hand-rolled files to `recyclebin/`. Verify no helper methods are lost. Estimated effort: half a day.

### Top 3 — Lift `VfxImageSource.image_name` to `BindableString` (Finding 1.3.A)

**Rationale.** Already documented as actionable in cycle plan v0.7.0 line 22. One-line schema change plus thin runtime adjustment. Independent of L2 and host-resolver consolidation. The smallest finding in the sweep — pure throughput win.

**Next step.** Edit `tui-vfx-recipes/src/recipe_schema/scene/cls_ra_image_source.rs`. Update scene-layer composer to call `image_name.evaluate(&runtime_params).unwrap_or(literal_default)`. Add Intention 38 `_bindable` debug recipe.

---

## 3. Things that LOOK like buy-once but aren't

These are findings the leader might otherwise pursue. Each has a specific reason it does *not* earn consolidation.

### 3.1 Loader-error chain (Finding 1.4.A)

13+ `*Error` enums in `tui-vfx-recipes/src/v3/`. Looks like aggressive proliferation. **Isn't.** Each enum is a thin `#[derive(thiserror::Error)]` wrapper that uses `#[from]` to compose prior stages. The pattern is idiomatic Rust per Intention 24 rule 2; consolidating to a mega-enum would lose stage-precise diagnostic provenance that the validator and probe tooling depend on. **Leave alone.**

### 3.2 `BindableEnum<T>` for runtime-bindable enum fields (Finding 1.3.B)

The cycle plan mentions `aspect: VfxImageAspect` as a deferred Phase 7.bytes concern. Looks like an obvious next bindable-lift. **Isn't.** No in-tree recipe asks for "bind the aspect at runtime." Per Intention 24 rule 1: real value, not abstract principle. Without a driver, building `BindableEnum<T>` is speculative complexity. **Leave alone until driver appears.**

### 3.3 `FontRegistry` / `AssetRegistry` consolidation today (Finding 1.2.C)

Two registries with identical APIs. Looks ready to consolidate. **Isn't yet.** Two-of-a-kind is the watch-it threshold; three is the trigger. The third resolver question is openly named in cycle plan v0.7.0 as V3 scene-layer territory. Consolidating today commits to a shape before the third use case constrains it. **Wait for the V3 scene-layer design session.**

### 3.4 Aggressive `cls_filter_spec.rs` split-up (Finding 1.6.A)

The 2193-LOC enum file looks like an OFPF emergency. **Real but not buy-once.** The pressure is steady, not climbing toward a breaking-change cliff. The right venue is the V3 cross-family work where a focused split-up phase already makes sense. Don't bundle this with the trait-bundle work — different blast radii, different sequencing.

### 3.5 BindableValue::evaluate signature unification (Finding 1.1.B)

Three different `evaluate` signatures across the Bindable family. Looks like a churn-prevention win. **Premature.** Renaming a stable surface today before BindableU16 grows a Signal arm commits to a `BindingContext` shape before the parameter set has stabilized. Bundle into Finding 1.2.A's `Bindable<T>` work when it lands.

---

## 4. Findings summary

| # | Finding | Risk | Recommendation |
|---|---|---|---|
| 1.1.A | Filter / Mask / Sampler spatial-context bundle | L | Do now (after Slice 6.6) |
| 1.1.B | Bindable*::evaluate signature unification | M | Wait for third trigger |
| 1.2.A | Bindable<T> generalization | L | Next slice |
| 1.2.B | Pool<T> generalization | S | **Do now** |
| 1.2.C | FontRegistry / AssetRegistry merge | S | Wait for third trigger |
| 1.3.A | VfxImageSource.image_name → BindableString | S | **Do now** |
| 1.3.B | BindableEnum<T> for enum fields | — | Leave alone (speculative) |
| 1.4.A | Loader Error enum consolidation | — | Leave alone (false positive) |
| 1.6.A | FilterSpec / PreparedFilter / SplitFlap split-up | L | Next slice |
| 1.7.A | BindableValue cross-crate home | S | Bundle into 1.2.A |
| 1.8.A | tui-vfx-core/schema cycles | S | Wait for third trigger |
| 1.9.A | Hand-written ConfigSchema audit | M | Next slice (as validation infra) |

**By risk tier:** S=6, M=2, L=4.

**By recommendation:** Do now=3 (incl. 1.1.A which queues behind Slice 6.6); Next slice=3; Wait for third=3; Leave alone=2; Bundle into another finding=1.

---

## 5. Open architectural questions for human judgment

These findings surface decisions that need a leader call before implementation:

1. **Finding 1.1.A — bundle home.** Reuse `ShaderContext` directly (cross-crate semantic leak from style into compositor) vs. define `PipelineCellContext` as a sibling type? Affects Intention 1 (grid-first compositor independence).

2. **Finding 1.2.A — Bindable<T> home crate.** Move BindableValue upstream to `tui-vfx-style` to join its siblings, or keep it in `tui-vfx-compositor`? Decision is prerequisite to consolidation. Tied to the Vfx*-prefix ruling per Intention 8 — should the consolidated type be `VfxBindable<T>` (it crosses crate boundaries as wire-format data) or a local helper?

3. **Finding 1.2.B — alias rustdoc preservation.** The five hand-rolled pools each carry domain-specific rustdoc examples. When migrating to `Pool<T>` aliases, do those examples migrate to alias-level `#[doc(...)]` comments, or get distilled into one `Pool<T>` example with cross-references? Affects Intention 12 (rustdoc coverage as engineering contract).

4. **Finding 1.6.A — split-up timing.** Should `cls_filter_spec.rs` (2193 LOC) be split before or after the V3 cross-family work that adds 5+ more variants? Splitting first makes the variant additions cleaner; splitting after lets the additions inform the split structure.

5. **Finding 1.9.A — derive vs. hand-write policy.** Should V3 enforce "hand-written `impl ConfigSchema` requires a justification comment," or is a softer "audit periodically" stance enough? Validation-infrastructure win level (Intention 25) depends on the answer.

---

## 6. Options and recommendations for the open questions

The five questions in §5 are answered here in option form. Every option cites either a re-run `ofpf-*` probe (logged in §0 and re-checked for v1.1.0) or a direct file path with line numbers. Recommendations cite Intentions explicitly. Where a recommendation surfaces a new sub-question, it is logged as **Open follow-on**.

The probes re-run for v1.1.0 (incremental to §0):

| Re-run probe | Confirmation |
|---|---|
| `wc -l crates/tui-vfx-content/src/pool/cls_*.rs` | ImagePool 110, TextPool 144, FontPool 102, EffectPool 129, PresetPool 278; total 763 LOC across the five sibling files |
| `grep -nE "^pub struct\|^impl " crates/tui-vfx-content/src/pool/cls_*.rs` | All five share the `{ items, policy }` shape; only `TextPool::new` carries a per-call `sanitize()` step (`cls_text_pool.rs:51`) |
| `Read crates/tui-vfx-style/src/traits/cls_shader_context.rs:280–320` | `ShaderContext` already carries `local_x/y, width, height, screen_x/y, t, phase, runtime_params: Arc<ShaderRuntimeParams>, roles: Arc<RoleMap>` — eight fields the unbundled trio does not |
| `Read crates/tui-vfx-recipes/src/recipe_schema/scene/cls_ra_image_source.rs` | `image_name: String` at line 29; `tint: Option<Color>` and `aspect: VfxImageAspect` are the other two fields |
| `Read crates/tui-vfx-style/src/models/cls_bindable_string.rs:55–90` | `BindableString::evaluate(&ShaderRuntimeParams) -> Option<&str>` — the existing call shape that the lift inherits |
| `grep -rnE "VfxImageSource\|image_name" crates/` | `image_name` already used as an asset-map key in `cls_preset_pool.rs:49` (V3 already references the future binding form via `Option<String>`) |

---

### 6.1 Question 1 — Filter / Mask / Sampler bundle home

**The decision.** Reuse `ShaderContext` directly (cross-crate semantic leak from `tui-vfx-style` into `tui-vfx-compositor`) versus define a sibling type `PipelineCellContext` in `tui-vfx-compositor/src/traits/`?

**Option A — Reuse `ShaderContext` directly.**
- Concretely: `fn apply(&self, cell: &mut Cell, ctx: &ShaderContext)` in all three traits, importing `tui_vfx_style::ShaderContext`.
- *What this buys us:* zero duplication of the eight-field bundle. Filters/masks/samplers get role-map awareness and `runtime_params` for free, matching `StyleShader::style_at` exactly. The fourth-sibling pattern (`StyleShader`) becomes the canonical pattern.
- *What we lose:* `tui-vfx-compositor` gains a non-trivial dependency on `tui-vfx-style`'s shader semantics. Today the compositor traits depend only on `tui_vfx_types::Cell`. This violates Intention 1's "grid-first, ecosystem-agnostic compositor" framing if the compositor traits start importing style-layer types.

**Option B — Define `VfxPipelineCellContext` in `tui-vfx-compositor/src/traits/`.**
- Concretely: a new `cls_pipeline_cell_context.rs` carrying the same eight fields. `ShaderContext` and `VfxPipelineCellContext` then have a `From<&VfxPipelineCellContext> for ShaderContext` (or an upstream conversion in style) at the seam.
- *What this buys us:* preserves Intention 1 — compositor traits don't import style. Trait surface is owned by the crate that owns the trait. The `Vfx` prefix carries the wire-relevant intent (per Intention 8 the type is contract-producing, used by every Filter/Mask/Sampler impl across crate boundaries).
- *What we lose:* eight-field shape duplicated in two places. Risk of divergence between `ShaderContext` and `VfxPipelineCellContext` once one grows a field the other does not.

**Option C — Hoist a shared `cls_cell_context.rs` into `tui-vfx-types` (the lowest-common crate).**
- Concretely: define `VfxCellContext` in `tui-vfx-types` (where `Cell` already lives) and let both `ShaderContext` and the compositor traits *embed* or *alias* it.
- *What this buys us:* SSOT (Intention 26 rule 1 — "shared semantics live once") with no upstream-imports-downstream cycle. `tui-vfx-types` already owns `Cell`, so adding the spatial context next to it is the architecturally cleanest home.
- *What we lose:* moves `runtime_params: Arc<ShaderRuntimeParams>` and `roles: Arc<RoleMap>` either *out of `ShaderContext`* or duplicates them across two structs — both `RoleMap` and `ShaderRuntimeParams` currently live in `tui-vfx-style`. The hoist either pulls them into `tui-vfx-types` (heavier types crate) or keeps `ShaderContext` as a *superset* (compositor uses a leaner `VfxCellContext`, style adds runtime+roles on top).

**Recommended: Option C, with the leaner-base / superset shape.** Per Intention 1 (grid-first compositor independence) the compositor must not import `tui-vfx-style`. Per Intention 26 rule 1 (shared semantics live once) we should not duplicate the eight-field shape across two crates. Per Intention 8 (`Vfx*` prefix on contract-producing wire-format types) the new type — used by every Filter/Mask/Sampler impl that consumers ship — gets the `Vfx` prefix. The natural shape is `VfxCellContext { local_x, local_y, width, height, screen_x, screen_y, t, phase }` in `tui-vfx-types`; `ShaderContext` then becomes `{ cell: VfxCellContext, runtime_params, roles }` (composition over inheritance). Filter/Mask/Sampler take `&VfxCellContext` and don't see `runtime_params` until 1.2.A's binding consolidation gives them a reason to.

**What this buys us / what we lose.** Buys: Intention 1 preserved, Intention 26 SSOT, Intention 8 prefix-correct, future role-map and runtime-params additions are *additive on the style side only*. Loses: `ShaderContext` becomes a wrapper rather than the canonical bundle, requiring a small migration of existing field accesses (`ctx.local_x` → `ctx.cell.local_x` or via `Deref`).

**Open follow-on.** Should `ShaderContext` implement `Deref<Target = VfxCellContext>` to keep existing `ctx.local_x` accesses unbroken, or does the explicit `ctx.cell.local_x` path improve readability per Intention 24 rule 4? Decide at slice-start.

---

### 6.2 Question 2 — Bindable<T> home crate and Vfx*-prefix ruling

**The decision.** Where does the consolidated `Bindable<T>` live, and does it carry the `Vfx*` prefix?

**Re-grounding evidence.** `cls_bindable_string.rs:63` and `cls_bindable_u16.rs:52` live in `tui-vfx-style/src/models/`; `cls_bindable_value.rs:79` lives in `tui-vfx-compositor/src/types/`. Per Intention 8 the prefix tests are: (a) wire-format data, (b) errors from public APIs, (c) contract-producing traits. All three Bindable types (1) deserialize from recipe JSON, (2) flow as `FilterSpec` field types across the recipe→pipeline boundary, (3) appear in `ConfigSchema` output. Test (a) is satisfied unambiguously.

**Option A — Move BindableValue into `tui-vfx-style`, keep names unprefixed.**
- *What this buys us:* one home for the family. No cross-crate split. Mirrors `BindableU16`/`BindableString`'s existing site. Intention 26 SSOT applies.
- *What we lose:* `tui-vfx-style` would need `SignalContext` and the `mixed_signals_schema::SignalOrFloat` bridge (currently consumed only by compositor). Style today doesn't depend on `SignalContext`; adding it pulls signal-resolution semantics one crate upstream, which is one extra import edge per `cargo build`.

**Option B — Define `Bindable<T>` in a new `tui-vfx-bindable` micro-crate.**
- *What this buys us:* the family lives in a leaf crate that both `tui-vfx-style` and `tui-vfx-compositor` can depend on without circularity. The crate becomes the SSOT for the bindable wire-format vocabulary.
- *What we lose:* an additional crate to maintain — a real Intention 24 rule 1 cost ("real value, not abstract principle"). Three siblings + two named-not-built (`BindableColor`, `BindableF32`) is exactly the rule-of-three threshold, not a comfortable margin. A new crate is heavyweight justification.

**Option C — Move `Bindable<T>` into `tui-vfx-core` next to the existing schema-bridge types.**
- *What this buys us:* `tui-vfx-core` already hosts the cross-cutting schema and signal-bridge types (`SignalOrFloat`, `EasingType`). Both style and compositor depend on core. Adding the unified bindable here costs no new crate edge; both downstream crates already pay the import.
- *What we lose:* `tui-vfx-core` grows another vocabulary. Per Intention 24 rule 6 ("watch for the rationalization chain") this should fall out of one criterion: does the type fit core's existing role? Yes — core already owns the schema-bridge types that bindable composes with.

**Prefix sub-decision.** Prefixed `VfxBindable<T>` versus unprefixed `Bindable<T>`. Per Intention 8: the type is wire-format data crossing crate boundaries. The three-test criterion places it on the *prefixed* side. The current names (`BindableU16`, `BindableString`, `BindableValue`) are out of compliance with Intention 8 today; V3's clean-sheet rename moment (Intention 10) is the right time to correct.

**Recommended: Option C with `VfxBindable<T>`.** Per Intention 26 (SSOT) the consolidated home is the crate both downstream consumers already depend on — `tui-vfx-core`. Per Intention 8 (three-test prefix) the type carries `Vfx`. Per Intention 23 rule 4 (additive migration) the existing names alias to the new generic during the transition window: `pub type VfxBindableString = VfxBindable<String>;`, with the legacy `pub use VfxBindableString as BindableString;` `#[doc(hidden)]` pattern that V3's `Ra*` → `Vfx*` cutover already uses (`cls_ra_image_source.rs:55` is the canonical exemplar).

**What this buys us / what we lose.** Buys: Intentions 8/23/26 all satisfied in one cutover; no new crate; matches V3's existing alias-migration mechanic. Loses: `tui-vfx-core` grows by one type family. Compared to a new micro-crate this is strictly cheaper.

**Open follow-on.** The `Signal` arm's `SignalContext` parameter and the lifetime asymmetry between `BindableString::evaluate<'a>` and the others (Finding 1.1.B) want to consolidate into one `BindingContext` bundle. Defer that bundle's shape to the slice that lands `BindableColor` — three signatures is the minimum for confident consolidation per Intention 23 rule 5.

---

### 6.3 Question 3 — Pool<T> alias rustdoc preservation

**The decision.** When the five hand-rolled pool types collapse into `Pool<T>` aliases, how do their domain-specific rustdoc examples migrate?

**Re-grounding evidence.** `wc -l` confirms the five pools at 763 LOC total. `grep -nE "^pub struct|^impl"` shows four of the five carry only the canonical four-method API; `cls_text_pool.rs:51` carries an additional `sanitize()` step inside `TextPool::new`. The rustdoc on `cls_image_pool.rs` (lines 11–34) is image-specific and walks the asset-map name-not-bytes story; `cls_text_pool.rs` rustdoc covers sanitization; the others are similarly domain-anchored.

**Option A — Migrate every per-pool rustdoc onto the alias as `#[doc(...)]` attributes.**
- *What this buys us:* the alias-level docstring keeps the exact same landing-page experience for consumers who type `ImagePool` into rustdoc. Intention 12 (documentation as engineering contract) is preserved item-for-item.
- *What we lose:* the alias declarations grow from one line to ~30. Five aliases × ~30 lines of `#[doc = "..."]` attributes = ~150 lines of new docstring scaffolding to maintain. Per Intention 24 rule 3 ("reduce lines and reduce complexity at the call site") this is mostly transcription work, not new value.

**Option B — Distill into one `Pool<T>` rustdoc + cross-references on each alias.**
- *What this buys us:* one canonical example on `Pool<T>`, one short alias docstring per pool that names the typical T (`ImagePool` is `Pool<String>` for asset-map keys; see `Pool` for the full API). Intention 24 rule 4 (improve readability for the reader) — readers who land on `ImagePool` get pointed at `Pool` once and learn the whole family.
- *What we lose:* domain-specific examples (the asset-map name-not-bytes story; the sanitize-on-construct contract) scatter into the alias docstrings rather than landing on `Pool<T>` itself. A reader of `Pool<T>` doesn't see why a pool of strings is interesting.

**Option C — Hybrid: `Pool<T>` carries the canonical four-method usage; each alias carries a domain-specific example block.**
- *What this buys us:* `Pool<T>` has the API contract; each alias carries ~10 lines of domain rustdoc explaining *why this pool exists* (asset-map keys for `ImagePool`, sanitized lines for `TextPool`, etc.). Average alias declaration grows from 1 to ~12 lines, total ~60 new lines.
- *What we lose:* small maintenance cost per alias. No domain context lost.

**Recommended: Option C (hybrid).** Per Intention 12A V3 schema-bearing types must have V2-grade rustdoc; the pool aliases qualify. Per Intention 24 rule 4 (readability measured at the reader side) a reader who lands on `TextPool` needs to know about sanitize-on-construct; that fact does not belong on the generic `Pool<T>`. Per Intention 23 rule 6 (document the consolidation rationale) `Pool<T>` carries the *why* of the abstraction; each alias carries the *what is special about this T*. **Critical sub-rule:** `TextPool`'s `sanitize()` step (`cls_text_pool.rs:51`) is *not* a docstring difference — it is behavioral. The migration must preserve that step, either as a `Pool<T>` constructor that takes an `impl Fn(&T) -> T` sanitizer (rejected — Intention 24 rule 1, no other pool wants it) or by keeping `TextPool` as a thin newtype wrapper rather than a type alias (recommended).

**What this buys us / what we lose.** Buys: full Intention 12 coverage with minimal scaffold cost; no behavioral regression in `TextPool`'s sanitize contract; canonical `Pool<T>` as the API teaching surface. Loses: `TextPool` becomes the one "non-alias" exception in the family — which is actually Intention 23 rule 5 in action (don't force-fit; honest duplication beats wrong abstraction).

**Open follow-on.** Should the `cargo xtask docs generate` pipeline (per Intention 25) check that every alias has a `#[doc(...)]` attribute pointing back to `Pool<T>`, or is that overreach? Recommend: light grep gate, not full tooling.

---

### 6.4 Question 4 — `cls_filter_spec.rs` split-up timing

**The decision.** Split `cls_filter_spec.rs` (2193 LOC, 11× hard limit) before or after the V3 cross-family work that adds 5+ new variants?

**Re-grounding evidence.** `cls_filter_spec.rs` confirmed at 2193 LOC. The OFPF hard limit for `cls_` is 200 LOC — the file is at 1100% of that limit. `cls_mask_spec.rs` (781 LOC) and `cls_sampler_spec.rs` (756 LOC) are also over but follow the same single-file pattern; per Intention 23 the rule-of-three for "tagged-enum file at OFPF pressure" is met *today*.

**Option A — Split first, then add variants.**
- *What this buys us:* new variants land into a structured per-variant directory rather than appending to a 2193-LOC file. Reviewer cognitive load is lower for each new variant PR. Intention 23 rule 2 (top-down periodic review at major milestones) — V3 *is* the major milestone, the survey already happened, this is the action.
- *What we lose:* the split-up slice happens before knowing exactly which variants the V3 work needs. There's a small risk that V3 introduces a variant family (e.g., a new "compositional" filter base) that wants a second-axis directory layout the first split didn't anticipate.

**Option B — Add variants first, then split with full information.**
- *What this buys us:* the split-up phase has perfect information about every variant the V3 work introduces. Per Intention 24 rule 1 (real value) the split shape can be designed against actual variant taxonomy rather than predicted.
- *What we lose:* every new variant is a 50–80-line edit to a file already at 11× the hard limit. Reviewer fatigue compounds. The "this file is already too big to read" mental model worsens before it improves. Per Intention 25 rule 2 (mechanize drift classes already seen) — the drift class is "tagged-enum file growing past hard limit"; V3 will make it worse before splitting fixes it.

**Option C — Split first, but only the structurally-easy variants; defer split decisions for unusual variants.**
- *What this buys us:* mechanical splits (variants that are pure data with no shared helpers) extract immediately; variants with shared internal helpers stay in `cls_filter_spec.rs` until the V3 work clarifies the helper boundaries. Smallest intervention per Intention 25 rule 5.
- *What we lose:* a half-split state. The file shrinks but does not reach the hard limit. Reviewers see a partial migration and have to decide where to put new variants.

**Recommended: Option A (split first).** Per Intention 23 rule 2 V3 is the explicit "top-down periodic review" milestone; the catalog is known well enough today to design the split. Per Intention 26 rule 6 ("retrospective corrections are valid and encouraged") any V3 variant that doesn't fit the first-pass split is fixed in-flight, not waited-on. Per Intention 25 rule 5 the smallest viable split is *one file per variant* (matches the pattern `mask_spec.rs` and `sampler_spec.rs` would also adopt) — no novel directory taxonomy required. Sequence: split `cls_filter_spec.rs` first as a standalone slice; V3 cross-family variants land into the post-split structure. **Important contract:** the public `FilterSpec` enum stays as a re-exporting tagged enum (consumers continue to write `FilterSpec::Tint { ... }`); only the *body* of each variant moves into a per-variant typed struct. Per Intention 23 rule 4 (additive migration, never breaking churn) this is invisible to recipe authors.

**What this buys us / what we lose.** Buys: V3 variant-additions are clean; OFPF hard-limit pressure relieved; matches existing per-variant precedent in sister `*Spec` files. Loses: one slice of "pre-V3 plumbing" work that appears to add no recipe-author-visible value. The Intention 24 step-back test passes because the value is reviewer cognitive load, not abstract organization.

**Open follow-on.** Does the split slice also touch `cls_prepared_filter.rs` (1941 LOC) and `cls_split_flap.rs` (1642 LOC)? Recommend: separate slices, sequenced after FilterSpec because they exhibit different splitting axes (PreparedFilter has runtime state; SplitFlap is one-effect-many-helpers).

---

### 6.5 Question 5 — Hand-written ConfigSchema derive policy

**The decision.** Should V3 enforce "hand-written `impl ConfigSchema` requires a justification comment" via a hard gate, or is a softer "audit periodically" stance enough?

**Re-grounding evidence.** `ofpf-content "ConfigSchema for"` returns 13+ hand-written impls (per §1.9.A). Five of them have legitimate must-be-hand-written reasons (primitives, schema bridges, wire-format primitives like `Color`). The remaining ~6–8 (`PathType`, `RoleTag`, `BindableString`, `BindableU16`, `StyleRegion`, `VfxCursorPrimary/Trail`) plausibly *could* be derived if either the macro or the type shape were adjusted. Intention 12A: "V3 schema-bearing types must support `ConfigSchema` derivation or have an explicit reason not to."

**Option A — Hard gate: every hand-written `impl ConfigSchema` requires a `// derive-justification: ...` comment, enforced by `cargo xtask` check.**
- *What this buys us:* mechanical Intention 12A enforcement. Per Intention 25 rule 2 (mechanize drift classes already seen) the drift exists today; the check is justified by visible pain. Per Intention 25 rule 5 the smallest mechanization is a grep over `impl ConfigSchema for` that asserts each is preceded by a `// derive-justification:` comment.
- *What we lose:* one new gate in `cargo xtask` (small). Authors of new bridge types have to write a one-line comment they might consider obvious.

**Option B — Soft audit: periodic `cargo xtask` warning that lists hand-written impls without justification, but does not fail the build.**
- *What this buys us:* visibility without breaking velocity. Authors see the audit output during routine V3 work and can address drift incrementally.
- *What we lose:* per Intention 25 rule 1 ("look for infrastructure wins during every substantive change") and rule 7 ("document the resolution playbook inside the check") — a warning-only check is the failure mode of "later never arrives." Soft gates that do not block merges are routinely ignored.

**Option C — Differ-based gate: `cargo xtask` check that diffs hand-written `ConfigSchema` impls against what `#[derive(ConfigSchema)]` would have produced and flags drift.**
- *What this buys us:* catches the *real* drift class (hand-written impl that has silently fallen behind the macro's output). Strongest possible enforcement.
- *What we lose:* per Intention 25 rule 5 ("smallest intervention that delivers full coverage") this is overshoot. The differ requires running the macro twice, capturing both outputs, and comparing — a non-trivial xtask. Per Intention 24 rule 1 (real value, current scale) no observed drift today between hand-written and derived. The differ is speculative defense.

**Recommended: Option A (hard gate with justification comment).** Per Intention 25 rule 2 the drift class is observed (Intention 12A's "explicit reason not to derive" is currently informal). Per rule 5 the smallest mechanization is a grep — ~15 lines of bash or a thin xtask subcommand. Per rule 7 the failure message documents the playbook ("Add a `// derive-justification: <reason>` comment above this impl, or replace it with `#[derive(ConfigSchema)]`"). Per Intention 23 rule 6 the comment doubles as the consolidation rationale a future reader needs. This bundles cleanly with the §1.9.A "next slice" recommendation.

**What this buys us / what we lose.** Buys: Intention 12A enforcement; Intention 25 infrastructure win; one-line comment cost amortized over every future schema-bearing type; the comment surface itself becomes a useful audit log when grep-walking the codebase. Loses: one new xtask command and the muscle-memory cost of writing the justification comment. Net: clearly positive.

**Open follow-on.** Does the gate also enforce that the justification comment *names* the test in `mixed_signals_schema.rs` (or wherever) that exercises the hand-written impl? Recommend: not at first. Add only if drift is observed.

---

## 7. Top-down architecture diagrams

This section pictures the Top 3 conversions from §2 in `BEFORE` / `AFTER` block diagrams. Each diagram is preceded by a one-paragraph "what changes" intro and followed by a "what this buys us" bullet list. Diagrams use Unicode box-drawing characters and are bounded to ≤100 columns. Boxes marked `[pub]` are public crate-surface types; `+++` marks new structure introduced by the recommendation; `~~~` marks signatures changed by the recommendation; `===` marks elements that retire. Caller and semantic boundaries that **do not change** are drawn unchanged in both panels — the leader can see at a glance what stays the same.

Legend (consistent across all three diagrams):

```
┌────┐  box, structural unit (struct, trait, file)         [pub]   public surface marker
│    │                                                     +++     newly introduced
└────┘                                                     ~~~     signature/shape changes
  ▼     ownership / containment                            ===     retired / superseded
  ►     call direction                                     ◄       data return / read
  ║     emphasized boundary (crate edge)
```

---

### 7.1 Filter / Mask / Sampler spatial-context bundle (Finding 1.1.A)

**What changes.** Three sibling traits in `tui-vfx-compositor/src/traits/` today carry four-or-five positional `u16`/`f64` parameters that are the same conceptual "where am I, when am I" bundle. The recommendation adds **one** new struct (`VfxCellContext` in `tui-vfx-types`, per §6.1's Option C) and rewrites three trait method signatures to take `&VfxCellContext` instead of the positional tuple. Every Filter/Mask/Sampler **impl file stays in place** (~56 files across `crates/tui-vfx-compositor/src/{filters,masks,samplers}/`); each impl's body changes from `x, y, width, height, t` to `ctx.local_x, ctx.local_y, ctx.width, ctx.height, ctx.t`. Pipeline dispatch sites change from "build five positional args" to "build one `VfxCellContext` per cell." Recipe authors see no schema change. Public-facing wire format is untouched.

```
============================== BEFORE — current layout ===========================================

tui-vfx-types crate (already exists; not yet hosting context)
  ┌────────────────────┐ [pub]
  │  Cell              │     <— shared cell type, used by all three traits
  └────────────────────┘
                                   ║ crate boundary ║
tui-vfx-compositor crate
  ┌──────────────────────────┐    ┌──────────────────────────┐    ┌──────────────────────────┐
  │ trait Filter   [pub]     │    │ trait Mask   [pub]       │    │ trait Sampler   [pub]    │
  │  fn apply(&self,         │    │  fn is_visible(&self,    │    │  fn sample(&self,        │
  │    cell: &mut Cell,      │    │    x: u16, y: u16,       │    │    dest_x: u16,          │
  │    x: u16, y: u16,       │    │    w: u16, h: u16,       │    │    dest_y: u16,          │
  │    width: u16,           │    │    progress: f64) -> bool│    │    width: u16,           │
  │    height: u16,          │    │                          │    │    height: u16,          │
  │    t: f64);              │    │                          │    │    t: f64)               │
  │                          │    │                          │    │    -> Option<(u16,u16)>; │
  │  v3.0.0 (BREAKING twice) │    │  v1.0.0 (one churn)      │    │  v2.0.0 (BREAKING twice) │
  └────────────┬─────────────┘    └────────────┬─────────────┘    └────────────┬─────────────┘
               ▼                                ▼                                ▼
       31 impl files                    13 impl files                    12 impl files
  filters/cls_*.rs (×31)           masks/cls_*.rs (×13)             samplers/cls_*.rs (×12)
  each duplicates the 5-tuple      each duplicates the 5-tuple       each duplicates the 5-tuple

                                   ║ crate boundary ║
tui-vfx-style crate (the proof-of-pattern; already bundled)
  ┌─────────────────────────────────────────────────┐  [pub]
  │ struct ShaderContext { local_x, local_y,        │     <— eight-field bundle, runtime_params,
  │   width, height, screen_x, screen_y, t, phase,  │         roles. The pattern Filter/Mask/
  │   runtime_params, roles }                       │         Sampler do *not* yet share.
  └────────────────────┬────────────────────────────┘
                       ▼
  ┌─────────────────────────────────────────────────┐  [pub]
  │ trait StyleShader { fn style_at(ctx, base) }    │
  └─────────────────────────────────────────────────┘

============================== AFTER — recommended layout ========================================

tui-vfx-types crate
  ┌────────────────────┐ [pub]
  │  Cell              │     <— unchanged
  └────────────────────┘
  ┌────────────────────────────────────────────┐ [pub] +++
  │  VfxCellContext { local_x, local_y,        │     <— NEW: one shared spatial bundle.
  │    width, height, screen_x, screen_y,      │         Lives next to Cell. Compositor and
  │    t, phase }                              │         style both depend on it (no cycle).
  └─────────────────┬──────────────────────────┘
                    │
        ┌───────────┴───────────────────────┐
        ▼                                   ▼
                                   ║ crate boundary ║
tui-vfx-compositor crate                          tui-vfx-style crate
  ┌────────────────────────────┐ ~~~                  ┌──────────────────────────────┐ ~~~
  │ trait Filter [pub]         │                      │ struct ShaderContext {       │
  │  fn apply(&self,           │                      │   cell: VfxCellContext,      │
  │    cell: &mut Cell,        │                      │   runtime_params: Arc<...>,  │
  │    ctx: &VfxCellContext);  │                      │   roles: Arc<RoleMap>,       │
  └─────────────┬──────────────┘                      │ }                            │
                ▼                                     │ // composes over the new     │
        31 impl files                                 │ // shared bundle             │
  filters/cls_*.rs (×31)                              └──────────────┬───────────────┘
  bodies use ctx.local_x etc.                                        ▼
                                                          trait StyleShader (unchanged surface;
  ┌────────────────────────────┐ ~~~                       ctx.cell.* is the new field path)
  │ trait Mask [pub]           │
  │  fn is_visible(&self,      │
  │    ctx: &VfxCellContext)   │
  │    -> bool;                │
  └─────────────┬──────────────┘
                ▼
        13 impl files
  masks/cls_*.rs (×13)

  ┌────────────────────────────┐ ~~~
  │ trait Sampler [pub]        │
  │  fn sample(&self,          │
  │    ctx: &VfxCellContext)   │
  │    -> Option<(u16,u16)>;   │
  └─────────────┬──────────────┘
                ▼
        12 impl files
  samplers/cls_*.rs (×12)
```

**What this buys us:**

- **Future-proof signature.** Adding `runtime_params` or role-map awareness to Filter/Mask/Sampler is a *style-side change* (extend `ShaderContext`) — not a breaking churn across 56 impls. Filter is at v3.0.0 having absorbed two breakings already; this prevents v4.0.0.
- **Single source of truth (Intention 26).** Eight-field shape lives in one place (`VfxCellContext` in `tui-vfx-types`) and both compositor and style compose over it. No drift between two parallel bundles.
- **Crate-boundary discipline (Intention 1).** Compositor traits do not import `tui-vfx-style`. Grid-first compositor independence preserved.
- **Performance neutral.** A `&VfxCellContext` is a single pointer; today's positional `u16`s already pass through the `dyn` Filter virtual call. Per-frame allocation is identical (the pipeline already builds a per-cell context before dispatch in some paths). No new vtable indirection.
- **Mechanical migration.** Each impl rewrite is `s/x: u16, y: u16, .../ctx: &VfxCellContext/` plus body `s/x/ctx.local_x/g`. Per-file edit is small; reviewer cost is low.

---

### 7.2 `Pool<T>` generalization (Finding 1.2.B)

**What changes.** Five sibling pool structs in `crates/tui-vfx-content/src/pool/` currently each carry their own struct definition + impl block (`{ items, policy }` shape, four-method API). The recommendation defines one generic `Pool<T>` in a new `cls_pool.rs` and rewrites four of the five files to type aliases. The fifth (`TextPool`) stays as a thin newtype wrapper because its `new()` constructor calls `sanitize()` per item — behavior the other four pools do not want and that Intention 24 rule 1 forbids forcing into the generic. Consumer import paths (`use tui_vfx_content::pool::{ImagePool, TextPool, ...}`) are unchanged. ~250 LOC of struct + impl scaffolding retires.

```
============================== BEFORE — current layout ===========================================

tui-vfx-content crate, pool/ module (763 LOC across five sibling files)

  ┌──────────────────────────┐ [pub]   ┌──────────────────────────┐ [pub]
  │ cls_image_pool.rs (110)  │         │ cls_text_pool.rs (144)   │
  │  pub struct ImagePool {  │         │  pub struct TextPool {   │
  │    items: Vec<String>,   │         │    items: Vec<String>,   │
  │    policy: PoolPolicy }  │         │    policy: PoolPolicy }  │
  │  impl ImagePool {        │         │  impl TextPool {         │
  │    new, pick, is_empty } │         │    new (calls sanitize), │
  └────────────┬─────────────┘         │    pick, is_empty }      │
               │                       └────────────┬─────────────┘
               │                                    │
  ┌──────────────────────────┐ [pub]   ┌──────────────────────────┐ [pub]
  │ cls_font_pool.rs (102)   │         │ cls_effect_pool.rs (129) │
  │  pub struct FontPool {   │         │  pub struct EffectPool { │
  │    items: Vec<String>,   │         │    items: Vec<...>,      │
  │    policy: PoolPolicy }  │         │    policy: PoolPolicy }  │
  │  impl FontPool {         │         │  impl EffectPool {       │
  │    new, pick, is_empty } │         │    new, pick, is_empty } │
  └────────────┬─────────────┘         └────────────┬─────────────┘
               │                                    │
  ┌──────────────────────────┐ [pub]
  │ cls_preset_pool.rs (278) │
  │  pub struct PresetPool { │
  │    items: Vec<Preset>,   │
  │    policy: PoolPolicy }  │
  │  impl PresetPool {       │
  │    new, pick, is_empty } │
  └────────────┬─────────────┘
               │
               ▼
        ┌──────────────────┐ [pub]
        │ col_pool_policy  │     <— shared policy enum (already SSOT)
        │  PoolPolicy      │
        └──────────────────┘

  Each of the five files duplicates:  struct decl + impl Self + #[derive(...,ConfigSchema)] +
  per-pool rustdoc + per-pool inline tests.

============================== AFTER — recommended layout ========================================

tui-vfx-content crate, pool/ module

  ┌──────────────────────────────────────────────┐ [pub] +++
  │ cls_pool.rs                                  │
  │   pub struct Pool<T> {                       │
  │     pub items: Vec<T>,                       │
  │     pub policy: PoolPolicy }                 │     <— canonical generic, four-method API,
  │   impl<T> Pool<T> {                          │         consolidation rationale rustdoc per
  │     new, pick, is_empty }                    │         Intention 23 rule 6
  └──────────┬───────────────────────────────────┘
             │
             ├──────────────────────────────────────────────────────────────────┐
             ▼                                                                   ▼
  ┌──────────────────────────────────────┐ [pub] ~~~  ┌──────────────────────────────────┐ [pub] ~~~
  │ pool/mod.rs (alias hub)              │             │ cls_text_pool.rs (newtype)       │
  │  pub type ImagePool  = Pool<String>; │             │  pub struct TextPool(            │
  │  pub type FontPool   = Pool<String>; │             │    Pool<String>);                │
  │  pub type EffectPool = Pool<...>;    │             │  impl TextPool {                 │
  │  pub type PresetPool = Pool<Preset>; │             │    new (calls sanitize per item) │
  │  pub use cls_text_pool::TextPool;    │             │    pick, is_empty (delegate)     │
  └──────────────────────────────────────┘             │  }                               │
             ▼                                         │  // sanitize is behavioral —     │
        ┌──────────────────┐ [pub]                     │  // cannot type-alias            │
        │ col_pool_policy  │ <— unchanged              └──────────────────┬───────────────┘
        │  PoolPolicy      │                                              │
        └──────────────────┘                                              ▼
                                                               (composes Pool<String>)

  Retired (moved to recyclebin/):                                    === ===
    cls_image_pool.rs (110)    === cls_font_pool.rs   (102) === cls_effect_pool.rs (129) ===
    cls_preset_pool.rs (278)   === (only struct + impl bodies; alias declarations replace them)

  Net change: ~250 LOC removed (struct + impl scaffolding); ~30 LOC added (Pool<T> + aliases +
              TextPool newtype). Public type names unchanged → no consumer migration.
```

**What this buys us:**

- **Sixth pool is one line.** Adding `pub type ColorPool = Pool<Color>;` in `mod.rs` is the entire change — no new file, no new tests, no new rustdoc scaffolding. Per Intention 23 rule 1 the generalization pays back on the *next* addition.
- **Public surface preserved.** Every consumer import (`tui_vfx_content::pool::ImagePool`, `tui_vfx_content::pool::TextPool`) continues to work unchanged. Per Intention 23 rule 4 (additive migration, never breaking churn) this consolidation is invisible to recipe authors and downstream crates.
- **Behavioral preservation.** `TextPool`'s `sanitize()` step (`cls_text_pool.rs:51`) is preserved as a thin newtype wrapper — the one case where the generic Pool<T> would lose information. Honest duplication beats wrong abstraction (Intention 23 rule 5).
- **Performance neutral.** `Pool<T>` monomorphizes per-T at compile time; the resulting code is identical to today's hand-rolled structs. `pick()`, `is_empty()`, and the four-method API have no virtual dispatch.
- **Documentation rationale.** `Pool<T>` carries the consolidation comment Intention 23 rule 6 requires; each alias carries a one-line domain note. Future contributors see the pattern and don't re-fragment it.

---

### 7.3 `VfxImageSource.image_name → BindableString` (Finding 1.3.A)

**What changes.** The field `image_name: String` on `VfxImageSource` (`crates/tui-vfx-recipes/src/recipe_schema/scene/cls_ra_image_source.rs:29`) becomes `image_name: BindableString`. Recipe JSON gains the binding-form alternative (`{"binding": "splash_logo"}`); the literal-form (`"image_name": "logo_light"`) keeps working unchanged via lenient bare-string deserialization. The scene-layer composer's resolver path now calls `image_name.evaluate(&runtime_params).unwrap_or(literal_default)` before the existing AssetMap lookup. AssetRegistry/ImagePool surfaces and asset-byte resolution are unchanged. Phase 7.bytes (full runtime end-to-end) stays deferred per cycle plan v0.7.0; this lift delivers the *authoring* surface only.

```
============================== BEFORE — current layout ===========================================

  recipe JSON                                     ║ tui-vfx-recipes crate ║
                                                  ║                       ║
  { "image": {                                    ║ ┌──────────────────┐  ║   plain field
    "image_name": "logo_light",                   ║ │ VfxImageSource   │  ║   String, no binding
    "aspect": "fit"                               ║ │   image_name:    │  ║   shape — the literal
  } }                                             ║ │     String       │ [pub]   surface only.
       │                                          ║ │   tint, aspect   │  ║
       │ deserialize                              ║ └─────────┬────────┘  ║
       ▼                                          ║           ▼           ║
                                                  ║                       ║
                                                  ║ ┌────────────────────┐║
                                                  ║ │ scene-layer        │║
                                                  ║ │ composer (runtime) │║
                                                  ║ └─────────┬──────────┘║
                                                  ║           ▼           ║
                                                              ▼
                                                  ║ tui-vfx-content crate ║
                                                  ║                       ║
                                                  ║ ┌────────────────────┐║
                                                  ║ │  ImagePool         │║       picks an
                                                  ║ │  (asset-name keys) │ [pub]  asset name
                                                  ║ └─────────┬──────────┘║       from the pool
                                                  ║           ▼           ║
                                                  ║ ┌────────────────────┐║
                                                  ║ │ AssetRegistry /    │║       resolves
                                                  ║ │ AssetMap           │ [pub]  name → bytes
                                                  ║ └─────────┬──────────┘║
                                                  ║           ▼           ║
                                                          .rss bytes  →  blit

  Today the field is the literal asset-map key. The recipe author CANNOT write
    "image_name": { "binding": "splash_logo" }
  to defer the asset choice to runtime_params at playback time.

============================== AFTER — recommended layout ========================================

  recipe JSON (literal form, unchanged)           ║ tui-vfx-recipes crate ║
                                                  ║                       ║
  { "image": {                                    ║ ┌──────────────────┐  ║   ~~~ field type
    "image_name": "logo_light",                   ║ │ VfxImageSource   │  ║   changes; lenient
    "aspect": "fit"                               ║ │   image_name:    │  ║   bare-string deser
  } }                                             ║ │   BindableString │ [pub]  preserves the
                                                  ║ │   tint, aspect   │  ║   literal form.
  recipe JSON (binding form, NEW)            +++  ║ └─────────┬────────┘  ║
                                                  ║           │           ║
  { "image": {                                    ║           │ evaluate(&runtime_params)
    "image_name": {"binding":"splash_logo"},      ║           ▼           ║
    "aspect": "fit"                               ║ ┌────────────────────┐║
  } }                                             ║ │ scene-layer        │║   ~~~ adds the
       │                                          ║ │ composer (runtime) │║   evaluate() call;
       │ deserialize (Literal | Binding | Signal) ║ │   .evaluate(rp)    │║   .unwrap_or(default)
       ▼                                          ║ │   .unwrap_or("…") │║   handles missing
                                                  ║ └─────────┬──────────┘║   binding keys.
                                                  ║           ▼           ║
                                                              ▼
                                                  ║ tui-vfx-content crate ║
                                                  ║                       ║                    +++
  runtime_params (per-frame from playback) ─────► ║ ┌────────────────────┐║    (the binding lookup
                                                  ║ │ ShaderRuntimeParams│║    happens *before*
                                                  ║ │   .get_text(key)   │ [pub] the pool/registry
                                                  ║ └─────────┬──────────┘║    resolves bytes)
                                                  ║           │ resolved name
                                                  ║           ▼           ║
                                                  ║ ┌────────────────────┐║
                                                  ║ │  ImagePool         │║       unchanged surface
                                                  ║ │  (asset-name keys) │ [pub]
                                                  ║ └─────────┬──────────┘║
                                                  ║           ▼           ║
                                                  ║ ┌────────────────────┐║
                                                  ║ │ AssetRegistry /    │║       unchanged surface
                                                  ║ │ AssetMap           │ [pub]
                                                  ║ └─────────┬──────────┘║
                                                  ║           ▼           ║
                                                          .rss bytes  →  blit

  Diff scope: ~1 line in cls_ra_image_source.rs (field type) + ~3 lines in scene-layer composer
              (evaluate + unwrap_or). Phase 7.bytes (asset-byte resolution from a binding) stays
              deferred — this lift delivers authoring surface only. The literal-form recipe path
              is bit-identical to today.
```

**What this buys us:**

- **Authoring surface unblocks now.** Recipes can declare `{"binding": "splash_logo"}` for asset choice today, even though the runtime end-to-end (Phase 7.bytes) is deferred. Per cycle plan v0.7.0 line 22 this is *independently actionable*.
- **Pattern parity (Intention 26 SSOT).** `MechanicalContentSource::Preset.font` (Phase 6, shipped) is already `Option<BindableString>`. This lift brings `image_name` to the same surface — one Bindable shape across the recipe schema, not a parallel "asset name vs. font name" seam.
- **Validator alignment (Intention 28).** The existing V3 binding-form validator (which already accepts `{"binding": "name"}`) covers this field automatically once the type changes. No new validator code.
- **Backward-compatible.** Literal-form recipes (`"image_name": "logo_light"`) continue to deserialize via `BindableString::Literal`. Per Intention 23 rule 4 (additive migration, never breaking churn) no recipe author edits anything.
- **Performance neutral.** `BindableString::evaluate` is a one-branch match; the `Literal` arm returns a borrow with no allocation. The runtime path adds one `.evaluate()` call before the unchanged AssetMap lookup — identical hot-path cost.

---

## 8. Impact analysis: chainable effects, V3 features, performance

This section answers three triage questions the leader posed before committing to the recommendations: (1) does any finding compromise the V3 chainable-effects mechanism — the `StepInput<T> = ParamValue<T> | HintRef<T>` tree-pipeline composition where one step feeds another via named hints (`displacement`, `sampled_color`, `cell_density`, `alpha_mask`)?; (2) what other V3 surfaces does each Top 3 finding touch beyond §3–§5; (3) what is the per-finding performance impact at the 60 fps / 16.7 ms budget.

**Grounding note on the chainable-effects mechanism.** Per `steering/MARKETING.md:225–264` and `docs/design/tui-vfx-v3-upgrade-plan/40_decisions.md:298–375` (Decisions 6 + 7), `StepInput`, `HintRef`, and `ParamValue` are **V3 design surfaces**, not yet code. `ofpf-defs StepInput` and `ofpf-defs HintRef` return zero hits today; `ofpf-defs ParamValue` returns only `ShaderRuntimeParamValue` at `tui-vfx-style/src/traits/cls_shader_context.rs:24` (the runtime-binding precursor, not the V3 type). The chainable-effects machinery — named-hint declaration on producers, `HintRef<T>` resolution on consumers, per-layer hint namespace per Decision 7 — will land in a future V3 slice. Today the compositor's hot path dispatches sampler → mask → filter → shader sequentially per cell with no inter-step hint payload (`crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs:684–840` `render_loop`); the producer-to-consumer channel that hosts chainable effects does not yet exist as a runtime data path.

This means every finding in this sweep operates at a **different architectural level** from the chainable-effects mechanism:
- Findings 1.1.A, 1.1.B touch *trait method signatures* on individual step renderers — the per-cell call shape, not the inter-step data flow.
- Findings 1.2.A, 1.2.B, 1.2.C, 1.7.A touch *type generalization within a single struct family* — independent of how steps connect.
- Findings 1.3.A, 1.3.B touch *recipe-schema field types* — the authoring surface, not the runtime composition.
- Findings 1.4.A, 1.6.A, 1.8.A, 1.9.A touch *file/error/cycle organization* — invisible to the runtime data path.

Read top-down with the spec layout from `docs/design/tui-vfx-v3-upgrade-plan/00_INDEX.md:35–36` (`StepInput<T> = ParamValue<T> | HintRef<T>` — Decisions 6 + 7), the chainable-effects mechanism lives **above** every finding's locus. The findings change inputs to or organization of the leaves; the chain shape is unaffected.

---

### 8.1 Chainable-effects impact

**Top 3 — individual treatment.**

#### Finding 1.1.A (Filter / Mask / Sampler spatial-context bundle)

**Touches chainable-effects? NO.** The bundle changes `fn apply(&self, cell, x, y, width, height, t)` to `fn apply(&self, cell, ctx: &VfxCellContext)` — a per-call shape change inside the renderer. Per `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs:716–720`, `render_loop` calls `sample_sampler_chain(samplers, local_x, local_y, w16, h16, options.t)` then `check_prepared_masks(...)` then `apply_filters(...)`; each call is the *primary* per-cell payload. Decision 7's named-hint output (e.g. a sampler's `displacement` channel) is a *second* payload alongside this primary call, not the call itself. Adding `VfxCellContext` to the primary-call signature does nothing to the (future) hint side-channel. When V3's HintRef machinery lands, sampler impls will publish hints either via a return-tuple addition or via a `&mut HintFrame` ctx field added to `VfxCellContext` — both extensions are additive on top of the bundle, not blocked by it. The bundle in fact *helps*: `VfxCellContext` is the natural single home for a future `&mut HintFrame` field, replacing what would otherwise be a seventh positional parameter on `apply`.

**Verdict: zero risk. Bundle is the natural extension point for hint propagation when Decision 7's runtime lands.**

#### Finding 1.2.B (Pool<T> generalization)

**Touches chainable-effects? NO.** Pools live at `crates/tui-vfx-content/src/pool/cls_*.rs`; per `ofpf-blast crates/tui-vfx-content/src/pool/cls_image_pool.rs` they have 5 dependents and are consumed before pipeline evaluation (asset/text/font/effect/preset *selection*, not per-cell render). The chainable-effects channel runs strictly inside the compositor's per-cell `render_loop`; `Pool<T>` is upstream of the loop. `pick()` runs once per recipe instantiation or transformer step, not per cell per frame. No StepInput/HintRef code path touches a `Pool` field.

**Verdict: zero risk. Pools are pre-pipeline; chainable effects are intra-pipeline.**

#### Finding 1.3.A (VfxImageSource.image_name → BindableString)

**Touches chainable-effects? NO.** `image_name` is a *scene-layer source* field (`tui-vfx-recipes/src/recipe_schema/scene/cls_ra_image_source.rs:29` per cycle plan v0.7.0 line 22). Scene-layer sources resolve to a content surface *before* the per-layer pipeline runs (per Decision 5 in `40_decisions.md` and the per-layer pipeline framing in `40_decisions.md:266`). The chainable-effects mechanism operates *inside* a layer's pipeline; it does not flow backward into the layer's source. `BindableString::evaluate` (per `crates/tui-vfx-style/src/models/cls_bindable_string.rs:63`) reads `ShaderRuntimeParams` — the `RuntimeBinding` arm of `ParamValue<T>` — which is one of the three `ParamValue<T>` forms in Decision 6 but *not* a `HintRef<T>`. The lift extends `ParamValue<String>`-ish coverage into `image_name`; it does not connect that field to step output hints. Per Decision 7, scene-layer sources are not a hint producer or consumer; they're the *content* the pipeline subsequently transforms.

**Verdict: zero risk. Scene-layer source is upstream of the layer's hint-graph; the lift is `ParamValue`-side, not `HintRef`-side.**

**Lower-tier findings (grouped).**

| Finding | Touches StepInput/HintRef? | One-line evidence |
|---|---|---|
| 1.1.B (Bindable*::evaluate signature unification) | NO | Bindable types feed individual step parameters via `ParamValue::RuntimeBinding`-style resolution (`cls_bindable_string.rs:63`); the unified `BindingContext` bundle would change the *call shape* of `evaluate`, not the `HintRef` resolution path. |
| 1.2.A (Bindable<T> generalization) | NO | Same — `Bindable<T>` is the `ParamValue::RuntimeBinding | Constant | Signal` shape's host (Decision 6 framing in `40_decisions.md:308–323`). It is *one half* of `StepInput<T>`; consolidating its three concrete instances does not touch the other half (`HintRef<T>`). |
| 1.2.C (FontRegistry / AssetRegistry merge) | NO | Registries are pre-pipeline name→bytes resolvers. No hint flows through them. |
| 1.3.B (BindableEnum<T>) | NO | Speculative; same reasoning as 1.3.A — recipe-schema field type change, not a pipeline data path. |
| 1.4.A (Loader Error consolidation — leave alone) | NO | Loader errors fire at recipe load time, before any pipeline evaluation. |
| 1.6.A (cls_filter_spec.rs split-up) | NO | Per-variant file split is internal organization of `FilterSpec` — the public enum surface and dispatch are unchanged (per §6.4 recommendation: "the public `FilterSpec` enum stays as a re-exporting tagged enum"). Per-variant fields will hold `StepInput<T>` when V3's chainable-effects machinery lands; the split makes that easier to author, not harder. |
| 1.7.A (BindableValue cross-crate home) | NO | Same locus as 1.2.A; bundling resolves only the home-crate question, not the data-path question. |
| 1.8.A (tui-vfx-core/schema cycles) | NO | Schema-writer module-level cycles; never crosses into compositor pipeline code. |
| 1.9.A (Hand-written ConfigSchema audit) | NO | Schema-derive policy; affects which types can self-describe to the validator. The validator inspects `HintRef<T>` producer/consumer matching at recipe load (per `docs/design/tui-vfx-v3-upgrade-plan/59_validator_and_canonicalization.md:75`); the audit never touches that walk. |

**§8.1 Verdict: zero chainable-effects risk across all 12 findings.** `StepInput`, `HintRef`, and `ParamValue` are V3 design surfaces (`docs/design/tui-vfx-v3-upgrade-plan/40_decisions.md:298–375`), not yet code. Every finding operates either upstream of the per-layer pipeline (scene sources, pools, registries, loaders) or at the trait-signature/struct-shape level inside a single step (Filter/Mask/Sampler signatures, Bindable types, FilterSpec organization). None of them claim or constrain the inter-step hint-propagation channel that Decision 7 will introduce. Finding 1.1.A is the only one that *helps* the future chainable-effects mechanism — `VfxCellContext` is the natural carrier for a future `&mut HintFrame` field, so the bundle pre-pays for the hint-channel ergonomics.

---

### 8.2 Other V3 feature surfaces touched

This subsection enumerates the V3-feature surfaces the **Top 3** findings touch, beyond what §3–§5 already noted. V3 surfaces inventoried: theme integration / `Scope::Role`; three-level composition (signal / parameter / hint); per-layer pipelines (Decision 5); scene-layer sources; recipe envelope blocks (`requires_bindings`, `requires_assets`, `requires_tokens` per `40_decisions.md:225,266`); validator gates (per `59_validator_and_canonicalization.md`); probe/trace observability (per `MARKETING.md:278`); the `Vfx*`-prefix cutover (Intention 8 / Decision 4); the future *recipe envelope* tokenization split (Open Q #14).

#### Finding 1.1.A — Filter/Mask/Sampler bundle

| V3 surface | Impact | Evidence |
|---|---|---|
| Three-level composition (Level 2: ParamValue) | **Enabling.** Filter/Mask/Sampler spec fields hold `BindableValue` today (Finding 1.1.B); when those fields evolve to `ParamValue<T>` (Decision 6), the trait method's `&VfxCellContext` is where the per-cell evaluation context lives. Bundle is forward-compatible; positional `(x,y,w,h,t)` would not be. | `docs/design/tui-vfx-v3-upgrade-plan/40_decisions.md:308–323` |
| Per-layer pipelines (Decision 5) | **Neutral.** Per-layer pipelines (`40_decisions.md:266`) instantiate the same trait dispatch for each layer's pipeline. The bundle changes the call shape uniformly across all layers; nothing layer-specific is added or removed. | `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs:684` |
| Theme integration / Scope::Role | **Enabling.** `ShaderContext` already carries `roles: Arc<RoleMap>` (`crates/tui-vfx-style/src/traits/cls_shader_context.rs:1.2.0` CLOG entry). Per §6.1 Option C, `VfxCellContext` keeps spatial-only fields and the role map stays on `ShaderContext`. Future role-aware Filter/Mask/Sampler variants get role access additively (extend `ShaderContext` or accept `&ShaderContext` in the trait that wants it) without re-churning the trait. | `cls_shader_context.rs:1` (1.2.0 added role-awareness fields) |
| Probe/trace observability | **Neutral.** Trace events fire from `pipeline_inspector.rs` at known boundaries (`crates/tui-vfx-compositor/src/traits/pipeline_inspector.rs`). The bundle changes data shape, not the boundary points. | `orc_render_pipeline.rs:774` `render_loop_inspected` |
| Validator gates | **Neutral.** Validator inspects recipe schema and pipeline tree shape, not the trait method signature. | `docs/design/tui-vfx-v3-upgrade-plan/59_validator_and_canonicalization.md` |
| Vfx*-prefix cutover (Intention 8) | **Aligned.** The recommended `VfxCellContext` name carries `Vfx` per Intention 8 (contract-producing wire-format type used by every Filter/Mask/Sampler impl). Lands clean inside the V3 cutover window. | `steering/INTENTIONS.md` Intention 8 |

#### Finding 1.2.B — Pool<T> generalization

| V3 surface | Impact | Evidence |
|---|---|---|
| Recipe envelope blocks (`requires_assets`, `requires_tokens`) | **Neutral.** Pools consume the resolved keys; the envelope blocks declare them. Generalizing the pool struct does not touch the envelope schema. | `40_decisions.md:225` |
| Scene-layer sources | **Neutral.** `ImagePool` and `MechanicalContentSource::Preset.font` use pools downstream of the scene-layer source; the alias keeps every consumer's import path unchanged (per §7.2 "Public surface preserved"). | §7.2 |
| Validator / probe surfaces | **Neutral.** Pool `pick()` results flow into the existing `SamplerApplied`/`ContentResolved` trace events; type alias is invisible to trace emission. | `crates/tui-vfx-debug/benches/bench_full_trace_60fps.rs:1` |
| New V3 pool kinds (`ColorPool`, `ScopePool`, `SubstitutionPool` for tokenization Open Q #14) | **Enabling.** Adding the sixth pool becomes a one-line `pub type` rather than a new file (per §7.2 "Sixth pool is one line"). The Open Q #14 tokenization work (per `40_decisions.md:338`) is one of the named third-trigger candidates. | §1.2.B "Why now" |
| Theme integration / Scope::Role | **Neutral.** `Pool<RoleTag>` is plausible but not driven; the alias hub absorbs it cheaply if the use case lands. | — |

#### Finding 1.3.A — VfxImageSource.image_name → BindableString

| V3 surface | Impact | Evidence |
|---|---|---|
| Three-level composition (Level 2: ParamValue) | **Enabling.** Lifting `image_name` to `BindableString` brings the field onto the `ParamValue::RuntimeBinding`-shaped surface. When `BindableString` consolidates into `Bindable<T>` (Finding 1.2.A) the field inherits the unified type without re-edit. | `40_decisions.md:308` |
| Recipe envelope blocks (`requires_bindings`) | **Enabling.** Once `image_name` accepts `{"binding": "splash_logo"}`, recipes that bind it must declare `splash_logo` in `requires_bindings` per Decision 6's load-time discovery model. The validator-side coverage already exists for other `BindableString` sites; this lift inherits the gate. | `40_decisions.md:338` |
| Scene-layer sources (Decision 5) | **Neutral.** `VfxImageSource` *is* a scene-layer source. The lift extends the source's authoring surface; the rest of the layer pipeline composition is unchanged. | §1.3.A "What" |
| Validator gates | **Aligned.** The existing V3 binding-form validator (per §7.3 "Validator alignment") accepts `{"binding": "name"}` for any `BindableString` field. No new validator code. | §7.3 |
| Probe/trace observability | **Enabling.** A binding-form `image_name` becomes visible to `--probe` as a per-frame resolution event, comparable to `MechanicalContentSource::Preset.font`'s existing trace. The `pipeline-validator --probe` per-step output dump (`MARKETING.md:280`) gains one more field. | `MARKETING.md:280` |
| Vfx*-prefix cutover | **Aligned.** `VfxImageSource` already carries the prefix; no rename. The lift uses `BindableString` (will be `VfxBindable<String>` post-1.2.A); the alias-migration mechanic is the same one cycle plan uses. | §6.2 "Recommended" |

**§8.2 Verdict: Top 3 findings touch eight V3 surfaces total — six "enabling/aligned" (the change strictly grows V3 capability), two "neutral." Zero "compromised." Most enabling surface: Finding 1.3.A pre-pays the binding-discovery wiring for the `requires_bindings` envelope block.**

---

### 8.3 Performance analysis

**Hot-path baseline.** The 60 fps / 16.7 ms budget applies. The acceptance bench is `crates/tui-vfx-debug/benches/bench_full_trace_60fps.rs` (representative 80×24, 4-layer, full-pipeline frame ≤ 2 ms of trace overhead at 60 fps). The geometry bench is `crates/tui-vfx-geometry/benches/easing.rs`. The per-cell hot path is `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs:684–840` (`render_loop` and `render_loop_inspected`); allocations inside the `for y { for x { ... } }` body are the regression class to watch.

**Categorization rule.** Positive = measurable hot-path improvement (allocation removal, branch elimination, cache improvement). Neutral = compiles to identical or near-identical code (type aliases, monomorphized generics, single struct-pointer parameter swap). Negligible = adds one match arm or one field-access indirection — under cache-line noise. Negative = adds per-cell allocation, vtable indirection where none existed, or breaks monomorphization.

| # | Finding | Category | Reasoning | Bench that would catch a regression |
|---|---|---|---|---|
| 1.1.A | Filter/Mask/Sampler bundle | **Neutral** | Replaces five positional `u16`/`f64` parameters with one `&VfxCellContext` (single pointer). Filter/Mask/Sampler are already `dyn` virtual calls; passing one fat-pointer-ish reference vs. five scalars compiles to comparable register/stack usage. The bundle's 8 fields are 4 × `u16` + 2 × `u16` + `f64` + `f64` = 20 bytes — fits one cache line. The pipeline already builds a per-cell context implicitly today (the `local_x, local_y, w16, h16, options.t` tuple is materialized at every dispatch in `orc_render_pipeline.rs:716–810`); making it explicit is a refactor of where the data lives, not a new allocation. | `bench_full_trace_60fps` (per-frame budget); regression visible if any impl unexpectedly allocates inside `apply` after migration. |
| 1.1.B | Bindable*::evaluate signature unification | **Neutral** | A `&BindingContext { loop_t, signal_ctx, runtime_params }` is one pointer; today's signatures pass 1–3 pointers. Same per-call cost. `evaluate` is called once per parameter resolution, not per cell. | `bench_full_trace_60fps` (binding resolution count). |
| 1.2.A | Bindable<T> generalization | **Neutral** | `Bindable<T>` monomorphizes per-T at compile time — `Bindable<String>`, `Bindable<u16>`, `Bindable<f32>` produce identical code to today's hand-rolled types. Match-arm dispatch on `Literal | Binding | Signal` is field-access-equivalent. Per Intention 8 / 23 rule 4 the alias-migration window keeps the existing names; existing call sites compile to the same instructions. | `bench_full_trace_60fps`; geometry bench unchanged. |
| 1.2.B | Pool<T> generalization | **Neutral** | Type aliases (`pub type ImagePool = Pool<String>;`) preserve monomorphization. `pick()` and `is_empty()` are direct calls (no virtual dispatch). The `TextPool` newtype wrapper adds one method-forward call per `pick()`, which is inlined by the optimizer. Pools are consumed at recipe-instantiation time, not per-cell. | `bench_full_trace_60fps` if a pool ends up on a per-cell path (it does not today); regression would be a delegation-without-inline that surfaced only in non-LTO debug builds. |
| 1.2.C | FontRegistry / AssetRegistry merge (deferred) | **Neutral** | Not landing — no impact. If it did land, `BTreeMap<String, Vec<u8>>` lookups remain the same shape. | — |
| 1.3.A | VfxImageSource.image_name → BindableString | **Negligible** | Adds one match-arm dispatch (`Literal | Binding`) before the existing `AssetMap` lookup in the scene-layer composer. The Literal arm returns `Some(value.as_str())` (`cls_bindable_string.rs:65`) — no allocation. The Binding arm calls `runtime_params.get_text(key)` which is one `BTreeMap` get on `String` keys — already on the asset-resolution path indirectly. Scene-layer source resolution is **per-layer-per-frame**, not per-cell; the cost amortizes to a few invocations per frame. | `bench_full_trace_60fps` if scene-layer composer is later called per-cell (it is not). |
| 1.3.B | BindableEnum<T> (speculative — leave alone) | **Neutral** | Not landing. | — |
| 1.4.A | Loader Error consolidation (leave alone) | **Neutral** | Not landing. Loader errors fire at recipe load time outside the frame budget anyway. | — |
| 1.6.A | cls_filter_spec.rs split-up | **Neutral** | Per-variant file split is purely a *file-tree* change; the `FilterSpec` enum remains a tagged-enum that pattern-matches the same way. The discriminant dispatch is identical at the machine-code level whether the variant body lives inline or in a separate file. | `bench_full_trace_60fps` would catch any accidental `Box<dyn>`-ification during the split (it is not in the recommendation). |
| 1.7.A | BindableValue cross-crate home | **Neutral** | A move from `tui-vfx-compositor/src/types/` to `tui-vfx-style` (or `tui-vfx-core` per §6.2 Option C) is a build-graph change, not a runtime change. Code is identical. | — |
| 1.8.A | tui-vfx-core/schema cycles | **Negligible** | Cycles are intra-module; resolving them (if ever pursued) is purely a refactor. Schema-writer code runs at recipe-emit time, not per frame. | — |
| 1.9.A | Hand-written ConfigSchema audit | **Positive** | The recommended xtask gate (per §6.5 Option A) catches drift between hand-written impls and the derive macro. Drift today is silent; mechanizing the check is an Intention 25 infrastructure win. **Note** (per Intention 25 framing): the gate prevents a drift class that is *future* hot-path-relevant only via the validator's recipe-load path; the per-frame hot path is unaffected. Categorized Positive for the validator-load-time class, not for the 60 fps frame budget. | Audit gate itself; no per-frame bench. |
| 1.1.A footprint sub-effect | (within 1.1.A) | **Positive** | Today's `render_loop` (`orc_render_pipeline.rs:716–840`) materializes `(local_x, local_y, w16, h16, options.t)` at every dispatch site. Bundling lets the loop construct one `VfxCellContext` per cell and pass `&ctx` thrice (sampler/mask/filter), removing two redundant tuple-builds per cell. ~3 fewer scalar-arg pushes per dispatch × 80×24×4 layers ≈ 23k fewer arg-pushes per frame — under cache-line noise but directionally positive. | `bench_full_trace_60fps`. |

**Summary count.** Positive = 2 (1.9.A audit gate; 1.1.A bundle's redundant-arg-build elimination). Neutral = 8. Negligible = 2 (1.3.A scene-layer match-arm; 1.8.A intra-module cycles). Negative = 0.

**Follow-on observation (Intention 25 territory; out of scope per task brief).** Several findings sit outside `bench_full_trace_60fps`'s coverage envelope:
- Pool `pick()` and registry `resolve()` paths are pre-pipeline; no bench measures their per-frame impact today.
- Bindable `evaluate` is called from filter/mask/sampler `prepare_*` paths but not directly counted by the 60 fps bench.
- The chainable-effects hint-propagation channel itself, when V3's runtime lands, will need its own bench (per Intention 25 the bench should land *with* the channel, not after).

These are noted as Intention 25 audit follow-ons for a future bench-coverage sweep, not as open recommendations from this sweep.

**§8.3 Verdict: zero performance-negative findings; two positive (one infrastructure-class via 1.9.A, one micro-positive via 1.1.A); the rest neutral or negligible. No finding requires benchmark verification before commit beyond the existing `bench_full_trace_60fps` regression watch that any compositor change already triggers.**

---

## 9. Appendix — verified evidence trails

For each "do now" finding, the leader can re-run these probes to confirm the evidence is current.

### Finding 1.1.A (Filter / Mask / Sampler bundle)

```bash
ofpf-orientation --root /usr/projects/tui-vfx
cat /usr/projects/tui-vfx/crates/tui-vfx-compositor/src/traits/filter.rs
cat /usr/projects/tui-vfx/crates/tui-vfx-compositor/src/traits/mask.rs
cat /usr/projects/tui-vfx/crates/tui-vfx-compositor/src/traits/sampler.rs
cat /usr/projects/tui-vfx/crates/tui-vfx-style/src/traits/tr_style_shader.rs
cat /usr/projects/tui-vfx/crates/tui-vfx-style/src/traits/cls_shader_context.rs
ofpf-blast crates/tui-vfx-compositor/src/traits/filter.rs
ofpf-blast crates/tui-vfx-compositor/src/traits/mask.rs
ofpf-blast crates/tui-vfx-compositor/src/traits/sampler.rs
ls /usr/projects/tui-vfx/crates/tui-vfx-compositor/src/{filters,masks,samplers}/
```

### Finding 1.2.B (Pool<T>)

```bash
ofpf-defs ImagePool
ls /usr/projects/tui-vfx/crates/tui-vfx-content/src/pool/
grep -n "pub struct\|pub fn" /usr/projects/tui-vfx/crates/tui-vfx-content/src/pool/cls_*.rs
ofpf-blast crates/tui-vfx-content/src/pool/cls_image_pool.rs
```

### Finding 1.3.A (VfxImageSource.image_name)

```bash
grep -rn "image_name" /usr/projects/tui-vfx-recipes/src/recipe_schema/scene/
cat /usr/projects/tui-vfx/docs/design/tui-vfx-mechanical-circular-content-cycles-plan.md | grep -A3 "Phase 7.schema"
```

### Finding 1.2.A (Bindable<T>)

```bash
ofpf-defs Bindable
wc -l /usr/projects/tui-vfx/crates/tui-vfx-style/src/models/cls_bindable_*.rs /usr/projects/tui-vfx/crates/tui-vfx-compositor/src/types/cls_bindable_value.rs
grep -n "BindableColor\|BindableF32" /usr/projects/tui-vfx/docs/design/*.md
```

<!-- <FILE>docs/design/tui-vfx-buy-once-architecture-sweep.md</FILE> - <DESC>Repository-wide buy-once/cry-once architectural sweep</DESC> -->
<!-- <VERS>END OF VERSION: 1.2.0</VERS> -->
