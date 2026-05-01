// <FILE>crates/tui-vfx-compost/src/render/cls_effect_stack.rs</FILE> - <DESC>Ordered native effect stack for compost rendering</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Effect stacks carry element write policies alongside authored effect family slots.</WCTX>
// <CLOG>0.3.0: MINOR — report applied effects after lifecycle active-node filtering.
// 0.2.0: MINOR — store cell and role write policies with the effect stack.
// 0.1.1: PATCH — read applied effect kinds directly from stored stages and remove the unused raw-stage accessor.
// 0.1.0: INIT — add ordered effect stage container and family-slot views.</CLOG>

use tui_vfx_contract::{CellWritePolicy, RoleWritePolicy};

use crate::render::{EffectFamily, EffectStage, SampleContext, is_node_active};

#[derive(Clone, Debug)]
pub(crate) struct EffectStack<'a> {
    stages: Vec<EffectStage<'a>>,
    cell_write_policy: CellWritePolicy,
    role_write_policy: RoleWritePolicy,
}

impl<'a> EffectStack<'a> {
    pub(crate) fn new(
        stages: Vec<EffectStage<'a>>,
        cell_write_policy: CellWritePolicy,
        role_write_policy: RoleWritePolicy,
    ) -> Self {
        Self {
            stages,
            cell_write_policy,
            role_write_policy,
        }
    }

    pub(crate) fn cell_write_policy(&self) -> CellWritePolicy {
        self.cell_write_policy
    }

    pub(crate) fn role_write_policy(&self) -> &RoleWritePolicy {
        &self.role_write_policy
    }

    pub(crate) fn content_stages(&self) -> impl Iterator<Item = EffectStage<'a>> + '_ {
        self.stages_for_family(EffectFamily::Content)
    }

    pub(crate) fn style_stages(&self) -> impl Iterator<Item = EffectStage<'a>> + '_ {
        self.stages_for_family(EffectFamily::Style)
    }

    pub(crate) fn sampler_stages(&self) -> impl Iterator<Item = EffectStage<'a>> + '_ {
        self.stages_for_family(EffectFamily::Sampler)
    }

    pub(crate) fn mask_stages(&self) -> impl Iterator<Item = EffectStage<'a>> + '_ {
        self.stages_for_family(EffectFamily::Mask)
    }

    pub(crate) fn shader_stages(&self) -> impl Iterator<Item = EffectStage<'a>> + '_ {
        self.stages_for_family(EffectFamily::Shader)
    }

    pub(crate) fn filter_stages(&self) -> impl Iterator<Item = EffectStage<'a>> + '_ {
        self.stages_for_family(EffectFamily::Filter)
    }

    fn stages_for_family(
        &self,
        family: EffectFamily,
    ) -> impl Iterator<Item = EffectStage<'a>> + '_ {
        self.stages
            .iter()
            .copied()
            .filter(move |stage| stage.family() == family)
    }

    pub(crate) fn applied_effect_kinds(&self, sample: &SampleContext) -> Vec<String> {
        self.stages
            .iter()
            .filter(|stage| is_node_active(stage.node(), sample))
            .map(|stage| stage.node().effect.as_str().to_string())
            .collect()
    }
}

// <FILE>crates/tui-vfx-compost/src/render/cls_effect_stack.rs</FILE> - <DESC>Ordered native effect stack for compost rendering</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
