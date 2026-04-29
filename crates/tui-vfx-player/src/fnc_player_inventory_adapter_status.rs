// <FILE>crates/tui-vfx-player/src/fnc_player_inventory_adapter_status.rs</FILE> - <DESC>Classify K0 inventory adapter status</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: isolate inventory adapter classification vocabulary.</WCTX>
// <CLOG>0.1.0: INIT — split effect/source adapter status helpers from report DTO.</CLOG>

/// Return the K0 adapter classification used by inventory reports.
pub fn effect_adapter_status(effect_id: &str, descriptor_covered: bool) -> &'static str {
    if !descriptor_covered {
        return "missingDescriptor";
    }
    match effect_id {
        "mask.wipe" | "mask.checkers" => "visible",
        "filter.dim" | "filter.tint" | "filter.invert" | "filter.greyscale" | "mask.none"
        | "sampler.sineWave" => "noop",
        "mask.dissolve"
        | "sampler.ripple"
        | "shader.borderSweep"
        | "shader.linearGradient"
        | "style.baseStyleOverride"
        | "style.colorFade" => "unsupported",
        _ => "unknown",
    }
}

/// Return the K0 source adapter classification used by inventory reports.
pub fn source_adapter_status(source_id: &str, descriptor_covered: bool) -> &'static str {
    if !descriptor_covered {
        return "missingDescriptor";
    }
    match source_id {
        "source.card" => "visible",
        _ => "unknown",
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_player_inventory_adapter_status.rs</FILE> - <DESC>Classify K0 inventory adapter status</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
