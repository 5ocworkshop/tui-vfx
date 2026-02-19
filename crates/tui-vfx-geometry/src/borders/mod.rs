// <FILE>tui-vfx-geometry/src/borders/mod.rs</FILE>
// <DESC>Border/chrome helpers (generic, reusable)</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>OFPF normalization: split helpers into fnc_* slices</WCTX>
// <CLOG>Re-exported border trim spec and fnc_* helpers</CLOG>

pub mod border_trim_spec;
pub mod fnc_clipped_edges;
pub mod fnc_vanishing_edge_trim_spec;

pub use border_trim_spec::{BorderSegment, BorderTrimSpec, ClippedEdges};
pub use fnc_clipped_edges::clipped_edges;
pub use fnc_vanishing_edge_trim_spec::vanishing_edge_trim_spec;

// <FILE>tui-vfx-geometry/src/borders/mod.rs</FILE>
// <DESC>Border/chrome helpers (generic, reusable)</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
