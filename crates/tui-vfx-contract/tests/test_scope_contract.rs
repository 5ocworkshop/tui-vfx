// <FILE>crates/tui-vfx-contract/tests/test_scope_contract.rs</FILE> - <DESC>Scope contract tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Scope contract tests: prove accepted built-in scope evaluation and explicit index-set scopes.</WCTX>
// <CLOG>0.2.0: MINOR — cover non-contiguous row and column index scopes.
// 0.1.0: INIT — cover modulo, non-empty, outer-band, and inner scope matching.</CLOG>

use tui_vfx_contract::{
    CoordinateSpace, NumericRange, RoleSpace, ScopeEvalInput, ScopeSpec, Value, ValueSource,
};
use tui_vfx_types::RoleTag;

fn input(x: usize, y: usize) -> ScopeEvalInput {
    ScopeEvalInput {
        destination_x: x,
        destination_y: y,
        sampled_source_x: x,
        sampled_source_y: y,
        sampled_source_role: RoleTag::Text,
        destination_role: RoleTag::Text,
        destination_width: Some(5),
        destination_height: Some(4),
        sampled_source_width: Some(5),
        sampled_source_height: Some(4),
        destination_glyph: Some("X".to_string()),
        sampled_source_glyph: Some("X".to_string()),
    }
}

#[test]
fn modulo_scopes_match_expected_rows_and_columns() {
    assert!(
        ScopeSpec::ModuloRows {
            modulus: 3,
            remainder: 1
        }
        .matches(
            &input(0, 4),
            CoordinateSpace::DestinationLocal,
            RoleSpace::Destination
        )
    );
    assert!(
        ScopeSpec::ModuloColumns {
            modulus: 4,
            remainder: 2
        }
        .matches(
            &input(6, 0),
            CoordinateSpace::DestinationLocal,
            RoleSpace::Destination
        )
    );
}

#[test]
fn index_set_scopes_match_non_contiguous_rows_and_columns() {
    let trim_rows = ScopeSpec::Rows {
        indices: vec![0, 10],
    };
    assert!(trim_rows.matches(
        &input(2, 0),
        CoordinateSpace::DestinationLocal,
        RoleSpace::Destination
    ));
    assert!(trim_rows.matches(
        &input(2, 10),
        CoordinateSpace::DestinationLocal,
        RoleSpace::Destination
    ));
    assert!(!trim_rows.matches(
        &input(2, 5),
        CoordinateSpace::DestinationLocal,
        RoleSpace::Destination
    ));

    let trim_columns = ScopeSpec::Columns {
        indices: vec![1, 4],
    };
    assert!(trim_columns.matches(
        &input(1, 2),
        CoordinateSpace::DestinationLocal,
        RoleSpace::Destination
    ));
    assert!(trim_columns.matches(
        &input(4, 2),
        CoordinateSpace::DestinationLocal,
        RoleSpace::Destination
    ));
    assert!(!trim_columns.matches(
        &input(3, 2),
        CoordinateSpace::DestinationLocal,
        RoleSpace::Destination
    ));
}

#[test]
fn content_and_band_scopes_use_optional_evaluation_context() {
    assert!(ScopeSpec::NonEmpty.matches(
        &input(2, 2),
        CoordinateSpace::DestinationLocal,
        RoleSpace::Destination
    ));
    assert!(ScopeSpec::OuterBand.matches(
        &input(0, 2),
        CoordinateSpace::DestinationLocal,
        RoleSpace::Destination
    ));
    assert!(ScopeSpec::Inner.matches(
        &input(2, 2),
        CoordinateSpace::DestinationLocal,
        RoleSpace::Destination
    ));
    assert!(!ScopeSpec::Inner.matches(
        &input(0, 2),
        CoordinateSpace::DestinationLocal,
        RoleSpace::Destination
    ));
}

#[test]
fn cell_scope_uses_static_literal_map_and_sampled_fallback_coordinates() {
    let literal_cell = ScopeSpec::Cell {
        x: Box::new(ValueSource::Literal {
            value: Value::Integer(2),
        }),
        y: Box::new(ValueSource::Literal {
            value: Value::Integer(1),
        }),
    };
    assert!(literal_cell.matches(
        &input(2, 1),
        CoordinateSpace::DestinationLocal,
        RoleSpace::Destination
    ));

    let mapped_cell = ScopeSpec::Cell {
        x: Box::new(ValueSource::Map {
            from: Box::new(ValueSource::Literal {
                value: Value::Number(0.5),
            }),
            input: NumericRange {
                min: Some(0.0),
                max: Some(1.0),
            },
            output: NumericRange {
                min: Some(0.0),
                max: Some(4.0),
            },
            clamp: true,
        }),
        y: Box::new(ValueSource::SampledField {
            field: "surfaceAngleFrom".to_string(),
            x: Box::new(ValueSource::Literal {
                value: Value::Integer(0),
            }),
            y: Box::new(ValueSource::Literal {
                value: Value::Integer(0),
            }),
            fallback: Some(Value::Integer(3)),
        }),
    };
    assert!(mapped_cell.matches(
        &input(2, 3),
        CoordinateSpace::DestinationLocal,
        RoleSpace::Destination
    ));
    assert!(!mapped_cell.matches(
        &input(1, 3),
        CoordinateSpace::DestinationLocal,
        RoleSpace::Destination
    ));
}

// <FILE>crates/tui-vfx-contract/tests/test_scope_contract.rs</FILE> - <DESC>Scope contract tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
