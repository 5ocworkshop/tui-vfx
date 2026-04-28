// <FILE>crates/tui-vfx-contract/src/cls_value.rs</FILE> - <DESC>Tagged effect input literal value DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F1: represent typed canonical effect input literals.</WCTX>
// <CLOG>0.1.0: INIT — add strict tagged Value enum with schema-backed payloads.</CLOG>

use tui_vfx_types::{Color, Rect, RoleTag};

use crate::{ScopeSpec, ValueKind};

/// Canonical tagged literal value used by effect input specs.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "camelCase",
    deny_unknown_fields
)]
pub enum Value {
    /// Explicit absence of a value.
    Null,
    /// Boolean true/false literal.
    Boolean(
        /// Boolean payload.
        bool,
    ),
    /// Signed integer literal without a fractional component.
    Integer(
        /// Integer payload.
        i64,
    ),
    /// Floating-point numeric literal.
    Number(
        /// Number payload.
        f64,
    ),
    /// Short machine-facing string literal.
    String(
        /// String payload.
        String,
    ),
    /// Human-facing text literal.
    Text(
        /// Text payload.
        String,
    ),
    /// RGBA color literal.
    Color(
        /// Color payload.
        Color,
    ),
    /// Duration literal expressed as seconds.
    Duration(
        /// Duration payload in seconds.
        f64,
    ),
    /// Closed-set enum literal.
    Enum(
        /// Selected enum value.
        String,
    ),
    /// Semantic role literal.
    Role(
        /// Role payload.
        RoleTag,
    ),
    /// Surface scope literal.
    Scope(
        /// Scope payload.
        ScopeSpec,
    ),
    /// Cell rectangle literal.
    Rect(
        /// Rectangle payload.
        Rect,
    ),
}

impl Value {
    /// Return the closed kind tag for this literal value.
    pub const fn kind(&self) -> ValueKind {
        match self {
            Self::Null => ValueKind::Null,
            Self::Boolean(_) => ValueKind::Boolean,
            Self::Integer(_) => ValueKind::Integer,
            Self::Number(_) => ValueKind::Number,
            Self::String(_) => ValueKind::String,
            Self::Text(_) => ValueKind::Text,
            Self::Color(_) => ValueKind::Color,
            Self::Duration(_) => ValueKind::Duration,
            Self::Enum(_) => ValueKind::Enum,
            Self::Role(_) => ValueKind::Role,
            Self::Scope(_) => ValueKind::Scope,
            Self::Rect(_) => ValueKind::Rect,
        }
    }

    /// Return this value as an f64 when its kind supports numeric range checks.
    pub const fn as_range_number(&self) -> Option<f64> {
        match self {
            Self::Integer(value) => Some(*value as f64),
            Self::Number(value) | Self::Duration(value) => Some(*value),
            _ => None,
        }
    }

    /// Return this value as an enum string when it is an enum literal.
    pub fn as_enum_value(&self) -> Option<&str> {
        match self {
            Self::Enum(value) => Some(value.as_str()),
            _ => None,
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_value.rs</FILE> - <DESC>Tagged effect input literal value DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
