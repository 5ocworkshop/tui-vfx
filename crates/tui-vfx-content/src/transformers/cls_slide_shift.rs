// <FILE>tui-vfx-content/src/transformers/cls_slide_shift.rs</FILE> - <DESC>SlideShift transformer</DESC>
// <VERS>VERSION: 1.3.0</VERS>
// <WCTX>SlideShift barrier span support</WCTX>
// <CLOG>Allow shift_width barrier span and silence clippy arity warning</CLOG>

use crate::traits::TextTransformer;
use crate::types::{SlideShiftFlowMode, SlideShiftLineMode};
use crate::utils::fnc_graphemes::len_graphemes;
use mixed_signals::prelude::SignalContext;
use std::borrow::Cow;

/// Sliding text that shifts rows after crossing a column span.
#[derive(Debug, Clone)]
pub struct SlideShift {
    start_col: i16,
    end_col: i16,
    start_row: i16,
    shift_col: i16,
    shift_width: u16,
    row_shift: i16,
    line_mode: SlideShiftLineMode,
    flow_mode: SlideShiftFlowMode,
}

impl SlideShift {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        start_col: i16,
        end_col: i16,
        start_row: i16,
        shift_col: i16,
        shift_width: u16,
        row_shift: i16,
        line_mode: SlideShiftLineMode,
        flow_mode: SlideShiftFlowMode,
    ) -> Self {
        Self {
            start_col,
            end_col,
            start_row,
            shift_col,
            shift_width,
            row_shift,
            line_mode,
            flow_mode,
        }
    }
}

impl TextTransformer for SlideShift {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        _signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        let progress = progress.clamp(0.0, 1.0);
        let delta_col = (self.end_col - self.start_col) as f64;
        let col_f = self.start_col as f64 + delta_col * progress;
        let moving_right = delta_col >= 0.0;
        let text_len = target.split('\n').map(len_graphemes).max().unwrap_or(0) as f64;
        let leading = col_f;
        let trailing = if text_len > 0.0 {
            col_f + text_len - 1.0
        } else {
            col_f
        };
        let shift_width = self.shift_width.max(1) as f64;
        let shift_start = self.shift_col as f64;
        let shift_end = shift_start + shift_width - 1.0;
        let shift_crossed = match self.flow_mode {
            SlideShiftFlowMode::StayShifted => {
                if moving_right {
                    leading > shift_end
                } else {
                    leading < shift_start
                }
            }
            SlideShiftFlowMode::FlowBack => {
                if text_len == 0.0 {
                    false
                } else {
                    let min_edge = leading.min(trailing);
                    let max_edge = leading.max(trailing);
                    max_edge >= shift_start && min_edge <= shift_end
                }
            }
        };

        let row = self.start_row + if shift_crossed { self.row_shift } else { 0 };
        let row_pad = row.max(0) as usize;
        let col_pad = (col_f.round() as i16).max(0) as usize;

        if row_pad == 0 && col_pad == 0 {
            return Cow::Borrowed(target);
        }

        let mut output = String::new();
        if row_pad > 0 {
            output.push_str(&"\n".repeat(row_pad));
        }

        if col_pad == 0 {
            output.push_str(target);
            return Cow::Owned(output);
        }

        let indent = " ".repeat(col_pad);
        match self.line_mode {
            SlideShiftLineMode::Block => {
                for (idx, line) in target.split('\n').enumerate() {
                    if idx > 0 {
                        output.push('\n');
                    }
                    output.push_str(&indent);
                    output.push_str(line);
                }
            }
            SlideShiftLineMode::FirstLineOnly => {
                let mut iter = target.split('\n');
                if let Some(first) = iter.next() {
                    output.push_str(&indent);
                    output.push_str(first);
                }
                for line in iter {
                    output.push('\n');
                    output.push_str(line);
                }
            }
        }

        Cow::Owned(output)
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_slide_shift.rs</FILE> - <DESC>SlideShift transformer</DESC>
// <VERS>END OF VERSION: 1.3.0</VERS>
