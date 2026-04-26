// <FILE>tui-vfx-content/src/transformers/cls_glyph_cascade.rs</FILE> - <DESC>Glyph-cascade transformer for evolve-like text transitions</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Slice 6.6 of mechanical circular content cycles plan: TextTransformer signature now takes &TransformContext<'_>.</WCTX>
// <CLOG>1.1.0: TextTransformer signature now takes &TransformContext<'_>; this transformer ignores the context and underscores the parameter.</CLOG>

use crate::traits::{TextTransformer, TransformContext};
use crate::types::{
    DissolveDirection, GlyphCascadeAlphabet, GlyphCascadeMode, GlyphCascadePattern,
};
use mixed_signals::random::hash_to_index;
use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct GlyphCascade {
    alphabet: GlyphCascadeAlphabet,
    pattern: GlyphCascadePattern,
    direction: DissolveDirection,
    seed: u64,
    mode: GlyphCascadeMode,
}

impl GlyphCascade {
    pub fn new(
        alphabet: GlyphCascadeAlphabet,
        pattern: GlyphCascadePattern,
        direction: DissolveDirection,
        seed: u64,
        mode: GlyphCascadeMode,
    ) -> Self {
        Self {
            alphabet,
            pattern,
            direction,
            seed,
            mode,
        }
    }

    fn threshold(&self, i: usize, total: usize, text: &str) -> f32 {
        if total == 0 {
            return 0.0;
        }
        match self.pattern {
            GlyphCascadePattern::Sequential => self.sequential_threshold(i, total),
            GlyphCascadePattern::Random => self.random_threshold(i, total),
            GlyphCascadePattern::EdgeIn => self.edge_in_threshold(i, total),
            GlyphCascadePattern::EdgeOut => self.edge_out_threshold(i, total),
            GlyphCascadePattern::ByWord => self.by_word_threshold(i, text),
            GlyphCascadePattern::ByLine => self.by_line_threshold(i, text),
        }
    }

    fn sequential_threshold(&self, i: usize, total: usize) -> f32 {
        let pos = match self.direction {
            DissolveDirection::LeftToRight => i as f32 / total as f32,
            DissolveDirection::RightToLeft => (total - 1 - i) as f32 / total as f32,
            DissolveDirection::CenterIn => {
                let center = total as f32 / 2.0;
                let dist = (i as f32 - center).abs();
                1.0 - (dist / center.max(1.0))
            }
            DissolveDirection::CenterOut => {
                let center = total as f32 / 2.0;
                let dist = (i as f32 - center).abs();
                dist / center.max(1.0)
            }
        };
        pos.clamp(0.0, 1.0)
    }

    fn random_threshold(&self, i: usize, total: usize) -> f32 {
        hash_to_index(self.seed, i as u64, total) as f32 / total as f32
    }

    fn edge_in_threshold(&self, i: usize, total: usize) -> f32 {
        let center = total as f32 / 2.0;
        let dist = (i as f32 - center).abs();
        1.0 - (dist / center.max(1.0)).clamp(0.0, 1.0)
    }

    fn edge_out_threshold(&self, i: usize, total: usize) -> f32 {
        let center = total as f32 / 2.0;
        let dist = (i as f32 - center).abs();
        (dist / center.max(1.0)).clamp(0.0, 1.0)
    }

    fn by_word_threshold(&self, char_idx: usize, text: &str) -> f32 {
        let graphemes: Vec<&str> = text.graphemes(true).collect();
        let mut word_indices = Vec::with_capacity(graphemes.len());
        let mut word_count = 0;
        let mut in_word = false;
        for g in &graphemes {
            let is_space = g.chars().all(|c| c.is_whitespace());
            if !is_space && !in_word {
                in_word = true;
                word_count += 1;
            } else if is_space && in_word {
                in_word = false;
            }
            word_indices.push(if in_word { word_count - 1 } else { word_count });
        }
        let total_words = text.split_whitespace().count().max(1);
        let word_idx = *word_indices.get(char_idx).unwrap_or(&0);
        hash_to_index(self.seed, word_idx as u64, total_words) as f32 / total_words as f32
    }

    fn by_line_threshold(&self, char_idx: usize, text: &str) -> f32 {
        let graphemes: Vec<&str> = text.graphemes(true).collect();
        let mut line_idx = 0;
        for (idx, g) in graphemes.iter().enumerate() {
            if idx == char_idx {
                break;
            }
            if *g == "\n" {
                line_idx += 1;
            }
        }
        let total_lines = text.lines().count().max(1);
        hash_to_index(self.seed, line_idx as u64, total_lines) as f32 / total_lines as f32
    }

    fn glyph_at(&self, progress: f32) -> char {
        let glyphs = self.alphabet.glyphs();
        let idx =
            (progress.clamp(0.0, 1.0) * (glyphs.len().saturating_sub(1)) as f32).round() as usize;
        glyphs[idx.min(glyphs.len().saturating_sub(1))]
    }
}

impl TextTransformer for GlyphCascade {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        _ctx: &TransformContext<'_>,
    ) -> Cow<'a, str> {
        if target.is_empty() {
            return Cow::Borrowed(target);
        }

        let graphemes: Vec<&str> = target.graphemes(true).collect();
        let total = graphemes.len().max(1);
        let progress = progress.clamp(0.0, 1.0) as f32;
        let mut out = String::with_capacity(target.len());

        for (i, g) in graphemes.iter().enumerate() {
            if g.chars().all(|c| c.is_whitespace()) {
                out.push_str(g);
                continue;
            }

            let threshold = self.threshold(i, total, target);
            let local = if progress <= threshold {
                0.0
            } else {
                ((progress - threshold) / (1.0 - threshold).max(f32::EPSILON)).clamp(0.0, 1.0)
            };
            match self.mode {
                GlyphCascadeMode::IntoTarget => {
                    if local >= 1.0 {
                        out.push_str(g);
                    } else {
                        out.push(self.glyph_at(local));
                    }
                }
                GlyphCascadeMode::FromTarget => {
                    if local <= 0.0 {
                        out.push_str(g);
                    } else {
                        out.push(self.glyph_at(local));
                    }
                }
                GlyphCascadeMode::GlyphsOnly => out.push(self.glyph_at(local)),
            }
        }

        Cow::Owned(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mixed_signals::prelude::SignalContext;
    use tui_vfx_style::traits::ShaderRuntimeParams;

    fn empty_ctx() -> (SignalContext, ShaderRuntimeParams) {
        (SignalContext::default(), ShaderRuntimeParams::new())
    }

    #[test]
    fn into_target_lands_on_target() {
        let effect = GlyphCascade::new(
            GlyphCascadeAlphabet::Circles,
            GlyphCascadePattern::Sequential,
            DissolveDirection::LeftToRight,
            7,
            GlyphCascadeMode::IntoTarget,
        );
        let (sig, params) = empty_ctx();
        assert_eq!(
            effect.transform("TEST", 1.0, &TransformContext::new(&sig, &params)),
            "TEST"
        );
    }

    #[test]
    fn glyphs_only_uses_alphabet() {
        let effect = GlyphCascade::new(
            GlyphCascadeAlphabet::Shaded,
            GlyphCascadePattern::Sequential,
            DissolveDirection::LeftToRight,
            7,
            GlyphCascadeMode::GlyphsOnly,
        );
        let (sig, params) = empty_ctx();
        let rendered = effect.transform("AB", 0.5, &TransformContext::new(&sig, &params));
        assert!(
            rendered
                .chars()
                .all(|c| [' ', '░', '▒', '▓', '█'].contains(&c))
        );
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_glyph_cascade.rs</FILE> - <DESC>Glyph-cascade transformer for evolve-like text transitions</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
