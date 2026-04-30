// <FILE>crates/tui-vfx-player/src/fnc_resolve_source_asset.rs</FILE> - <DESC>Resolve player-owned source asset references</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Source fidelity adapters: introduce an explicit asset resolver seam without rasterization or command execution.</WCTX>
// <CLOG>0.1.0: INIT — resolve image asset identifiers into deterministic player fallback evidence.</CLOG>

use tui_vfx_contract::{SourceInputId, SourceSpec};

use crate::{PlayerSampleRequest, PlayerStyledGrid, fnc_resolve_value_source::resolve_text};

/// Request passed to player-owned source asset resolvers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerSourceAssetRequest<'a> {
    /// Resolved symbolic asset id from source inputs.
    pub asset_id: &'a str,
}

/// Resolver boundary for source assets consumed by player source adapters.
pub trait PlayerSourceAssetResolver {
    /// Resolve a source.image asset into bounded player-owned material or a diagnostic outcome.
    fn resolve_image_asset(
        &self,
        request: PlayerSourceAssetRequest<'_>,
    ) -> PlayerSourceAssetResolution;
}

/// Default resolver used by the skeleton player when no backend/material resolver is attached.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MissingSourceAssetResolver;

impl PlayerSourceAssetResolver for MissingSourceAssetResolver {
    fn resolve_image_asset(
        &self,
        request: PlayerSourceAssetRequest<'_>,
    ) -> PlayerSourceAssetResolution {
        PlayerSourceAssetResolution::MissingFallback {
            asset_id: request.asset_id.to_string(),
        }
    }
}

/// Bounded player resolution result for source assets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlayerSourceAssetResolution {
    /// Resolver supplied terminal-cell material directly consumable by the player.
    ResolvedGrid {
        /// Resolved compact rows.
        rows: Vec<String>,
        /// Resolved styled-cell substrate for the rows.
        styled_grid: PlayerStyledGrid,
    },
    /// No material resolver was available; render deterministic fallback evidence.
    MissingFallback {
        /// Resolved symbolic asset id.
        asset_id: String,
    },
    /// Resolver rejected the asset without executing commands or rasterizing material.
    Unsupported {
        /// Resolved symbolic asset id.
        asset_id: String,
        /// Human-facing reason for unsupported material.
        reason: String,
    },
}

/// Resolve the image asset id input before passing it across the resolver seam.
pub(crate) fn resolve_image_source_asset_id(
    source: &SourceSpec,
    request: &PlayerSampleRequest,
) -> String {
    resolve_text(
        source.inputs.get(&SourceInputId::new("asset")),
        &request.signals,
        "missing",
    )
}

// <FILE>crates/tui-vfx-player/src/fnc_resolve_source_asset.rs</FILE> - <DESC>Resolve player-owned source asset references</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
