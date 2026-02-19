// <FILE>tui-vfx-debug/src/lib.rs</FILE> - <DESC>Centralized debug logger crate root</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>WG3: Debug Logger Integration</WCTX>
// <CLOG>Initial creation as standalone crate</CLOG>

mod config;
mod logger;

pub use config::{LogLevel, ModuleConfig};
pub use logger::{DebugLogger, LogEntry, Logger, create_logger, get_global_logger};

// <FILE>tui-vfx-debug/src/lib.rs</FILE> - <DESC>Centralized debug logger crate root</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
