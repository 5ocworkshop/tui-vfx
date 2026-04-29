// <FILE>crates/tui-vfx-player/src/fnc_render_hash.rs</FILE> - <DESC>Deterministic skeleton player render hash</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: make repeated render evidence stable across runs.</WCTX>
// <CLOG>0.1.0: INIT — add small FNV-1a hash helper without external dependencies.</CLOG>

/// Compute a deterministic FNV-1a hash over player-visible render tokens.
pub fn render_hash(parts: &[String]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

// <FILE>crates/tui-vfx-player/src/fnc_render_hash.rs</FILE> - <DESC>Deterministic skeleton player render hash</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
