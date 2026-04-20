// <FILE>crates/tui-vfx-content/src/sources/cls_rocketsplash_image.rs</FILE> - <DESC>Source primitive that loads a rocketsplash `.rss` splash asset and blits its cells into any tui-vfx Grid, so images compose natively with every downstream VFX primitive (drop shadow, wipe, glisten, etc.).</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Stage 1 of the splash library + VFX integration plan — bridges rocketsplash-rt static image assets into the tui-vfx compositing pipeline.</WCTX>
// <CLOG>0.1.0: initial; thin wrapper over rocketsplash_rt::Splash with an ergonomic blit-to-grid method.</CLOG>

use rocketsplash_rt::{Error as RocketsplashError, Splash};
use tui_vfx_types::Grid;

use super::fnc_blit_render_buffer_to_grid::blit_render_buffer_to_grid;

/// A rocketsplash static image asset, ready to blit onto any
/// [`tui_vfx_types::Grid`]. Wraps [`rocketsplash_rt::Splash`] with an
/// ergonomic grid-blit method so `.rss` assets compose with every tui-vfx
/// primitive (drop shadow, `UnderlineWipe`, `GlistenBandShader`, etc.).
///
/// # Example
/// ```ignore
/// use tui_vfx_content::sources::RocketsplashImage;
/// let bytes = include_bytes!("my_logo.rss");
/// let image = RocketsplashImage::from_bytes(bytes)?;
/// image.blit_into_grid(&mut grid, 10, 5);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// The cells written inherit the colors, text styles, and character payload
/// from the `.rss` asset. Cells with zero opacity in the source are skipped
/// (preserving any grid content underneath), so rocketsplash assets layer
/// cleanly on top of other content.
#[derive(Clone, Debug)]
pub struct RocketsplashImage {
    splash: Splash,
}

impl RocketsplashImage {
    /// Load a rocketsplash image from raw `.rss` bytes.
    ///
    /// Typical use: `RocketsplashImage::from_bytes(include_bytes!("logo.rss"))?`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RocketsplashError> {
        Ok(Self {
            splash: Splash::from_bytes(bytes)?,
        })
    }

    /// Wrap an already-loaded [`Splash`] without re-parsing.
    pub fn from_splash(splash: Splash) -> Self {
        Self { splash }
    }

    /// Image dimensions in terminal cells: `(width, height)`.
    pub fn dimensions(&self) -> (usize, usize) {
        self.splash.dimensions()
    }

    /// Borrow the underlying [`Splash`] for direct access to any method
    /// rocketsplash-rt exposes (metadata, ANSI writers, etc.).
    pub fn splash(&self) -> &Splash {
        &self.splash
    }

    /// Blit every non-transparent cell of this image into `grid` with its
    /// top-left at `(offset_x, offset_y)`. Cells that would fall outside
    /// the grid bounds are silently clipped; zero-opacity source cells are
    /// skipped so any existing grid content is preserved in the gaps.
    pub fn blit_into_grid(&self, grid: &mut dyn Grid, offset_x: usize, offset_y: usize) {
        blit_render_buffer_to_grid(self.splash.buffer(), grid, offset_x, offset_y);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes_rejects_empty_payload() {
        // Empty byte slice is not a valid MessagePack-encoded splash.
        assert!(RocketsplashImage::from_bytes(&[]).is_err());
    }

    #[test]
    fn from_bytes_rejects_garbage_payload() {
        // Non-splash bytes should surface the Error::InvalidFormat variant.
        let garbage: Vec<u8> = (0..32).collect();
        assert!(RocketsplashImage::from_bytes(&garbage).is_err());
    }
}

// <FILE>crates/tui-vfx-content/src/sources/cls_rocketsplash_image.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
