// <FILE>crates/tui-vfx-contract/tests/test_scope_contract.rs</FILE> - <DESC>Scope contract tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.13 schema decision burn-down: prove accepted built-in scope evaluation.</WCTX>
// <CLOG>0.1.0: INIT — cover modulo, non-empty, outer-band, and inner scope matching.</CLOG>

use tui_vfx_contract::{CoordinateSpace, RoleSpace, ScopeEvalInput, ScopeSpec};
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

// <FILE>crates/tui-vfx-contract/tests/test_scope_contract.rs</FILE> - <DESC>Scope contract tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
