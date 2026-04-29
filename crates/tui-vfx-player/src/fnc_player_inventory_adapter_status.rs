// <FILE>crates/tui-vfx-player/src/fnc_player_inventory_adapter_status.rs</FILE> - <DESC>Classify inventory adapter status</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Primitive adapter work: classify newly supported text-grid adapters.</WCTX>
// <CLOG>0.2.0: MINOR — mark dissolve and ripple as visible text-grid adapters.
// 0.1.0: INIT — split effect/source adapter status helpers from report DTO.</CLOG>

/// Return the effect adapter classification used by inventory reports.
pub fn effect_adapter_status(effect_id: &str, descriptor_covered: bool) -> &'static str {
    if !descriptor_covered {
        return "missingDescriptor";
    }
    match effect_id {
        "mask.wipe" | "mask.checkers" | "mask.dissolve" | "sampler.ripple" => "visible",
        "filter.dim" | "filter.tint" | "filter.invert" | "filter.greyscale" | "mask.none"
        | "sampler.sineWave" => "noop",
        "shader.borderSweep"
        | "shader.linearGradient"
        | "style.baseStyleOverride"
        | "style.colorFade" => "unsupported",
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
// <VERS>END OF VERSION: 0.2.0</VERS>
