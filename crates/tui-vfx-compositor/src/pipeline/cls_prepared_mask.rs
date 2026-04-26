// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_mask.rs</FILE> - <DESC>Prepared mask enum for pipeline rendering</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Slice 6.6 §F.3 — migrate Mask dispatcher to build VfxCellContext once per dispatch</WCTX>
// <CLOG>1.2.0: MINOR — PreparedMask::is_visible now takes &VfxCellContext and delegates to trait impls; all 11 match arms updated. Tests build ctx once and reuse &ctx.</CLOG>

use crate::masks::{
    cls_blinds::Blinds, cls_cellular::Cellular, cls_checkers::Checkers, cls_diamond::Diamond,
    cls_dissolve::Dissolve, cls_materialize::Materialize, cls_noise_dither::NoiseDither,
    cls_path_reveal::PathReveal, cls_radial::Radial, cls_spotlight::Spotlight, cls_wipe::Wipe,
};
use crate::traits::mask::Mask;
use crate::types::cls_mask_spec::MaskSpec;
use smallvec::SmallVec;
use tui_vfx_types::VfxCellContext;

pub(crate) enum PreparedMask {
    None,
    Wipe(Wipe),
    Dissolve(Dissolve),
    Checkers(Checkers),
    Blinds(Blinds),
    Iris(Spotlight),
    Diamond(Diamond),
    Materialize(Materialize),
    NoiseDither(NoiseDither),
    PathReveal(PathReveal),
    Radial(Radial),
    Cellular(Cellular),
}

impl PreparedMask {
    pub(crate) fn is_visible(&self, ctx: &VfxCellContext) -> bool {
        match self {
            PreparedMask::None => true,
            PreparedMask::Wipe(mask) => mask.is_visible(ctx),
            PreparedMask::Dissolve(mask) => mask.is_visible(ctx),
            PreparedMask::Checkers(mask) => mask.is_visible(ctx),
            PreparedMask::Blinds(mask) => mask.is_visible(ctx),
            PreparedMask::Iris(mask) => mask.is_visible(ctx),
            PreparedMask::Diamond(mask) => mask.is_visible(ctx),
            PreparedMask::Materialize(mask) => mask.is_visible(ctx),
            PreparedMask::NoiseDither(mask) => mask.is_visible(ctx),
            PreparedMask::PathReveal(mask) => mask.is_visible(ctx),
            PreparedMask::Radial(mask) => mask.is_visible(ctx),
            PreparedMask::Cellular(mask) => mask.is_visible(ctx),
        }
    }

    pub(crate) fn name(&self) -> &'static str {
        match self {
            PreparedMask::None => "None",
            PreparedMask::Wipe(_) => "Wipe",
            PreparedMask::Dissolve(_) => "Dissolve",
            PreparedMask::Checkers(_) => "Checkers",
            PreparedMask::Blinds(_) => "Blinds",
            PreparedMask::Iris(_) => "Iris",
            PreparedMask::Diamond(_) => "Diamond",
            PreparedMask::Materialize(_) => "Materialize",
            PreparedMask::NoiseDither(_) => "NoiseDither",
            PreparedMask::PathReveal(_) => "PathReveal",
            PreparedMask::Radial(_) => "Radial",
            PreparedMask::Cellular(_) => "Cellular",
        }
    }
}

pub(crate) fn prepare_mask(spec: &MaskSpec) -> PreparedMask {
    match spec {
        MaskSpec::None => PreparedMask::None,
        MaskSpec::Wipe { soft_edge, .. } => {
            // resolve_wipe() handles reveal/hide/direction priority and returns invert flag
            let resolved = spec.resolve_wipe().unwrap();
            PreparedMask::Wipe(Wipe::new_with_invert(
                resolved.direction,
                *soft_edge,
                resolved.invert,
            ))
        }
        MaskSpec::Dissolve { seed, chunk_size } => {
            PreparedMask::Dissolve(Dissolve::new(*seed, *chunk_size))
        }
        MaskSpec::Checkers { cell_size } => PreparedMask::Checkers(Checkers::new(*cell_size)),
        MaskSpec::Blinds { orientation, count } => {
            PreparedMask::Blinds(Blinds::new(*orientation, *count))
        }
        MaskSpec::Iris { shape, soft_edge } => {
            PreparedMask::Iris(Spotlight::new(*shape, *soft_edge))
        }
        MaskSpec::Diamond { soft_edge } => PreparedMask::Diamond(Diamond::new(*soft_edge)),
        MaskSpec::Materialize {
            origin,
            seed,
            chunk_size,
            noise,
            soft_edge,
        } => PreparedMask::Materialize(Materialize::new(
            *origin,
            *seed,
            *chunk_size,
            *noise,
            *soft_edge,
        )),
        MaskSpec::NoiseDither { seed, matrix } => {
            PreparedMask::NoiseDither(NoiseDither::new(*seed, *matrix))
        }
        MaskSpec::PathReveal { path, soft_edge } => {
            PreparedMask::PathReveal(PathReveal::new(path.clone(), *soft_edge))
        }
        MaskSpec::Radial { origin, soft_edge } => {
            PreparedMask::Radial(Radial::new(*origin, *soft_edge))
        }
        MaskSpec::Cellular {
            pattern,
            seed,
            cell_count,
        } => PreparedMask::Cellular(Cellular::new(*pattern, *seed, *cell_count)),
    }
}

pub(crate) fn prepare_masks(masks: &[MaskSpec]) -> SmallVec<[PreparedMask; 2]> {
    let mut prepared = SmallVec::new();
    for mask in masks {
        prepared.push(prepare_mask(mask));
    }
    prepared
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::masks::{
        cls_cellular::CellularPattern,
        cls_path_reveal::{RevealPathType, SpiralDirection},
        cls_radial::RadialOrigin,
    };
    use crate::types::{DitherMatrix, IrisShape, Orientation, WipeDirection};

    #[test]
    fn test_prepare_mask_covers_every_mask_variant() {
        let cases = vec![
            (MaskSpec::None, "None"),
            (
                MaskSpec::Wipe {
                    reveal: Some(WipeDirection::LeftToRight),
                    hide: None,
                    direction: None,
                    soft_edge: true,
                },
                "Wipe",
            ),
            (
                MaskSpec::Dissolve {
                    seed: 1,
                    chunk_size: 2,
                },
                "Dissolve",
            ),
            (MaskSpec::Checkers { cell_size: 2 }, "Checkers"),
            (
                MaskSpec::Blinds {
                    orientation: Orientation::Vertical,
                    count: 4,
                },
                "Blinds",
            ),
            (
                MaskSpec::Iris {
                    shape: IrisShape::Box,
                    soft_edge: true,
                },
                "Iris",
            ),
            (MaskSpec::Diamond { soft_edge: true }, "Diamond"),
            (
                MaskSpec::Materialize {
                    origin: RadialOrigin::Center,
                    seed: 5,
                    chunk_size: 2,
                    noise: 0.2,
                    soft_edge: true,
                },
                "Materialize",
            ),
            (
                MaskSpec::NoiseDither {
                    seed: 9,
                    matrix: DitherMatrix::Bayer8,
                },
                "NoiseDither",
            ),
            (
                MaskSpec::PathReveal {
                    path: RevealPathType::Spiral {
                        rotations: 3.0,
                        direction: SpiralDirection::Clockwise,
                    },
                    soft_edge: false,
                },
                "PathReveal",
            ),
            (
                MaskSpec::Radial {
                    origin: RadialOrigin::BottomRight,
                    soft_edge: true,
                },
                "Radial",
            ),
            (
                MaskSpec::Cellular {
                    pattern: CellularPattern::Hexagonal,
                    seed: 3,
                    cell_count: 12,
                },
                "Cellular",
            ),
        ];

        for (spec, expected_name) in cases {
            assert_eq!(
                prepare_mask(&spec).name(),
                expected_name,
                "expected {expected_name} for {spec:?}"
            );
        }
    }

    #[test]
    fn test_prepare_mask_applies_hide_wipe_resolution() {
        let prepared = prepare_mask(&MaskSpec::Wipe {
            reveal: Some(WipeDirection::LeftToRight),
            hide: Some(WipeDirection::RightToLeft),
            direction: Some(WipeDirection::TopToBottom),
            soft_edge: false,
        });

        assert_eq!(prepared.name(), "Wipe");
        let ctx_visible = VfxCellContext::new(2, 0, 5, 1, 0, 0, 1.0);
        let ctx_hidden = VfxCellContext::new(2, 0, 5, 1, 0, 0, 0.0);
        assert!(
            prepared.is_visible(&ctx_visible),
            "hide wipes should be fully visible at exit start"
        );
        assert!(
            !prepared.is_visible(&ctx_hidden),
            "hide wipes should be hidden at exit end after prepare_mask resolves invert=true"
        );
    }
}

// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_mask.rs</FILE> - <DESC>Prepared mask enum for pipeline rendering</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
