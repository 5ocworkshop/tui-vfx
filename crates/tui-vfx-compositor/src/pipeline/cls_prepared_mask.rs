// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_mask.rs</FILE> - <DESC>Prepared mask enum for pipeline rendering</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Ergonomic reveal/hide schema for intuitive mask direction semantics</WCTX>
// <CLOG>Use resolve_wipe() to handle reveal/hide/direction fields and pass invert flag</CLOG>

use crate::masks::{
    cls_blinds::Blinds, cls_cellular::Cellular, cls_checkers::Checkers, cls_diamond::Diamond,
    cls_dissolve::Dissolve, cls_noise_dither::NoiseDither, cls_path_reveal::PathReveal,
    cls_radial::Radial, cls_spotlight::Spotlight, cls_wipe::Wipe,
};
use crate::traits::mask::Mask;
use crate::types::cls_mask_spec::MaskSpec;
use smallvec::SmallVec;

pub(crate) enum PreparedMask {
    None,
    Wipe(Wipe),
    Dissolve(Dissolve),
    Checkers(Checkers),
    Blinds(Blinds),
    Iris(Spotlight),
    Diamond(Diamond),
    NoiseDither(NoiseDither),
    PathReveal(PathReveal),
    Radial(Radial),
    Cellular(Cellular),
}

impl PreparedMask {
    pub(crate) fn is_visible(
        &self,
        local_x: u16,
        local_y: u16,
        width: u16,
        height: u16,
        t: f64,
    ) -> bool {
        match self {
            PreparedMask::None => true,
            PreparedMask::Wipe(mask) => mask.is_visible(local_x, local_y, width, height, t),
            PreparedMask::Dissolve(mask) => mask.is_visible(local_x, local_y, width, height, t),
            PreparedMask::Checkers(mask) => mask.is_visible(local_x, local_y, width, height, t),
            PreparedMask::Blinds(mask) => mask.is_visible(local_x, local_y, width, height, t),
            PreparedMask::Iris(mask) => mask.is_visible(local_x, local_y, width, height, t),
            PreparedMask::Diamond(mask) => mask.is_visible(local_x, local_y, width, height, t),
            PreparedMask::NoiseDither(mask) => mask.is_visible(local_x, local_y, width, height, t),
            PreparedMask::PathReveal(mask) => mask.is_visible(local_x, local_y, width, height, t),
            PreparedMask::Radial(mask) => mask.is_visible(local_x, local_y, width, height, t),
            PreparedMask::Cellular(mask) => mask.is_visible(local_x, local_y, width, height, t),
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

// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_mask.rs</FILE> - <DESC>Prepared mask enum for pipeline rendering</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
