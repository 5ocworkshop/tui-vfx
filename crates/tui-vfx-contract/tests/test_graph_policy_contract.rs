// <FILE>crates/tui-vfx-contract/tests/test_graph_policy_contract.rs</FILE> - <DESC>Canonical graph policy, order, and binding validation tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase G3: prove topology validation while preserving linear order fallback.</WCTX>
// <CLOG>0.2.0: MINOR — add unknown, duplicate, and coverage topology tests.
// 0.1.0: INIT — lock policy support, deterministic ordering, and F2 binding reuse.</CLOG>

mod support;

use support::{base_graph, binding_to, literal_source, signal_source};
use tui_vfx_contract::{
    CellWritePolicy, DescriptorValidationError, GraphSpec, GraphStep, NodeId, NodeSpec,
    ParallelMergePolicy, RoleWritePolicy, RoleWritePolicyKind, ScopeKind, ScopeSpec,
};
use tui_vfx_types::{Rect, RoleTag};

#[test]
fn graph_defaults_empty_collections_and_omits_topology() {
    let graph: GraphSpec = serde_json::from_value(serde_json::json!({
        "id": "emptyGraph",
        "version": "3.1"
    }))
    .expect("graph with omitted empty collections deserializes");

    let explicit_empty: GraphSpec = serde_json::from_value(serde_json::json!({
        "id": "emptyGraph",
        "version": "3.1",
        "parameters": {},
        "signals": {},
        "bindings": [],
        "effects": {},
        "nodes": {},
        "order": [],
        "topology": null
    }))
    .expect("graph with explicit empty collections and null topology deserializes");

    assert_eq!(graph, explicit_empty);
    assert!(graph.parameters.is_empty());
    assert!(graph.signals.is_empty());
    assert!(graph.bindings.is_empty());
    assert!(graph.effects.is_empty());
    assert!(graph.nodes.is_empty());
    assert!(graph.order.is_empty());
    assert_eq!(graph.topology, None);

    let json = serde_json::to_value(&graph).expect("graph serializes");
    assert!(json.get("parameters").is_none());
    assert!(json.get("signals").is_none());
    assert!(json.get("bindings").is_none());
    assert!(json.get("effects").is_none());
    assert!(json.get("nodes").is_none());
    assert!(json.get("order").is_none());
    assert!(json.get("topology").is_none());
}

#[test]
fn node_optional_scope_and_write_policies_accept_null_or_omission() {
    let omitted: NodeSpec = serde_json::from_value(serde_json::json!({
        "id": "fadeIn",
        "effect": "terminal.opacity",
        "inputs": {}
    }))
    .expect("node with omitted optional policies deserializes");

    assert_eq!(omitted.scope, None);
    assert_eq!(omitted.cell_write_policy, None);
    assert_eq!(omitted.role_write_policy, None);

    let explicit_null: NodeSpec = serde_json::from_value(serde_json::json!({
        "id": "fadeIn",
        "effect": "terminal.opacity",
        "inputs": {},
        "scope": null,
        "cellWritePolicy": null,
        "roleWritePolicy": null
    }))
    .expect("node with explicit null optional policies deserializes");

    assert_eq!(omitted, explicit_null);

    let json = serde_json::to_value(&omitted).expect("node serializes");
    assert!(json.get("scope").is_none());
    assert!(json.get("cellWritePolicy").is_none());
    assert!(json.get("roleWritePolicy").is_none());
}

#[test]
fn graph_rejects_unsupported_scope_for_effect() {
    let mut graph = base_graph(literal_source());
    graph.nodes.get_mut(&NodeId::new("fadeIn")).unwrap().scope = Some(ScopeSpec::Rect {
        rect: Rect::new(0, 0, 2, 2),
    });

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::UnsupportedScopeKind {
            requested: ScopeKind::Rect
        })
    ));
}

#[test]
fn graph_rejects_unsupported_cell_write_policy_for_effect() {
    let mut graph = base_graph(literal_source());
    graph
        .nodes
        .get_mut(&NodeId::new("fadeIn"))
        .unwrap()
        .cell_write_policy = Some(CellWritePolicy::SkipTransparentEmpty);

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::UnsupportedCellWritePolicy {
            requested: CellWritePolicy::SkipTransparentEmpty
        })
    ));
}

#[test]
fn graph_rejects_unsupported_role_write_policy_for_effect() {
    let mut graph = base_graph(literal_source());
    graph
        .nodes
        .get_mut(&NodeId::new("fadeIn"))
        .unwrap()
        .role_write_policy = Some(RoleWritePolicy::SetExplicit {
        role: RoleTag::Shadow,
    });

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::UnsupportedRoleWritePolicy {
            requested: RoleWritePolicyKind::SetExplicit
        })
    ));
}

#[test]
fn graph_rejects_order_reference_to_unknown_node() {
    let mut graph = base_graph(literal_source());
    graph.order = vec![NodeId::new("fadeIn"), NodeId::new("missing")];

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::UnknownOrderNode { id }) if id.as_str() == "missing"
    ));
}

#[test]
fn graph_rejects_duplicate_order_entries() {
    let mut graph = base_graph(literal_source());
    graph.order = vec![NodeId::new("fadeIn"), NodeId::new("fadeIn")];

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::DuplicateOrderNode { id }) if id.as_str() == "fadeIn"
    ));
}

#[test]
fn graph_rejects_node_missing_from_order() {
    let mut graph = base_graph(literal_source());
    graph.order.clear();

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::NodeMissingFromOrder { id }) if id.as_str() == "fadeIn"
    ));
}

#[test]
fn graph_accepts_explicit_sequence_topology() {
    let mut graph = base_graph(literal_source());
    graph.topology = Some(GraphStep::sequence(vec![GraphStep::node(NodeId::new(
        "fadeIn",
    ))]));

    assert!(graph.validate().is_ok());
}

#[test]
fn graph_accepts_explicit_parallel_topology() {
    let mut graph = base_graph(literal_source());
    graph.topology = Some(GraphStep::parallel(
        vec![GraphStep::node(NodeId::new("fadeIn"))],
        ParallelMergePolicy::ChildOrderLastWriterWins,
    ));

    assert!(graph.validate().is_ok());
}

#[test]
fn graph_rejects_topology_reference_to_unknown_node() {
    let mut graph = base_graph(literal_source());
    graph.topology = Some(GraphStep::node(NodeId::new("missing")));

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::UnknownOrderNode { id }) if id.as_str() == "missing"
    ));
}

#[test]
fn graph_rejects_duplicate_topology_node_entries() {
    let mut graph = base_graph(literal_source());
    graph.topology = Some(GraphStep::sequence(vec![
        GraphStep::node(NodeId::new("fadeIn")),
        GraphStep::node(NodeId::new("fadeIn")),
    ]));

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::DuplicateOrderNode { id }) if id.as_str() == "fadeIn"
    ));
}

#[test]
fn graph_rejects_node_missing_from_topology() {
    let mut graph = base_graph(literal_source());
    graph.topology = Some(GraphStep::sequence(vec![]));

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::NodeMissingFromOrder { id }) if id.as_str() == "fadeIn"
    ));
}

#[test]
fn graph_accepts_f2_parameter_binding() {
    let mut graph = base_graph(literal_source());
    graph.bindings = vec![binding_to("opacity", signal_source("audioLevel"))];

    assert!(graph.validate().is_ok());
}

#[test]
fn graph_rejects_binding_to_unknown_parameter() {
    let mut graph = base_graph(literal_source());
    graph.bindings = vec![binding_to("missing", signal_source("audioLevel"))];

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::UnknownBindingParameterTarget { id })
            if id.as_str() == "missing"
    ));
}

#[test]
fn graph_rejects_binding_to_non_bindable_parameter() {
    let mut graph = base_graph(literal_source());
    graph.bindings = vec![binding_to("locked", signal_source("audioLevel"))];

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::ParameterNotBindable { id }) if id.as_str() == "locked"
    ));
}

#[test]
fn g1_does_not_add_runtime_or_recipe_execution() {
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut combined = String::new();
    for entry in std::fs::read_dir(src_dir).expect("contract src can be read") {
        let path = entry.expect("entry can be read").path();
        if path.extension().is_some_and(|extension| extension == "rs") {
            combined.push_str(&std::fs::read_to_string(path).expect("contract source can be read"));
        }
    }

    assert!(!combined.contains("ParameterStore"));
    assert!(!combined.contains("SignalStore"));
    assert!(!combined.contains("RuntimeBinding"));
    assert!(!combined.contains("RecipeCompiler"));
    assert!(!combined.contains("TriggerEngine"));
}

// <FILE>crates/tui-vfx-contract/tests/test_graph_policy_contract.rs</FILE> - <DESC>Canonical graph policy, order, and binding validation tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
