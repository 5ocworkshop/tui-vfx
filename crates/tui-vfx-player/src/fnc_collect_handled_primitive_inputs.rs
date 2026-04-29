// <FILE>crates/tui-vfx-player/src/fnc_collect_handled_primitive_inputs.rs</FILE> - <DESC>Collect player-handled primitive input fields</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Player evidence tooling: name current adapter-handled inputs for primitive field coverage.</WCTX>
// <CLOG>0.2.0: MINOR — add handled fields for K2.9 simple masks.
// 0.1.0: INIT — add current handled-input lookup for represented primitives.</CLOG>

use std::collections::BTreeSet;

/// Return the current player-handled input fields for a primitive descriptor id.
pub(crate) fn collect_handled_primitive_inputs(
    descriptor_id: &str,
    _used: &BTreeSet<String>,
) -> BTreeSet<String> {
    match descriptor_id {
        "source.card" => fields(["message", "width", "height"]),
        "filter.dim" => fields(["factor", "applyTo"]),
        "filter.tint" => fields(["color", "strength", "applyTo"]),
        "filter.invert" => fields(["applyTo"]),
        "filter.greyscale" => fields(["strength", "applyTo"]),
        "mask.none" => BTreeSet::new(),
        "mask.wipe" => fields(["direction", "softEdge"]),
        "mask.checkers" => fields(["cellSize"]),
        "mask.dissolve" => fields(["chunkSize", "seed"]),
        "mask.blinds" => fields(["orientation", "count"]),
        "mask.radial" => fields(["origin", "softEdge"]),
        "mask.iris" => fields(["shape", "softEdge"]),
        "mask.diamond" => fields(["softEdge"]),
        "sampler.sineWave" => fields(["axis", "amplitude", "frequency", "speed", "phaseOffset"]),
        "sampler.ripple" => fields(["amplitude", "center", "speed", "wavelength"]),
        "style.colorFade" => fields(["colorSpace", "target"]),
        "style.baseStyleOverride" => fields(["foreground", "background"]),
        "shader.linearGradient" => fields([
            "startColor",
            "endColor",
            "angleDeg",
            "intensity",
            "colorSpace",
        ]),
        "shader.borderSweep" => fields(["color", "speed", "length"]),
        _ => BTreeSet::new(),
    }
}

fn fields<const N: usize>(values: [&str; N]) -> BTreeSet<String> {
    values.into_iter().map(str::to_string).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnc_unknown_descriptor_does_not_claim_used_inputs_handled() {
        let used = fields(["unreviewedField"]);

        let handled = collect_handled_primitive_inputs("future.effect", &used);

        assert!(handled.is_empty());
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_collect_handled_primitive_inputs.rs</FILE> - <DESC>Collect player-handled primitive input fields</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
