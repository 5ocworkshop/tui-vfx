// main.rs
// Cargo.toml dependencies:
//   ratatui = "0.29"
//   crossterm = "0.28"
//   rand = "0.8"
//
// Esc exits. Left / Right switch between Beams and Sweep.

use std::{
    collections::HashSet,
    error::Error,
    io::{self, IsTerminal, Read},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{seq::SliceRandom, Rng};
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::Rect,
    style::{Color as TuiColor, Style},
    widgets::Widget,
    Terminal,
};

const FPS: u64 = 60;
const RESTART_HOLD_FRAMES: u32 = 45;
const BACKGROUND: Rgb = Rgb::new(40, 42, 54);

const DEMO_TEXT: &str = r#"                          _______ _______ _______
                         (_______|_______|_______)
                             _       _    _____
                            | |     | |  |  ___)
                            | |     | |  | |_____
                            |_|     |_|  |_______)
                         ==========================
TerminalTextEffects applies visual effects to text in the terminal.

The TTE animation engine has the following features:
        * Xterm 256 / RGB hex color support
        * Complex character movement via Paths, Waypoints, and
          motion easing.
        * Complex animations via Scenes with symbol/color changes,
          layers, easing, and Path synced progression.
        * Event handling for Path/Scene state changes with
          custom callback support and many pre-defined actions.
        * Variable stop/step color gradient generation.
        * Extensive effect customization via per-effect arguments.
        * Runs inline, preserving terminal state and workflow.
Installation:
        * pip install TerminalTextEffects
        * pipx install TerminalTextEffects
More Info:
    https://github.com/ChrisBuilds/terminaltexteffects
    https://pypi.org/project/terminaltexteffects/"#;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input_or_demo();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let result = run(&mut terminal, input);

    terminal.show_cursor()?;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}

fn read_input_or_demo() -> String {
    let mut input = String::new();
    if !io::stdin().is_terminal() {
        let _ = io::stdin().read_to_string(&mut input);
    }
    if input.trim().is_empty() {
        DEMO_TEXT.to_string()
    } else {
        input
    }
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    input: String,
) -> Result<(), Box<dyn Error>> {
    let size = terminal.size()?;
    let mut app = App::new(EffectKind::Beams, size.width.max(1), size.height.max(1), input);
    let tick_rate = Duration::from_nanos(1_000_000_000 / FPS);
    let mut last_tick = Instant::now() - tick_rate;

    loop {
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Left => app.previous(),
                    KeyCode::Right => app.next(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let size = terminal.size()?;
            app.resize(size.width.max(1), size.height.max(1));
            app.tick();
            terminal.draw(|frame| {
                let area = frame.area();
                frame.render_widget(EffectWidget { terminal: app.terminal() }, area);
            })?;
            last_tick = Instant::now();
        }
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EffectKind {
    Beams,
    Sweep,
}

impl EffectKind {
    fn next(self) -> Self {
        match self {
            Self::Beams => Self::Sweep,
            Self::Sweep => Self::Beams,
        }
    }

    fn previous(self) -> Self {
        self.next()
    }
}

struct App {
    kind: EffectKind,
    effect: EffectState,
    input: String,
    width: u16,
    height: u16,
    hold_frames: u32,
}

impl App {
    fn new(kind: EffectKind, width: u16, height: u16, input: String) -> Self {
        let effect = EffectState::new(kind, width, height, &input);
        Self {
            kind,
            effect,
            input,
            width,
            height,
            hold_frames: 0,
        }
    }

    fn terminal(&self) -> &SimTerminal {
        self.effect.terminal()
    }

    fn resize(&mut self, width: u16, height: u16) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.reset();
        }
    }

    fn reset(&mut self) {
        self.effect = EffectState::new(self.kind, self.width, self.height, &self.input);
        self.hold_frames = 0;
    }

    fn set_kind(&mut self, kind: EffectKind) {
        self.kind = kind;
        self.reset();
    }

    fn next(&mut self) {
        self.set_kind(self.kind.next());
    }

    fn previous(&mut self) {
        self.set_kind(self.kind.previous());
    }

    fn tick(&mut self) {
        if self.effect.is_finished() {
            self.hold_frames += 1;
            if self.hold_frames >= RESTART_HOLD_FRAMES {
                self.reset();
            }
        } else {
            self.effect.tick();
        }
    }
}

enum EffectState {
    Beams(BeamsSim),
    Sweep(SweepSim),
}

impl EffectState {
    fn new(kind: EffectKind, width: u16, height: u16, input: &str) -> Self {
        match kind {
            EffectKind::Beams => Self::Beams(BeamsSim::new(width, height, input)),
            EffectKind::Sweep => Self::Sweep(SweepSim::new(width, height, input)),
        }
    }

    fn terminal(&self) -> &SimTerminal {
        match self {
            Self::Beams(effect) => &effect.terminal,
            Self::Sweep(effect) => &effect.terminal,
        }
    }

    fn tick(&mut self) {
        match self {
            Self::Beams(effect) => effect.tick(),
            Self::Sweep(effect) => effect.tick(),
        }
    }

    fn is_finished(&self) -> bool {
        match self {
            Self::Beams(effect) => effect.is_finished(),
            Self::Sweep(effect) => effect.is_finished(),
        }
    }
}

struct EffectWidget<'a> {
    terminal: &'a SimTerminal,
}

impl<'a> Widget for EffectWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let bg_style = Style::default().bg(BACKGROUND.to_tui());
        for y in 0..area.height {
            for x in 0..area.width {
                buf.get_mut(area.x + x, area.y + y)
                    .set_symbol(" ")
                    .set_style(bg_style);
            }
        }

        for ch in &self.terminal.characters {
            if !ch.visible {
                continue;
            }
            if ch.coord.column < 1
                || ch.coord.row < 1
                || ch.coord.column > self.terminal.canvas.width as i32
                || ch.coord.row > self.terminal.canvas.height as i32
            {
                continue;
            }
            let x = (ch.coord.column - 1) as u16;
            let y = (self.terminal.canvas.height as i32 - ch.coord.row) as u16;
            if x >= area.width || y >= area.height {
                continue;
            }

            let mut style = Style::default().bg(BACKGROUND.to_tui());
            if let Some(fg) = ch.visual.colors.fg {
                style = style.fg(fg.to_tui());
            }
            if let Some(bg) = ch.visual.colors.bg {
                style = style.bg(bg.to_tui());
            }
            let mut encoded = [0_u8; 4];
            let symbol = ch.visual.symbol.encode_utf8(&mut encoded);
            buf.get_mut(area.x + x, area.y + y)
                .set_symbol(symbol)
                .set_style(style);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coord {
    column: i32,
    row: i32,
}

impl Coord {
    const fn new(column: i32, row: i32) -> Self {
        Self { column, row }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn from_hex(hex: &str) -> Self {
        let h = hex.trim().trim_start_matches('#');
        let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(0);
        Self { r, g, b }
    }

    fn to_tui(self) -> TuiColor {
        TuiColor::Rgb(self.r, self.g, self.b)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ColorPair {
    fg: Option<Rgb>,
    bg: Option<Rgb>,
}

impl ColorPair {
    const fn fg(color: Rgb) -> Self {
        Self {
            fg: Some(color),
            bg: None,
        }
    }

    const fn empty() -> Self {
        Self { fg: None, bg: None }
    }
}

#[derive(Clone, Debug)]
struct Visual {
    symbol: char,
    colors: ColorPair,
}

impl Visual {
    const fn new(symbol: char, colors: ColorPair) -> Self {
        Self { symbol, colors }
    }
}

#[derive(Clone, Debug)]
struct FrameSpec {
    visual: Visual,
    duration: u16,
}

#[derive(Clone, Debug, Default)]
struct SceneSpec {
    frames: Vec<FrameSpec>,
}

impl SceneSpec {
    fn add_frame(&mut self, symbol: char, duration: u16, colors: ColorPair) {
        self.frames.push(FrameSpec {
            visual: Visual::new(symbol, colors),
            duration: duration.max(1),
        });
    }

    fn apply_gradient_to_symbols(&mut self, symbols: &[char], duration: u16, fg_gradient: &Gradient) {
        let color_pairs: Vec<ColorPair> = fg_gradient
            .spectrum
            .iter()
            .copied()
            .map(ColorPair::fg)
            .collect();

        if symbols.is_empty() || color_pairs.is_empty() {
            return;
        }

        if symbols.len() >= color_pairs.len() {
            let map = cyclic_distribution(symbols.len(), color_pairs.len());
            for (symbol_index, color_index) in map {
                self.add_frame(symbols[symbol_index], duration, color_pairs[color_index]);
            }
        } else {
            let map = cyclic_distribution(color_pairs.len(), symbols.len());
            for (color_index, symbol_index) in map {
                self.add_frame(symbols[symbol_index], duration, color_pairs[color_index]);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SceneKind {
    BeamRow,
    BeamColumn,
    Brighten,
    InitialSweep,
    SecondSweep,
}

#[derive(Clone, Debug, Default)]
struct Scenes {
    beam_row: Option<SceneSpec>,
    beam_column: Option<SceneSpec>,
    brighten: Option<SceneSpec>,
    initial_sweep: Option<SceneSpec>,
    second_sweep: Option<SceneSpec>,
}

impl Scenes {
    fn get(&self, kind: SceneKind) -> Option<&SceneSpec> {
        match kind {
            SceneKind::BeamRow => self.beam_row.as_ref(),
            SceneKind::BeamColumn => self.beam_column.as_ref(),
            SceneKind::Brighten => self.brighten.as_ref(),
            SceneKind::InitialSweep => self.initial_sweep.as_ref(),
            SceneKind::SecondSweep => self.second_sweep.as_ref(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ActiveScene {
    kind: SceneKind,
    frame_index: usize,
    ticks_elapsed: u16,
}

#[derive(Clone, Debug)]
struct Character {
    input_symbol: char,
    coord: Coord,
    visual: Visual,
    visible: bool,
    is_fill: bool,
    scenes: Scenes,
    active_scene: Option<ActiveScene>,
}

impl Character {
    fn new(input_symbol: char, coord: Coord, is_fill: bool) -> Self {
        Self {
            input_symbol,
            coord,
            visual: Visual::new(input_symbol, ColorPair::empty()),
            visible: false,
            is_fill,
            scenes: Scenes::default(),
            active_scene: None,
        }
    }

    fn activate_scene(&mut self, kind: SceneKind) {
        if let Some(scene) = self.scenes.get(kind) {
            if let Some(first) = scene.frames.first() {
                self.active_scene = Some(ActiveScene {
                    kind,
                    frame_index: 0,
                    ticks_elapsed: 0,
                });
                self.visual = first.visual.clone();
            }
        }
    }

    fn tick(&mut self) {
        let Some(active) = self.active_scene else {
            return;
        };
        let Some(scene) = self.scenes.get(active.kind) else {
            self.active_scene = None;
            return;
        };
        if scene.frames.is_empty() || active.frame_index >= scene.frames.len() {
            self.active_scene = None;
            return;
        }

        let frame = &scene.frames[active.frame_index];
        self.visual = frame.visual.clone();

        let mut next = active;
        next.ticks_elapsed += 1;
        if next.ticks_elapsed >= frame.duration {
            next.ticks_elapsed = 0;
            next.frame_index += 1;
        }
        if next.frame_index >= scene.frames.len() {
            self.active_scene = None;
        } else {
            self.active_scene = Some(next);
        }
    }

    fn is_active(&self) -> bool {
        self.active_scene.is_some()
    }
}

#[derive(Clone, Debug)]
struct Canvas {
    width: u16,
    height: u16,
    text_left: i32,
    text_right: i32,
    text_top: i32,
    text_bottom: i32,
}

impl Canvas {
    fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            text_left: 1,
            text_right: width.max(1) as i32,
            text_top: height.max(1) as i32,
            text_bottom: 1,
        }
    }
}

#[derive(Clone, Debug)]
struct SimTerminal {
    canvas: Canvas,
    characters: Vec<Character>,
}

impl SimTerminal {
    fn new(width: u16, height: u16, input: &str) -> Self {
        let width = width.max(1);
        let height = height.max(1);
        let mut canvas = Canvas::new(width, height);
        let mut characters = Vec::new();
        let mut occupied = vec![None::<usize>; width as usize * height as usize];

        let mut lines: Vec<String> = input.lines().map(|line| line.trim_end().to_string()).collect();
        if lines.is_empty() {
            lines.push("No Input.".to_string());
        }

        let input_height = lines.len().max(1) as i32;
        let input_width = lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(1)
            .max(1) as i32;

        let center_column = centered_axis(width as i32);
        let center_row = centered_axis(height as i32);
        let column_delta = center_column - (input_width / 2);
        let row_delta = center_row - (input_height / 2);

        for (line_index, line) in lines.iter().enumerate() {
            let input_row = input_height - line_index as i32;
            for (column_index, symbol) in line.chars().enumerate() {
                if symbol == ' ' {
                    continue;
                }
                let coord = Coord::new(column_index as i32 + 1 + column_delta, input_row + row_delta);
                if coord.column < 1
                    || coord.row < 1
                    || coord.column > width as i32
                    || coord.row > height as i32
                {
                    continue;
                }
                let idx = characters.len();
                characters.push(Character::new(symbol, coord, false));
                occupied[coord_to_index(width, coord)] = Some(idx);
            }
        }

        let input_coords: Vec<Coord> = characters.iter().filter(|c| !c.is_fill).map(|c| c.coord).collect();
        if !input_coords.is_empty() {
            canvas.text_left = input_coords.iter().map(|c| c.column).min().unwrap();
            canvas.text_right = input_coords.iter().map(|c| c.column).max().unwrap();
            canvas.text_bottom = input_coords.iter().map(|c| c.row).min().unwrap();
            canvas.text_top = input_coords.iter().map(|c| c.row).max().unwrap();
        }

        for row in 1..=height as i32 {
            for column in 1..=width as i32 {
                let coord = Coord::new(column, row);
                let pos = coord_to_index(width, coord);
                if occupied[pos].is_none() {
                    occupied[pos] = Some(characters.len());
                    characters.push(Character::new(' ', coord, true));
                }
            }
        }

        Self { canvas, characters }
    }

    fn set_character_visibility(&mut self, index: usize, visible: bool) {
        if let Some(ch) = self.characters.get_mut(index) {
            ch.visible = visible;
        }
    }

    fn activate_scene(&mut self, index: usize, kind: SceneKind) {
        if let Some(ch) = self.characters.get_mut(index) {
            ch.activate_scene(kind);
        }
    }

    fn update_active(&mut self, active_characters: &mut HashSet<usize>) {
        let active_now: Vec<usize> = active_characters.iter().copied().collect();
        for index in active_now {
            if let Some(ch) = self.characters.get_mut(index) {
                ch.tick();
            }
        }
        active_characters.retain(|index| {
            self.characters
                .get(*index)
                .map(Character::is_active)
                .unwrap_or(false)
        });
    }

    fn selected_indices(&self, include_fill: bool) -> Vec<usize> {
        self.characters
            .iter()
            .enumerate()
            .filter_map(|(i, ch)| (!ch.is_fill || include_fill).then_some(i))
            .collect()
    }

    fn grouped(&self, grouping: CharacterGroup, include_fill: bool) -> Vec<Vec<usize>> {
        let mut indices = self.selected_indices(include_fill);
        indices.sort_by_key(|&i| {
            let c = self.characters[i].coord;
            (c.row, c.column)
        });

        match grouping {
            CharacterGroup::ColumnLeftToRight | CharacterGroup::ColumnRightToLeft => {
                let mut columns = Vec::new();
                for column in 1..=self.canvas.width as i32 {
                    let group: Vec<usize> = indices
                        .iter()
                        .copied()
                        .filter(|&i| self.characters[i].coord.column == column)
                        .collect();
                    if !group.is_empty() {
                        columns.push(group);
                    }
                }
                if grouping == CharacterGroup::ColumnRightToLeft {
                    columns.reverse();
                }
                columns
            }
            CharacterGroup::RowTopToBottom => {
                let mut rows = Vec::new();
                for row in 1..=self.canvas.height as i32 {
                    let group: Vec<usize> = indices
                        .iter()
                        .copied()
                        .filter(|&i| self.characters[i].coord.row == row)
                        .collect();
                    if !group.is_empty() {
                        rows.push(group);
                    }
                }
                rows.reverse();
                rows
            }
            CharacterGroup::DiagonalTopLeftToBottomRight => {
                let mut diagonals = Vec::new();
                for diagonal in (1 - self.canvas.height as i32)..=(self.canvas.width as i32 - 1) {
                    let group: Vec<usize> = indices
                        .iter()
                        .copied()
                        .filter(|&i| {
                            let c = self.characters[i].coord;
                            c.column - c.row == diagonal
                        })
                        .collect();
                    if !group.is_empty() {
                        diagonals.push(group);
                    }
                }
                diagonals
            }
        }
    }
}

fn coord_to_index(width: u16, coord: Coord) -> usize {
    (coord.row as usize - 1) * width as usize + (coord.column as usize - 1)
}

fn centered_axis(max_value: i32) -> i32 {
    let mut center = (max_value / 2).max(1);
    if max_value % 2 == 1 && max_value > 1 {
        center += 1;
    }
    center
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CharacterGroup {
    ColumnLeftToRight,
    ColumnRightToLeft,
    RowTopToBottom,
    DiagonalTopLeftToBottomRight,
}

#[derive(Clone, Debug)]
struct Gradient {
    spectrum: Vec<Rgb>,
}

impl Gradient {
    fn new(stops: &[Rgb], steps: &[usize]) -> Self {
        assert!(!stops.is_empty(), "at least one color stop is required");
        let mut steps = steps.to_vec();
        if steps.is_empty() {
            steps.push(1);
        }
        if stops.len() == 1 {
            return Self {
                spectrum: vec![stops[0]; steps[0].max(1)],
            };
        }

        let pair_count = stops.len() - 1;
        if steps.len() < pair_count {
            let last = *steps.last().unwrap();
            steps.resize(pair_count, last);
        }
        steps.truncate(pair_count);

        let mut spectrum = Vec::new();
        for pair_index in 0..pair_count {
            let start = stops[pair_index];
            let end = stops[pair_index + 1];
            let step_count = steps[pair_index].max(1) as i32;
            let red_delta = (end.r as i32 - start.r as i32).div_euclid(step_count);
            let green_delta = (end.g as i32 - start.g as i32).div_euclid(step_count);
            let blue_delta = (end.b as i32 - start.b as i32).div_euclid(step_count);
            let range_start = if spectrum.is_empty() { 0 } else { 1 };
            for i in range_start..step_count {
                let r = (start.r as i32 + red_delta * i).clamp(0, 255) as u8;
                let g = (start.g as i32 + green_delta * i).clamp(0, 255) as u8;
                let b = (start.b as i32 + blue_delta * i).clamp(0, 255) as u8;
                spectrum.push(Rgb::new(r, g, b));
            }
            spectrum.push(end);
        }

        Self { spectrum }
    }

    fn get_color_at_fraction(&self, fraction: f64) -> Rgb {
        if self.spectrum.is_empty() {
            return Rgb::new(255, 255, 255);
        }
        let fraction = fraction.clamp(0.0, 1.0);
        for i in 1..=self.spectrum.len() {
            if fraction <= i as f64 / self.spectrum.len() as f64 {
                return self.spectrum[i - 1];
            }
        }
        *self.spectrum.last().unwrap()
    }

    fn vertical_mapping(&self, canvas: &Canvas) -> Vec<(Coord, Rgb)> {
        let mut mapping = Vec::new();
        let min_row = canvas.text_bottom.max(1);
        let max_row = canvas.text_top.max(min_row);
        let min_col = canvas.text_left.max(1);
        let max_col = canvas.text_right.max(min_col);
        let row_offset = min_row - 1;
        let denominator = (max_row - row_offset).max(1) as f64;
        for row in min_row..=max_row {
            let fraction = (row - row_offset) as f64 / denominator;
            let color = self.get_color_at_fraction(fraction);
            for column in min_col..=max_col {
                mapping.push((Coord::new(column, row), color));
            }
        }
        mapping
    }
}

fn color_for_coord(mapping: &[(Coord, Rgb)], coord: Coord) -> Rgb {
    mapping
        .iter()
        .find_map(|(c, color)| (*c == coord).then_some(*color))
        .unwrap_or_else(|| Rgb::from_hex("ffffff"))
}

fn adjust_color_brightness(color: Rgb, brightness: f64) -> Rgb {
    fn hue_to_rgb(lightness_scaled: f64, color_intensity: f64, mut hue_value: f64) -> f64 {
        if hue_value < 0.0 {
            hue_value += 1.0;
        }
        if hue_value > 1.0 {
            hue_value -= 1.0;
        }
        if hue_value < 1.0 / 6.0 {
            return lightness_scaled + (color_intensity - lightness_scaled) * 6.0 * hue_value;
        }
        if hue_value < 1.0 / 2.0 {
            return color_intensity;
        }
        if hue_value < 2.0 / 3.0 {
            return lightness_scaled + (color_intensity - lightness_scaled) * (2.0 / 3.0 - hue_value) * 6.0;
        }
        lightness_scaled
    }

    let normalized_red = color.r as f64 / 255.0;
    let normalized_green = color.g as f64 / 255.0;
    let normalized_blue = color.b as f64 / 255.0;

    let max_val = normalized_red.max(normalized_green).max(normalized_blue);
    let min_val = normalized_red.min(normalized_green).min(normalized_blue);
    let mut lightness = (max_val + min_val) / 2.0;
    let lightness_threshold = 0.5;

    let (hue_value, saturation) = if (max_val - min_val).abs() < f64::EPSILON {
        (0.0, 0.0)
    } else {
        let diff = max_val - min_val;
        let saturation = if lightness > lightness_threshold {
            diff / (2.0 - max_val - min_val)
        } else {
            diff / (max_val + min_val)
        };
        let mut hue_value = if (max_val - normalized_red).abs() < f64::EPSILON {
            (normalized_green - normalized_blue) / diff + if normalized_green < normalized_blue { 6.0 } else { 0.0 }
        } else if (max_val - normalized_green).abs() < f64::EPSILON {
            (normalized_blue - normalized_red) / diff + 2.0
        } else {
            (normalized_red - normalized_green) / diff + 4.0
        };
        hue_value /= 6.0;
        (hue_value, saturation)
    };

    lightness = (lightness * brightness).clamp(0.0, 1.0);

    let (red, green, blue) = if saturation == 0.0 {
        (lightness, lightness, lightness)
    } else {
        let color_intensity = if lightness < lightness_threshold {
            lightness * (1.0 + saturation)
        } else {
            lightness + saturation - lightness * saturation
        };
        let lightness_scaled = 2.0 * lightness - color_intensity;
        (
            hue_to_rgb(lightness_scaled, color_intensity, hue_value + 1.0 / 3.0),
            hue_to_rgb(lightness_scaled, color_intensity, hue_value),
            hue_to_rgb(lightness_scaled, color_intensity, hue_value - 1.0 / 3.0),
        )
    };

    Rgb::new((red * 255.0) as u8, (green * 255.0) as u8, (blue * 255.0) as u8)
}

fn cyclic_distribution(larger_len: usize, smaller_len: usize) -> Vec<(usize, usize)> {
    if larger_len == 0 || smaller_len == 0 {
        return Vec::new();
    }
    let repeat_factor = larger_len / smaller_len;
    let mut overflow_count = larger_len % smaller_len;
    let mut overflow_used = false;
    let mut smaller_index = 0_usize;
    let mut current_repeat_factor = 0_usize;
    let mut out = Vec::with_capacity(larger_len);

    for larger_index in 0..larger_len {
        if current_repeat_factor >= repeat_factor {
            if overflow_count > 0 {
                if overflow_used {
                    smaller_index += 1;
                    current_repeat_factor = 0;
                    overflow_used = false;
                } else {
                    overflow_used = true;
                    overflow_count -= 1;
                }
            } else {
                smaller_index += 1;
                current_repeat_factor = 0;
            }
        }
        current_repeat_factor += 1;
        out.push((larger_index, smaller_index.min(smaller_len - 1)));
    }

    out
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BeamDirection {
    Row,
    Column,
}

struct BeamGroup {
    characters: Vec<usize>,
    direction: BeamDirection,
    speed: f64,
    next_character_counter: f64,
    cursor: usize,
}

impl BeamGroup {
    fn new(
        mut characters: Vec<usize>,
        direction: BeamDirection,
        terminal: &SimTerminal,
        rng: &mut impl Rng,
    ) -> Self {
        match direction {
            BeamDirection::Row => characters.sort_by_key(|&i| terminal.characters[i].coord.column),
            BeamDirection::Column => characters.sort_by_key(|&i| terminal.characters[i].coord.row),
        }
        if rng.gen_bool(0.5) {
            characters.reverse();
        }
        let speed_int = match direction {
            BeamDirection::Row => rng.gen_range(15..=60),
            BeamDirection::Column => rng.gen_range(9..=15),
        };
        Self {
            characters,
            direction,
            speed: speed_int as f64 * 0.1,
            next_character_counter: 0.0,
            cursor: 0,
        }
    }

    fn complete(&self) -> bool {
        self.cursor >= self.characters.len()
    }

    fn increment_next_character_counter(&mut self) {
        self.next_character_counter += self.speed;
    }

    fn get_next_character(
        &mut self,
        terminal: &mut SimTerminal,
        active_characters: &mut HashSet<usize>,
    ) -> Option<usize> {
        if self.complete() {
            return None;
        }
        self.next_character_counter -= 1.0;
        let index = self.characters[self.cursor];
        self.cursor += 1;

        let already_active = terminal.characters[index].is_active();
        if !already_active {
            terminal.set_character_visibility(index, true);
        }
        let scene = match self.direction {
            BeamDirection::Row => SceneKind::BeamRow,
            BeamDirection::Column => SceneKind::BeamColumn,
        };
        terminal.activate_scene(index, scene);

        if already_active {
            None
        } else {
            active_characters.insert(index);
            Some(index)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BeamsPhase {
    Beams,
    FinalWipe,
    Complete,
}

struct BeamsSim {
    terminal: SimTerminal,
    pending_groups: Vec<BeamGroup>,
    active_groups: Vec<BeamGroup>,
    active_characters: HashSet<usize>,
    delay: u16,
    phase: BeamsPhase,
    final_wipe_groups: Vec<Vec<usize>>,
    final_wipe_cursor: usize,
}

impl BeamsSim {
    fn new(width: u16, height: u16, input: &str) -> Self {
        let mut rng = rand::thread_rng();
        let mut terminal = SimTerminal::new(width, height, input);
        let final_gradient = Gradient::new(
            &[
                Rgb::from_hex("8A008A"),
                Rgb::from_hex("00D1FF"),
                Rgb::from_hex("ffffff"),
            ],
            &[12],
        );
        let final_mapping = final_gradient.vertical_mapping(&terminal.canvas);
        let beam_gradient = Gradient::new(
            &[
                Rgb::from_hex("ffffff"),
                Rgb::from_hex("00D1FF"),
                Rgb::from_hex("8A008A"),
            ],
            &[2, 6],
        );

        for character in &mut terminal.characters {
            let final_fg = if character.is_fill {
                Rgb::from_hex("000000")
            } else {
                color_for_coord(&final_mapping, character.coord)
            };
            let faded_fg = adjust_color_brightness(final_fg, 0.3);
            let fg_fade_gradient = Gradient::new(&[final_fg, faded_fg], &[10]);
            let fg_brighten_gradient = Gradient::new(&[faded_fg, final_fg], &[10]);

            let mut beam_row = SceneSpec::default();
            beam_row.apply_gradient_to_symbols(&['▂', '▁', '_'], 2, &beam_gradient);
            beam_row.apply_gradient_to_symbols(&[character.input_symbol], 2, &fg_fade_gradient);

            let mut beam_column = SceneSpec::default();
            beam_column.apply_gradient_to_symbols(&['▌', '▍', '▎', '▏'], 2, &beam_gradient);
            beam_column.apply_gradient_to_symbols(&[character.input_symbol], 2, &fg_fade_gradient);

            let mut brighten = SceneSpec::default();
            brighten.apply_gradient_to_symbols(&[character.input_symbol], 4, &fg_brighten_gradient);

            character.scenes.beam_row = Some(beam_row);
            character.scenes.beam_column = Some(beam_column);
            character.scenes.brighten = Some(brighten);
        }

        let mut groups = Vec::new();
        for row in terminal.grouped(CharacterGroup::RowTopToBottom, true) {
            groups.push(BeamGroup::new(row, BeamDirection::Row, &terminal, &mut rng));
        }
        for column in terminal.grouped(CharacterGroup::ColumnLeftToRight, true) {
            groups.push(BeamGroup::new(column, BeamDirection::Column, &terminal, &mut rng));
        }
        groups.shuffle(&mut rng);

        let final_wipe_groups = terminal.grouped(CharacterGroup::DiagonalTopLeftToBottomRight, false);

        Self {
            terminal,
            pending_groups: groups,
            active_groups: Vec::new(),
            active_characters: HashSet::new(),
            delay: 0,
            phase: BeamsPhase::Beams,
            final_wipe_groups,
            final_wipe_cursor: 0,
        }
    }

    fn tick(&mut self) {
        if self.phase == BeamsPhase::Complete && self.active_characters.is_empty() {
            return;
        }

        match self.phase {
            BeamsPhase::Beams => {
                if self.delay == 0 {
                    if !self.pending_groups.is_empty() {
                        let count = rand::thread_rng().gen_range(1..=5);
                        for _ in 0..count {
                            if self.pending_groups.is_empty() {
                                break;
                            }
                            self.active_groups.push(self.pending_groups.remove(0));
                        }
                    }
                    self.delay = 6;
                } else {
                    self.delay -= 1;
                }

                for group in &mut self.active_groups {
                    group.increment_next_character_counter();
                    let count = group.next_character_counter as usize;
                    if count > 1 {
                        for _ in 0..count {
                            if group.complete() {
                                break;
                            }
                            let _ = group.get_next_character(&mut self.terminal, &mut self.active_characters);
                        }
                    }
                }
                self.active_groups.retain(|group| !group.complete());
                if self.pending_groups.is_empty()
                    && self.active_groups.is_empty()
                    && self.active_characters.is_empty()
                {
                    self.phase = BeamsPhase::FinalWipe;
                }
            }
            BeamsPhase::FinalWipe => {
                if self.final_wipe_cursor < self.final_wipe_groups.len() {
                    for _ in 0..3 {
                        if self.final_wipe_cursor >= self.final_wipe_groups.len() {
                            break;
                        }
                        for index in self.final_wipe_groups[self.final_wipe_cursor].clone() {
                            self.terminal.activate_scene(index, SceneKind::Brighten);
                            self.terminal.set_character_visibility(index, true);
                            self.active_characters.insert(index);
                        }
                        self.final_wipe_cursor += 1;
                    }
                } else {
                    self.phase = BeamsPhase::Complete;
                }
            }
            BeamsPhase::Complete => {}
        }

        self.terminal.update_active(&mut self.active_characters);
    }

    fn is_finished(&self) -> bool {
        self.phase == BeamsPhase::Complete && self.active_characters.is_empty()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SweepPhase {
    First,
    Second,
}

struct SweepSim {
    terminal: SimTerminal,
    groups_first_sweep: Vec<Vec<usize>>,
    groups_second_sweep: Vec<Vec<usize>>,
    easer: SequenceEaser,
    active_characters: HashSet<usize>,
    phase: SweepPhase,
    complete: bool,
}

impl SweepSim {
    fn new(width: u16, height: u16, input: &str) -> Self {
        let mut rng = rand::thread_rng();
        let mut terminal = SimTerminal::new(width, height, input);
        let final_gradient = Gradient::new(
            &[
                Rgb::from_hex("8A008A"),
                Rgb::from_hex("00D1FF"),
                Rgb::from_hex("ffffff"),
            ],
            &[8],
        );
        let final_mapping = final_gradient.vertical_mapping(&terminal.canvas);
        let shades_of_gray = [
            Rgb::from_hex("A0A0A0"),
            Rgb::from_hex("808080"),
            Rgb::from_hex("404040"),
            Rgb::from_hex("202020"),
            Rgb::from_hex("101010"),
        ];
        let sweep_symbols = ['█', '▓', '▒', '░'];

        for character in &mut terminal.characters {
            let mut initial_sweep = SceneSpec::default();
            for symbol in sweep_symbols {
                let color = *shades_of_gray.choose(&mut rng).unwrap();
                initial_sweep.add_frame(symbol, 5, ColorPair::fg(color));
            }
            initial_sweep.add_frame(character.input_symbol, 1, ColorPair::fg(Rgb::from_hex("808080")));

            let mut second_sweep = SceneSpec::default();
            for symbol in sweep_symbols {
                let color = *final_gradient.spectrum.choose(&mut rng).unwrap();
                second_sweep.add_frame(symbol, 5, ColorPair::fg(color));
            }
            let final_color = if character.is_fill {
                Rgb::from_hex("000000")
            } else {
                color_for_coord(&final_mapping, character.coord)
            };
            second_sweep.add_frame(character.input_symbol, 1, ColorPair::fg(final_color));

            character.scenes.initial_sweep = Some(initial_sweep);
            character.scenes.second_sweep = Some(second_sweep);
        }

        let groups_first_sweep = terminal.grouped(CharacterGroup::ColumnRightToLeft, true);
        let groups_second_sweep = terminal.grouped(CharacterGroup::ColumnLeftToRight, true);
        let easer = SequenceEaser::new(groups_first_sweep.clone(), 100);

        Self {
            terminal,
            groups_first_sweep,
            groups_second_sweep,
            easer,
            active_characters: HashSet::new(),
            phase: SweepPhase::First,
            complete: false,
        }
    }

    fn tick(&mut self) {
        if self.complete && self.active_characters.is_empty() {
            return;
        }

        self.easer.step();
        let added = self.easer.added.clone();
        for group in added {
            for index in group {
                if self.phase == SweepPhase::First {
                    self.terminal.set_character_visibility(index, true);
                }
                let scene = if self.phase == SweepPhase::First {
                    SceneKind::InitialSweep
                } else {
                    SceneKind::SecondSweep
                };
                self.terminal.activate_scene(index, scene);
                self.active_characters.insert(index);
            }
        }

        if self.easer.is_complete() && self.phase == SweepPhase::First {
            self.easer.set_sequence(self.groups_second_sweep.clone());
            self.easer.reset();
            self.phase = SweepPhase::Second;
        } else if self.easer.is_complete() && self.phase == SweepPhase::Second {
            self.complete = true;
        }

        self.terminal.update_active(&mut self.active_characters);
    }

    fn is_finished(&self) -> bool {
        self.complete && self.active_characters.is_empty()
    }
}

#[derive(Clone, Debug)]
struct SequenceEaser {
    sequence: Vec<Vec<usize>>,
    tracker: EasingTracker,
    added: Vec<Vec<usize>>,
    removed: Vec<Vec<usize>>,
    total: Vec<Vec<usize>>,
}

impl SequenceEaser {
    fn new(sequence: Vec<Vec<usize>>, total_steps: u32) -> Self {
        Self {
            sequence,
            tracker: EasingTracker::new(total_steps),
            added: Vec::new(),
            removed: Vec::new(),
            total: Vec::new(),
        }
    }

    fn set_sequence(&mut self, sequence: Vec<Vec<usize>>) {
        self.sequence = sequence;
    }

    fn step(&mut self) {
        let previous_eased = self.tracker.eased_value;
        let eased = self.tracker.step();
        let seq_len = self.sequence.len();
        if seq_len == 0 {
            self.added.clear();
            self.removed.clear();
            self.total.clear();
            return;
        }

        let length = (eased * seq_len as f64) as usize;
        let previous_length = (previous_eased * seq_len as f64) as usize;

        if length > previous_length {
            self.added = self.sequence[previous_length..length].to_vec();
            self.removed.clear();
        } else if length < previous_length {
            self.added.clear();
            self.removed = self.sequence[length..previous_length].to_vec();
        } else {
            self.added.clear();
            self.removed.clear();
        }
        self.total = self.sequence[..length].to_vec();
    }

    fn reset(&mut self) {
        self.tracker.reset();
        self.added.clear();
        self.removed.clear();
        self.total.clear();
    }

    fn is_complete(&self) -> bool {
        self.tracker.is_complete()
    }
}

#[derive(Clone, Copy, Debug)]
struct EasingTracker {
    total_steps: u32,
    current_step: u32,
    eased_value: f64,
    last_eased_value: f64,
}

impl EasingTracker {
    fn new(total_steps: u32) -> Self {
        Self {
            total_steps: total_steps.max(1),
            current_step: 0,
            eased_value: 0.0,
            last_eased_value: 0.0,
        }
    }

    fn step(&mut self) -> f64 {
        if self.current_step < self.total_steps {
            self.current_step += 1;
            let progress = self.current_step as f64 / self.total_steps as f64;
            self.eased_value = in_out_circ(progress).clamp(0.0, 1.0);
            self.last_eased_value = self.eased_value;
        }
        self.eased_value
    }

    fn reset(&mut self) {
        self.current_step = 0;
        self.eased_value = 0.0;
        self.last_eased_value = 0.0;
    }

    fn is_complete(&self) -> bool {
        self.current_step >= self.total_steps
    }
}

fn in_out_circ(progress_ratio: f64) -> f64 {
    if progress_ratio < 0.5 {
        (1.0 - (1.0 - (2.0 * progress_ratio).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * progress_ratio + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}
