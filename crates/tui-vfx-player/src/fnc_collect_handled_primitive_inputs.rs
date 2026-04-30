// <FILE>crates/tui-vfx-player/src/fnc_collect_handled_primitive_inputs.rs</FILE> - <DESC>Collect player-handled primitive input fields</DESC>
// <VERS>VERSION: 0.8.0</VERS>
// <WCTX>v3.1 descriptor/adapter migration: track player-handled primitive field coverage.</WCTX>
// <CLOG>0.8.0: MINOR — mark migrated spatial and rigid-shake style inputs as handled.
// 0.7.1: PATCH — mark procedural params handled by the player adapter.
// 0.7.0: MINOR — mark style.glitch fields handled after lowering support.
// 0.6.0: MINOR — mark rainbow and shader applyTo fields handled after lowering support.
// 0.5.1: PATCH — mark sampler.crtJitter decayMs as handled.
// 0.5.0: MINOR — mark shredder sampler speed/stripe fields as handled.
// 0.4.0: MINOR — add handled shader gradient/applyTo/position fields.
// 0.3.0: MINOR — add handled fields for source.text fixture coverage.</CLOG>

use std::collections::BTreeSet;

/// Return the current player-handled input fields for a primitive descriptor id.
pub(crate) fn collect_handled_primitive_inputs(
    descriptor_id: &str,
    _used: &BTreeSet<String>,
) -> BTreeSet<String> {
    match descriptor_id {
        "source.card" => fields([
            "message",
            "width",
            "height",
            "foreground",
            "background",
            "borderStyle",
            "borderTrim",
        ]),
        "source.text" => fields(["text", "width", "height"]),
        "source.ansi" => fields(["ansiText", "width", "height"]),
        "source.image" => fields(["asset", "width", "height", "fallbackGlyph"]),
        "source.procedural" => fields(["generator", "width", "height", "seed", "params"]),
        "filter.dim" => fields(["factor", "applyTo"]),
        "filter.tint" => fields(["color", "strength", "applyTo"]),
        "filter.invert" => fields(["applyTo"]),
        "filter.greyscale" => fields(["strength", "applyTo"]),
        "filter.pillButton" => fields([
            "progress",
            "activeColor",
            "inactiveColor",
            "bgColor",
            "buttonColor",
            "edgeWidth",
            "glisten",
        ]),
        "filter.fadeToCanvas" => fields(["canvasColor", "amount", "strength", "applyTo"]),
        "filter.patternFill" => fields(["pattern", "density"]),
        "filter.crt" => fields(["intensity", "glow", "scanlineStrength"]),
        "filter.matrixRain" => fields([
            "speed",
            "density",
            "affect",
            "chars",
            "glyphChangeHz",
            "headColor",
            "mode",
            "preset",
            "seed",
            "speedMax",
            "speedMin",
            "speedMultiplier",
            "tailColor",
            "trailMax",
            "trailMin",
        ]),
        "filter.vignette" => fields([
            "strength",
            "edgeColor",
            "applyTo",
            "progress",
            "bgColor",
            "color",
            "ditherAmount",
            "radius",
            "temporalDitherHz",
            "sides",
        ]),
        "filter.bracketEmphasis" => fields([
            "emphasisColor",
            "edgeWidth",
            "applyTo",
            "progress",
            "bgColor",
            "color",
            "left",
            "right",
        ]),
        "filter.dotIndicator" => fields([
            "activeColor",
            "inactiveColor",
            "period",
            "applyTo",
            "progress",
            "bgColor",
            "color",
            "indicatorChar",
            "position",
        ]),
        "filter.edgeGrow" => fields([
            "direction",
            "progress",
            "edgeColor",
            "applyTo",
            "bgColor",
            "edge",
            "fillColor",
            "marginWidth",
            "peakEighths",
            "restEighths",
        ]),
        "filter.hoverBar" => fields([
            "barColor",
            "thickness",
            "position",
            "applyTo",
            "progress",
            "baseEighths",
            "bgColor",
            "marginWidth",
            "maxEighths",
        ]),
        "filter.kittScanner" => fields([
            "scanColor",
            "trailColor",
            "speed",
            "width",
            "applyTo",
            "progress",
            "axis",
            "bandWidth",
            "boost",
            "boostSeparatorBg",
            "bpm",
            "powerlineMode",
        ]),
        "filter.underlineWipe" => fields([
            "underlineColor",
            "progress",
            "thickness",
            "applyTo",
            "bgColor",
            "color",
            "direction",
            "glisten",
            "gradient",
            "lineChar",
            "rowOffset",
        ]),
        "filter.subPixelBar" => fields([
            "barColor",
            "offset",
            "width",
            "applyTo",
            "progress",
            "animated",
            "direction",
            "filledColor",
            "unfilledColor",
        ]),
        "mask.none" => BTreeSet::new(),
        "mask.wipe" => fields(["direction", "softEdge", "easing"]),
        "mask.wipeCorner" => fields(["direction", "softEdge"]),
        "mask.pathReveal" => fields(["path", "softEdge"]),
        "mask.checkers" => fields(["cellSize"]),
        "mask.cellular" => fields(["cellSize", "seed", "threshold"]),
        "mask.dissolve" => fields(["chunkSize", "seed"]),
        "mask.noiseDither" => fields(["chunkSize", "seed"]),
        "mask.blinds" => fields(["orientation", "count"]),
        "mask.radial" => fields(["origin", "softEdge"]),
        "mask.materialize" | "mask.materializeCorner" => {
            fields(["origin", "softEdge", "chunkSize", "noise", "seed"])
        }
        "mask.iris" => fields(["shape", "softEdge"]),
        "mask.diamond" => fields(["softEdge"]),
        "sampler.sineWave" => fields(["axis", "amplitude", "frequency", "speed", "phaseOffset"]),
        "sampler.ripple" => fields(["amplitude", "center", "speed", "wavelength"]),
        "sampler.shredder" => fields([
            "sliceWidth",
            "offset",
            "stripeWidth",
            "oddSpeed",
            "evenSpeed",
        ]),
        "sampler.faultLine" => fields(["offset", "seed", "intensity", "splitBias"]),
        "sampler.radialTwist" => fields(["strength"]),
        "sampler.crt" => fields(["curvature", "scanlineStrength", "jitter"]),
        "sampler.crtJitter" => fields(["amplitude", "frequency", "decayMs", "seed"]),
        "style.colorFade" => fields(["colorSpace", "target"]),
        "style.colorShift" => fields(["hueShift", "saturationShift", "lightnessShift"]),
        "style.fadeIn" | "style.fadeOut" => fields(["from", "to", "ease", "easing", "applyTo"]),
        "style.pulse" => fields(["color", "pulseColor", "frequency", "applyTo"]),
        "style.italicWindow" => fields(["start", "end"]),
        "style.neonFlicker" => fields(["color", "dimAmount", "italicWindow", "stability"]),
        "style.rainbow" => fields(["rotationSpeed"]),
        "style.glitch" => fields(["seed", "intensity", "italicStart", "italicEnd"]),
        "style.rigidShakeStyle" => fields(["shakePeriod", "numShakes", "pauseDuration"]),
        "style.spatial" => fields(["shader"]),
        "style.baseStyleOverride" => fields(["foreground", "background"]),
        "style.outerBand"
        | "style.moduloRows"
        | "style.moduloColumns"
        | "style.nonEmpty"
        | "style.inner" => fields(["foreground", "background"]),
        "shader.linearGradient" => fields([
            "startColor",
            "endColor",
            "gradient",
            "applyTo",
            "angleDeg",
            "intensity",
            "colorSpace",
        ]),
        "shader.borderSweep" => fields(["color", "speed", "length", "position"]),
        "shader.revealWipe" => fields(["color", "direction"]),
        "shader.highlighter" => fields([
            "color",
            "bandWidth",
            "blendStrength",
            "textContrast",
            "mode",
            "softEdge",
            "direction",
            "rowMask",
            "applyTo",
        ]),
        "shader.focusField" => fields([
            "color",
            "centerX",
            "centerY",
            "radius",
            "intensity",
            "radiusX",
            "radiusY",
            "shape",
            "feather",
            "rectHeight",
            "rectWidth",
            "rectX",
            "rectY",
            "applyTo",
        ]),
        "shader.glistenBand" => fields([
            "color",
            "bandWidth",
            "direction",
            "blendStrength",
            "angleDeg",
            "head",
            "speed",
            "tail",
        ]),
        "shader.wayfindingNode" => fields([
            "currentIndex",
            "activeColor",
            "color",
            "futureStrength",
            "intensity",
            "nodes",
            "previousStrength",
            "radius",
        ]),
        "shader.barberPole" => fields([
            "angleDeg",
            "applyTo",
            "backgroundColor",
            "color",
            "gapWidth",
            "speed",
            "stripeColor",
            "stripeWidth",
        ]),
        "shader.diffusion" => fields([
            "applyTo",
            "centerX",
            "centerY",
            "color",
            "intensity",
            "radius",
            "source",
        ]),
        "shader.radar" => fields(["applyTo", "color", "speed", "tailLength"]),
        "content.typewriter" => fields([
            "speed",
            "speedVariance",
            "cursorCharacter",
            "cursorWake",
            "wakeCells",
        ]),
        "content.marquee" => fields(["speed", "direction", "width"]),
        "content.splitFlap" => fields([
            "settle",
            "cascade",
            "cycles",
            "charset",
            "tileWidth",
            "tileHeight",
            "jitter",
        ]),
        "content.wrapIndicator" => fields(["every"]),
        "content.scramble" => fields(["seed", "charset"]),
        "content.morph" => fields(["target"]),
        "content.redact" => fields(["symbol", "reveal"]),
        "content.mirror" => fields(["axis"]),
        "content.numeric" => fields(["value", "decimals", "prefix", "suffix"]),
        "content.dissolve" => fields(["replacement", "direction", "seed"]),
        "content.odometer" => fields([
            "direction",
            "travel",
            "fromMessage",
            "tileWidth",
            "tileHeight",
        ]),
        "content.cellMotion" => fields(["route", "stagger", "affect"]),
        "content.slideShift" => fields(["startCol", "endCol"]),
        "content.glitchShift" => fields(["amount", "seed"]),
        "content.scrambleGlitchShift" => fields(["seed", "charset", "amount"]),
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
// <VERS>END OF VERSION: 0.5.1</VERS>
