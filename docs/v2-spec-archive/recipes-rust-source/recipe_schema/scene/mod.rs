// <FILE>src/recipe_schema/scene/mod.rs</FILE> - <DESC>Scene-schema module for recipe-authored semantic layers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive scene-bearing recipe schema with string-friendly RoleTag/LayerId/Anchor serde helpers and dedicated OFPF files per scene type.</WCTX>
// <CLOG>0.1.0: add scene schema module, re-exports, and serde helpers for role-tag/layer-id/anchor fields.</CLOG>

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serializer};
use tui_vfx_geometry::types::Anchor;
use tui_vfx_types::{LayerId, RoleTag};

mod cls_ra_card_source;
mod cls_ra_image_source;
mod cls_ra_procedural_source;
mod cls_ra_scene_config;
mod cls_ra_scene_layer;
mod cls_ra_text_source;
mod enum_ra_content_source;
mod enum_ra_layer_overflow;
mod enum_ra_layer_placement;
mod enum_ra_layer_visibility;
mod enum_ra_scene_fit_policy;

pub use cls_ra_card_source::RaCardSource;
pub use cls_ra_image_source::{RaImageAspect, RaImageSource};
pub use cls_ra_procedural_source::RaProceduralSource;
pub use cls_ra_scene_config::RaSceneConfig;
pub use cls_ra_scene_layer::RaSceneLayer;
pub use cls_ra_text_source::{RaTextAlignment, RaTextSource};
pub use enum_ra_content_source::RaContentSource;
pub use enum_ra_layer_overflow::RaLayerOverflow;
pub use enum_ra_layer_placement::{RaAbsolutePlacement, RaAnchoredPlacement, RaLayerPlacement};
pub use enum_ra_layer_visibility::RaLayerVisibility;
pub use enum_ra_scene_fit_policy::RaSceneFitPolicy;

pub(crate) fn default_background_role() -> RoleTag {
    RoleTag::Background
}

pub(crate) fn deserialize_role_tag<'de, D>(deserializer: D) -> Result<RoleTag, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    Ok(RoleTag::from_shorthand(&value))
}

pub(crate) fn serialize_role_tag<S>(role: &RoleTag, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&role.shorthand_name())
}

pub(crate) fn deserialize_layer_id<'de, D>(deserializer: D) -> Result<LayerId, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(LayerId::from(String::deserialize(deserializer)?))
}

pub(crate) fn serialize_layer_id<S>(layer_id: &LayerId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(layer_id.as_str())
}

fn anchor_from_shorthand(value: &str) -> Option<Anchor> {
    match value {
        "top_left" => Some(Anchor::TopLeft),
        "top_center" => Some(Anchor::TopCenter),
        "top_right" => Some(Anchor::TopRight),
        "center_left" => Some(Anchor::MiddleLeft),
        "center" => Some(Anchor::Center),
        "center_right" => Some(Anchor::MiddleRight),
        "bottom_left" => Some(Anchor::BottomLeft),
        "bottom_center" => Some(Anchor::BottomCenter),
        "bottom_right" => Some(Anchor::BottomRight),
        _ => None,
    }
}

fn anchor_shorthand(anchor: Anchor) -> &'static str {
    match anchor {
        Anchor::TopLeft => "top_left",
        Anchor::TopCenter => "top_center",
        Anchor::TopRight => "top_right",
        Anchor::MiddleLeft => "center_left",
        Anchor::Center => "center",
        Anchor::MiddleRight => "center_right",
        Anchor::BottomLeft => "bottom_left",
        Anchor::BottomCenter => "bottom_center",
        Anchor::BottomRight => "bottom_right",
        _ => "center",
    }
}

pub(crate) fn deserialize_anchor<'de, D>(deserializer: D) -> Result<Anchor, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    anchor_from_shorthand(&value)
        .ok_or_else(|| D::Error::custom(format!("unknown anchor shorthand: {value}")))
}

pub(crate) fn serialize_anchor<S>(anchor: &Anchor, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(anchor_shorthand(*anchor))
}

// <FILE>src/recipe_schema/scene/mod.rs</FILE> - <DESC>Scene-schema module</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
