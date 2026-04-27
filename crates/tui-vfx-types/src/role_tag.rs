// <FILE>crates/tui-vfx-types/src/role_tag.rs</FILE> - <DESC>Per-cell semantic role tag enum (12 first-class variants + Custom)</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Packet 1.9.A.followup US-008: lift the hand-written RoleTag ConfigSchema impl with intentional-divergence-from-derive-output justification. The Custom(InternedRoleName) field's hand-emitted String::schema() shape is a deliberate simplification.</WCTX>
// <CLOG>0.1.1: PATCH — add CONFIGSCHEMA-JUSTIFICATION comment above the hand-written impl (kind=intentional-divergence-from-derive-output). Comment documents the Custom-variant field flatten that derive cannot replicate without trading divergences. No behavior change.</CLOG>

//! Per-cell semantic role tag.
//!
//! A `RoleTag` annotates a single cell of a `SemanticScene` with its semantic
//! role (background, text, title, border, …) so downstream per-cell pipeline
//! stages (shaders, masks, samplers, shadow extrusion) can target cells by
//! role rather than by guessing from glyph content.
//!
//! Twelve **first-class** variants are reserved for the common roles that
//! recipe authoring and the rest of the tui-vfx ecosystem can rely on by
//! name. Any other role name is available via `Custom(InternedRoleName)`
//! for ad-hoc semantic tagging (e.g. `"logo_silhouette"`,
//! `"card_inner_glow"`) without runtime cost.
//!
//! # Shorthand schema
//!
//! Recipes and human-readable schemas often use a single-word shorthand
//! string. The canonical schema:
//!
//! - lowercase variant name (e.g. `"background"`, `"text"`, `"procedural"`)
//!   resolves to the matching first-class variant.
//! - Any name not matching a first-class variant resolves to
//!   `Custom(InternedRoleName::new(name))`.
//! - The explicit `"custom:<name>"` prefix is also accepted for clarity.
//!
//! ```
//! use tui_vfx_types::{InternedRoleName, RoleTag};
//!
//! assert_eq!(RoleTag::from_shorthand("border"), RoleTag::Border);
//! assert_eq!(
//!     RoleTag::from_shorthand("my_custom_role"),
//!     RoleTag::Custom(InternedRoleName::new("my_custom_role")),
//! );
//! assert_eq!(
//!     RoleTag::from_shorthand("custom:my_custom_role"),
//!     RoleTag::Custom(InternedRoleName::new("my_custom_role")),
//! );
//! ```

use crate::InternedString;
use tui_vfx_core::{ConfigSchema, FieldMeta, SchemaField, SchemaNode, SchemaVariant};

/// Opaque interned name for a `RoleTag::Custom` variant.
///
/// A newtype wrapper around `InternedString` so custom role names are
/// distinct at the type level from other interned identifiers
/// (`LayerId`, `RecipeId`). Foreign code can read the inner name via
/// `as_str()` but cannot accidentally substitute a `LayerId` for a role
/// name or vice versa.
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct InternedRoleName(InternedString);

impl InternedRoleName {
    /// Construct an interned role name from a `&str`.
    pub fn new(s: &str) -> Self {
        Self(InternedString::new(s))
    }

    /// Borrow the role name as a `&str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for InternedRoleName {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for InternedRoleName {
    fn from(s: String) -> Self {
        Self(InternedString::from(s))
    }
}

/// Per-cell semantic role tag.
///
/// Twelve first-class variants cover the common roles. `Custom(name)` covers
/// ad-hoc recipe-declared roles. `#[non_exhaustive]` preserves room for
/// future first-class additions without a breaking change.
///
/// Every first-class variant has a stable numeric ID in `RoleInterner`
/// (Background=0, Text=1, …, Procedural=11); Custom IDs start at 12.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RoleTag {
    /// Fill cells (no glyph or styled space).
    Background,
    /// Primary content text.
    Text,
    /// Title / header text.
    Title,
    /// Secondary / sub text.
    Caption,
    /// Structural / decorative borders.
    Border,
    /// Image / pixel-art / glyph-art cells.
    Image,
    /// Small icon glyphs.
    Icon,
    /// Status / focus / selection markers.
    Indicator,
    /// Emphasized content (search hits, etc.).
    Highlight,
    /// Shadow content — source layer OR pipeline output.
    Shadow,
    /// Ornamental cells (dividers, ornaments).
    Decoration,
    /// Cells produced by a procedural source.
    Procedural,
    /// Author-declared custom role (e.g. `"logo_silhouette"`).
    Custom(InternedRoleName),
}

// CONFIGSCHEMA-JUSTIFICATION: intentional-divergence-from-derive-output: the Custom(InternedRoleName) variant's field is hand-emitted as `String::schema()` to give consumers a string-shaped tooltip, even though the actual field type is the InternedRoleName newtype. Migrating to #[derive(ConfigSchema)] would either (a) require an `impl ConfigSchema for InternedRoleName` (replacing one hand-written impl with another) or (b) emit `SchemaNode::Opaque { type_name: "InternedRoleName" }` via #[config(opaque)] — which changes the schema's representation of the Custom field. Both options trade one form of divergence for another; the existing hand-written impl is the deliberate simplification.
impl ConfigSchema for RoleTag {
    fn schema() -> SchemaNode {
        SchemaNode::Enum {
            name: "RoleTag".to_string(),
            description: Some("Per-cell semantic role tag".to_string()),
            json_name: None,
            tag_field: None,
            variants: vec![
                SchemaVariant::Unit {
                    name: "Background".to_string(),
                    description: Some("Fill/background cells".to_string()),
                    json_value: Some("background".to_string()),
                },
                SchemaVariant::Unit {
                    name: "Text".to_string(),
                    description: Some("Primary text cells".to_string()),
                    json_value: Some("text".to_string()),
                },
                SchemaVariant::Unit {
                    name: "Title".to_string(),
                    description: Some("Title/header cells".to_string()),
                    json_value: Some("title".to_string()),
                },
                SchemaVariant::Unit {
                    name: "Caption".to_string(),
                    description: Some("Caption/secondary text cells".to_string()),
                    json_value: Some("caption".to_string()),
                },
                SchemaVariant::Unit {
                    name: "Border".to_string(),
                    description: Some("Border cells".to_string()),
                    json_value: Some("border".to_string()),
                },
                SchemaVariant::Unit {
                    name: "Image".to_string(),
                    description: Some("Image/pixel-art cells".to_string()),
                    json_value: Some("image".to_string()),
                },
                SchemaVariant::Unit {
                    name: "Icon".to_string(),
                    description: Some("Icon cells".to_string()),
                    json_value: Some("icon".to_string()),
                },
                SchemaVariant::Unit {
                    name: "Indicator".to_string(),
                    description: Some("Indicator/status cells".to_string()),
                    json_value: Some("indicator".to_string()),
                },
                SchemaVariant::Unit {
                    name: "Highlight".to_string(),
                    description: Some("Highlight/emphasis cells".to_string()),
                    json_value: Some("highlight".to_string()),
                },
                SchemaVariant::Unit {
                    name: "Shadow".to_string(),
                    description: Some("Shadow-region cells".to_string()),
                    json_value: Some("shadow".to_string()),
                },
                SchemaVariant::Unit {
                    name: "Decoration".to_string(),
                    description: Some("Decorative cells".to_string()),
                    json_value: Some("decoration".to_string()),
                },
                SchemaVariant::Unit {
                    name: "Procedural".to_string(),
                    description: Some("Procedural-source cells".to_string()),
                    json_value: Some("procedural".to_string()),
                },
                SchemaVariant::Tuple {
                    name: "Custom".to_string(),
                    description: Some("Custom semantic role".to_string()),
                    json_value: Some("custom".to_string()),
                    items: vec![SchemaField::new(
                        "name",
                        String::schema(),
                        FieldMeta {
                            description: Some("Custom role name".to_string()),
                            ..Default::default()
                        },
                    )],
                },
            ],
        }
    }
}

impl RoleTag {
    /// All first-class variants in declaration order.
    ///
    /// Declaration order is stable: it defines the numeric IDs used by
    /// `RoleInterner` (Background=0, …, Procedural=11).
    pub const FIRST_CLASS: [RoleTag; 12] = [
        RoleTag::Background,
        RoleTag::Text,
        RoleTag::Title,
        RoleTag::Caption,
        RoleTag::Border,
        RoleTag::Image,
        RoleTag::Icon,
        RoleTag::Indicator,
        RoleTag::Highlight,
        RoleTag::Shadow,
        RoleTag::Decoration,
        RoleTag::Procedural,
    ];

    /// Parse a shorthand role string into a `RoleTag`.
    ///
    /// - Lowercase variant name → first-class variant.
    /// - `"custom:<name>"` → `Custom(InternedRoleName::new(<name>))`.
    /// - Any other string → `Custom(InternedRoleName::new(<name>))`.
    ///
    /// See the module docs for schema details.
    pub fn from_shorthand(s: &str) -> RoleTag {
        match s {
            "background" => RoleTag::Background,
            "text" => RoleTag::Text,
            "title" => RoleTag::Title,
            "caption" => RoleTag::Caption,
            "border" => RoleTag::Border,
            "image" => RoleTag::Image,
            "icon" => RoleTag::Icon,
            "indicator" => RoleTag::Indicator,
            "highlight" => RoleTag::Highlight,
            "shadow" => RoleTag::Shadow,
            "decoration" => RoleTag::Decoration,
            "procedural" => RoleTag::Procedural,
            other => {
                let name = other.strip_prefix("custom:").unwrap_or(other);
                RoleTag::Custom(InternedRoleName::new(name))
            }
        }
    }

    /// Return the canonical shorthand name for this role.
    ///
    /// For first-class variants this is the lowercase variant name
    /// (`"background"`, `"text"`, …). For `Custom(name)` this is the
    /// inner name (without any `custom:` prefix), which round-trips
    /// through `from_shorthand` as long as the name does not collide
    /// with a first-class shorthand.
    pub fn shorthand_name(&self) -> String {
        match self {
            RoleTag::Background => "background".to_string(),
            RoleTag::Text => "text".to_string(),
            RoleTag::Title => "title".to_string(),
            RoleTag::Caption => "caption".to_string(),
            RoleTag::Border => "border".to_string(),
            RoleTag::Image => "image".to_string(),
            RoleTag::Icon => "icon".to_string(),
            RoleTag::Indicator => "indicator".to_string(),
            RoleTag::Highlight => "highlight".to_string(),
            RoleTag::Shadow => "shadow".to_string(),
            RoleTag::Decoration => "decoration".to_string(),
            RoleTag::Procedural => "procedural".to_string(),
            RoleTag::Custom(name) => name.as_str().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_class_array_has_twelve_variants() {
        assert_eq!(RoleTag::FIRST_CLASS.len(), 12);
    }

    #[test]
    fn shorthand_round_trip_all_first_class() {
        for tag in RoleTag::FIRST_CLASS.iter() {
            let s = tag.shorthand_name();
            assert_eq!(&RoleTag::from_shorthand(&s), tag);
        }
    }
}

// <FILE>crates/tui-vfx-types/src/role_tag.rs</FILE> - <DESC>Per-cell semantic role tag enum</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
