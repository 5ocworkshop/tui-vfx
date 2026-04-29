// <FILE>crates/tui-vfx-player/src/fnc_player_inventory_adapter_status.rs</FILE> - <DESC>Classify inventory adapter status</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Primitive adapter work: classify supported text-grid and styled-cell adapters.</WCTX>
// <CLOG>0.4.0: MINOR — classify K2.9 simple masks as visible text-grid adapters.
// 0.3.0: MINOR — mark K2.5 styled primitive adapters as supported styled evidence.
// 0.2.0: MINOR — mark dissolve and ripple as visible text-grid adapters.
// 0.1.0: INIT — split effect/source adapter status helpers from report DTO.</CLOG>

/// Return the effect adapter classification used by inventory reports.
pub fn effect_adapter_status(effect_id: &str, descriptor_covered: bool) -> &'static str {
    if !descriptor_covered {
        return "missingDescriptor";
    }
    match effect_id {
        "mask.wipe" | "mask.checkers" | "mask.dissolve" | "mask.blinds" | "mask.radial"
        | "mask.iris" | "mask.diamond" | "sampler.ripple" => "visible",
        "filter.dim" | "filter.tint" | "filter.invert" | "filter.greyscale" | "mask.none"
        | "sampler.sineWave" => "noop",
        "shader.borderSweep"
        | "shader.linearGradient"
        | "style.baseStyleOverride"
        | "style.colorFade" => "styled",
        _ => "unknown",
    }
}

/// Return the source adapter classification used by inventory reports.
pub fn source_adapter_status(source_id: &str, descriptor_covered: bool) -> &'static str {
    if !descriptor_covered {
        return "missingDescriptor";
    }
    match source_id {
        "source.card" => "visible",
        _ => "unknown",
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_player_inventory_adapter_status.rs</FILE> - <DESC>Classify inventory adapter status</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
