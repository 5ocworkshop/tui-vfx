// <FILE>crates/tui-vfx-player-ui/src/cls_player_ui_state.rs</FILE> - <DESC>State for the visual player shell</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Player UI playback: start in enter phase and advance through lifecycle phases so migrated enter/exit effects visibly animate.</WCTX>
// <CLOG>0.4.0: MINOR — track FPS/frame time and black-canvas presentation state.
// 0.3.0: MINOR — advance UI playback through enter, dwell, and exit phases instead of ticking one static phase.
// 0.2.0: MINOR — generate rich effect/source controls with current values and mutation target metadata.
// 0.1.2: PATCH — make generated control target selection explicit without changing control ids.
// 0.1.1: PATCH — keep metadata footer at the physical end of the source file.
// 0.1.0: INIT — load recipe/catalog, sample player frames, and mutate UI controls.</CLOG>

use std::{collections::VecDeque, path::PathBuf};

use tui_vfx_contract::{
    DescriptorCatalog, DurationSpec, DwellPolicy, EffectDescriptor, LifecyclePhase, PhaseTiming,
    RecipeDocument, SourceDescriptor, Value, ValueKind, ValueSource, ValueSpec,
};
use tui_vfx_player::{
    PlayerFrameReport, PlayerRenderBackend, PlayerRenderBackendOptions, PlayerRenderBackendOutput,
    PlayerRenderBackendRequest, PlayerRenderCompositionMode, PlayerRenderIrReport,
    PlayerSampleRequest, PlayerSession, RecipePlayer, StyledCellRenderBackend,
    TextGridRenderBackend, load_descriptor_catalog,
};
use tui_vfx_player_backend_compositor::render_compositor_backend_request;

use crate::{
    CliOptions, PlayerUiCommand, PlayerUiControl,
    fnc_player_ui_state_support::{cycle_phase, dwell_trigger_fire_value, read_recipe},
};

/// Mutable state for one visual player UI session.
#[derive(Debug)]
pub struct PlayerUiState {
    /// Loaded recipe path.
    pub recipe_path: PathBuf,
    /// Canonical recipe browser root requested at startup.
    pub recipes_root: Option<PathBuf>,
    /// Active recipe document.
    pub recipe: RecipeDocument,
    player: RecipePlayer,
    session: PlayerSession,
    request: PlayerSampleRequest,
    /// Selected render backend for preview output.
    pub backend: String,
    /// Selected backend composition mode for compositor output.
    pub composition_mode: PlayerRenderCompositionMode,
    /// Fail rendering when a backend reports fallback.
    pub fail_on_fallback: bool,
    /// Whether descriptor-derived studio controls are shown.
    pub studio: bool,
    /// Generated studio controls derived from recipe signals and signal-backed node inputs.
    pub controls: Vec<PlayerUiControl>,
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
    /// Whether the player window canvas should use a black background.
    pub black_canvas: bool,
    frame_stats: PlayerUiFrameStats,
    /// Most recent player frame report.
    pub last_report: PlayerFrameReport,
    /// Most recent backend output derived from player render IR.
    pub last_backend_output: PlayerRenderBackendOutput,
}

impl PlayerUiState {
    /// Load a recipe, descriptor catalog, and initial player snapshot.
    pub fn load(options: &CliOptions) -> Result<Self, String> {
        let descriptor_load =
            load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
        let catalog = descriptor_load.catalog;
        let recipe = read_recipe(&options.recipe_path)?;
        let player = RecipePlayer::new(catalog.clone());
        let session = PlayerSession::new();
        let request = PlayerSampleRequest {
            phase: LifecyclePhase::Enter,
            phase_t: 0.0,
            absolute_t_ms: Some(0.0),
            width: options.width,
            height: options.height,
            ..PlayerSampleRequest::default()
        };
        let last_report = player.render_recipe(&recipe, &request);
        let mut render_ir = player.render_recipe_ir(&recipe, &request);
        render_ir.path = Some(options.recipe_path.display().to_string());
        let mut source_ir = player.render_recipe_source_ir(&recipe, &request);
        source_ir.path = Some(options.recipe_path.display().to_string());
        let composition_mode = PlayerRenderCompositionMode::parse(&options.composition_mode)?;
        let backend_options = PlayerRenderBackendOptions {
            composition_mode,
            fail_on_fallback: options.fail_on_fallback,
        };
        let last_backend_output = render_backend_output(
            &render_ir,
            &source_ir,
            &options.backend,
            &recipe,
            catalog.clone(),
            &request,
            backend_options,
        )?;
        let controls = generated_controls(&recipe, &catalog);
        Ok(Self {
            recipe_path: options.recipe_path.clone(),
            recipes_root: options.recipes_root.clone(),
            recipe,
            player,
            session,
            request,
            backend: options.backend.clone(),
            composition_mode,
            fail_on_fallback: options.fail_on_fallback,
            studio: options.studio,
            controls,
            paused: false,
            motion_disabled: false,
            elapsed_ms: 0,
            message: "loaded canonical v3.1 recipe through player".to_string(),
            show_help: false,
            black_canvas: false,
            frame_stats: PlayerUiFrameStats::default(),
            last_report,
            last_backend_output,
        })
    }

    /// Load another recipe path through the existing player/catalog.
    pub fn load_recipe_path(&mut self, path: PathBuf) -> Result<(), String> {
        self.reload_recipe_from_path(path, "loaded selected canonical v3.1 recipe through player")
    }

    /// Re-read the active recipe JSON from disk, reset volatile playback state, and render.
    pub fn reload_active_recipe_from_disk(&mut self) -> Result<(), String> {
        self.reload_recipe_from_path(
            self.recipe_path.clone(),
            "reloaded active recipe JSON from disk and reset player session",
        )
    }

    fn reload_recipe_from_path(&mut self, path: PathBuf, message: &str) -> Result<(), String> {
        self.recipe = read_recipe(&path)?;
        self.recipe_path = path;
        self.session.reset();
        self.request.signals.clear();
        self.request.runtime_input_overrides.clear();
        self.request.phase = LifecyclePhase::Enter;
        self.request.phase_t = 0.0;
        self.request.loop_t = None;
        self.request.absolute_t_ms = Some(0.0);
        self.elapsed_ms = 0;
        self.paused = false;
        self.controls = generated_controls(&self.recipe, self.player.descriptor_catalog());
        self.message = message.to_string();
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

    /// Return whether the player window canvas is currently forced to black.
    pub fn black_canvas_enabled(&self) -> bool {
        self.black_canvas
    }

    /// Return the current rolling frames-per-second estimate.
    pub fn fps(&self) -> f64 {
        self.frame_stats.fps()
    }

    /// Return the most recent frame time in milliseconds.
    pub fn frame_time_ms(&self) -> f64 {
        self.frame_stats.frame_time_ms()
    }

    /// Return a borrowed latest report.
    pub fn report(&self) -> &PlayerFrameReport {
        &self.last_report
    }

    /// Apply a generated studio control assignment and re-render.
    pub fn set_control_value(&mut self, key: &str, value: Value) -> Result<(), String> {
        let normalized_key = normalize_key(key);
        let control = self
            .controls
            .iter()
            .find(|control| {
                normalize_key(&control.id) == normalized_key
                    || normalize_key(&control.signal_id) == normalized_key
                    || normalize_key(&control.runtime_input) == normalized_key
                    || normalize_key(&control.input_name) == normalized_key
                    || normalize_key(&control.label) == normalized_key
            })
            .cloned()
            .ok_or_else(|| format!("unknown studio control `{key}`"))?;
        if control.target_kind == "signal" {
            self.request.signals.insert(
                tui_vfx_contract::SignalId::new(control.signal_id.clone()),
                value.clone(),
            );
            self.message = format!(
                "set studio control `{}` through signal `{}`",
                control.id, control.signal_id
            );
        } else {
            self.request
                .runtime_input_overrides
                .insert(control.runtime_input.clone(), value.clone());
            self.message = format!(
                "set studio control `{}` through runtime input `{}`",
                control.id, control.runtime_input
            );
        }
        self.update_control_current_value(&control.id, &value);
        self.render();
        Ok(())
    }

    /// Apply a simple keyboard-driven mutation for the selected studio control.
    pub fn mutate_studio_control_interactively(&mut self, index: usize) {
        let Some(control) = self.controls.get(index).cloned() else {
            self.message = "studio has no descriptor-derived controls".to_string();
            return;
        };
        let value = self.interactive_value_for_control(&control);
        if let Err(error) = self.set_control_value(&control.id, value) {
            self.message = error;
        }
    }

    fn update_control_current_value(&mut self, control_id: &str, value: &Value) {
        if let Some(control) = self
            .controls
            .iter_mut()
            .find(|control| control.id == control_id)
        {
            control.current_value = Some(value_to_json(value));
        }
    }

    fn interactive_value_for_control(&self, control: &PlayerUiControl) -> Value {
        let current_value = self.effective_control_value(control);
        match control.value_kind.as_str() {
            "boolean" => Value::Boolean(!boolean_value(current_value.as_ref()).unwrap_or(false)),
            "integer" | "duration" => {
                Value::Integer(next_integer_value(current_value.as_ref(), control))
            }
            "number" => Value::Number(next_number_value(current_value.as_ref(), control)),
            "color" => Value::String("#ff00ff".to_string()),
            "enum" => Value::String(next_enum_value(current_value.as_ref(), control)),
            _ => Value::String("STUDIO KEYBOARD OVERRIDE".to_string()),
        }
    }

    fn effective_control_value(&self, control: &PlayerUiControl) -> Option<Value> {
        if control.target_kind == "signal" {
            self.request
                .signals
                .get(&tui_vfx_contract::SignalId::new(control.signal_id.clone()))
                .cloned()
                .or_else(|| recipe_signal_fallback(&self.recipe, control))
                .or_else(|| json_to_value(control.current_value.as_ref()))
                .or_else(|| json_to_value(control.default_value.as_ref()))
        } else {
            self.request
                .runtime_input_overrides
                .get(&control.runtime_input)
                .cloned()
                .or_else(|| json_to_value(control.current_value.as_ref()))
                .or_else(|| json_to_value(control.default_value.as_ref()))
        }
    }

    /// Apply one UI command. Returns false when the UI should quit.
    pub fn apply_command(&mut self, command: PlayerUiCommand) -> bool {
        match command {
            PlayerUiCommand::Quit => return false,
            PlayerUiCommand::Help => self.show_help = !self.show_help,
            PlayerUiCommand::TogglePause => self.toggle_pause(),
            PlayerUiCommand::Reset => {
                if let Err(error) = self.reload_active_recipe_from_disk() {
                    self.message = error;
                }
            }
            PlayerUiCommand::ToggleMotionDisabled => self.toggle_motion_disabled(),
            PlayerUiCommand::ToggleBlackCanvas => self.toggle_black_canvas(),
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

    /// Advance the live playback clock by a measured frame delta.
    pub fn advance_time(&mut self, delta_ms: u64) {
        if self.show_help {
            return;
        }
        let delta_ms = delta_ms.max(1);
        self.frame_stats.record_frame_delta(delta_ms);
        self.tick(delta_ms);
        self.render();
    }

    fn render(&mut self) {
        self.last_report = self
            .session
            .render(&self.player, &self.recipe, &self.request);
        let mut render_ir = self.player.render_recipe_ir(&self.recipe, &self.request);
        render_ir.path = Some(self.recipe_path.display().to_string());
        let mut source_ir = self
            .player
            .render_recipe_source_ir(&self.recipe, &self.request);
        source_ir.path = Some(self.recipe_path.display().to_string());
        let backend_options = PlayerRenderBackendOptions {
            composition_mode: self.composition_mode,
            fail_on_fallback: self.fail_on_fallback,
        };
        match render_backend_output(
            &render_ir,
            &source_ir,
            &self.backend,
            &self.recipe,
            self.player.descriptor_catalog().clone(),
            &self.request,
            backend_options,
        ) {
            Ok(output) => self.last_backend_output = output,
            Err(error) => self.message = error,
        }
        if self.last_report.dwell_terminated && self.request.phase == LifecyclePhase::Dwell {
            self.request.phase = LifecyclePhase::Exit;
            self.message = if self.message.starts_with("fired canonical signal") {
                format!(
                    "dwell trigger fired; next sample moved to exit ({})",
                    self.message
                )
            } else {
                "dwell trigger fired; next sample moved to exit".to_string()
            };
        }
    }

    fn tick(&mut self, delta_ms: u64) {
        if self.paused || self.motion_disabled {
            self.message = "stable sample".to_string();
            return;
        }
        self.advance_lifecycle(delta_ms);
        let phase_duration_ms = current_phase_duration_ms(&self.recipe, self.request.phase);
        self.request.loop_t = Some(self.request.phase_t);
        self.request.absolute_t_ms = Some(self.elapsed_ms as f64);
        self.message = format!(
            "advanced {:?} phase to {:.2} using phase duration {}ms",
            self.request.phase, self.request.phase_t, phase_duration_ms
        );
    }

    fn advance_lifecycle(&mut self, delta_ms: u64) {
        self.elapsed_ms = self.elapsed_ms.saturating_add(delta_ms);
        let mut remaining_ms = delta_ms;
        loop {
            let phase_duration_ms = current_phase_duration_ms(&self.recipe, self.request.phase);
            let phase_elapsed_ms = (self.request.phase_t * phase_duration_ms as f64).round() as u64;
            let next_elapsed_ms = phase_elapsed_ms.saturating_add(remaining_ms);
            if next_elapsed_ms < phase_duration_ms {
                self.request.phase_t = phase_fraction(next_elapsed_ms, phase_duration_ms);
                return;
            }

            remaining_ms = next_elapsed_ms.saturating_sub(phase_duration_ms);
            self.request.phase = next_playback_phase(self.request.phase);
            self.request.phase_t = 0.0;
            if remaining_ms == 0 {
                return;
            }
        }
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        self.message = if self.paused { "paused" } else { "resumed" }.to_string();
    }

    fn toggle_motion_disabled(&mut self) {
        self.motion_disabled = !self.motion_disabled;
        self.request.phase_t = 1.0;
        self.request.loop_t = None;
        self.request.absolute_t_ms = (!self.motion_disabled).then_some(self.elapsed_ms as f64);
        self.message = if self.motion_disabled {
            "motion-disabled stable sample enabled"
        } else {
            "motion ticking enabled"
        }
        .to_string();
    }

    fn toggle_black_canvas(&mut self) {
        self.black_canvas = !self.black_canvas;
        self.message = if self.black_canvas {
            "black player canvas enabled"
        } else {
            "default player canvas restored"
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
        self.request.absolute_t_ms = None;
        self.message = format!("sample_t set to {:.2}", self.request.phase_t);
    }

    fn fire_trigger(&mut self) {
        match dwell_trigger_fire_value(&self.recipe) {
            Some((signal, value)) => {
                self.request.signals.insert(signal.clone(), value);
                self.request.phase = LifecyclePhase::Dwell;
                self.message = format!("fired canonical signal `{}`", signal.as_str());
            }
            None => self.message = "recipe has no signal-backed dwell trigger".to_string(),
        }
    }
}

#[derive(Debug, Default)]
struct PlayerUiFrameStats {
    frame_deltas_ms: VecDeque<u64>,
    cached_fps: f64,
    frame_time_ms: f64,
}

impl PlayerUiFrameStats {
    const WINDOW_SIZE: usize = 60;

    fn record_frame_delta(&mut self, delta_ms: u64) {
        self.frame_time_ms = delta_ms as f64;
        self.frame_deltas_ms.push_front(delta_ms.max(1));
        while self.frame_deltas_ms.len() > Self::WINDOW_SIZE {
            self.frame_deltas_ms.pop_back();
        }
        let total_ms: u64 = self.frame_deltas_ms.iter().copied().sum();
        if total_ms > 0 {
            self.cached_fps = self.frame_deltas_ms.len() as f64 * 1000.0 / total_ms as f64;
        }
    }

    fn fps(&self) -> f64 {
        self.cached_fps
    }

    fn frame_time_ms(&self) -> f64 {
        self.frame_time_ms
    }
}

fn current_phase_duration_ms(recipe: &RecipeDocument, phase: LifecyclePhase) -> u64 {
    recipe
        .lifecycle
        .as_ref()
        .and_then(|lifecycle| {
            lifecycle
                .phases
                .iter()
                .find(|phase_spec| phase_spec.phase == phase)
        })
        .and_then(|phase_spec| phase_timing_duration_ms(&phase_spec.timing))
        .filter(|duration| *duration > 0)
        .unwrap_or(1000)
}

fn phase_timing_duration_ms(timing: &PhaseTiming) -> Option<u64> {
    match timing {
        PhaseTiming::Fixed { duration } => Some(duration_ms(duration)),
        PhaseTiming::Dwell { policy } => dwell_policy_duration_ms(policy),
    }
}

fn dwell_policy_duration_ms(policy: &DwellPolicy) -> Option<u64> {
    match policy {
        DwellPolicy::Fixed { duration } => Some(duration_ms(duration)),
        DwellPolicy::Until { max_duration, .. } => max_duration.as_ref().map(duration_ms),
    }
}

fn duration_ms(duration: &DurationSpec) -> u64 {
    match duration {
        DurationSpec::Milliseconds { value } => *value,
        DurationSpec::Seconds { value } => (*value * 1000.0).round().max(0.0) as u64,
    }
}

fn phase_fraction(elapsed_ms: u64, phase_duration_ms: u64) -> f64 {
    if phase_duration_ms == 0 {
        return 1.0;
    }
    (elapsed_ms.min(phase_duration_ms) as f64) / (phase_duration_ms as f64)
}

fn next_playback_phase(phase: LifecyclePhase) -> LifecyclePhase {
    match phase {
        LifecyclePhase::Enter => LifecyclePhase::Dwell,
        LifecyclePhase::Dwell => LifecyclePhase::Exit,
        LifecyclePhase::Exit => LifecyclePhase::Enter,
    }
}

fn recipe_signal_fallback(recipe: &RecipeDocument, control: &PlayerUiControl) -> Option<Value> {
    recipe
        .graph
        .signals
        .get(&tui_vfx_contract::SignalId::new(control.signal_id.clone()))
        .and_then(|signal| signal.value.default.clone())
}

fn json_to_value(value: Option<&serde_json::Value>) -> Option<Value> {
    value.and_then(|value| {
        serde_json::from_value::<Value>(value.clone())
            .ok()
            .or_else(|| {
                serde_json::from_value::<ValueSource>(value.clone())
                    .ok()
                    .and_then(value_from_source)
            })
    })
}

fn value_from_source(source: ValueSource) -> Option<Value> {
    match source {
        ValueSource::Literal { value }
        | ValueSource::Parameter {
            fallback: Some(value),
            ..
        }
        | ValueSource::Signal {
            fallback: Some(value),
            ..
        }
        | ValueSource::GraphValue {
            fallback: Some(value),
            ..
        }
        | ValueSource::SampledField {
            fallback: Some(value),
            ..
        } => Some(value),
        _ => None,
    }
}

fn boolean_value(value: Option<&Value>) -> Option<bool> {
    match value {
        Some(Value::Boolean(value)) => Some(*value),
        _ => None,
    }
}

fn next_integer_value(value: Option<&Value>, control: &PlayerUiControl) -> i64 {
    let current = match value {
        Some(Value::Integer(value)) => *value,
        Some(Value::Number(value)) | Some(Value::Duration(value)) => value.round() as i64,
        _ => 0,
    };
    let next = current.saturating_add(1);
    clamp_integer(next, control)
}

fn next_number_value(value: Option<&Value>, control: &PlayerUiControl) -> f64 {
    let current = match value {
        Some(Value::Number(value)) | Some(Value::Duration(value)) => *value,
        Some(Value::Integer(value)) => *value as f64,
        _ => 0.0,
    };
    let step = control
        .range
        .and_then(|range| range.min.zip(range.max))
        .map(|(min, max)| ((max - min) / 20.0).abs().max(0.01))
        .unwrap_or(1.0);
    clamp_number(current + step, control)
}

fn next_enum_value(value: Option<&Value>, control: &PlayerUiControl) -> String {
    if control.allowed_values.is_empty() {
        return value
            .and_then(Value::as_enum_value)
            .unwrap_or("interactive")
            .to_string();
    }
    let current = value.and_then(Value::as_enum_value).or(match value {
        Some(Value::String(value)) => Some(value.as_str()),
        _ => None,
    });
    let next_index = current
        .and_then(|current| {
            control
                .allowed_values
                .iter()
                .position(|allowed| allowed == current)
        })
        .map(|index| (index + 1) % control.allowed_values.len())
        .unwrap_or(0);
    control.allowed_values[next_index].clone()
}

fn clamp_integer(value: i64, control: &PlayerUiControl) -> i64 {
    let Some(range) = control.range else {
        return value;
    };
    let min = range
        .min
        .map(|value| value.round() as i64)
        .unwrap_or(i64::MIN);
    let max = range
        .max
        .map(|value| value.round() as i64)
        .unwrap_or(i64::MAX);
    value.clamp(min, max)
}

fn clamp_number(value: f64, control: &PlayerUiControl) -> f64 {
    let Some(range) = control.range else {
        return value;
    };
    value
        .max(range.min.unwrap_or(f64::NEG_INFINITY))
        .min(range.max.unwrap_or(f64::INFINITY))
}

fn render_backend_output(
    render_ir: &PlayerRenderIrReport,
    source_ir: &PlayerRenderIrReport,
    backend: &str,
    recipe: &RecipeDocument,
    descriptor_catalog: tui_vfx_contract::DescriptorCatalog,
    sample: &PlayerSampleRequest,
    backend_options: PlayerRenderBackendOptions,
) -> Result<PlayerRenderBackendOutput, String> {
    let fail_on_fallback = backend_options.fail_on_fallback;
    let require_native_lowering = matches!(
        backend_options.composition_mode,
        PlayerRenderCompositionMode::Native
    );
    let output = match backend {
        "text" | "textGrid" | "text-grid" => TextGridRenderBackend.render(render_ir),
        "styled" | "styledCell" | "styled-cell" => StyledCellRenderBackend.render(render_ir),
        "compositor" => render_compositor_backend_request(&PlayerRenderBackendRequest {
            ir: render_ir.clone(),
            source_ir: source_ir.clone(),
            recipe: recipe.clone(),
            descriptor_catalog,
            sample: sample.clone(),
            backend_options,
        }),
        other => {
            return Err(format!(
                "unknown backend `{other}`; expected textGrid, styledCell, or compositor"
            ));
        }
    };
    if fail_on_fallback && output.fallback_used {
        return Err("backend fallback forbidden by --fail-on-fallback".to_string());
    }
    if require_native_lowering && !output.native_lowering_succeeded {
        return Err("native composition mode did not lower every graph node".to_string());
    }
    Ok(output)
}

fn generated_controls(
    recipe: &RecipeDocument,
    catalog: &DescriptorCatalog,
) -> Vec<PlayerUiControl> {
    let mut controls = Vec::new();
    for (source_instance_id, source) in &recipe.sources {
        let descriptor = source_descriptor(catalog, source.source.as_str());
        for (input_id, value_source) in &source.inputs {
            let descriptor_input =
                descriptor.and_then(|descriptor| descriptor.inputs.get(input_id));
            let runtime_input = format!(
                "source:{}:{}:{}",
                source.source.as_str(),
                source_instance_id.as_str(),
                input_id.as_str()
            );
            controls.push(PlayerUiControl {
                id: runtime_input.clone(),
                label: descriptor_input
                    .and_then(|input| input.display_name.clone())
                    .unwrap_or_else(|| input_id.as_str().to_string()),
                value_kind: descriptor_input
                    .map(|input| value_kind_label(input.value.kind).to_string())
                    .or_else(|| value_kind_from_source(recipe, value_source))
                    .unwrap_or_else(|| "unknown".to_string()),
                source_kind: "sourceInput".to_string(),
                descriptor_id: source.source.as_str().to_string(),
                node_id: None,
                source_instance_id: Some(source_instance_id.as_str().to_string()),
                input_name: input_id.as_str().to_string(),
                control_kind: descriptor_input
                    .map(|input| control_kind(&input.value))
                    .unwrap_or("valueInput")
                    .to_string(),
                current_value: Some(value_source_to_json(value_source)),
                default_value: descriptor_input
                    .and_then(|input| input.value.default.as_ref())
                    .map(value_to_json),
                range: descriptor_input.and_then(|input| input.value.range),
                allowed_values: descriptor_input
                    .map(|input| input.value.allowed_values.clone())
                    .unwrap_or_default(),
                runtime_mutability: descriptor_input
                    .map(|input| format!("{:?}", input.runtime_mutability))
                    .unwrap_or_else(|| "Unknown".to_string()),
                optional: descriptor_input
                    .map(|input| input.optional)
                    .unwrap_or(false),
                target_kind: "runtimeInputOverride".to_string(),
                signal_id: String::new(),
                runtime_input,
                source: format!("{}:{}", source.source.as_str(), input_id.as_str()),
            });
        }
    }
    for node_id in &recipe.graph.order {
        let Some(node) = recipe.graph.nodes.get(node_id) else {
            continue;
        };
        let descriptor = effect_descriptor(recipe, catalog, node.effect.as_str());
        for (input_id, source) in &node.inputs {
            let descriptor_input =
                descriptor.and_then(|descriptor| descriptor.inputs.get(input_id));
            let (target_kind, signal_id) = control_target(source);
            let runtime_input = format!(
                "effect:{}:{}:{}",
                node.effect.as_str(),
                node.id.as_str(),
                input_id.as_str()
            );
            controls.push(PlayerUiControl {
                id: runtime_input.clone(),
                label: descriptor_input
                    .and_then(|input| input.display_name.clone())
                    .unwrap_or_else(|| input_id.as_str().to_string()),
                value_kind: descriptor_input
                    .map(|input| value_kind_label(input.value.kind).to_string())
                    .or_else(|| value_kind_from_source(recipe, source))
                    .unwrap_or_else(|| "unknown".to_string()),
                source_kind: "descriptorInput".to_string(),
                descriptor_id: node.effect.as_str().to_string(),
                node_id: Some(node.id.as_str().to_string()),
                source_instance_id: None,
                input_name: input_id.as_str().to_string(),
                control_kind: descriptor_input
                    .map(|input| control_kind(&input.value))
                    .unwrap_or("valueInput")
                    .to_string(),
                current_value: Some(value_source_to_json(source)),
                default_value: descriptor_input
                    .and_then(|input| input.value.default.as_ref())
                    .map(value_to_json),
                range: descriptor_input.and_then(|input| input.value.range),
                allowed_values: descriptor_input
                    .map(|input| input.value.allowed_values.clone())
                    .unwrap_or_default(),
                runtime_mutability: descriptor_input
                    .map(|input| format!("{:?}", input.runtime_mutability))
                    .unwrap_or_else(|| "Unknown".to_string()),
                optional: descriptor_input
                    .map(|input| input.optional)
                    .unwrap_or(false),
                target_kind: target_kind.to_string(),
                signal_id,
                runtime_input,
                source: format!("{}:{}", node.effect.as_str(), input_id.as_str()),
            });
        }
    }
    controls.sort_by(|left, right| left.id.cmp(&right.id));
    controls
}

fn source_descriptor<'a>(
    catalog: &'a DescriptorCatalog,
    source_id: &str,
) -> Option<&'a SourceDescriptor> {
    catalog.packs.values().find_map(|pack| {
        pack.source_descriptors
            .get(&tui_vfx_contract::SourceId::new(source_id))
    })
}

fn effect_descriptor<'a>(
    recipe: &'a RecipeDocument,
    catalog: &'a DescriptorCatalog,
    effect_id: &str,
) -> Option<&'a EffectDescriptor> {
    recipe
        .graph
        .effects
        .get(&tui_vfx_contract::EffectId::new(effect_id))
        .or_else(|| {
            catalog.packs.values().find_map(|pack| {
                pack.effects
                    .get(&tui_vfx_contract::EffectId::new(effect_id))
            })
        })
}

fn signal_source_id(source: &ValueSource) -> Option<String> {
    match source {
        ValueSource::Signal { id, .. } => Some(id.as_str().to_string()),
        _ => None,
    }
}

fn control_target(source: &ValueSource) -> (&'static str, String) {
    match signal_source_id(source) {
        Some(signal_id) => ("signal", signal_id),
        None => ("runtimeInputOverride", String::new()),
    }
}

fn value_kind_from_source(recipe: &RecipeDocument, source: &ValueSource) -> Option<String> {
    match source {
        ValueSource::Signal { id, fallback } => recipe
            .graph
            .signals
            .get(id)
            .map(|signal| value_kind_label(signal.value.kind).to_string())
            .or_else(|| {
                fallback
                    .as_ref()
                    .map(|value| value_kind_label(value.kind()).to_string())
            }),
        ValueSource::Literal { value }
        | ValueSource::Parameter {
            fallback: Some(value),
            ..
        }
        | ValueSource::GraphValue {
            fallback: Some(value),
            ..
        } => Some(value_kind_label(value.kind()).to_string()),
        _ => None,
    }
}

fn control_kind(value: &ValueSpec) -> &'static str {
    match value.kind {
        ValueKind::Integer | ValueKind::Number | ValueKind::Duration if value.range.is_some() => {
            "slider"
        }
        ValueKind::Integer | ValueKind::Number => "numericInput",
        ValueKind::Duration => "durationInput",
        ValueKind::Boolean => "toggle",
        ValueKind::Enum => "select",
        ValueKind::Color => "colorPicker",
        ValueKind::Gradient => "gradientEditor",
        ValueKind::Structured => "structuredJsonEditor",
        ValueKind::Text | ValueKind::String => "textInput",
        _ => "valueInput",
    }
}

fn value_source_to_json(value: &ValueSource) -> serde_json::Value {
    serde_json::to_value(value).expect("value source serializes")
}

fn value_to_json(value: &Value) -> serde_json::Value {
    serde_json::to_value(value).expect("value serializes")
}

fn value_kind_label(kind: ValueKind) -> &'static str {
    match kind {
        ValueKind::Null => "null",
        ValueKind::Boolean => "boolean",
        ValueKind::Integer => "integer",
        ValueKind::Number => "number",
        ValueKind::String => "string",
        ValueKind::Text => "text",
        ValueKind::Color => "color",
        ValueKind::Gradient => "gradient",
        ValueKind::Duration => "duration",
        ValueKind::Enum => "enum",
        ValueKind::Role => "role",
        ValueKind::Scope => "scope",
        ValueKind::Rect => "rect",
        ValueKind::Structured => "structured",
    }
}

fn normalize_key(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

// <FILE>crates/tui-vfx-player-ui/src/cls_player_ui_state.rs</FILE> - <DESC>State for the visual player shell</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
