// <FILE>crates/tui-vfx-next/tests/test_graph_execution_order.rs</FILE> - <DESC>Graph execution order, scope, adapter, and binding proof tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G2: prove ordered node execution and boundary failures.</WCTX>
// <CLOG>0.1.0: INIT — add graph order, prior-node visibility, adapter, validation, and binding tests.</CLOG>

mod support;

use support::{binding_to, dim_node, glyph_node, graph, role_node, surface_with_cell};
use tui_vfx_next::{
    CellWritePolicy, DescriptorValidationError, EffectId, GraphExecutionContext,
    GraphExecutionError, GraphExecutor, NodeId, ParameterId, RoleWritePolicy, ScopeSpec, SignalId,
    Surface, Value, ValueSource,
};
use tui_vfx_types::RoleTag;

#[test]
fn graph_executor_order_is_node_order() {
    let mut graph = graph(vec![
        glyph_node(
            "first",
            ValueSource::Literal {
                value: Value::Text("A".to_string()),
            },
        ),
        glyph_node(
            "second",
            ValueSource::Literal {
                value: Value::Text("B".to_string()),
            },
        ),
    ]);
    let first = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('x', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("ordered graph executes");

    graph.order = vec![NodeId::new("second"), NodeId::new("first")];
    let reversed = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('x', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("reversed graph executes");

    assert_eq!(first.surface.cell(0, 0).unwrap().ch, 'B');
    assert_eq!(reversed.surface.cell(0, 0).unwrap().ch, 'A');
}

#[test]
fn graph_executor_later_node_sees_prior_node_role() {
    let mut scoped = glyph_node(
        "scopedGlyph",
        ValueSource::Literal {
            value: Value::Text("S".to_string()),
        },
    );
    scoped.scope = Some(ScopeSpec::Role {
        role: RoleTag::Shadow,
    });
    let graph = graph(vec![role_node("markShadow", RoleTag::Shadow), scoped]);

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('x', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("role visibility graph executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'S');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Shadow));
}

#[test]
fn graph_executor_reuses_scope_diagnostics() {
    let mut scoped = glyph_node(
        "miss",
        ValueSource::Literal {
            value: Value::Text("S".to_string()),
        },
    );
    scoped.scope = Some(ScopeSpec::Role {
        role: RoleTag::Shadow,
    });
    let graph = graph(vec![scoped]);

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('x', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("zero-scope graph executes with diagnostic");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'x');
    assert_eq!(outcome.diagnostics.len(), 1);
    assert_eq!(
        outcome.diagnostics[0].path.as_deref(),
        Some("graph.node[0].miss")
    );
}

#[test]
fn graph_executor_reuses_write_policy_semantics() {
    let mut node = dim_node(
        "skipEmpty",
        ValueSource::Literal {
            value: Value::Number(0.5),
        },
    );
    node.cell_write_policy = Some(CellWritePolicy::SkipTransparentEmpty);
    node.role_write_policy = Some(RoleWritePolicy::SetExplicit {
        role: RoleTag::Shadow,
    });
    let graph = graph(vec![node]);

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &Surface::new(1, 1, RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("skip-transparent graph executes");

    assert_eq!(outcome.written_cells, 0);
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Text));
}

#[test]
fn graph_executor_rejects_unknown_proof_effect_adapter() {
    let graph = graph(vec![glyph_node(
        "glyph",
        ValueSource::Literal {
            value: Value::Text("Z".to_string()),
        },
    )]);

    let error = GraphExecutor::new()
        .execute(
            &graph,
            &surface_with_cell('x', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect_err("missing adapter fails");

    assert!(matches!(
        error,
        GraphExecutionError::MissingProofAdapter { effect } if effect.as_str() == "proof.replaceGlyph"
    ));
}

#[test]
fn graph_executor_runs_graph_validation_before_execution() {
    let mut graph = graph(vec![glyph_node(
        "glyph",
        ValueSource::Literal {
            value: Value::Text("Z".to_string()),
        },
    )]);
    graph.nodes.get_mut(&NodeId::new("glyph")).unwrap().effect = EffectId::new("missing.effect");

    let error = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('x', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect_err("invalid graph fails before execution");

    assert!(matches!(
        error,
        GraphExecutionError::GraphValidation {
            error: DescriptorValidationError::UnknownEffect { .. }
        }
    ));
}

#[test]
fn graph_executor_does_not_apply_f2_bindings() {
    let mut graph = graph(vec![glyph_node(
        "glyph",
        ValueSource::Parameter {
            id: ParameterId::new("glyphParam"),
            fallback: None,
        },
    )]);
    graph.bindings = vec![binding_to(
        "glyphParam",
        ValueSource::Signal {
            id: SignalId::new("glyphSignal"),
            fallback: Some(Value::Text("B".to_string())),
        },
    )];

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('x', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("binding-valid graph executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'P');
}

// <FILE>crates/tui-vfx-next/tests/test_graph_execution_order.rs</FILE> - <DESC>Graph execution order, scope, adapter, and binding proof tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
