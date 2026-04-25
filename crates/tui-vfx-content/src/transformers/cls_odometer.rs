// <FILE>tui-vfx-content/src/transformers/cls_odometer.rs</FILE> - <DESC>Odometer transformer implementation</DESC>
// <VERS>VERSION: 3.0.0</VERS>
// <WCTX>Replace legacy digit interpolation with grid-first mechanical tile roll.</WCTX>
// <CLOG>Implement structured Odometer via private mechanical grid roll helpers.</CLOG>

use crate::mechanical::{
    MechanicalSizing, MechanicalTile, grid_to_text, paired_grids, roll_grid_window,
};
use crate::traits::TextTransformer;
use crate::types::{OdometerDirection, OdometerTravel};
use mixed_signals::prelude::SignalContext;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Odometer {
    direction: OdometerDirection,
    travel: OdometerTravel,
    tile_width: u16,
    tile_height: u16,
    from_message: Option<String>,
}

impl Odometer {
    pub fn new(
        direction: OdometerDirection,
        travel: OdometerTravel,
        tile_width: u16,
        tile_height: u16,
        from_message: Option<String>,
    ) -> Self {
        Self {
            direction,
            travel,
            tile_width,
            tile_height,
            from_message,
        }
    }
}

impl TextTransformer for Odometer {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        _signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }
        let Some(tile) = MechanicalTile::new(self.tile_width, self.tile_height) else {
            return Cow::Borrowed(target);
        };
        let source = paired_grids(
            self.from_message.as_deref(),
            target,
            MechanicalSizing::PadToMax,
        );
        let grid = roll_grid_window(&source, progress, self.direction, self.travel, tile);
        Cow::Owned(grid_to_text(&grid))
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_odometer.rs</FILE> - <DESC>Odometer transformer implementation</DESC>
// <VERS>END OF VERSION: 3.0.0</VERS>
