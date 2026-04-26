// <FILE>crates/tui-vfx-content/tests/transformers/test_cls_odometer.rs</FILE> - <DESC>Tests for structured mechanical Odometer tile-roll behavior</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Slice 6.6 of mechanical circular content cycles plan: TextTransformer signature now takes &TransformContext<'_>.</WCTX>
// <CLOG>0.3.0: route every transform call site through a TransformContext built from a OnceLock-cached SignalContext + ShaderRuntimeParams pair.</CLOG>

use mixed_signals::prelude::SignalContext;
use std::sync::OnceLock;
use tui_vfx_content::traits::{TextTransformer, TransformContext};
use tui_vfx_content::transformers::{Odometer, get_transformer};
use tui_vfx_content::types::{ContentEffect, OdometerDirection, OdometerTravel};
use tui_vfx_style::traits::ShaderRuntimeParams;

static CTX_PARTS: OnceLock<(SignalContext, ShaderRuntimeParams)> = OnceLock::new();

fn tctx() -> TransformContext<'static> {
    let p = CTX_PARTS.get_or_init(|| (SignalContext::default(), ShaderRuntimeParams::new()));
    TransformContext::new(&p.0, &p.1)
}

const FROM: &str = "AAA\nBBB\nCCC";
const TO: &str = "111\n222\n333";

fn odometer(direction: OdometerDirection) -> Odometer {
    Odometer::new(
        direction,
        OdometerTravel::Axis,
        1,
        3,
        Some(FROM.to_string()),
    )
}

fn sample(direction: OdometerDirection, progress: f64) -> String {
    odometer(direction)
        .transform(TO, progress, &tctx())
        .into_owned()
}

#[test]
fn parses_structured_odometer_recipe() {
    let json = r#"{
        "type": "odometer",
        "direction": "up",
        "travel": { "type": "axis" },
        "tile_width": 1,
        "tile_height": 3,
        "from_message": "AAA\nBBB\nCCC"
    }"#;

    let parsed: ContentEffect = serde_json::from_str(json).unwrap();

    assert!(
        matches!(parsed, ContentEffect::Odometer { direction: OdometerDirection::Up, travel: OdometerTravel::Axis, tile_width: 1, tile_height: 3, ref from_message, mechanical: None } if from_message.as_deref() == Some(FROM))
    );
}

#[test]
fn rejects_legacy_unit_odometer_recipe() {
    let parsed = serde_json::from_str::<ContentEffect>(r#"{ "type": "odometer" }"#);
    assert!(
        parsed.is_err(),
        "unit/default odometer must not deserialize"
    );
}

#[test]
fn rejects_string_roll_travel_shape() {
    let parsed = serde_json::from_str::<ContentEffect>(
        r#"{ "type": "odometer", "direction": "up", "travel": "axis", "tile_width": 1, "tile_height": 3, "from_message": "AAA\nBBB\nCCC" }"#,
    );
    assert!(parsed.is_err(), "travel must use tagged object shape");
}

#[test]
fn missing_from_message_uses_blank_source_grid() {
    let effect = ContentEffect::Odometer {
        direction: OdometerDirection::Up,
        travel: OdometerTravel::Axis,
        tile_width: 1,
        tile_height: 3,
        from_message: None,
        mechanical: None,
    };
    let tx = get_transformer(&effect);
    assert_eq!(
        tx.transform(TO, 0.34, &tctx()),
        format!("{}\n{}\n111", "   ", "   ")
    );
}

#[test]
fn up_rolls_rows_from_source_to_target() {
    assert_eq!(sample(OdometerDirection::Up, 0.0), FROM);
    assert_eq!(sample(OdometerDirection::Up, 0.34), "BBB\nCCC\n111");
    assert_eq!(sample(OdometerDirection::Up, 0.67), "CCC\n111\n222");
    let full = odometer(OdometerDirection::Up).transform(TO, 1.0, &tctx());
    assert!(matches!(full, std::borrow::Cow::Borrowed(_)));
    assert_eq!(full, TO);
}

#[test]
fn down_rolls_target_in_from_top() {
    assert_eq!(sample(OdometerDirection::Down, 0.34), "333\nAAA\nBBB");
    assert_eq!(sample(OdometerDirection::Down, 0.67), "222\n333\nAAA");
}

#[test]
fn left_and_right_roll_columns() {
    assert_eq!(sample(OdometerDirection::Left, 0.34), "AA1\nBB2\nCC3");
    assert_eq!(sample(OdometerDirection::Left, 0.67), "A11\nB22\nC33");
    assert_eq!(sample(OdometerDirection::Right, 0.34), "1AA\n2BB\n3CC");
    assert_eq!(sample(OdometerDirection::Right, 0.67), "11A\n22B\n33C");
}

#[test]
fn diagonal_composes_horizontal_and_vertical_offsets() {
    assert_eq!(sample(OdometerDirection::UpLeft, 0.34), "BB \nCC \n  1");
}

#[test]
fn content_effect_dispatches_structured_odometer() {
    let effect = ContentEffect::Odometer {
        direction: OdometerDirection::Up,
        travel: OdometerTravel::Axis,
        tile_width: 1,
        tile_height: 3,
        from_message: Some(FROM.to_string()),
        mechanical: None,
    };
    let tx = get_transformer(&effect);
    assert_eq!(tx.transform(TO, 0.34, &tctx()), "BBB\nCCC\n111");
    assert_eq!(effect.name(), "Odometer");
    assert_eq!(
        effect.terse_description(),
        "Mechanical cell-grid rolling display"
    );
    assert!(!effect.key_parameters().is_empty());
}
#[test]
fn multi_cell_odometer_rolls_cell_steps_inside_glyph_window() {
    let effect = Odometer::new(
        OdometerDirection::Up,
        OdometerTravel::Axis,
        2,
        3,
        Some("AABB\nCCDD\nEEFF".to_string()),
    );

    assert_eq!(
        effect.transform("1122\n3344\n5566", 0.34, &tctx()),
        "CCDD\nEEFF\n1122"
    );
}

// <FILE>crates/tui-vfx-content/tests/transformers/test_cls_odometer.rs</FILE> - <DESC>Tests for structured mechanical Odometer tile-roll behavior</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
