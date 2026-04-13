// <FILE>crates/tui-vfx-probe/src/cls_probe_error.rs</FILE> - <DESC>Probe error type</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1 pipeline probe scaffolding</WCTX>
// <CLOG>NEW: Add a minimal probe error enum for scene validation and runtime failures</CLOG>

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbeError {
    InvalidScene(String),
    InvalidRequest(String),
    Io(String),
    Json(String),
}

impl Display for ProbeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidScene(message) => write!(f, "invalid scene: {message}"),
            Self::InvalidRequest(message) => write!(f, "invalid request: {message}"),
            Self::Io(message) => write!(f, "io error: {message}"),
            Self::Json(message) => write!(f, "json error: {message}"),
        }
    }
}

impl Error for ProbeError {}

impl From<std::io::Error> for ProbeError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

impl From<serde_json::Error> for ProbeError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value.to_string())
    }
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_error.rs</FILE> - <DESC>Probe error type</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
