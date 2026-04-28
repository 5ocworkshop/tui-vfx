// <FILE>crates/tui-vfx-next/tests/test_graph_execution_topology.rs</FILE> - <DESC>Graph topology snapshot and merge proof tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G3: prove sequence, parallel snapshot, and channel-aware merge semantics.</WCTX>
// <CLOG>0.1.0: INIT — add topology execution tests for GraphExecutor.</CLOG>

mod support;

use support::{background_node, foreground_node, glyph_node, graph, role_node, surface_with_cell};
use tui_vfx_next::{
    CellChannel, GraphExecutionContext, GraphExecutionError, GraphExecutor, GraphStep,
    ParallelMergePolicy, ScopeSpec, Value, ValueSource,
};
use tui_vfx_types::{Color, RoleTag};

#[test]
fn topology_sequence_later_child_sees_earlier_child_write() {
    let mut scoped = glyph_node(
        "scopedGlyph",
        ValueSource::Literal {
            value: Value::Text("S".to_string()),
        },
    );
    scoped.scope = Some(ScopeSpec::Role {
        role: RoleTag::Shadow,
    });
    let mut graph = graph(vec![role_node("markShadow", RoleTag::Shadow), scoped]);
    graph.topology = Some(GraphStep::sequence(vec![
        GraphStep::node(tui_vfx_next::NodeId::new("markShadow")),
        GraphStep::node(tui_vfx_next::NodeId::new("scopedGlyph")),
    ]));

    let outcome = execute(&graph);

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'S');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Shadow));
}

#[test]
fn topology_parallel_children_read_same_snapshot() {
    let mut scoped = glyph_node(
        "scopedGlyph",
        ValueSource::Literal {
            value: Value::Text("S".to_string()),
        },
    );
    scoped.scope = Some(ScopeSpec::Role {
        role: RoleTag::Shadow,
    });
    let mut graph = graph(vec![role_node("markShadow", RoleTag::Shadow), scoped]);
    graph.topology = Some(GraphStep::parallel(
        vec![
            GraphStep::node(tui_vfx_next::NodeId::new("markShadow")),
            GraphStep::node(tui_vfx_next::NodeId::new("scopedGlyph")),
        ],
        ParallelMergePolicy::ChildOrderLastWriterWins,
    ));

    let outcome = execute(&graph);

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'x');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Shadow));
}

#[test]
fn topology_parallel_different_channel_writes_compose() {
    let mut graph = graph(vec![
        foreground_node("fg", Color::RED),
        background_node("bg", Color::BLUE),
    ]);
    graph.topology = Some(GraphStep::parallel(
        vec![
            GraphStep::node(tui_vfx_next::NodeId::new("fg")),
            GraphStep::node(tui_vfx_next::NodeId::new("bg")),
        ],
        ParallelMergePolicy::ChildOrderLastWriterWins,
    ));

    let outcome = execute(&graph);
    let cell = outcome.surface.cell(0, 0).unwrap();

    assert_eq!(cell.fg, Color::RED);
    assert_eq!(cell.bg, Color::BLUE);
}

#[test]
fn topology_parallel_same_channel_conflict_child_order_last_wins() {
    let mut graph = graph(vec![
        foreground_node("red", Color::RED),
        foreground_node("green", Color::GREEN),
    ]);
    graph.topology = Some(GraphStep::parallel(
        vec![
            GraphStep::node(tui_vfx_next::NodeId::new("red")),
            GraphStep::node(tui_vfx_next::NodeId::new("green")),
        ],
        ParallelMergePolicy::ChildOrderLastWriterWins,
    ));

    let outcome = execute(&graph);

    assert_eq!(outcome.surface.cell(0, 0).unwrap().fg, Color::GREEN);
}

#[test]
fn topology_parallel_same_channel_conflict_can_error() {
    let mut graph = graph(vec![
        foreground_node("red", Color::RED),
        foreground_node("green", Color::GREEN),
    ]);
    graph.topology = Some(GraphStep::parallel(
        vec![
            GraphStep::node(tui_vfx_next::NodeId::new("red")),
            GraphStep::node(tui_vfx_next::NodeId::new("green")),
        ],
        ParallelMergePolicy::ErrorOnSameChannelConflict,
    ));

    let error = GraphExecutor::with_standard_proof_adapters()
        .execute(
            &graph,
            &surface_with_cell('x', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect_err("same-channel conflict fails");

    assert!(matches!(
        error,
        GraphExecutionError::ParallelMergeConflict {
            channel: CellChannel::Foreground,
            ..
        }
    ));
}

#[test]
fn topology_parallel_nested_sequence_branch_reads_own_prior_step() {
    let mut scoped = glyph_node(
        "scopedGlyph",
        ValueSource::Literal {
            value: Value::Text("S".to_string()),
        },
    );
    scoped.scope = Some(ScopeSpec::Role {
        role: RoleTag::Shadow,
    });
    let mut graph = graph(vec![
        role_node("markShadow", RoleTag::Shadow),
        scoped,
        background_node("bg", Color::BLUE),
    ]);
    graph.topology = Some(GraphStep::parallel(
        vec![
            GraphStep::sequence(vec![
                GraphStep::node(tui_vfx_next::NodeId::new("markShadow")),
                GraphStep::node(tui_vfx_next::NodeId::new("scopedGlyph")),
            ]),
            GraphStep::node(tui_vfx_next::NodeId::new("bg")),
        ],
        ParallelMergePolicy::ChildOrderLastWriterWins,
    ));

    let outcome = execute(&graph);
    let cell = outcome.surface.cell(0, 0).unwrap();

    assert_eq!(cell.ch, 'S');
    assert_eq!(cell.bg, Color::BLUE);
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Shadow));
}

fn execute(graph: &tui_vfx_next::GraphSpec) -> tui_vfx_next::GraphExecutionOutcome {
    GraphExecutor::with_standard_proof_adapters()
        .execute(
            graph,
            &surface_with_cell('x', RoleTag::Text),
            &GraphExecutionContext::new(),
        )
        .expect("topology graph executes")
}

// <FILE>crates/tui-vfx-next/tests/test_graph_execution_topology.rs</FILE> - <DESC>Graph topology snapshot and merge proof tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
