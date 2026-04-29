// <FILE>crates/tui-vfx-player-ui/src/cls_player_ui_state.rs</FILE> - <DESC>State for the K1 visual player shell</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K1: keep UI state layered on K0 player/session APIs.</WCTX>
// <CLOG>0.1.0: INIT — load recipe/catalog, sample K0 frames, and mutate UI controls.</CLOG>

use std::path::PathBuf;

use tui_vfx_contract::{LifecyclePhase, RecipeDocument, Value};
use tui_vfx_player::{
    PlayerFrameReport, PlayerSampleRequest, PlayerSession, RecipePlayer, load_descriptor_catalog,
};

use crate::{
    CliOptions, PlayerUiCommand,
    fnc_player_ui_state_support::{cycle_phase, dwell_trigger_signal, read_recipe},
};

/// Mutable state for one visual player UI session.
#[derive(Debug)]
pub struct PlayerUiState {
    /// Loaded recipe path.
    pub recipe_path: PathBuf,
    /// Active recipe document.
    pub recipe: RecipeDocument,
    player: RecipePlayer,
    session: PlayerSession,
    request: PlayerSampleRequest,
    /// Whether elapsed ticking is paused.
    pub paused: bool,
    /// Whether sample_t stays stable instead of ticking.
    pub motion_disabled: bool,
    /// Elapsed UI time in milliseconds.
    pub elapsed_ms: u64,
    /// Last user-facing status message.
    pub message: String,
    /// Whether help should be visible.
    pub show_help: bool,
    /// Most recent K0 frame report.
    pub last_report: PlayerFrameReport,
}

impl PlayerUiState {
    /// Load a recipe, descriptor catalog, and initial K0 snapshot.
    pub fn load(options: &CliOptions) -> Result<Self, String> {
        let descriptor_load =
            load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
        let recipe = read_recipe(&options.recipe_path)?;
        let player = RecipePlayer::new(descriptor_load.catalog);
        let session = PlayerSession::new();
        let request = PlayerSampleRequest {
            width: options.width,
            height: options.height,
            ..PlayerSampleRequest::default()
        };
        let last_report = player.render_recipe(&recipe, &request);
        Ok(Self {
            recipe_path: options.recipe_path.clone(),
            recipe,
            player,
            session,
            request,
            paused: false,
            motion_disabled: false,
            elapsed_ms: 0,
            message: "loaded canonical v3.1 recipe through K0 player".to_string(),
            show_help: false,
            last_report,
        })
    }

    /// Load another recipe path through the existing K0 player/catalog.
    pub fn load_recipe_path(&mut self, path: PathBuf) -> Result<(), String> {
        self.recipe = read_recipe(&path)?;
        self.recipe_path = path;
        self.session.reset();
        self.request.signals.clear();
        self.elapsed_ms = 0;
        self.paused = false;
        self.message = "loaded selected canonical v3.1 recipe through K0 player".to_string();
        self.render();
        Ok(())
    }

    /// Return the active phase.
    pub fn phase(&self) -> LifecyclePhase {
        self.request.phase
    }

    /// Return sample_t / phase_t.
    pub fn phase_t(&self) -> f64 {
        self.request.phase_t
    }

    /// Return optional loop_t.
    pub fn loop_t(&self) -> Option<f64> {
        self.request.loop_t
    }

    /// Return a borrowed latest report.
    pub fn report(&self) -> &PlayerFrameReport {
        &self.last_report
    }

    /// Apply one UI command. Returns false when the UI should quit.
    pub fn apply_command(&mut self, command: PlayerUiCommand) -> bool {
        match command {
            PlayerUiCommand::Quit => return false,
            PlayerUiCommand::Help => self.show_help = !self.show_help,
            PlayerUiCommand::TogglePause => self.toggle_pause(),
            PlayerUiCommand::Reset => self.reset(),
            PlayerUiCommand::ToggleMotionDisabled => self.toggle_motion_disabled(),
            PlayerUiCommand::PreviousPhase => self.cycle_phase(-1),
            PlayerUiCommand::NextPhase => self.cycle_phase(1),
            PlayerUiCommand::ScrubBackward => self.scrub(-0.05),
            PlayerUiCommand::ScrubForward => self.scrub(0.05),
            PlayerUiCommand::FireTrigger => self.fire_trigger(),
            PlayerUiCommand::Tick => self.tick(100),
            PlayerUiCommand::Render => {}
        }
        self.render();
        true
    }

    fn render(&mut self) {
        self.last_report = self
            .session
            .render(&self.player, &self.recipe, &self.request);
        if self.last_report.dwell_terminated && self.request.phase == LifecyclePhase::Dwell {
            self.request.phase = LifecyclePhase::Exit;
            self.message = "dwell trigger fired; next sample moved to exit".to_string();
        }
    }

    fn tick(&mut self, delta_ms: u64) {
        if self.paused || self.motion_disabled {
            self.message = "stable sample".to_string();
            return;
        }
        self.elapsed_ms = self.elapsed_ms.saturating_add(delta_ms);
        self.request.phase_t = ((self.elapsed_ms % 1000) as f64) / 1000.0;
        self.request.loop_t = Some(self.request.phase_t);
        self.message = format!("advanced elapsed time to {}ms", self.elapsed_ms);
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        self.message = if self.paused { "paused" } else { "resumed" }.to_string();
    }

    fn reset(&mut self) {
        self.session.reset();
        self.request.signals.clear();
        self.request.phase = LifecyclePhase::Dwell;
        self.request.phase_t = 1.0;
        self.request.loop_t = None;
        self.elapsed_ms = 0;
        self.message = "reset player session and runtime inputs".to_string();
    }

    fn toggle_motion_disabled(&mut self) {
        self.motion_disabled = !self.motion_disabled;
        self.request.phase_t = 1.0;
        self.request.loop_t = None;
        self.message = if self.motion_disabled {
            "motion-disabled stable sample enabled"
        } else {
            "motion ticking enabled"
        }
        .to_string();
    }

    fn cycle_phase(&mut self, delta: i32) {
        self.request.phase = cycle_phase(self.request.phase, delta);
        self.message = format!("phase set to {:?}", self.request.phase);
    }

    fn scrub(&mut self, delta: f64) {
        self.request.phase_t = (self.request.phase_t + delta).clamp(0.0, 1.0);
        self.request.loop_t = Some(self.request.phase_t);
        self.message = format!("sample_t set to {:.2}", self.request.phase_t);
    }

    fn fire_trigger(&mut self) {
        match dwell_trigger_signal(&self.recipe) {
            Some(signal) => {
                self.request
                    .signals
                    .insert(signal.clone(), Value::Boolean(true));
                self.request.phase = LifecyclePhase::Dwell;
                self.message = format!("fired canonical signal `{}`", signal.as_str());
            }
            None => self.message = "recipe has no signal-backed dwell trigger".to_string(),
        }
    }
}

// <FILE>crates/tui-vfx-player-ui/src/cls_player_ui_state.rs</FILE> - <DESC>State for the K1 visual player shell</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
