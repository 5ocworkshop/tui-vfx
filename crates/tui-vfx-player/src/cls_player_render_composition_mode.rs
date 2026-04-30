// <FILE>crates/tui-vfx-player/src/cls_player_render_composition_mode.rs</FILE> - <DESC>Backend-neutral composition mode vocabulary for player render backends</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Native compositor lowering: expose explicit backend composition modes without importing compositor DTOs into player core.</WCTX>
// <CLOG>0.1.0: INIT — add irResolved/native/auto mode parsing and serialization labels.</CLOG>

/// Backend composition strategy requested by player CLI/UI callers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlayerRenderCompositionMode {
    /// Preserve the K2.19 player-IR-resolved backend behavior.
    #[default]
    IrResolved,
    /// Require direct recipe graph/effect lowering into the selected backend.
    Native,
    /// Try native lowering first and report explicit fallback when unsupported nodes remain.
    Auto,
}

impl PlayerRenderCompositionMode {
    /// Parse a user-facing composition mode label.
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "irResolved" | "ir-resolved" | "ir_resolved" => Ok(Self::IrResolved),
            "native" => Ok(Self::Native),
            "auto" => Ok(Self::Auto),
            other => Err(format!(
                "unknown composition mode `{other}`; expected irResolved, native, or auto"
            )),
        }
    }

    /// Return the stable JSON/user-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::IrResolved => "irResolved",
            Self::Native => "native",
            Self::Auto => "auto",
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_render_composition_mode.rs</FILE> - <DESC>Backend-neutral composition mode vocabulary for player render backends</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
