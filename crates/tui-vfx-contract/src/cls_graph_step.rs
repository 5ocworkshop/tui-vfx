// <FILE>crates/tui-vfx-contract/src/cls_graph_step.rs</FILE> - <DESC>Canonical graph execution topology step DTO</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>New kernel Phase G3: keep topology wire fields camelCase.</WCTX>
// <CLOG>0.1.1: PATCH — serialize parallel merge policy as mergePolicy.
// 0.1.0: INIT — add schema-backed recursive topology step contract.</CLOG>

use crate::{NodeId, ParallelMergePolicy};

/// Canonical execution topology for a graph.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum GraphStep {
    /// Execute one declared graph node.
    Node {
        /// Graph-local node id to execute.
        node: NodeId,
    },
    /// Execute child steps in order, where each child reads the prior output.
    Sequence {
        /// Ordered child steps.
        children: Vec<GraphStep>,
    },
    /// Execute child branches against the same input snapshot, then merge deltas.
    Parallel {
        /// Authored child branches. Child order is deterministic merge order.
        children: Vec<GraphStep>,
        /// Policy used when multiple branches write the same cell channel.
        #[serde(rename = "mergePolicy")]
        merge_policy: ParallelMergePolicy,
    },
}

impl GraphStep {
    /// Build a topology node step.
    pub fn node(node: NodeId) -> Self {
        Self::Node { node }
    }

    /// Build a sequence step from ordered children.
    pub fn sequence(children: Vec<Self>) -> Self {
        Self::Sequence { children }
    }

    /// Build a parallel step with an explicit merge policy.
    pub fn parallel(children: Vec<Self>, merge_policy: ParallelMergePolicy) -> Self {
        Self::Parallel {
            children,
            merge_policy,
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_graph_step.rs</FILE> - <DESC>Canonical graph execution topology step DTO</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
