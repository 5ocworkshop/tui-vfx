// <FILE>crates/tui-vfx-player-ui/src/cls_player_ui_command.rs</FILE> - <DESC>Visual player UI command vocabulary</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase K1: model simplified demo-inspired keybindings.</WCTX>
// <CLOG>0.3.0: MINOR — add runtime studio-pane toggle command.
// 0.2.0: MINOR — add black-canvas presentation toggle command.
// 0.1.0: INIT — add pause, reset, motion, phase, scrub, trigger, help, tick, and quit commands.</CLOG>

/// One visual player command parsed from a key or line token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerUiCommand {
    /// Quit the interactive shell.
    Quit,
    /// Toggle help text.
    Help,
    /// Pause or resume ticking.
    TogglePause,
    /// Reset sample time, signals, and trigger latch state.
    Reset,
    /// Toggle motion-disabled stable sample mode.
    ToggleMotionDisabled,
    /// Toggle the player window canvas between default and black.
    ToggleBlackCanvas,
    /// Toggle descriptor-derived studio controls beside the preview.
    ToggleStudio,
    /// Cycle to the previous lifecycle phase.
    PreviousPhase,
    /// Cycle to the next lifecycle phase.
    NextPhase,
    /// Decrease sample_t.
    ScrubBackward,
    /// Increase sample_t.
    ScrubForward,
    /// Fire a canonical signal-backed dwell trigger when present.
    FireTrigger,
    /// Advance elapsed time by one UI tick.
    Tick,
    /// Re-render without mutating state.
    Render,
}

impl PlayerUiCommand {
    /// Parse command aliases from deterministic script input.
    pub fn parse(token: &str) -> Option<Self> {
        match token.trim() {
            "q" | "quit" => Some(Self::Quit),
            "?" | "help" => Some(Self::Help),
            " " | "space" | "pause" => Some(Self::TogglePause),
            "r" | "reset" | "reload" => Some(Self::Reset),
            "m" | "motion" => Some(Self::ToggleMotionDisabled),
            "b" | "black" | "black-canvas" | "background" => Some(Self::ToggleBlackCanvas),
            "s" | "studio" => Some(Self::ToggleStudio),
            "[" | "prev" | "left-phase" => Some(Self::PreviousPhase),
            "]" | "next" | "right-phase" => Some(Self::NextPhase),
            "left" | "h" | "-" => Some(Self::ScrubBackward),
            "right" | "l" | "+" => Some(Self::ScrubForward),
            "t" | "trigger" => Some(Self::FireTrigger),
            "tick" | "enter" | "" => Some(Self::Tick),
            "render" | "draw" => Some(Self::Render),
            _ => None,
        }
    }
}

// <FILE>crates/tui-vfx-player-ui/src/cls_player_ui_command.rs</FILE> - <DESC>Visual player UI command vocabulary</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
