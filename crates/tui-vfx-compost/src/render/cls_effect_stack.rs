// <FILE>crates/tui-vfx-compost/src/render/cls_effect_stack.rs</FILE> - <DESC>Ordered native effect stack for compost rendering</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Effect stacks carry element write policies and graph topology alongside authored effect family slots.</WCTX>
// <CLOG>0.6.0: MINOR — expose synthetic graph-stage counts for disjoint trace identity.
// 0.5.0: MINOR — expose stable authored stage indices for trace evidence.
// 0.4.1: PATCH — expose stage count for synthetic trace event ordering.
// 0.4.0: PATCH — leave applied-effect evidence to scope-aware observability.
// 0.3.0: MINOR — report applied effects after lifecycle active-node filtering.
// 0.2.0: MINOR — store cell and role write policies with the effect stack.
// 0.1.1: PATCH — read applied effect kinds directly from stored stages and remove the unused raw-stage accessor.
// 0.1.0: INIT — add ordered effect stage container and family-slot views.</CLOG>

use tui_vfx_contract::{CellWritePolicy, GraphStep, NodeId, RoleWritePolicy};

use crate::render::{EffectFamily, EffectStage};

#[derive(Clone, Debug)]
pub(crate) struct EffectStack<'a> {
    stages: Vec<EffectStage<'a>>,
    cell_write_policy: CellWritePolicy,
    role_write_policy: RoleWritePolicy,
    topology: Option<&'a GraphStep>,
}

impl<'a> EffectStack<'a> {
    pub(crate) fn new(
        stages: Vec<EffectStage<'a>>,
        cell_write_policy: CellWritePolicy,
        role_write_policy: RoleWritePolicy,
        topology: Option<&'a GraphStep>,
    ) -> Self {
        Self {
            stages,
            cell_write_policy,
            role_write_policy,
            topology,
        }
    }

    pub(crate) fn topology(&self) -> Option<&'a GraphStep> {
        self.topology
    }

    pub(crate) fn indexed_stages(&self) -> impl Iterator<Item = (usize, EffectStage<'a>)> + '_ {
        self.stages.iter().copied().enumerate()
    }

    pub(crate) fn stage_for_node(&self, node: &NodeId) -> Option<EffectStage<'a>> {
        self.stages
            .iter()
            .copied()
            .find(|stage| stage.node_id() == node)
    }

    pub(crate) fn stage_index_for_node(&self, node: &NodeId) -> Option<usize> {
        self.stages.iter().position(|stage| stage.node_id() == node)
    }

    pub(crate) fn stage_count(&self) -> usize {
        self.stages.len()
    }

    pub(crate) fn synthetic_graph_stage_count(&self) -> usize {
        self.topology.map(count_synthetic_graph_stages).unwrap_or(0)
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
}

fn count_synthetic_graph_stages(step: &GraphStep) -> usize {
    match step {
        GraphStep::Node { .. } => 0,
        GraphStep::Sequence { children } => children.iter().map(count_synthetic_graph_stages).sum(),
        GraphStep::Parallel { children, .. } => {
            1 + children
                .iter()
                .map(count_synthetic_graph_stages)
                .sum::<usize>()
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/cls_effect_stack.rs</FILE> - <DESC>Ordered native effect stack for compost rendering</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
