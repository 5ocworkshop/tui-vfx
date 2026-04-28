// <FILE>crates/tui-vfx-next/tests/test_graph_execution_values.rs</FILE> - <DESC>Graph execution value-source proof tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G2: prove canonical graph execution resolves literal, parameter, signal, and map sources.</WCTX>
// <CLOG>0.1.0: INIT — add value-resolution proof tests for GraphExecutor.</CLOG>

mod support;

use support::{dim_node, glyph_node, graph, surface_with_cell};
use tui_vfx_next::{
    GraphExecutionContext, GraphExecutionError, GraphExecutor, NumericRange, ParameterId, SignalId,
    Value, ValueSource,
};
use tui_vfx_types::{Color, RoleTag};

#[test]
fn graph_executor_runs_literal_input_node() {
    let graph = graph(vec![glyph_node(
        "glyph",
        ValueSource::Literal {
            value: Value::Text("Z".to_string()),
        },
    )]);

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('A', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("literal graph executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'Z');
    assert_eq!(outcome.executed_nodes[0].as_str(), "glyph");
}

#[test]
fn graph_executor_resolves_parameter_input() {
    let graph = graph(vec![glyph_node(
        "glyph",
        ValueSource::Parameter {
            id: ParameterId::new("glyphParam"),
            fallback: None,
        },
    )]);
    let context = GraphExecutionContext::new()
        .with_parameter(ParameterId::new("glyphParam"), Value::Text("O".to_string()));

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(&graph, &surface_with_cell('A', RoleTag::Text), &context)
        .expect("parameter override graph executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'O');
}

#[test]
fn graph_executor_uses_parameter_default_when_override_absent() {
    let graph = graph(vec![glyph_node(
        "glyph",
        ValueSource::Parameter {
            id: ParameterId::new("glyphParam"),
            fallback: None,
        },
    )]);

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('A', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("parameter default graph executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'P');
}

#[test]
fn graph_executor_resolves_signal_input() {
    let graph = graph(vec![glyph_node(
        "glyph",
        ValueSource::Signal {
            id: SignalId::new("glyphSignal"),
            fallback: None,
        },
    )]);
    let context = GraphExecutionContext::new()
        .with_signal(SignalId::new("glyphSignal"), Value::Text("S".to_string()));

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(&graph, &surface_with_cell('A', RoleTag::Text), &context)
        .expect("signal graph executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'S');
}

#[test]
fn graph_executor_uses_signal_fallback_when_missing() {
    let graph = graph(vec![glyph_node(
        "glyph",
        ValueSource::Signal {
            id: SignalId::new("glyphSignal"),
            fallback: Some(Value::Text("F".to_string())),
        },
    )]);

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('A', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("signal fallback graph executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'F');
}

#[test]
fn graph_executor_rejects_missing_required_signal_without_default() {
    let graph = graph(vec![dim_node(
        "dim",
        ValueSource::Signal {
            id: SignalId::new("level"),
            fallback: None,
        },
    )]);

    let error = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('A', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect_err("missing required signal fails");

    assert!(matches!(
        error,
        GraphExecutionError::MissingSignalValue { id } if id.as_str() == "level"
    ));
}

#[test]
fn graph_executor_resolves_numeric_map_source() {
    let graph = graph(vec![dim_node(
        "dim",
        ValueSource::Map {
            from: Box::new(ValueSource::Signal {
                id: SignalId::new("level"),
                fallback: None,
            }),
            input: NumericRange {
                min: Some(0.0),
                max: Some(1.0),
            },
            output: NumericRange {
                min: Some(0.0),
                max: Some(1.0),
            },
            clamp: true,
        },
    )]);
    let context =
        GraphExecutionContext::new().with_signal(SignalId::new("level"), Value::Number(0.5));

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(&graph, &surface_with_cell('A', RoleTag::Text), &context)
        .expect("numeric map graph executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().fg, Color::gray(128));
}

// <FILE>crates/tui-vfx-next/tests/test_graph_execution_values.rs</FILE> - <DESC>Graph execution value-source proof tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
