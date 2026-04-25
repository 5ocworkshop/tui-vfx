// <FILE>crates/tui-vfx-content/tests/transformers/test_cls_split_flap_tiles.rs</FILE> - <DESC>SplitFlap multi-cell Solari tile behavior tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase 3 mechanical display primitives: add SplitFlap tile geometry coverage.</WCTX>
// <CLOG>0.2.0: prove multi-cell tile mode honors cascade and cycles controls.
// 0.1.0: add failing tests for tile serde, 1x1 compatibility, validation, and center-hinged frames.</CLOG>

use tui_vfx_content::prelude::*;
use tui_vfx_content::transformers::{SplitFlap, SplitFlapCharset, SplitFlapDispersion};

fn canonical_tile_effect(tile_width: u16, tile_height: u16) -> ContentEffect {
    serde_json::from_value(serde_json::json!({
        "type": "split_flap",
        "from_message": "OLD\nOLD\nOLD\nOLD",
        "speed": 1.0,
        "cycles": 0.0,
        "jitter": 0.0,
        "settle_hinge": true,
        "spring_settle": true,
        "rolling_flip": true,
        "dispersion": "center_out",
        "tile_width": tile_width,
        "tile_height": tile_height
    }))
    .expect("split_flap tile JSON should deserialize")
}

#[test]
fn split_flap_deserializes_tile_geometry_fields() {
    match canonical_tile_effect(3, 4) {
        ContentEffect::SplitFlap {
            tile_width,
            tile_height,
            ..
        } => {
            assert_eq!(tile_width, 3);
            assert_eq!(tile_height, 4);
        }
        other => panic!("expected split flap, got {other:?}"),
    }
}

#[test]
fn split_flap_legacy_1x1_matches_existing_transformer_path() {
    let target = "AZ\nBY";
    let old = SplitFlap::new_mechanical(
        SignalOrFloat::from(1.0),
        SignalOrFloat::from(0.0_f32),
        SignalOrFloat::from(0.0_f32),
        0.0,
        SplitFlapCharset::Alpha,
        false,
        0.0,
        true,
        false,
        false,
    )
    .with_from_message("AA\nBB")
    .with_rolling_flip(true)
    .with_dispersion(SplitFlapDispersion::CenterOut);

    let effect = serde_json::from_value::<ContentEffect>(serde_json::json!({
        "type": "split_flap",
        "speed": 1.0,
        "cascade": 0.0,
        "cycles": 0.0,
        "jitter": 0.0,
        "charset": "alpha",
        "settle_hinge": true,
        "from_message": "AA\nBB",
        "rolling_flip": true,
        "dispersion": "center_out",
        "tile_width": 1,
        "tile_height": 1
    }))
    .unwrap();

    assert_eq!(
        effect.apply(target, 0.5),
        old.transform(
            target,
            0.5,
            &mixed_signals::prelude::SignalContext::default()
        )
    );
    assert_eq!(effect.apply(target, 1.0), target);
}

#[test]
fn split_flap_rejects_invalid_tile_heights_with_explicit_noop() {
    let target = "NEW\nNEW\nNEW\nNEW";
    for invalid_height in [0, 3, 10] {
        let effect = canonical_tile_effect(3, invalid_height);
        assert_eq!(effect.apply(target, 0.5), target);
    }
    let zero_width = canonical_tile_effect(0, 4);
    assert_eq!(zero_width.apply(target, 0.5), target);
    let flat_wide = canonical_tile_effect(2, 1);
    assert_eq!(flat_wide.apply(target, 0.5), target);
}

#[test]
fn split_flap_accepts_even_tile_heights_2_4_6_8() {
    for height in [2, 4, 6, 8] {
        let from = (0..height).map(|_| "OLD").collect::<Vec<_>>().join("\n");
        let to = (0..height).map(|_| "NEW").collect::<Vec<_>>().join("\n");
        let mut effect = canonical_tile_effect(3, height);
        if let ContentEffect::SplitFlap { from_message, .. } = &mut effect {
            *from_message = Some(from);
        }
        assert_ne!(effect.apply(&to, 0.5), to);
        assert_eq!(effect.apply(&to, 1.0), to);
    }
}

#[test]
fn split_flap_tile_height_4_renders_center_hinged_frames() {
    let effect = canonical_tile_effect(3, 4);
    let target = "NEW\nNEW\nNEW\nNEW";

    assert_eq!(effect.apply(target, 0.0), "OLD\nOLD\nOLD\nOLD");
    assert_eq!(effect.apply(target, 0.25), "OLD\nOLD\nNEW\nNEW");
    assert_eq!(effect.apply(target, 0.75), "OLD\nNEW\nNEW\nNEW");
    assert_eq!(effect.apply(target, 1.0), target);
}
#[test]
fn split_flap_tile_mode_honors_cascade_per_tile() {
    let target = "NEWXYZ\nNEWXYZ\nNEWXYZ\nNEWXYZ";
    let simultaneous = serde_json::from_value::<ContentEffect>(serde_json::json!({
        "type": "split_flap",
        "from_message": "OLDOLD\nOLDOLD\nOLDOLD\nOLDOLD",
        "speed": 1.0,
        "cascade": 0.0,
        "cycles": 0.0,
        "jitter": 0.0,
        "dispersion": "cascade",
        "tile_width": 3,
        "tile_height": 4
    }))
    .unwrap();
    let cascaded = serde_json::from_value::<ContentEffect>(serde_json::json!({
        "type": "split_flap",
        "from_message": "OLDOLD\nOLDOLD\nOLDOLD\nOLDOLD",
        "speed": 1.0,
        "cascade": 0.8,
        "cycles": 0.0,
        "jitter": 0.0,
        "dispersion": "cascade",
        "tile_width": 3,
        "tile_height": 4
    }))
    .unwrap();

    assert_eq!(
        cascaded.apply(target, 0.5),
        "OLDOLD\nOLDOLD\nNEWOLD\nNEWOLD"
    );
    assert_ne!(cascaded.apply(target, 0.5), simultaneous.apply(target, 0.5));
}

#[test]
fn split_flap_tile_mode_honors_cycles() {
    let target = "NEW\nNEW\nNEW\nNEW";
    let no_cycles = serde_json::from_value::<ContentEffect>(serde_json::json!({
        "type": "split_flap",
        "from_message": "OLD\nOLD\nOLD\nOLD",
        "speed": 1.0,
        "cascade": 0.0,
        "cycles": 0.0,
        "jitter": 0.0,
        "tile_width": 3,
        "tile_height": 4
    }))
    .unwrap();
    let with_cycle = serde_json::from_value::<ContentEffect>(serde_json::json!({
        "type": "split_flap",
        "from_message": "OLD\nOLD\nOLD\nOLD",
        "speed": 1.0,
        "cascade": 0.0,
        "cycles": 1.0,
        "jitter": 0.0,
        "tile_width": 3,
        "tile_height": 4
    }))
    .unwrap();

    assert_ne!(with_cycle.apply(target, 0.4), no_cycles.apply(target, 0.4));
}

// <FILE>crates/tui-vfx-content/tests/transformers/test_cls_split_flap_tiles.rs</FILE> - <DESC>SplitFlap multi-cell Solari tile behavior tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
