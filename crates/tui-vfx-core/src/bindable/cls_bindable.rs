// <FILE>crates/tui-vfx-core/src/bindable/cls_bindable.rs</FILE> - <DESC>Generic VfxBindable<T, S> envelope: Literal | Binding | Signal with S = Never default for non-signal types. Hand-written ConfigSchema gates on T: ConfigSchema and S: BindableSignal so non-signal instantiations omit a phantom Signal variant from the schema. Specialized inherent impls per (T, S) preserve the three legacy evaluate signatures without forcing a single shape across the family.</DESC>
// <VERS>VERSION: 0.1.2</VERS>
// <WCTX>Packet 1.9.A.followup US-005: add CONFIGSCHEMA-JUSTIFICATION marker to the existing rustdoc block above the generic VfxBindable<T,S> ConfigSchema impl so the audit gate sees the justification at the source site.</WCTX>
// <CLOG>0.1.2: PATCH — extend the existing /// rustdoc block above the generic ConfigSchema impl with a CONFIGSCHEMA-JUSTIFICATION line (kind=derive-cannot-handle-generic-T). Refresh the prior rustdoc to point at lib.rs (the live macro) rather than the dead fnc_impl_config_schema.rs sibling. No behavior change.</CLOG>

use mixed_signals::traits::SignalContext;
use mixed_signals::types::SignalOrFloat;
use serde::{Deserialize, Serialize};

use crate::schema::{ConfigSchema, FieldMeta, SchemaField, SchemaNode, SchemaVariant};

/// Project-local uninhabited type used as the default `S` parameter for
/// non-signal [`VfxBindable`] instantiations. `std::convert::Infallible`
/// would be the natural choice but it does not implement `Serialize` /
/// `Deserialize`, and orphan rules forbid us from adding those impls
/// downstream — so we define a sibling type and implement the traits
/// manually here.
///
/// Construction is impossible: `Never` has no variants. `Serialize`'s
/// body matches on `*self` and is therefore exhaustive without any
/// arms; `Deserialize` always returns an error because no value of
/// this type can ever be produced.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Never {}

impl Serialize for Never {
    fn serialize<S: serde::Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
        match *self {}
    }
}

impl<'de> Deserialize<'de> for Never {
    fn deserialize<D: serde::Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom(
            "tui_vfx_core::bindable::Never has no inhabitants — cannot deserialize",
        ))
    }
}

/// Read-side interface for the runtime parameter map that resolves
/// [`VfxBindable::Binding`] arms.
///
/// `tui-vfx-style`'s `ShaderRuntimeParams` implements this trait so the
/// inherent `evaluate` methods on each specialized [`VfxBindable`]
/// instantiation stay in `tui-vfx-core` (where the generic lives) without
/// the core crate having to depend on style-side types. Other runtime
/// surfaces (offline players, validators, fixture harnesses) can implement
/// the trait against their own parameter store.
pub trait RuntimeParamsRead {
    /// Look up `key` as a `u16` runtime parameter. Returns `None` if the
    /// key is missing or the stored value cannot be coerced.
    fn get_u16(&self, key: &str) -> Option<u16>;
    /// Look up `key` as a borrowed string slice. Returns `None` if the
    /// key is missing or the stored value is not text-shaped.
    fn get_text(&self, key: &str) -> Option<&str>;
    /// Look up `key` as an `f32`. Returns `None` if the key is missing or
    /// the stored value cannot be coerced.
    fn get_f32(&self, key: &str) -> Option<f32>;
}

/// Helper trait that lets non-signal [`VfxBindable`] instantiations omit a
/// phantom `Signal(Never)` variant from their generated schema.
///
/// [`Never`]'s impl returns `None` so the schema for
/// `VfxBindable<u16, Never>` shows two variants (`Literal`, `Binding`)
/// rather than three. `SignalOrFloat`'s impl returns `Some(...)` so
/// `VfxBindable<f32, SignalOrFloat>`'s schema shows the full three-variant
/// surface.
///
/// Authors adding a new signal-bearing `S` implement this trait with
/// `Some(...)`. New non-signal types reuse the [`Never`] default and
/// inherit the `None` impl below.
pub trait BindableSignal: ConfigSchema + Clone + PartialEq + 'static {
    /// Return the schema variant for the `Signal` arm, or `None` when the
    /// arm is uninhabited and should be omitted from the generated schema.
    fn signal_variant_schema() -> Option<SchemaVariant>;
}

impl BindableSignal for Never {
    fn signal_variant_schema() -> Option<SchemaVariant> {
        None
    }
}

impl ConfigSchema for Never {
    /// Schema for an uninhabited type: an empty enum. The schema is
    /// emitted for completeness but no value of this type can ever be
    /// constructed at runtime.
    fn schema() -> SchemaNode {
        SchemaNode::Enum {
            name: "Never".to_string(),
            description: Some("Uninhabited type — no value can be constructed.".to_string()),
            json_name: None,
            tag_field: None,
            variants: Vec::new(),
        }
    }
}

impl BindableSignal for SignalOrFloat {
    fn signal_variant_schema() -> Option<SchemaVariant> {
        Some(SchemaVariant::Tuple {
            name: "Signal".to_string(),
            description: Some(
                "A signal expression evaluated against the per-frame SignalContext.".to_string(),
            ),
            json_value: Some("signal".to_string()),
            items: vec![SchemaField::new(
                "expression",
                <SignalOrFloat as ConfigSchema>::schema(),
                FieldMeta::default(),
            )],
        })
    }
}

/// A value that resolves to a literal `T`, a named runtime parameter, or
/// (when the generic permits it) a signal expression of type `S`.
///
/// # Type aliases
///
/// - [`VfxBindableU16`] — coordinate bindings.
/// - [`VfxBindableString`] — font / asset / locale bindings.
/// - [`VfxBindableValue`] — filter-parameter bindings (literal, runtime,
///   or signal).
///
/// # JSON wire format
///
/// All instantiations accept the tagged forms:
///
/// ```json
/// { "literal": <T> }
/// { "binding": "name" }
/// { "signal":  <S> }   // only inhabited when S is non-Never
/// ```
///
/// Plus a lenient bare form: a JSON value of shape `T` parses as
/// `Literal(T)`. For [`VfxBindableValue`] (where `S = SignalOrFloat`) a
/// bare object shaped like a `SignalOrFloat` (e.g. `{"sine": ...}`) parses
/// as `Signal(...)` via the `BareSignal` fallback. Plain numbers always
/// land in `Literal`, the cleaner home for static values.
///
/// Serialization always emits the tagged form.
///
/// # Signal-arm uninhabitedness
///
/// Non-signal instantiations default `S` to [`Never`], a project-local
/// uninhabited type. The `Signal` arm exists at the type level but cannot
/// be constructed; match arms over a `VfxBindableU16` therefore use
/// `Self::Signal(never) => match *never {}` for exhaustive zero-cost
/// dispatch (or `unreachable!()` if a body is needed).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    rename_all = "snake_case",
    bound(
        serialize = "T: Serialize, S: Serialize",
        deserialize = "T: Deserialize<'de> + Clone + PartialEq + 'static, \
                       S: Deserialize<'de> + Clone + PartialEq + 'static"
    ),
    from = "VfxBindableRepr<T, S>"
)]
pub enum VfxBindable<T, S = Never>
where
    T: Clone + PartialEq + 'static,
    S: Clone + PartialEq + 'static,
{
    /// A concrete literal value of type `T`.
    Literal(T),
    /// A named runtime parameter resolved per frame against a
    /// [`RuntimeParamsRead`] surface.
    Binding(String),
    /// A signal expression of type `S`. Uninhabited when `S = Never`.
    Signal(S),
}

/// Lenient on-disk representation. Variant order matters under
/// `serde(untagged)`: tagged shapes are tried first, then the bare-`T`
/// fallback for plain literals, then the bare-`S` fallback for objects
/// that match the signal payload shape (used by [`VfxBindableValue`] to
/// preserve the historical `{"sine": ...}` and `{"static": N}` parses).
///
/// When `S = Never`, `Signal` and `BareSignal` arms can never
/// successfully deserialize because `Never`'s `Deserialize` impl always
/// errors — so non-signal instantiations get the two-shape surface
/// authors expect.
#[derive(Debug, Deserialize)]
#[serde(
    untagged,
    bound(
        deserialize = "T: Deserialize<'de> + Clone + PartialEq + 'static, \
                       S: Deserialize<'de> + Clone + PartialEq + 'static"
    )
)]
enum VfxBindableRepr<T, S>
where
    T: Clone + PartialEq + 'static,
    S: Clone + PartialEq + 'static,
{
    /// `{"binding": "name"}`
    Binding { binding: String },
    /// `{"literal": <T>}`
    Literal { literal: T },
    /// `{"signal": <S>}`
    Signal { signal: S },
    /// Bare `T` value: `42`, `"hello"`, `0.5`.
    Bare(T),
    /// Bare `S` value: an object that matches the signal payload shape
    /// without an enclosing `{"signal": ...}` wrapper. Only inhabited
    /// when `S` is non-[`Never`].
    BareSignal(S),
}

impl<T, S> From<VfxBindableRepr<T, S>> for VfxBindable<T, S>
where
    T: Clone + PartialEq + 'static,
    S: Clone + PartialEq + 'static,
{
    fn from(repr: VfxBindableRepr<T, S>) -> Self {
        match repr {
            VfxBindableRepr::Binding { binding } => VfxBindable::Binding(binding),
            VfxBindableRepr::Literal { literal } => VfxBindable::Literal(literal),
            VfxBindableRepr::Signal { signal } => VfxBindable::Signal(signal),
            VfxBindableRepr::Bare(value) => VfxBindable::Literal(value),
            VfxBindableRepr::BareSignal(signal) => VfxBindable::Signal(signal),
        }
    }
}

impl<T, S> Default for VfxBindable<T, S>
where
    T: Default + Clone + PartialEq + 'static,
    S: Clone + PartialEq + 'static,
{
    /// Defaults to `Literal(T::default())`. Static values live in
    /// `Literal`, not `Signal`, even on instantiations that have a
    /// signal arm — the literal path is canonical and serializes
    /// to `{"literal": <T>}`.
    fn default() -> Self {
        Self::Literal(T::default())
    }
}

/// Blanket `From<T>` produces the [`VfxBindable::Literal`] arm. This is
/// the ergonomic on-ramp for literal construction:
/// `let v: VfxBindableU16 = 42_u16.into();`. Conversions that need to
/// land in a non-Literal arm (e.g. `From<SignalOrFloat>` collapsing
/// `Static` into Literal but real signals into Signal) are added as
/// specialized impls per instantiation.
impl<T, S> From<T> for VfxBindable<T, S>
where
    T: Clone + PartialEq + 'static,
    S: Clone + PartialEq + 'static,
{
    fn from(value: T) -> Self {
        Self::Literal(value)
    }
}

// Conditional `Eq` for instantiations where both type parameters are
// themselves `Eq`. `VfxBindable<u16>` and `VfxBindable<String>` qualify;
// `VfxBindable<f32, _>` does not (f32 is only PartialEq, never Eq).
impl<T, S> Eq for VfxBindable<T, S>
where
    T: Eq + Clone + PartialEq + 'static,
    S: Eq + Clone + PartialEq + 'static,
{
}

/// Hand-written [`ConfigSchema`]. The `#[derive(ConfigSchema)]` macro at
/// `crates/tui-vfx-core-macros/src/lib.rs` forwards generic parameters
/// but does **not** emit `where T: ConfigSchema, S: ConfigSchema` bounds
/// (sweep finding 1.9.A is the queued macro improvement). Until that
/// ships, this hand-written impl gates on the bounds explicitly and
/// consults [`BindableSignal`] so non-signal instantiations (where
/// `S = Never`) emit a two-variant schema instead of an
/// inhabited-but-unreachable three-variant one.
///
/// CONFIGSCHEMA-JUSTIFICATION: derive-cannot-handle-generic-T: the live
/// derive macro at tui-vfx-core-macros/src/lib.rs:352 forwards
/// `where_clause` verbatim and does NOT synthesize `T: ConfigSchema`
/// bounds for type parameters used in the body. Additionally, the
/// `S::signal_variant_schema()` runtime trait dispatch that conditionally
/// includes the Signal variant is not expressible in the derive's
/// compile-time emission. The macro-extension packet that would unblock
/// this is tracked separately.
impl<T, S> ConfigSchema for VfxBindable<T, S>
where
    T: ConfigSchema + Clone + PartialEq + 'static,
    S: BindableSignal,
{
    fn schema() -> SchemaNode {
        let mut variants = vec![
            SchemaVariant::Tuple {
                name: "Literal".to_string(),
                description: Some("A concrete literal value.".to_string()),
                json_value: Some("literal".to_string()),
                items: vec![SchemaField::new(
                    "value",
                    T::schema(),
                    FieldMeta {
                        help: Some("Literal value of the bindable's underlying type.".to_string()),
                        description: None,
                        default: None,
                        range: None,
                        json_key: None,
                        optional: false,
                    },
                )],
            },
            SchemaVariant::Tuple {
                name: "Binding".to_string(),
                description: Some("A named runtime parameter resolved per frame.".to_string()),
                json_value: Some("binding".to_string()),
                items: vec![SchemaField::new(
                    "name",
                    SchemaNode::Primitive {
                        type_name: "String".to_string(),
                        range: None,
                    },
                    FieldMeta {
                        help: Some("Runtime parameter name to look up.".to_string()),
                        description: None,
                        default: None,
                        range: None,
                        json_key: None,
                        optional: false,
                    },
                )],
            },
        ];
        if let Some(signal_variant) = S::signal_variant_schema() {
            variants.push(signal_variant);
        }
        SchemaNode::Enum {
            name: "VfxBindable".to_string(),
            description: Some(
                "A literal value, a named runtime binding, or (when applicable) a signal \
                 expression."
                    .to_string(),
            ),
            json_name: None,
            tag_field: None,
            variants,
        }
    }
}

// ---------------------------------------------------------------------------
// Type aliases — the consumer-facing names.
// ---------------------------------------------------------------------------

/// A bindable `u16` for cell coordinates and other integer position
/// fields. No signal arm — `S` defaults to [`Never`].
///
/// JSON shapes: `42` (bare), `{"literal": 42}`, `{"binding": "key"}`.
/// Use [`Self::evaluate`] to resolve against a [`RuntimeParamsRead`]
/// surface, [`Self::literal`] for synchronous literal-only paths.
pub type VfxBindableU16 = VfxBindable<u16>;

/// A bindable [`String`] for font / asset / locale name bindings. No
/// signal arm — `S` defaults to [`Never`].
///
/// JSON shapes: `"name"` (bare), `{"literal": "name"}`,
/// `{"binding": "key"}`. [`Self::evaluate`] returns a borrowed slice;
/// [`Self::binding_key`] enumerates references for validator passes.
pub type VfxBindableString = VfxBindable<String>;

/// A bindable `f32` filter-parameter value with a real signal arm
/// carrying [`SignalOrFloat`].
///
/// JSON shapes: `0.5` (bare → `Literal`), `{"literal": 0.5}`,
/// `{"binding": "progress"}`, `{"signal": <SignalOrFloat>}`, plus the
/// bare-signal fallback for inline `SignalSpec` payloads. Static values
/// route canonically through `Literal` rather than
/// `Signal(SignalOrFloat::Static(_))`.
pub type VfxBindableValue = VfxBindable<f32, SignalOrFloat>;

// ---------------------------------------------------------------------------
// Specialized inherent impls. Each instantiation carries its own
// `evaluate` signature and accessors; the surface is preserved verbatim
// from the three retired hand-rolled types.
// ---------------------------------------------------------------------------

impl VfxBindable<u16, Never> {
    /// Resolve to a concrete `u16`, consulting `runtime_params` for the
    /// binding case. Returns `None` if the binding is missing — callers
    /// typically `unwrap_or` a coordinate-specific default.
    pub fn evaluate<R: RuntimeParamsRead + ?Sized>(&self, runtime_params: &R) -> Option<u16> {
        match self {
            Self::Literal(value) => Some(*value),
            Self::Binding(key) => runtime_params.get_u16(key),
            Self::Signal(never) => match *never {},
        }
    }

    /// Return the literal value if this is a `Literal` variant, else
    /// `None`. Used by synchronous code paths that lack a runtime-params
    /// reference.
    pub fn literal(&self) -> Option<u16> {
        match self {
            Self::Literal(value) => Some(*value),
            Self::Binding(_) => None,
            Self::Signal(never) => match *never {},
        }
    }
}

impl VfxBindable<String, Never> {
    /// Resolve to a borrowed string slice, consulting `runtime_params` for
    /// the binding case. The returned slice borrows from either `self`
    /// (Literal) or `runtime_params` (Binding); the lifetime is the
    /// shorter of the two input borrows.
    pub fn evaluate<'a, R: RuntimeParamsRead + ?Sized>(
        &'a self,
        runtime_params: &'a R,
    ) -> Option<&'a str> {
        match self {
            Self::Literal(value) => Some(value.as_str()),
            Self::Binding(key) => runtime_params.get_text(key),
            Self::Signal(never) => match *never {},
        }
    }

    /// Return the literal value if this is a `Literal` variant, else
    /// `None`.
    pub fn literal(&self) -> Option<&str> {
        match self {
            Self::Literal(value) => Some(value.as_str()),
            Self::Binding(_) => None,
            Self::Signal(never) => match *never {},
        }
    }

    /// Return the binding key if this is a `Binding` variant, else
    /// `None`. Useful for validator passes that enumerate references
    /// without resolving them.
    pub fn binding_key(&self) -> Option<&str> {
        match self {
            Self::Binding(key) => Some(key.as_str()),
            Self::Literal(_) => None,
            Self::Signal(never) => match *never {},
        }
    }
}

impl VfxBindable<f32, SignalOrFloat> {
    /// Resolve to an `f32`, consulting the signal context for the signal
    /// case and `runtime_params` for the binding case. Returns `None` for
    /// missing bindings or signal-build failures (collapsed to a single
    /// "no value" sentinel for caller convenience).
    pub fn evaluate<R: RuntimeParamsRead + ?Sized>(
        &self,
        loop_t: f64,
        signal_ctx: &SignalContext,
        runtime_params: &R,
    ) -> Option<f32> {
        match self {
            Self::Literal(value) => Some(*value),
            Self::Binding(key) => runtime_params.get_f32(key),
            Self::Signal(signal) => signal.evaluate(loop_t, signal_ctx).ok(),
        }
    }

    /// Construct a static-literal bindable value from an `f32`. Produces
    /// a `Literal(value)` — the canonical home for static values, even
    /// on instantiations that carry a signal arm.
    pub fn static_f32(value: f32) -> Self {
        Self::Literal(value)
    }
}

/// Specialized `From<SignalOrFloat>` for [`VfxBindableValue`]. The
/// blanket `From<T>` would not apply (the input type is `S`, not `T`),
/// and we want the canonicalisation: `SignalOrFloat::Static(v)`
/// collapses to [`VfxBindable::Literal`] (the cleaner home for static
/// values), real signal expressions stay in [`VfxBindable::Signal`].
impl From<SignalOrFloat> for VfxBindable<f32, SignalOrFloat> {
    fn from(value: SignalOrFloat) -> Self {
        match value {
            SignalOrFloat::Static(v) => Self::Literal(v),
            other => Self::Signal(other),
        }
    }
}

// <FILE>crates/tui-vfx-core/src/bindable/cls_bindable.rs</FILE> - <DESC>Generic VfxBindable<T, S> envelope: Literal | Binding | Signal with S = Never default for non-signal types. Hand-written ConfigSchema gates on T: ConfigSchema and S: BindableSignal so non-signal instantiations omit a phantom Signal variant from the schema. Specialized inherent impls per (T, S) preserve the three legacy evaluate signatures without forcing a single shape across the family.</DESC>
// <VERS>END OF VERSION: 0.1.2</VERS>
