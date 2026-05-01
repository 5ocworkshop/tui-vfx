// <FILE>crates/tui-vfx-compost/src/render/cls_effect_stage.rs</FILE> - <DESC>Native effect stage classification for compost rendering</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Effect stack substrate classifies canonical v3.1 effect nodes without non-native render DTOs.</WCTX>
// <CLOG>0.1.0: INIT — add native effect family and stage metadata.</CLOG>

use tui_vfx_contract::{NodeId, NodeSpec};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EffectFamily {
    Content,
    Style,
    Shader,
    Filter,
    Mask,
    Sampler,
    Unknown,
}

impl EffectFamily {
    pub(crate) fn from_effect_id(effect: &str) -> Self {
        match effect.split_once('.').map(|(family, _)| family) {
            Some("content") => Self::Content,
            Some("style") => Self::Style,
            Some("shader") => Self::Shader,
            Some("filter") => Self::Filter,
            Some("mask") => Self::Mask,
            Some("sampler") => Self::Sampler,
            _ => Self::Unknown,
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Content => "content",
            Self::Style => "style",
            Self::Shader => "shader",
            Self::Filter => "filter",
            Self::Mask => "mask",
            Self::Sampler => "sampler",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct EffectStage<'a> {
    node_id: &'a NodeId,
    node: &'a NodeSpec,
    family: EffectFamily,
}

impl<'a> EffectStage<'a> {
    pub(crate) fn new(node_id: &'a NodeId, node: &'a NodeSpec) -> Self {
        Self {
            node_id,
            node,
            family: EffectFamily::from_effect_id(node.effect.as_str()),
        }
    }

    pub(crate) fn node_id(&self) -> &'a NodeId {
        self.node_id
    }

    pub(crate) fn node(&self) -> &'a NodeSpec {
        self.node
    }

    pub(crate) fn family(&self) -> EffectFamily {
        self.family
    }
}

// <FILE>crates/tui-vfx-compost/src/render/cls_effect_stage.rs</FILE> - <DESC>Native effect stage classification for compost rendering</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
