// <FILE>crates/tui-vfx-contract/src/canonicalize/cls_canonicalization_error.rs</FILE> - <DESC>Canonicalize error type with JSON-path context</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of canonicalize: error model with structured kinds and a JSON path stack for diagnostics.</WCTX>
// <CLOG>0.1.0: INIT — add structured canonicalization error type.</CLOG>

use std::fmt;

/// Error returned when authoring shorthand cannot be canonicalized.
///
/// The `path` field records where in the input JSON the failure originated
/// so authors can locate the problem without re-deriving the location from
/// the message.
#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalizationError {
    pub kind: CanonicalizationErrorKind,
    pub path: Vec<JsonPathSegment>,
    pub message: String,
}

impl CanonicalizationError {
    pub fn new(kind: CanonicalizationErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            path: Vec::new(),
            message: message.into(),
        }
    }

    pub fn at(mut self, segment: JsonPathSegment) -> Self {
        self.path.insert(0, segment);
        self
    }
}

impl fmt::Display for CanonicalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "canonicalization error")?;
        if !self.path.is_empty() {
            write!(f, " at ")?;
            for (idx, seg) in self.path.iter().enumerate() {
                match seg {
                    JsonPathSegment::Field(name) => {
                        if idx == 0 {
                            write!(f, "{name}")?;
                        } else {
                            write!(f, ".{name}")?;
                        }
                    }
                    JsonPathSegment::Index(i) => write!(f, "[{i}]")?,
                }
            }
        }
        write!(f, ": {}", self.message)
    }
}

impl std::error::Error for CanonicalizationError {}

/// One step in a JSON pointer recorded against a canonicalization failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonPathSegment {
    Field(String),
    Index(usize),
}

impl JsonPathSegment {
    pub fn field(name: impl Into<String>) -> Self {
        Self::Field(name.into())
    }
}

/// Categorical reason for a canonicalization failure.
#[derive(Debug, Clone, PartialEq)]
pub enum CanonicalizationErrorKind {
    /// Author-side spelling has no matching alias in the per-axis table.
    UnknownAlias { axis: String, from: String },
    /// Preset name has no matching expansion entry.
    UnknownPreset { axis: String, preset: String },
    /// Color value is structurally invalid (bad hex, wrong tuple length, etc.).
    MalformedColor,
    /// Color name is not in the named-color set.
    UnknownNamedColor { name: String },
    /// Duration string does not parse as `<number><suffix>` with a known suffix.
    MalformedDuration { value: String },
    /// Phase string is not one of `enter`, `dwell`, `exit`, or `all`.
    UnknownPhase { name: String },
    /// Scope shape does not match any known author-side shorthand.
    InvalidScopeShape,
    /// Binding reference does not match `$bind:<id>[?<fallback>]`.
    InvalidBindingRef { value: String },
    /// Asset reference does not match `$asset:<id>` or the object form.
    InvalidAssetRef,
    /// `extends` chain contains a cycle.
    ExtendsChainCycle { chain: Vec<String> },
    /// `extends` references a path that cannot be resolved.
    ExtendsTargetNotFound { path: String },
    /// A required field was missing from the canonicalized output.
    MissingRequired { field: String },
    /// Final type-check (serde::from_value) of the canonicalized JSON failed.
    SerdeError { underlying: String },
    /// `RecipeDocument::validate` rejected the canonical output after construction.
    ValidationFailed { underlying: String },
    /// Bare-value envelope lifting could not infer a canonical kind.
    EnvelopeLiftFailed,
    /// Author input has the wrong JSON shape for the position (object vs. array vs. scalar).
    UnexpectedJsonShape { expected: String },
    /// Internal table data is malformed; indicates a bug or table corruption.
    MalformedTable { table: String, detail: String },
    /// Author-side authoring shape is real but the canonical V3.1 contract
    /// has no path to represent it yet. Blocks the recipe with a clear
    /// reason rather than silently discarding the author's intent.
    UnsupportedShorthand { detail: String },
    /// An author-side input value cannot be carried by the canonical
    /// `NodeSpec.inputs` slot (which is `BTreeMap<EffectInputId, ValueSource>`).
    /// Structural inputs like `paths`, `stops`, `nodes`, `pattern`, and
    /// `signal` need a contract-level mechanism that does not yet exist.
    UnrepresentableInput { effect: String, param: String },
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/cls_canonicalization_error.rs</FILE> - <DESC>Canonicalize error type with JSON-path context</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
