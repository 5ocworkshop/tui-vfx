// <FILE>tui-vfx-geometry/src/widgets/types.rs</FILE>
// <DESC>Types for shared numpad grid selection</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Focus + selection state shared across configurators</WCTX>
// <CLOG>Added DirectionNumpadSelection and triplet focus types</CLOG>

use crate::types::{Anchor, PathType, SlideDirection};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TripletGridFocus {
    Start,
    Dwell,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowOrientation {
    Inward,
    Outward,
}

impl TripletGridFocus {
    pub fn next(self) -> Self {
        match self {
            TripletGridFocus::Start => TripletGridFocus::Dwell,
            TripletGridFocus::Dwell => TripletGridFocus::End,
            TripletGridFocus::End => TripletGridFocus::Start,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            TripletGridFocus::Start => TripletGridFocus::End,
            TripletGridFocus::Dwell => TripletGridFocus::Start,
            TripletGridFocus::End => TripletGridFocus::Dwell,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectionNumpadSelection {
    /// The numpad digit group (1..=9) selecting a quadrant/edge/center.
    pub digit: char,
    /// Which direction variant is selected within the digit's cycle list.
    pub variant: u8,
}

impl DirectionNumpadSelection {
    pub fn new(digit: char) -> Option<Self> {
        if !matches!(digit, '1'..='9') {
            return None;
        }
        Some(Self { digit, variant: 0 })
    }

    pub fn default_for_direction(direction: SlideDirection) -> Self {
        use SlideDirection::*;
        #[allow(unreachable_patterns)]
        let digit = match direction {
            FromTopLeft => '7',
            FromTop => '8',
            FromTopRight => '9',
            FromLeft => '4',
            Default => '5',
            FromRight => '6',
            FromBottomLeft => '1',
            FromBottom => '2',
            FromBottomRight => '3',
            _ => '5',
        };
        let mut sel = Self { digit, variant: 0 };
        sel.set_to_direction(direction);
        sel
    }

    pub fn set_digit(&mut self, digit: char) {
        if matches!(digit, '1'..='9') {
            self.digit = digit;
            self.variant = 0;
        }
    }

    pub fn cycle(&mut self) {
        let len = crate::widgets::col_numpad_mapping::direction_cycle_for_digit(self.digit).len();
        if len == 0 {
            self.variant = 0;
            return;
        }
        self.variant = (self.variant + 1) % (len as u8);
    }

    pub fn resolve(self) -> SlideDirection {
        let list = crate::widgets::col_numpad_mapping::direction_cycle_for_digit(self.digit);
        if list.is_empty() {
            return SlideDirection::Default;
        }
        list[(self.variant as usize).min(list.len() - 1)]
    }

    fn set_to_direction(&mut self, direction: SlideDirection) {
        let list = crate::widgets::col_numpad_mapping::direction_cycle_for_digit(self.digit);
        if let Some(idx) = list.iter().position(|&d| d == direction) {
            self.variant = idx as u8;
        } else {
            self.variant = 0;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitDirectionSelection {
    SameAsEnter,
    Override(DirectionNumpadSelection),
}

impl ExitDirectionSelection {
    pub fn resolve(self, enter: DirectionNumpadSelection) -> SlideDirection {
        match self {
            ExitDirectionSelection::SameAsEnter => enter.resolve(),
            ExitDirectionSelection::Override(sel) => sel.resolve(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectionSelectionMotion {
    pub anchor: Anchor,
    /// The user-selected direction variant (for display / intent).
    pub hint_direction: SlideDirection,
    /// The base slide direction used for offscreen positioning.
    pub base_direction: SlideDirection,
    /// Path interpolation style (e.g. Arc bulge for edge "arc hints").
    pub path: PathType,
}

// <FILE>tui-vfx-geometry/src/widgets/types.rs</FILE>
// <DESC>Types for shared numpad grid selection</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
