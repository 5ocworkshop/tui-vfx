// <FILE>tui-vfx-content/src/transformers/fnc_get_transformer.rs</FILE> - <DESC>Factory function for content transformers</DESC>
// <VERS>VERSION: 3.10.0</VERS>
// <WCTX>Wire structured Odometer tile-roll configuration into transformer factory.</WCTX>
// <CLOG>Construct Odometer from direction/travel/tile/from_message fields.</CLOG>

use crate::traits::TextTransformer;
use crate::transformers::{
    Dissolve, GlitchShift, GlyphCascade, Marquee, Mirror, Morph, Numeric, Odometer, Redact,
    Scramble, ScrambleGlitchShift, SlideShift, SplitFlap, Typewriter, WrapIndicator,
};
use crate::types::ContentEffect;
pub fn get_transformer(effect: &ContentEffect) -> Box<dyn TextTransformer> {
    match effect {
        ContentEffect::Typewriter {
            speed_variance,
            cursor: _,
        } => {
            // Per-frame evaluation: pass SignalOrFloat directly
            // Note: cursor is handled at render layer, not by transformer
            Box::new(Typewriter::new(speed_variance.clone()))
        }
        ContentEffect::Scramble {
            resolve_pace,
            charset,
            seed,
        } => {
            // Per-frame evaluation: pass SignalOrFloat directly
            Box::new(Scramble::new(*seed, *charset, resolve_pace.clone()))
        }
        ContentEffect::GlitchShift {
            shift_amount,
            glitch_start,
            glitch_end,
            seed,
        } => Box::new(GlitchShift::new(
            *shift_amount,
            glitch_start.clone(),
            glitch_end.clone(),
            *seed,
        )),
        ContentEffect::ScrambleGlitchShift {
            resolve_pace,
            charset,
            scramble_seed,
            shift_amount,
            glitch_start,
            glitch_end,
        } => Box::new(ScrambleGlitchShift::new(
            *scramble_seed,
            *charset,
            *shift_amount,
            glitch_start.clone(),
            glitch_end.clone(),
            resolve_pace.clone(),
        )),
        ContentEffect::GlyphCascade {
            alphabet,
            pattern,
            direction,
            seed,
            mode,
        } => Box::new(GlyphCascade::new(
            alphabet.clone(),
            pattern.clone(),
            *direction,
            *seed,
            *mode,
        )),
        ContentEffect::SplitFlap {
            speed,
            cascade,
            cycles,
            jitter,
            charset,
            settle_overshoot,
            leading_blocks,
            settle_hinge,
            spring_settle,
            authentic_timing,
            from_message,
            rolling_flip,
            flip_preview,
            flip_flicker,
            dispersion,
        } => {
            let mut sf = SplitFlap::new_mechanical(
                speed.clone(),
                cascade.clone(),
                cycles.clone(),
                *jitter,
                *charset,
                *settle_overshoot,
                *leading_blocks,
                *settle_hinge,
                *spring_settle,
                *authentic_timing,
            )
            .with_rolling_flip(*rolling_flip)
            .with_flip_preview(*flip_preview)
            .with_flip_flicker(*flip_flicker)
            .with_dispersion(*dispersion);
            if let Some(from) = from_message {
                sf = sf.with_from_message(from.clone());
            }
            Box::new(sf)
        }
        ContentEffect::Odometer {
            direction,
            travel,
            tile_width,
            tile_height,
            from_message,
        } => Box::new(Odometer::new(
            *direction,
            *travel,
            *tile_width,
            *tile_height,
            from_message.clone(),
        )),
        ContentEffect::Redact { symbol } => Box::new(Redact::new(*symbol)),
        ContentEffect::Numeric { format } => Box::new(Numeric::new(format)),
        ContentEffect::Marquee { speed, width } => Box::new(Marquee::new(*width, speed.clone())),
        ContentEffect::SlideShift {
            start_col,
            end_col,
            start_row,
            shift_col,
            shift_width,
            row_shift,
            line_mode,
            flow_mode,
        } => Box::new(SlideShift::new(
            *start_col,
            *end_col,
            *start_row,
            *shift_col,
            *shift_width,
            *row_shift,
            *line_mode,
            *flow_mode,
        )),
        ContentEffect::Mirror { axis } => Box::new(Mirror::new(*axis)),
        ContentEffect::Dissolve {
            replacement,
            pattern,
            direction,
            seed,
        } => Box::new(Dissolve::new(
            *replacement,
            pattern.clone(),
            *direction,
            *seed,
        )),
        ContentEffect::Morph {
            source,
            progression,
            direction,
            seed,
        } => Box::new(Morph::new(source.clone(), *progression, *direction, *seed)),
        ContentEffect::WrapIndicator { prefix, suffix } => {
            Box::new(WrapIndicator::new(prefix.clone(), suffix.clone()))
        }
    }
}

// <FILE>tui-vfx-content/src/transformers/fnc_get_transformer.rs</FILE> - <DESC>Factory function for content transformers</DESC>
// <VERS>END OF VERSION: 3.10.0</VERS>
