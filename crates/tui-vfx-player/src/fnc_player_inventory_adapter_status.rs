// <FILE>crates/tui-vfx-player/src/fnc_player_inventory_adapter_status.rs</FILE> - <DESC>Classify inventory adapter status</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>K2.11 source descriptor pilot: classify source.text as visible.</WCTX>
// <CLOG>0.5.0: MINOR — classify descriptor-backed source.text as a visible source adapter.</CLOG>

/// Return the effect adapter classification used by inventory reports.
pub fn effect_adapter_status(effect_id: &str, descriptor_covered: bool) -> &'static str {
    if !descriptor_covered {
        return "missingDescriptor";
    }
    match effect_id {
        "mask.wipe"
        | "mask.pathReveal"
        | "mask.checkers"
        | "mask.dissolve"
        | "mask.noiseDither"
        | "mask.blinds"
        | "mask.radial"
        | "mask.materialize"
        | "mask.iris"
        | "mask.diamond"
        | "sampler.ripple"
        | "sampler.shredder"
        | "sampler.faultLine"
        | "sampler.radialTwist"
        | "content.typewriter"
        | "content.marquee"
        | "content.splitFlap"
        | "content.wrapIndicator"
        | "content.scramble"
        | "content.morph" => "visible",
        "filter.dim"
        | "filter.tint"
        | "filter.invert"
        | "filter.greyscale"
        | "filter.pillButton"
        | "filter.fadeToCanvas"
        | "filter.patternFill"
        | "filter.crt"
        | "filter.matrixRain"
        | "mask.none"
        | "sampler.sineWave" => "noop",
        "shader.borderSweep"
        | "shader.linearGradient"
        | "style.baseStyleOverride"
        | "style.colorFade"
        | "style.outerBand"
        | "style.moduloRows"
        | "style.moduloColumns"
        | "style.nonEmpty"
        | "style.inner"
        | "shader.revealWipe"
        | "shader.highlighter"
        | "shader.focusField"
        | "shader.glistenBand"
        | "shader.wayfindingNode" => "styled",
        _ => "unknown",
    }
}

/// Return the source adapter classification used by inventory reports.
pub fn source_adapter_status(source_id: &str, descriptor_covered: bool) -> &'static str {
    if !descriptor_covered {
        return "missingDescriptor";
    }
    match source_id {
        "source.card" | "source.text" | "source.ansi" | "source.image" | "source.procedural" => {
            "visible"
        }
        _ => "unknown",
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_player_inventory_adapter_status.rs</FILE> - <DESC>Classify inventory adapter status</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
