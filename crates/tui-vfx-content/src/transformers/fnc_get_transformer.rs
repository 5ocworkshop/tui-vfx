// <FILE>tui-vfx-content/src/transformers/fnc_get_transformer.rs</FILE> - <DESC>Factory function for content transformers</DESC>
// <VERS>VERSION: 3.12.0</VERS>
// <WCTX>Packet 69-A: rate-bearing fields on ContentEffect changed type from SignalOrFloat to VfxBindableValue. The dispatcher passes them through with .clone(), so no body change is needed — version bump records the family touch per Intention 34.</WCTX>
// <CLOG>3.12.0: PATCH — no logic change. Field-type migration on ContentEffect (Packet 69-A) flows through .clone() to the constructors transparently. Metadata envelope bumped to register this file as part of the bindable-parity family.</CLOG>

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
        } => Box::new(Typewriter::new(speed_variance.clone())),
        ContentEffect::Scramble {
            resolve_pace,
            charset,
            seed,
        } => Box::new(Scramble::new(*seed, *charset, resolve_pace.clone())),
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
            tile_width,
            tile_height,
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
            .with_dispersion(*dispersion)
            .with_tile_size(*tile_width, *tile_height);
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
            mechanical,
        } => Box::new(
            Odometer::new(
                *direction,
                *travel,
                *tile_width,
                *tile_height,
                from_message.clone(),
            )
            .with_mechanical(mechanical.clone()),
        ),
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
// <VERS>END OF VERSION: 3.12.0</VERS>
