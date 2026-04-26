// <FILE>crates/tui-vfx-content/src/mechanical/fnc_expand_cycle_preset.rs</FILE> - <DESC>Expand named MechanicalCyclePreset enum to its exact face string list</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2 of mechanical circular content cycles plan: preset face expansion that mirrors SplitFlapCharset constants exactly.</WCTX>
// <CLOG>0.1.0: introduce expand_cycle_preset; lock face lists to byte-equal SplitFlapCharset::Alpha/Digits/Uppercase pools and DecimalDigits 0..=9.</CLOG>

use crate::types::MechanicalCyclePreset;

/// Expand a named preset to its ordered face list.
///
/// Each preset's exact face order is part of the public schema contract
/// — adding or reordering an existing preset is a breaking change.
/// Tests in this module assert byte-equality against the existing
/// `SplitFlapCharset` constants so the preset path produces identical
/// face supply to legacy charset cycling.
pub(crate) fn expand_cycle_preset(preset: MechanicalCyclePreset) -> Vec<String> {
    match preset {
        MechanicalCyclePreset::DecimalDigits => ('0'..='9').map(|c| c.to_string()).collect(),
        MechanicalCyclePreset::SplitFlapAlpha => SPLIT_FLAP_ALPHA_FACES
            .iter()
            .map(|c| c.to_string())
            .collect(),
        MechanicalCyclePreset::SplitFlapDigits => SPLIT_FLAP_DIGITS_FACES
            .iter()
            .map(|c| c.to_string())
            .collect(),
        MechanicalCyclePreset::SplitFlapUppercase => SPLIT_FLAP_UPPER_FACES
            .iter()
            .map(|c| c.to_string())
            .collect(),
    }
}

/// Mirror of `crates/tui-vfx-content/src/transformers/cls_split_flap.rs::ALPHA_CHARS`.
/// The SplitFlap legacy path uses that constant directly; this mirror
/// keeps the preset path byte-equal without taking a non-public
/// dependency on the constant.
const SPLIT_FLAP_ALPHA_FACES: &[char] = &[
    ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.',
    ',', '-', '!', '?',
];
const SPLIT_FLAP_DIGITS_FACES: &[char] = &[' ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SPLIT_FLAP_UPPER_FACES: &[char] = &[
    ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_digits_is_zero_through_nine_in_order() {
        let faces = expand_cycle_preset(MechanicalCyclePreset::DecimalDigits);
        assert_eq!(faces.len(), 10);
        for (i, face) in faces.iter().enumerate() {
            assert_eq!(face, &i.to_string());
        }
    }

    #[test]
    fn split_flap_alpha_matches_legacy_charset_byte_for_byte() {
        let faces = expand_cycle_preset(MechanicalCyclePreset::SplitFlapAlpha);
        let expected: Vec<String> = [
            ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
            'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6',
            '7', '8', '9', '.', ',', '-', '!', '?',
        ]
        .iter()
        .map(|c| c.to_string())
        .collect();
        assert_eq!(faces, expected);
        assert_eq!(faces.len(), 42);
    }

    #[test]
    fn split_flap_alpha_does_not_contain_slash() {
        let faces = expand_cycle_preset(MechanicalCyclePreset::SplitFlapAlpha);
        assert!(
            !faces.iter().any(|f| f == "/"),
            "split_flap_alpha must not include '/' — recipes needing it must use ordered faces",
        );
    }

    #[test]
    fn split_flap_digits_starts_with_space_then_zero_through_nine() {
        let faces = expand_cycle_preset(MechanicalCyclePreset::SplitFlapDigits);
        assert_eq!(faces.len(), 11);
        assert_eq!(faces[0], " ");
        for (i, face) in faces.iter().skip(1).enumerate() {
            assert_eq!(face, &i.to_string());
        }
    }

    #[test]
    fn split_flap_uppercase_starts_with_space_then_a_through_z() {
        let faces = expand_cycle_preset(MechanicalCyclePreset::SplitFlapUppercase);
        assert_eq!(faces.len(), 27);
        assert_eq!(faces[0], " ");
        let mut chars = ('A'..='Z').collect::<Vec<_>>();
        chars.insert(0, ' ');
        let expected: Vec<String> = chars.iter().map(|c| c.to_string()).collect();
        assert_eq!(faces, expected);
    }

    #[test]
    fn presets_have_no_duplicate_faces() {
        for preset in [
            MechanicalCyclePreset::DecimalDigits,
            MechanicalCyclePreset::SplitFlapAlpha,
            MechanicalCyclePreset::SplitFlapDigits,
            MechanicalCyclePreset::SplitFlapUppercase,
        ] {
            let faces = expand_cycle_preset(preset);
            let mut sorted = faces.clone();
            sorted.sort();
            sorted.dedup();
            assert_eq!(
                faces.len(),
                sorted.len(),
                "preset {preset:?} has duplicates"
            );
        }
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_expand_cycle_preset.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
