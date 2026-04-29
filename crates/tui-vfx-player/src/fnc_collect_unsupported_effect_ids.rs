// <FILE>crates/tui-vfx-player/src/fnc_collect_unsupported_effect_ids.rs</FILE> - <DESC>Collect unsupported effect ids from player diagnostics</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.2: share unsupported id extraction for visual evidence.</WCTX>
// <CLOG>0.1.0: INIT — add distinct unsupportedEffectAdapter id extraction.</CLOG>

use std::collections::BTreeSet;

use crate::PlayerError;

/// Extract distinct unsupported effect ids from player diagnostics.
pub(crate) fn collect_unsupported_effect_ids(errors: &[PlayerError]) -> Vec<String> {
    errors
        .iter()
        .filter(|error| error.code == "unsupportedEffectAdapter")
        .filter_map(|error| error.details.get("effect"))
        .filter_map(|effect| effect.as_str())
        .map(str::to_string)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_collect_unsupported_effect_ids.rs</FILE> - <DESC>Collect unsupported effect ids from player diagnostics</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
