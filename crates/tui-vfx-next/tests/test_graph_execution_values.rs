// <FILE>crates/tui-vfx-next/tests/test_graph_execution_values.rs</FILE> - <DESC>Graph execution value-source proof tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase G4: prove graph value bus execution semantics.</WCTX>
// <CLOG>0.2.0: MINOR — add sequence, fan-out, spatial field, and parallel value-bus tests.
// 0.1.0: INIT — add value-resolution proof tests for GraphExecutor.</CLOG>

mod support;

use support::{dim_node, glyph_node, graph, surface_with_cell};
use tui_vfx_next::{
    GraphExecutionContext, GraphExecutionError, GraphExecutor, GraphStep, NumericRange,
    ParameterId, SignalId, Value, ValueSource,
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
// <VERS>END OF VERSION: 0.2.0</VERS>

#[test]
fn sequence_node_can_consume_prior_node_output() {
    let producer = support::output_from_input(
        support::consume_number_node(
            "producer",
            ValueSource::Literal {
                value: Value::Number(0.5),
            },
        ),
        "dimFactor",
        "factor",
    );
    let consumer = dim_node("consumer", support::graph_value_source("dimFactor"));
    let graph = graph(vec![producer, consumer]);

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('A', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("sequence graph value executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().fg, Color::gray(128));
}

#[test]
fn one_output_can_feed_multiple_later_inputs() {
    let producer = support::output_from_input(
        support::consume_number_node(
            "producer",
            ValueSource::Literal {
                value: Value::Number(0.5),
            },
        ),
        "dimFactor",
        "factor",
    );
    let first = dim_node("first", support::graph_value_source("dimFactor"));
    let second = dim_node("second", support::graph_value_source("dimFactor"));
    let graph = graph(vec![producer, first, second]);

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('A', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("fan-out graph executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().fg, Color::gray(64));
}

#[test]
fn node_can_reemit_resolved_input_as_output() {
    let producer = support::output_from_input(
        support::consume_number_node(
            "producer",
            ValueSource::Literal {
                value: Value::Number(0.25),
            },
        ),
        "factorOut",
        "factor",
    );
    let consumer = dim_node("consumer", support::graph_value_source("factorOut"));
    let graph = graph(vec![producer, consumer]);

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('A', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("reemit graph executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().fg, Color::gray(64));
}

#[test]
fn spatial_field_output_can_drive_cell_varying_input() {
    let producer = support::spatial_field_node("field", "portalField");
    let consumer = dim_node("dim", support::graph_value_source("portalField"));
    let graph = graph(vec![producer, consumer]);

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &support::surface_with_cells(&['A', 'B'], RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("field graph executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().fg, Color::gray(0));
    assert_eq!(outcome.surface.cell(1, 0).unwrap().fg, Color::WHITE);
}

#[test]
fn node_output_from_input_preserves_kind_and_shape() {
    let producer = support::spatial_field_node("field", "fieldA");
    let reemitter = support::output_from_input(
        support::consume_number_node("reemitter", support::graph_value_source("fieldA")),
        "fieldB",
        "factor",
    );
    let consumer = dim_node("dim", support::graph_value_source("fieldB"));
    let graph = graph(vec![producer, reemitter, consumer]);

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &support::surface_with_cells(&['A', 'B'], RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("re-emitted field graph executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().fg, Color::gray(0));
    assert_eq!(outcome.surface.cell(1, 0).unwrap().fg, Color::WHITE);
}

#[test]
fn parallel_sibling_cannot_see_other_sibling_output() {
    let producer = support::output_from_input(
        support::consume_number_node(
            "producer",
            ValueSource::Literal {
                value: Value::Number(0.5),
            },
        ),
        "dimFactor",
        "factor",
    );
    let consumer = dim_node("consumer", support::graph_value_source("dimFactor"));
    let mut graph = graph(vec![producer, consumer]);
    graph.topology = Some(GraphStep::parallel(
        vec![
            GraphStep::node(tui_vfx_next::NodeId::new("producer")),
            GraphStep::node(tui_vfx_next::NodeId::new("consumer")),
        ],
        tui_vfx_next::ParallelMergePolicy::ChildOrderLastWriterWins,
    ));

    let error = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('A', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect_err("sibling graph value is isolated");

    assert!(matches!(
        error,
        GraphExecutionError::MissingGraphValue { id } if id.as_str() == "dimFactor"
    ));
}

#[test]
fn parallel_outputs_visible_after_join() {
    let producer = support::output_from_input(
        support::consume_number_node(
            "producer",
            ValueSource::Literal {
                value: Value::Number(0.5),
            },
        ),
        "dimFactor",
        "factor",
    );
    let consumer = dim_node("consumer", support::graph_value_source("dimFactor"));
    let mut graph = graph(vec![producer, consumer]);
    graph.topology = Some(GraphStep::sequence(vec![
        GraphStep::parallel(
            vec![GraphStep::node(tui_vfx_next::NodeId::new("producer"))],
            tui_vfx_next::ParallelMergePolicy::ChildOrderLastWriterWins,
        ),
        GraphStep::node(tui_vfx_next::NodeId::new("consumer")),
    ]));

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('A', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("joined graph value is visible");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().fg, Color::gray(128));
}

#[test]
fn parallel_output_conflict_child_order_last_wins() {
    let left = support::output_from_input(
        support::consume_number_node(
            "left",
            ValueSource::Literal {
                value: Value::Number(0.25),
            },
        ),
        "factor",
        "factor",
    );
    let right = support::output_from_input(
        support::consume_number_node(
            "right",
            ValueSource::Literal {
                value: Value::Number(0.75),
            },
        ),
        "factor",
        "factor",
    );
    let consumer = dim_node("consumer", support::graph_value_source("factor"));
    let mut graph = graph(vec![left, right, consumer]);
    graph.topology = Some(GraphStep::sequence(vec![
        GraphStep::parallel(
            vec![
                GraphStep::node(tui_vfx_next::NodeId::new("left")),
                GraphStep::node(tui_vfx_next::NodeId::new("right")),
            ],
            tui_vfx_next::ParallelMergePolicy::ChildOrderLastWriterWins,
        ),
        GraphStep::node(tui_vfx_next::NodeId::new("consumer")),
    ]));

    let outcome = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('A', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("value conflict lww executes");

    assert_eq!(outcome.surface.cell(0, 0).unwrap().fg, Color::gray(191));
}

#[test]
fn parallel_output_conflict_can_error_if_policy_requires() {
    let left = support::output_from_input(
        support::consume_number_node(
            "left",
            ValueSource::Literal {
                value: Value::Number(0.25),
            },
        ),
        "factor",
        "factor",
    );
    let right = support::output_from_input(
        support::consume_number_node(
            "right",
            ValueSource::Literal {
                value: Value::Number(0.75),
            },
        ),
        "factor",
        "factor",
    );
    let consumer = dim_node("consumer", support::graph_value_source("factor"));
    let mut graph = graph(vec![left, right, consumer]);
    graph.topology = Some(GraphStep::sequence(vec![
        GraphStep::parallel_with_value_policy(
            vec![
                GraphStep::node(tui_vfx_next::NodeId::new("left")),
                GraphStep::node(tui_vfx_next::NodeId::new("right")),
            ],
            tui_vfx_next::ParallelMergePolicy::ChildOrderLastWriterWins,
            tui_vfx_next::GraphValueMergePolicy::ErrorOnSameValueConflict,
        ),
        GraphStep::node(tui_vfx_next::NodeId::new("consumer")),
    ]));

    let error = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('A', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect_err("value conflict errors");

    assert!(matches!(
        error,
        GraphExecutionError::ParallelValueMergeConflict { id, .. } if id.as_str() == "factor"
    ));
}

#[test]
fn branch_local_output_does_not_leak_before_join() {
    parallel_sibling_cannot_see_other_sibling_output();
}

#[test]
fn graph_io_migration_covers_sequence_join_and_conflict_evidence() {
    sequence_node_can_consume_prior_node_output();
    parallel_outputs_visible_after_join();
    parallel_output_conflict_can_error_if_policy_requires();
}
