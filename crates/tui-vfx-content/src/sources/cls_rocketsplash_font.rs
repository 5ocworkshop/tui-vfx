// <FILE>crates/tui-vfx-content/src/sources/cls_rocketsplash_font.rs</FILE> - <DESC>Source primitive that loads a rocketsplash `.rsf` font atlas and dynamically rasterizes text against it, blitting the rendered cells into any tui-vfx Grid. Enables font-rendered brand text, labels, and headings composed with the full VFX pipeline.</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Stage 1 of the splash library + VFX integration plan — brings rocketsplash's font-to-braille-art rasterizer into the tui-vfx content layer so any consumer can produce big branded text composable with every downstream effect.</WCTX>
// <CLOG>0.1.0: initial; wraps rocketsplash_rt::Font + TextBuilder with ergonomic blit-to-grid at the end of the render chain.</CLOG>

use rocketsplash_rt::{Error as RocketsplashError, Font};
use tui_vfx_types::Grid;

use super::fnc_blit_render_buffer_to_grid::blit_render_buffer_to_grid;

/// A rocketsplash font atlas, loaded from a `.rsf` file and ready to
/// rasterize arbitrary text into a [`tui_vfx_types::Grid`].
///
/// Produces the same [`rocketsplash_rt::RenderBuffer`] substrate as
/// [`crate::sources::RocketsplashImage`], so rendered text composes with
/// every downstream tui-vfx primitive (shadows, wipes, glisten, filters,
/// text transformers on top of the art layer, etc.).
///
/// # Example
/// ```ignore
/// use tui_vfx_content::sources::RocketsplashFont;
/// let font = RocketsplashFont::from_bytes(include_bytes!("dejavu_bold_20.rsf"))?;
/// font.render("HELLO WORLD")
///     .color((255, 200, 0))
///     .blit_into_grid(&mut grid, 4, 2)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Rocketsplash fonts rasterize via sub-cell braille/block patterns so
/// glyphs can scale continuously from ~20px to ~400px. Font atlases are
/// cached in the [`Font`] value, so repeated `render(...)` calls against
/// the same instance are cheap.
#[derive(Clone, Debug)]
pub struct RocketsplashFont {
    font: Font,
}

impl RocketsplashFont {
    /// Load a rocketsplash font from raw `.rsf` bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RocketsplashError> {
        Ok(Self {
            font: Font::from_bytes(bytes)?,
        })
    }

    /// Wrap an already-loaded [`Font`] without re-parsing.
    pub fn from_font(font: Font) -> Self {
        Self { font }
    }

    /// Borrow the underlying [`Font`] for direct access to any method
    /// rocketsplash-rt exposes (`available_chars`, `has_glyph`,
    /// `line_height`, style probing, etc.).
    pub fn font(&self) -> &Font {
        &self.font
    }

    /// Begin rendering `text` against this font atlas. Returns a builder
    /// that mirrors [`rocketsplash_rt::TextBuilder`] but terminates in
    /// [`FontRender::blit_into_grid`] instead of ANSI output.
    pub fn render<'a>(&'a self, text: &'a str) -> FontRender<'a> {
        FontRender {
            builder: self.font.render(text),
        }
    }
}

/// Builder for a single font-rendered text block. Configure colors,
/// gradients, drop shadow, alignment, spacing via the chainable methods,
/// then call [`blit_into_grid`](FontRender::blit_into_grid) to rasterize
/// onto a tui-vfx [`Grid`].
pub struct FontRender<'a> {
    builder: rocketsplash_rt::TextBuilder<'a>,
}

impl<'a> FontRender<'a> {
    /// Set a solid text color as an `(r, g, b)` tuple or any type that
    /// converts into [`rocketsplash_rt::Color`].
    pub fn color(mut self, color: impl Into<rocketsplash_rt::Color>) -> Self {
        self.builder = self.builder.color(color);
        self
    }

    /// Horizontal gradient from `start` to `end`.
    pub fn gradient(
        mut self,
        start: impl Into<rocketsplash_rt::Color>,
        end: impl Into<rocketsplash_rt::Color>,
    ) -> Self {
        self.builder = self.builder.gradient(start, end);
        self
    }

    /// Vertical gradient from `top` to `bottom`.
    pub fn vertical_gradient(
        mut self,
        top: impl Into<rocketsplash_rt::Color>,
        bottom: impl Into<rocketsplash_rt::Color>,
    ) -> Self {
        self.builder = self.builder.vertical_gradient(top, bottom);
        self
    }

    /// Drop shadow with offset `(dx, dy)` and the given color.
    pub fn shadow(mut self, dx: i8, dy: i8, color: impl Into<rocketsplash_rt::Color>) -> Self {
        self.builder = self.builder.shadow(dx, dy, color);
        self
    }

    /// Quick-enable a sensible default drop shadow.
    pub fn drop_shadow(mut self) -> Self {
        self.builder = self.builder.drop_shadow();
        self
    }

    /// Apply a text style (bold / italic / underline / reverse). Requires
    /// that the font atlas carries glyphs for the requested style; see
    /// [`Font::has_style`](rocketsplash_rt::Font::has_style).
    pub fn style(mut self, style: rocketsplash_rt::TextStyle) -> Self {
        self.builder = self.builder.style(style);
        self
    }

    /// Letter-spacing adjustment in sub-cell units (can be negative to
    /// tighten tracking).
    pub fn spacing(mut self, adjust: i8) -> Self {
        self.builder = self.builder.spacing(adjust);
        self
    }

    /// Horizontal alignment.
    pub fn align(mut self, align: rocketsplash_rt::Align) -> Self {
        self.builder = self.builder.align(align);
        self
    }

    /// Fallback behaviour for missing glyphs.
    pub fn fallback(mut self, mode: rocketsplash_rt::FallbackMode) -> Self {
        self.builder = self.builder.fallback(mode);
        self
    }

    /// Terminal color mode for the final rasterization.
    pub fn color_mode(mut self, mode: rocketsplash_rt::ColorMode) -> Self {
        self.builder = self.builder.color_mode(mode);
        self
    }

    /// Rasterize the configured text through the font atlas and blit the
    /// resulting cells into `grid` at `(offset_x, offset_y)`. Zero-opacity
    /// cells (the negative space around glyphs) are skipped, so the text
    /// layers cleanly on top of any underlying grid content.
    pub fn blit_into_grid(
        self,
        grid: &mut dyn Grid,
        offset_x: usize,
        offset_y: usize,
    ) -> Result<(), RocketsplashError> {
        let buffer = self.builder.build_buffer()?;
        blit_render_buffer_to_grid(&buffer, grid, offset_x, offset_y);
        Ok(())
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
        assert!(RocketsplashFont::from_bytes(&[]).is_err());
    }

    #[test]
    fn from_bytes_rejects_garbage_payload() {
        let garbage: Vec<u8> = (0..32).collect();
        assert!(RocketsplashFont::from_bytes(&garbage).is_err());
    }
}

// <FILE>crates/tui-vfx-content/src/sources/cls_rocketsplash_font.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
