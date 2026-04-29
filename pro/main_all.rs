// main.rs
// Cargo.toml dependencies:
//   ratatui = "0.29"
//   crossterm = "0.28"
//   rand = "0.8"
//
// Esc exits. Left / Right page through all effects. The active effect name is
// drawn in the upper-left corner.

use std::{
    error::Error,
    io::{self, IsTerminal, Read},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::Rect,
    style::{Color as TuiColor, Modifier, Style},
    widgets::Widget,
    Terminal,
};

const FPS: u64 = 60;
const BG: Rgb = Rgb::new(40, 42, 54);
const LABEL_BG: Rgb = Rgb::new(24, 24, 32);
const WHITE: Rgb = Rgb::new(255, 255, 255);
const BLACK: Rgb = Rgb::new(0, 0, 0);

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
    if input.trim().is_empty() { DEMO_TEXT.to_string() } else { input }
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, input: String) -> Result<(), Box<dyn Error>> {
    let size = terminal.size()?;
    let mut app = App::new(size.width.max(1), size.height.max(1), input);
    let tick_rate = Duration::from_nanos(1_000_000_000 / FPS);
    let mut last_tick = Instant::now() - tick_rate;

    loop {
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }
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
            let cells = app.cells();
            let title = app.title();
            terminal.draw(|frame| {
                frame.render_widget(EffectWidget { cells: &cells, title: &title }, frame.area());
            })?;
            last_tick = Instant::now();
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EffectKind {
    Beams,
    BinaryPath,
    Blackhole,
    BouncyBalls,
    Bubbles,
    Burn,
    ColorShift,
    Crumble,
    Decrypt,
    ErrorCorrect,
    Expand,
    Fireworks,
    Highlight,
    LaserEtch,
    Matrix,
    MiddleOut,
    OrbittingVolley,
    Overflow,
    Pour,
    Print,
    Rain,
    RandomSequence,
    Rings,
    Scattered,
    Slice,
    Slide,
    Smoke,
    Spotlights,
    Spray,
    Sweep,
    Swarm,
    SynthGrid,
    Thunderstorm,
    Unstable,
    VhsTape,
    Waves,
    Wipe,
}

const EFFECTS: [EffectKind; 37] = [
    EffectKind::Beams,
    EffectKind::BinaryPath,
    EffectKind::Blackhole,
    EffectKind::BouncyBalls,
    EffectKind::Bubbles,
    EffectKind::Burn,
    EffectKind::ColorShift,
    EffectKind::Crumble,
    EffectKind::Decrypt,
    EffectKind::ErrorCorrect,
    EffectKind::Expand,
    EffectKind::Fireworks,
    EffectKind::Highlight,
    EffectKind::LaserEtch,
    EffectKind::Matrix,
    EffectKind::MiddleOut,
    EffectKind::OrbittingVolley,
    EffectKind::Overflow,
    EffectKind::Pour,
    EffectKind::Print,
    EffectKind::Rain,
    EffectKind::RandomSequence,
    EffectKind::Rings,
    EffectKind::Scattered,
    EffectKind::Slice,
    EffectKind::Slide,
    EffectKind::Smoke,
    EffectKind::Spotlights,
    EffectKind::Spray,
    EffectKind::Sweep,
    EffectKind::Swarm,
    EffectKind::SynthGrid,
    EffectKind::Thunderstorm,
    EffectKind::Unstable,
    EffectKind::VhsTape,
    EffectKind::Waves,
    EffectKind::Wipe,
];

impl EffectKind {
    fn index(self) -> usize { EFFECTS.iter().position(|k| *k == self).unwrap_or(0) }
    fn next(self) -> Self { EFFECTS[(self.index() + 1) % EFFECTS.len()] }
    fn previous(self) -> Self { EFFECTS[(self.index() + EFFECTS.len() - 1) % EFFECTS.len()] }
    fn cycle(self) -> u32 {
        match self {
            Self::Matrix | Self::Thunderstorm | Self::VhsTape | Self::Spotlights | Self::ColorShift => 900,
            Self::Rings | Self::Blackhole | Self::Unstable | Self::Crumble => 760,
            Self::Print | Self::Overflow | Self::SynthGrid => 720,
            _ => 600,
        }
    }
    fn name(self) -> &'static str {
        match self {
            Self::Beams => "beams",
            Self::BinaryPath => "binarypath",
            Self::Blackhole => "blackhole",
            Self::BouncyBalls => "bouncyballs",
            Self::Bubbles => "bubbles",
            Self::Burn => "burn",
            Self::ColorShift => "colorshift",
            Self::Crumble => "crumble",
            Self::Decrypt => "decrypt",
            Self::ErrorCorrect => "errorcorrect",
            Self::Expand => "expand",
            Self::Fireworks => "fireworks",
            Self::Highlight => "highlight",
            Self::LaserEtch => "laseretch",
            Self::Matrix => "matrix",
            Self::MiddleOut => "middleout",
            Self::OrbittingVolley => "orbittingvolley",
            Self::Overflow => "overflow",
            Self::Pour => "pour",
            Self::Print => "print",
            Self::Rain => "rain",
            Self::RandomSequence => "randomsequence",
            Self::Rings => "rings",
            Self::Scattered => "scattered",
            Self::Slice => "slice",
            Self::Slide => "slide",
            Self::Smoke => "smoke",
            Self::Spotlights => "spotlights",
            Self::Spray => "spray",
            Self::Sweep => "sweep",
            Self::Swarm => "swarm",
            Self::SynthGrid => "synthgrid",
            Self::Thunderstorm => "thunderstorm",
            Self::Unstable => "unstable",
            Self::VhsTape => "vhstape",
            Self::Waves => "waves",
            Self::Wipe => "wipe",
        }
    }
}

struct App {
    kind: EffectKind,
    tick: u32,
    width: u16,
    height: u16,
    input: String,
    glyphs: Vec<Glyph>,
}

impl App {
    fn new(width: u16, height: u16, input: String) -> Self {
        let glyphs = layout_text(width, height, &input);
        Self { kind: EffectKind::Beams, tick: 0, width, height, input, glyphs }
    }
    fn resize(&mut self, width: u16, height: u16) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.glyphs = layout_text(width, height, &self.input);
            self.tick = 0;
        }
    }
    fn next(&mut self) { self.kind = self.kind.next(); self.tick = 0; }
    fn previous(&mut self) { self.kind = self.kind.previous(); self.tick = 0; }
    fn tick(&mut self) { self.tick = self.tick.wrapping_add(1); }
    fn title(&self) -> String { format!(" {}  {}/{} ", self.kind.name(), self.kind.index() + 1, EFFECTS.len()) }
    fn cells(&self) -> Vec<DrawCell> { self.render_effect() }

    fn render_effect(&self) -> Vec<DrawCell> {
        match self.kind {
            EffectKind::Beams => self.effect_beams(),
            EffectKind::BinaryPath => self.effect_binarypath(),
            EffectKind::Blackhole => self.effect_blackhole(),
            EffectKind::BouncyBalls => self.effect_bouncyballs(),
            EffectKind::Bubbles => self.effect_bubbles(),
            EffectKind::Burn => self.effect_burn(),
            EffectKind::ColorShift => self.effect_colorshift(),
            EffectKind::Crumble => self.effect_crumble(),
            EffectKind::Decrypt => self.effect_decrypt(),
            EffectKind::ErrorCorrect => self.effect_errorcorrect(),
            EffectKind::Expand => self.effect_expand(),
            EffectKind::Fireworks => self.effect_fireworks(),
            EffectKind::Highlight => self.effect_highlight(),
            EffectKind::LaserEtch => self.effect_laseretch(),
            EffectKind::Matrix => self.effect_matrix(),
            EffectKind::MiddleOut => self.effect_middleout(),
            EffectKind::OrbittingVolley => self.effect_orbittingvolley(),
            EffectKind::Overflow => self.effect_overflow(),
            EffectKind::Pour => self.effect_pour(),
            EffectKind::Print => self.effect_print(),
            EffectKind::Rain => self.effect_rain(),
            EffectKind::RandomSequence => self.effect_randomsequence(),
            EffectKind::Rings => self.effect_rings(),
            EffectKind::Scattered => self.effect_scattered(),
            EffectKind::Slice => self.effect_slice(),
            EffectKind::Slide => self.effect_slide(),
            EffectKind::Smoke => self.effect_smoke(),
            EffectKind::Spotlights => self.effect_spotlights(),
            EffectKind::Spray => self.effect_spray(),
            EffectKind::Sweep => self.effect_sweep(),
            EffectKind::Swarm => self.effect_swarm(),
            EffectKind::SynthGrid => self.effect_synthgrid(),
            EffectKind::Thunderstorm => self.effect_thunderstorm(),
            EffectKind::Unstable => self.effect_unstable(),
            EffectKind::VhsTape => self.effect_vhstape(),
            EffectKind::Waves => self.effect_waves(),
            EffectKind::Wipe => self.effect_wipe(),
        }
    }

    fn t(&self) -> f64 { (self.tick % self.kind.cycle()) as f64 }
    fn cycle(&self) -> f64 { self.kind.cycle() as f64 }
    fn final_color(&self, g: &Glyph) -> Rgb { final_color_for(self.kind, g, self.width, self.height) }

    fn push_final(&self, cells: &mut Vec<DrawCell>, g: &Glyph, layer: i32) {
        cells.push(DrawCell::new(g.x, g.y, g.ch, self.final_color(g), layer));
    }

    fn reveal_progress(&self, g: &Glyph, total_span: f64, duration: f64, salt: u64) -> f64 {
        let start = hash01(g.index as u64, salt) * total_span;
        ((self.t() - start) / duration).clamp(0.0, 1.0)
    }

    fn ordered_progress(&self, g: &Glyph, duration: f64, salt: u64) -> f64 {
        let rank = hash01(g.index as u64, salt);
        let start = rank * (self.cycle() - duration).max(1.0);
        ((self.t() - start) / duration).clamp(0.0, 1.0)
    }

    fn effect_wipe(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let max_diag = (self.width + self.height) as f64;
        let sweep = self.t() / 420.0 * max_diag;
        for g in &self.glyphs {
            let diag = (g.x + (self.height as i32 - g.y)) as f64;
            let p = (sweep - diag).clamp(0.0, 12.0) / 12.0;
            if p > 0.0 {
                let ch = shimmer(['█', '▓', '▒', '░', g.ch], p, self.tick, g.index);
                let c = gradient(&[Rgb::from_hex("833ab4"), Rgb::from_hex("fd1d1d"), Rgb::from_hex("fcb045"), self.final_color(g)], p);
                cells.push(DrawCell::new(g.x, g.y, ch, c, 1));
            }
        }
        cells
    }

    fn effect_randomsequence(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        for g in &self.glyphs {
            let p = self.ordered_progress(g, 70.0, 11);
            if p > 0.0 {
                let ch = shimmer(['█', '▓', '▒', '░', g.ch], p, self.tick, g.index);
                cells.push(DrawCell::new(g.x, g.y, ch, gradient(&[BLACK, self.final_color(g)], p), 1));
            }
        }
        cells
    }

    fn effect_highlight(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let band = self.t() / 500.0 * ((self.width + self.height) as f64 + 20.0) - 10.0;
        for g in &self.glyphs {
            let base = self.final_color(g);
            let d = ((g.x + (self.height as i32 - g.y)) as f64 - band).abs();
            let strength = (1.0 - d / 8.0).clamp(0.0, 1.0);
            let c = brighten(base, 1.0 + strength * 0.95);
            cells.push(DrawCell::new(g.x, g.y, g.ch, c, 1));
        }
        cells
    }

    fn effect_colorshift(&self) -> Vec<DrawCell> {
        let stops = hexes(&["e81416", "ffa500", "faeb36", "79c314", "487de7", "4b369d", "70369d", "e81416"]);
        self.glyphs.iter().map(|g| {
            let f = ((self.t() / 180.0) + g.x as f64 / self.width.max(1) as f64 + g.y as f64 / self.height.max(1) as f64) % 1.0;
            DrawCell::new(g.x, g.y, g.ch, gradient(&stops, f), 1)
        }).collect()
    }

    fn effect_expand(&self) -> Vec<DrawCell> {
        self.move_from(|app, _g| (app.width as f64 / 2.0, app.height as f64 / 2.0), 0.0, 420.0, WHITE, EaseKind::InOutQuart)
    }

    fn effect_scattered(&self) -> Vec<DrawCell> {
        self.move_from(|app, g| {
            let rx = hash01(g.index as u64, 21) * app.width as f64;
            let ry = hash01(g.index as u64, 22) * app.height as f64;
            (rx, ry)
        }, 25.0, 430.0, Rgb::from_hex("ff9048"), EaseKind::InOutBack)
    }

    fn effect_spray(&self) -> Vec<DrawCell> {
        self.move_from(|app, _| (app.width as f64 - 1.0, app.height as f64 / 2.0), 520.0, 90.0, WHITE, EaseKind::OutExpo)
    }

    fn effect_rain(&self) -> Vec<DrawCell> {
        self.move_from(|_app, g| (g.x as f64, -8.0 - hash01(g.index as u64, 31) * 25.0), 360.0, 150.0, Rgb::from_hex("78B9F2"), EaseKind::InQuart)
    }

    fn effect_bouncyballs(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        for g in &self.glyphs {
            let start = hash01(g.index as u64, 33) * 360.0;
            let p = ((self.t() - start) / 190.0).clamp(0.0, 1.0);
            if p <= 0.0 { continue; }
            let e = ease_out_bounce(p);
            let y0 = -10.0 - hash01(g.index as u64, 34) * 20.0;
            let y = lerp(y0, g.y as f64, e);
            let x = g.x as f64;
            let ball_symbols = ['*', 'o', 'O', '0', '.'];
            let ch = if p < 0.85 { ball_symbols[(g.index + (self.tick as usize / 5)) % ball_symbols.len()] } else { g.ch };
            let color = gradient(&[Rgb::from_hex("d1f4a5"), self.final_color(g)], p);
            cells.push(DrawCell::new(x.round() as i32, y.round() as i32, ch, color, 1));
        }
        cells
    }

    fn effect_pour(&self) -> Vec<DrawCell> {
        self.move_from(|_app, g| (g.x as f64, -1.0), 420.0, 90.0, WHITE, EaseKind::InQuad)
    }

    fn effect_slide(&self) -> Vec<DrawCell> {
        self.move_from(|app, g| {
            if g.y % 2 == 0 { (-8.0, g.y as f64) } else { (app.width as f64 + 8.0, g.y as f64) }
        }, 460.0, 110.0, Rgb::from_hex("833ab4"), EaseKind::InOutQuad)
    }

    fn effect_slice(&self) -> Vec<DrawCell> {
        let mid = self.width as i32 / 2;
        self.move_from(|app, g| {
            if g.x <= mid { (g.x as f64, -4.0) } else { (g.x as f64, app.height as f64 + 4.0) }
        }, 0.0, 360.0, WHITE, EaseKind::InOutExpo)
    }

    fn effect_middleout(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let center_x = self.width as f64 / 2.0;
        let center_y = self.height as f64 / 2.0;
        for g in &self.glyphs {
            let p1 = (self.t() / 220.0).clamp(0.0, 1.0);
            let p2 = ((self.t() - 220.0) / 260.0).clamp(0.0, 1.0);
            let mid = (g.x as f64, center_y);
            let (x, y) = if p2 <= 0.0 {
                (lerp(center_x, mid.0, ease_in_out_sine(p1)), lerp(center_y, mid.1, ease_in_out_sine(p1)))
            } else {
                (lerp(mid.0, g.x as f64, ease_in_out_sine(p2)), lerp(mid.1, g.y as f64, ease_in_out_sine(p2)))
            };
            let p = p1.max(p2);
            cells.push(DrawCell::new(x.round() as i32, y.round() as i32, g.ch, gradient(&[WHITE, self.final_color(g)], p), 1));
        }
        cells
    }

    fn effect_binarypath(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        for g in &self.glyphs {
            let start = hash01(g.index as u64, 41) * 420.0;
            let p = ((self.t() - start) / 160.0).clamp(0.0, 1.0);
            if p <= 0.0 { continue; }
            let outside = if hash01(g.index as u64, 42) < 0.5 { (-4.0, g.y as f64) } else { (g.x as f64, -4.0) };
            let mid = if hash01(g.index as u64, 43) < 0.5 { (outside.0, g.y as f64) } else { (g.x as f64, outside.1) };
            let (x, y) = polyline(&[outside, mid, (g.x as f64, g.y as f64)], p);
            let ch = if p < 0.88 { if ((self.tick / 4 + g.index as u32) % 2) == 0 { '0' } else { '1' } } else { g.ch };
            let c = gradient(&[Rgb::from_hex("044E29"), Rgb::from_hex("45bf55"), self.final_color(g)], p);
            cells.push(DrawCell::new(x.round() as i32, y.round() as i32, ch, c, 1));
        }
        cells
    }

    fn effect_errorcorrect(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let n = self.glyphs.len().max(1);
        for (i, g) in self.glyphs.iter().enumerate() {
            let pair_i = (i * 17 + 3) % n;
            let other = &self.glyphs[pair_i];
            let start = hash01(i as u64, 51) * 420.0;
            let p = ((self.t() - start) / 80.0).clamp(0.0, 1.0);
            let wrong = hash01(i as u64, 52) < 0.18;
            let (x0, y0) = if wrong { (other.x as f64, other.y as f64) } else { (g.x as f64, g.y as f64) };
            let x = lerp(x0, g.x as f64, p);
            let y = lerp(y0, g.y as f64, p);
            let color = if wrong && p < 0.99 { gradient(&[Rgb::from_hex("e74c3c"), Rgb::from_hex("45bf55")], p) } else { self.final_color(g) };
            cells.push(DrawCell::new(x.round() as i32, y.round() as i32, g.ch, color, 1));
        }
        cells
    }

    fn effect_decrypt(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        for g in &self.glyphs {
            let start = (g.index as f64 / self.glyphs.len().max(1) as f64) * 220.0;
            let p = ((self.t() - start) / 320.0).clamp(0.0, 1.0);
            if p <= 0.0 { continue; }
            let ch = if p < 0.75 { random_cipher(g.index as u64, self.tick / (if p < 0.45 { 2 } else { 8 })) } else { g.ch };
            let color = if p < 0.75 { [Rgb::from_hex("008000"), Rgb::from_hex("00cb00"), Rgb::from_hex("00ff00")][g.index % 3] } else { gradient(&[WHITE, self.final_color(g)], (p - 0.75) / 0.25) };
            cells.push(DrawCell::new(g.x, g.y, ch, color, 1));
        }
        cells
    }

    fn effect_waves(&self) -> Vec<DrawCell> {
        let symbols = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█', '▇', '▆', '▅', '▄', '▃', '▂', '▁'];
        let mut cells = Vec::new();
        let sweep = self.t() / 450.0 * (self.width as f64 + 20.0) - 10.0;
        for g in &self.glyphs {
            let p = ((sweep - g.x as f64) / 12.0).clamp(0.0, 1.0);
            if p <= 0.0 { continue; }
            let wave = ((self.tick as usize / 2) + g.y as usize + g.index) % symbols.len();
            let ch = if p < 0.9 { symbols[wave] } else { g.ch };
            let c = gradient(&[Rgb::from_hex("f0ff65"), Rgb::from_hex("ffb102"), Rgb::from_hex("31a0d4"), self.final_color(g)], p);
            cells.push(DrawCell::new(g.x, g.y, ch, c, 1));
        }
        cells
    }

    fn effect_sweep(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let w = self.width as f64;
        let t = self.t();
        for g in &self.glyphs {
            if t < 260.0 {
                let sweep = w - (t / 260.0) * (w + 10.0);
                let d = (g.x as f64 - sweep).abs();
                if (g.x as f64) >= sweep || d < 4.0 {
                    let p = (1.0 - d / 6.0).clamp(0.0, 1.0);
                    let ch = shimmer(['█', '▓', '▒', '░', g.ch], p, self.tick, g.index);
                    let c = gradient(&[Rgb::from_hex("A0A0A0"), Rgb::from_hex("101010")], 1.0 - p);
                    cells.push(DrawCell::new(g.x, g.y, ch, c, 1));
                }
            } else {
                cells.push(DrawCell::new(g.x, g.y, g.ch, Rgb::from_hex("808080"), 0));
                let sweep = ((t - 260.0) / 260.0) * (w + 10.0) - 5.0;
                let d = (g.x as f64 - sweep).abs();
                if (g.x as f64) <= sweep || d < 4.0 {
                    let p = (1.0 - d / 6.0).clamp(0.0, 1.0);
                    let ch = shimmer(['█', '▓', '▒', '░', g.ch], p, self.tick, g.index);
                    let c = gradient(&[Rgb::from_hex("8A008A"), Rgb::from_hex("00D1FF"), WHITE, self.final_color(g)], p);
                    cells.push(DrawCell::new(g.x, g.y, ch, c, 2));
                }
            }
        }
        cells
    }

    fn effect_beams(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let t = self.t();
        for g in &self.glyphs {
            let row_start = hash01(g.y as u64, 101) * 250.0;
            let col_start = hash01(g.x as u64, 102) * 250.0;
            let row_pos = ((t - row_start) * 0.65) as i32;
            let col_pos = ((t - col_start) * 0.45) as i32;
            let row_hit = (g.x - row_pos).abs() <= 2;
            let col_hit = (g.y - col_pos).abs() <= 2;
            let revealed = t > row_start + g.x as f64 * 1.4 || t > col_start + g.y as f64 * 2.1 || t > 390.0;
            if revealed {
                let mut c = self.final_color(g);
                if row_hit || col_hit { c = gradient(&[WHITE, Rgb::from_hex("00D1FF"), Rgb::from_hex("8A008A")], hash01(g.index as u64, self.tick as u64)); }
                cells.push(DrawCell::new(g.x, g.y, g.ch, c, 1));
            }
            if row_hit {
                let symbols = ['▂', '▁', '_'];
                cells.push(DrawCell::new(g.x, g.y, symbols[(self.tick as usize / 2 + g.index) % 3], Rgb::from_hex("00D1FF"), 3));
            }
            if col_hit {
                let symbols = ['▌', '▍', '▎', '▏'];
                cells.push(DrawCell::new(g.x, g.y, symbols[(self.tick as usize / 2 + g.index) % 4], WHITE, 4));
            }
        }
        cells
    }

    fn effect_matrix(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let rain_symbols: Vec<char> = "2598Z*):.\"=+-¦|_ｦｱｳｴｵｶｷｹｺｻｼｽｾｿﾀﾂﾃﾅﾆﾇﾈﾊﾋﾎﾏﾐﾑﾒﾓﾔﾕﾗﾘﾜ".chars().collect();
        for col in 0..self.width as i32 {
            let len = 3 + (hash01(col as u64, 61) * self.height as f64 * 0.9) as i32;
            let speed = 0.25 + hash01(col as u64, 62) * 0.9;
            let head = ((self.t() * speed + hash01(col as u64, 63) * self.height as f64 * 2.0) as i32) % (self.height as i32 + len + 1) - len;
            for j in 0..len {
                let y = head + j;
                if y >= 0 && y < self.height as i32 {
                    let bright = j == len - 1;
                    let f = j as f64 / len.max(1) as f64;
                    let c = if bright { Rgb::from_hex("dbffdb") } else { gradient(&[Rgb::from_hex("92be92"), Rgb::from_hex("185318")], f) };
                    let ch = rain_symbols[((col as usize * 17 + y as usize * 7 + self.tick as usize / 4) % rain_symbols.len())];
                    cells.push(DrawCell::new(col, y, ch, c, 0));
                }
            }
        }
        let reveal = ((self.t() - 520.0) / 260.0).clamp(0.0, 1.0) * self.width as f64;
        for g in &self.glyphs {
            if (g.x as f64) <= reveal {
                cells.push(DrawCell::new(g.x, g.y, g.ch, self.final_color(g), 2));
            }
        }
        cells
    }

    fn effect_fireworks(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let colors = hexes(&["88F7E2", "44D492", "F5EB67", "FFA15C", "FA233E"]);
        for g in &self.glyphs {
            let start = hash01(g.index as u64, 71) * 360.0;
            let p = ((self.t() - start) / 220.0).clamp(0.0, 1.0);
            if p <= 0.0 { continue; }
            let shell = colors[g.index % colors.len()];
            let apex = (hash01(g.index as u64, 72) * self.width as f64, hash01(g.index as u64, 73) * (self.height as f64 * 0.5));
            let bloom = (apex.0 + (hash01(g.index as u64, 74) - 0.5) * 24.0, apex.1 + (hash01(g.index as u64, 75) - 0.5) * 12.0);
            let (x, y) = polyline(&[(g.x as f64, self.height as f64 + 2.0), apex, bloom, (g.x as f64, g.y as f64)], p);
            let ch = if p < 0.25 { 'o' } else { g.ch };
            cells.push(DrawCell::new(x.round() as i32, y.round() as i32, ch, gradient(&[shell, WHITE, shell, self.final_color(g)], p), 1));
        }
        cells
    }

    fn effect_blackhole(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let center = (self.width as f64 / 2.0, self.height as f64 / 2.0);
        let t = self.t();
        for g in &self.glyphs {
            let star = (hash01(g.index as u64, 81) * self.width as f64, hash01(g.index as u64, 82) * self.height as f64);
            let p = if t < 280.0 { t / 280.0 } else if t < 430.0 { 1.0 } else { ((t - 430.0) / 240.0).clamp(0.0, 1.0) };
            let (x, y, ch, c) = if t < 280.0 {
                let e = EaseKind::InExpo.apply(p);
                let (x, y) = (lerp(star.0, center.0, e), lerp(star.1, center.1, e));
                let syms = ['*', '\'', '`', '¤', '•', '°', '·'];
                (x, y, syms[g.index % syms.len()], gradient(&[Rgb::from_hex("4a4a4d"), WHITE, BLACK], p))
            } else if t < 430.0 {
                let ang = g.index as f64 * 0.4 + t / 12.0;
                let r = 4.0 + (g.index % 8) as f64 * 0.4;
                (center.0 + ang.cos() * r * 2.0, center.1 + ang.sin() * r, '*', WHITE)
            } else {
                let e = EaseKind::OutExpo.apply(p);
                (lerp(center.0, g.x as f64, e), lerp(center.1, g.y as f64, e), g.ch, gradient(&[Rgb::from_hex("ff7326"), self.final_color(g)], p))
            };
            cells.push(DrawCell::new(x.round() as i32, y.round() as i32, ch, c, 1));
        }
        cells
    }

    fn effect_rings(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let center = (self.width as f64 / 2.0, self.height as f64 / 2.0);
        let t = self.t();
        for g in &self.glyphs {
            let radius = 2.0 + (g.index % 14) as f64 * 0.9;
            let angle = g.index as f64 * 0.55 + t / (16.0 + (g.index % 9) as f64);
            let ring = (center.0 + angle.cos() * radius * 2.0, center.1 + angle.sin() * radius);
            let p = if t < 180.0 { t / 180.0 } else if t < 520.0 { 1.0 } else { 1.0 - ((t - 520.0) / 180.0).clamp(0.0, 1.0) };
            let (x, y) = if t < 180.0 {
                let start = (hash01(g.index as u64, 91) * self.width as f64, hash01(g.index as u64, 92) * self.height as f64);
                (lerp(start.0, ring.0, p), lerp(start.1, ring.1, p))
            } else if t < 520.0 {
                ring
            } else {
                let q = ((t - 520.0) / 180.0).clamp(0.0, 1.0);
                (lerp(ring.0, g.x as f64, q), lerp(ring.1, g.y as f64, q))
            };
            cells.push(DrawCell::new(x.round() as i32, y.round() as i32, g.ch, gradient(&[Rgb::from_hex("ab48ff"), Rgb::from_hex("e7b2b2"), Rgb::from_hex("fffebd"), self.final_color(g)], 1.0 - p), 1));
        }
        cells
    }

    fn effect_bubbles(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        for g in &self.glyphs {
            let group = g.index / 12;
            let start = group as f64 * 24.0;
            let p = ((self.t() - start) / 210.0).clamp(0.0, 1.0);
            if p <= 0.0 { continue; }
            let anchor_x = hash01(group as u64, 101) * self.width as f64;
            let x = lerp(anchor_x + (hash01(g.index as u64, 102) - 0.5) * 12.0, g.x as f64, EaseKind::InOutSine.apply(p));
            let y = lerp(-10.0 - group as f64, g.y as f64, EaseKind::InOutSine.apply(p));
            let ch = if p < 0.75 { g.ch } else if p < 0.9 { '*' } else { g.ch };
            let c = gradient(&[Rgb::from_hex("d33aff"), Rgb::from_hex("02ff7f"), WHITE, self.final_color(g)], p);
            cells.push(DrawCell::new(x.round() as i32, y.round() as i32, ch, c, 1));
        }
        cells
    }

    fn effect_burn(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let fire = hexes(&["ffffff", "fff75d", "fe650d", "8A003C", "510100"]);
        for g in &self.glyphs {
            let start = (self.height as i32 - g.y + g.x) as f64 * 4.0;
            let p = ((self.t() - start) / 120.0).clamp(0.0, 1.0);
            if p <= 0.0 {
                cells.push(DrawCell::new(g.x, g.y, g.ch, Rgb::from_hex("837373"), 1));
            } else if p < 0.75 {
                let syms = ['\'', '.', '▖', '▙', '█', '▜', '▀', '▝', '.'];
                cells.push(DrawCell::new(g.x, g.y, shimmer(syms, p / 0.75, self.tick, g.index), gradient(&fire, p / 0.75), 1));
                if hash01(g.index as u64, self.tick as u64 / 5) < 0.35 {
                    cells.push(DrawCell::new(g.x + ((hash01(g.index as u64, 111) - 0.5) * 6.0) as i32, g.y - (p * 8.0) as i32, '.', Rgb::from_hex("888888"), 0));
                }
            } else {
                cells.push(DrawCell::new(g.x, g.y, g.ch, gradient(&[*fire.last().unwrap(), self.final_color(g)], (p - 0.75) / 0.25), 1));
            }
        }
        cells
    }

    fn effect_crumble(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let t = self.t();
        for g in &self.glyphs {
            let start = hash01(g.index as u64, 121) * 180.0;
            let p = ((t - start) / 130.0).clamp(0.0, 1.0);
            if t < 320.0 {
                let y = lerp(g.y as f64, self.height as f64 - 1.0, ease_out_bounce(p));
                let ch = if p < 0.5 { g.ch } else { ['*', '.', ','][(g.index + self.tick as usize / 3) % 3] };
                cells.push(DrawCell::new(g.x, y.round() as i32, ch, brighten(self.final_color(g), 0.55), 1));
            } else if t < 470.0 {
                let p2 = ((t - 320.0) / 150.0).clamp(0.0, 1.0);
                let y = lerp(self.height as f64 - 1.0, -2.0, p2);
                cells.push(DrawCell::new(g.x, y.round() as i32, '.', Rgb::from_hex("c7c7c7"), 1));
            } else {
                let p3 = ((t - 470.0) / 180.0).clamp(0.0, 1.0);
                let y = lerp(-2.0, g.y as f64, EaseKind::OutExpo.apply(p3));
                cells.push(DrawCell::new(g.x, y.round() as i32, g.ch, gradient(&[WHITE, self.final_color(g)], p3), 1));
            }
        }
        cells
    }

    fn effect_smoke(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let center = (self.width as f64 / 2.0, self.height as f64 / 2.0);
        let radius = self.t() / 430.0 * ((self.width + self.height) as f64 / 1.4);
        for g in &self.glyphs {
            let d = distance((g.x as f64, g.y as f64), center);
            let p = ((radius - d) / 12.0).clamp(0.0, 1.0);
            if p <= 0.0 {
                cells.push(DrawCell::new(g.x, g.y, g.ch, Rgb::from_hex("7A7A7A"), 0));
            } else if p < 0.75 {
                let syms = ['░', '▒', '▓', '▒', '░'];
                cells.push(DrawCell::new(g.x, g.y, syms[(g.index + self.tick as usize / 3) % syms.len()], gradient(&[Rgb::from_hex("242424"), WHITE, Rgb::from_hex("8A008A"), Rgb::from_hex("00D1FF")], p), 1));
            } else {
                cells.push(DrawCell::new(g.x, g.y, g.ch, gradient(&[Rgb::from_hex("00D1FF"), self.final_color(g)], (p - 0.75) / 0.25), 1));
            }
        }
        cells
    }

    fn effect_laseretch(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let ordered: Vec<&Glyph> = {
            let mut v: Vec<&Glyph> = self.glyphs.iter().collect();
            v.sort_by_key(|g| g.x - g.y);
            v
        };
        let idx = ((self.t() / 430.0) * ordered.len() as f64) as usize;
        for (n, g) in ordered.iter().enumerate() {
            if n < idx {
                cells.push(DrawCell::new(g.x, g.y, g.ch, self.final_color(g), 1));
            } else if n == idx {
                cells.push(DrawCell::new(g.x, g.y, '^', Rgb::from_hex("ffe680"), 2));
            }
        }
        if let Some(target) = ordered.get(idx.min(ordered.len().saturating_sub(1))) {
            let mut x = target.x;
            let mut y = target.y;
            while x < self.width as i32 && y >= 0 {
                cells.push(DrawCell::new(x, y, '/', Rgb::from_hex("376cff"), 3));
                x += 1;
                y -= 1;
            }
        }
        cells
    }

    fn effect_spotlights(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let t = self.t() / 24.0;
        let spots = [
            ((t.sin() + 1.0) * 0.5 * self.width as f64, (t.cos() + 1.0) * 0.5 * self.height as f64),
            (((t * 0.7).cos() + 1.0) * 0.5 * self.width as f64, (((t * 1.3).sin() + 1.0) * 0.5 * self.height as f64)),
            (self.width as f64 / 2.0, self.height as f64 / 2.0),
        ];
        let radius = self.width.min(self.height).max(2) as f64 / 2.0;
        for g in &self.glyphs {
            let d = spots.iter().map(|s| distance((g.x as f64, g.y as f64), *s)).fold(9999.0, f64::min);
            let bright_factor = if self.t() > 560.0 { 1.0 } else { (1.0 - d / radius).clamp(0.2, 1.0) };
            cells.push(DrawCell::new(g.x, g.y, g.ch, brighten(self.final_color(g), bright_factor), 1));
        }
        if self.t() < 560.0 {
            for s in spots { cells.push(DrawCell::new(s.0.round() as i32, s.1.round() as i32, 'O', Rgb::from_hex("fffebd"), 3)); }
        }
        cells
    }

    fn effect_synthgrid(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let grid_p = (self.t() / 130.0).clamp(0.0, 1.0);
        let row_gap = (self.height as i32 / 5).max(3);
        let col_gap = (self.width as i32 / 5).max(6);
        for row in (0..self.height as i32).step_by(row_gap as usize) {
            let max_x = (self.width as f64 * grid_p) as i32;
            for x in 0..max_x { cells.push(DrawCell::new(x, row, '─', Rgb::from_hex("CC00CC"), 1)); }
        }
        for col in (0..self.width as i32).step_by(col_gap as usize) {
            let max_y = (self.height as f64 * grid_p) as i32;
            for y in 0..max_y { cells.push(DrawCell::new(col, y, '│', Rgb::from_hex("ffffff"), 1)); }
        }
        let reveal = ((self.t() - 130.0) / 360.0).clamp(0.0, 1.0);
        for g in &self.glyphs {
            let block = ((g.x / col_gap) + (g.y / row_gap)) as f64;
            let p = (reveal * 10.0 - block).clamp(0.0, 1.0);
            if p > 0.0 {
                let ch = if p < 0.85 { ['░', '▒', '▓'][(g.index + self.tick as usize / 2) % 3] } else { g.ch };
                cells.push(DrawCell::new(g.x, g.y, ch, gradient(&[Rgb::from_hex("8A008A"), Rgb::from_hex("00D1FF"), WHITE, self.final_color(g)], p), 2));
            }
        }
        cells
    }

    fn effect_thunderstorm(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        for g in &self.glyphs {
            let base = if self.t() < 700.0 { brighten(self.final_color(g), 0.5) } else { self.final_color(g) };
            cells.push(DrawCell::new(g.x, g.y, g.ch, base, 1));
        }
        for n in 0..80 {
            let x0 = hash01(n, 141) * self.width as f64 - self.height as f64;
            let y = ((self.t() * (0.7 + hash01(n, 142)) + hash01(n, 143) * self.height as f64) as i32) % (self.height as i32 + 4) - 2;
            let x = x0 as i32 + y;
            cells.push(DrawCell::new(x, y, ['\\', '.', ','][n as usize % 3], Rgb::from_hex("aaaaff"), 0));
        }
        if ((self.t() as u32 / 90) % 4) == 1 && self.t() < 680.0 {
            let col = (hash01((self.tick / 90) as u64, 144) * self.width as f64) as i32;
            let mut x = col;
            for y in 0..self.height as i32 {
                let ch = if hash01(y as u64, self.tick as u64 / 90) < 0.33 { '/' } else if hash01(y as u64, 9) < 0.66 { '\\' } else { '|' };
                cells.push(DrawCell::new(x.clamp(0, self.width as i32 - 1), y, ch, Rgb::from_hex("68A3E8"), 4));
                x += if ch == '\\' { 1 } else if ch == '/' { -1 } else { 0 };
            }
        }
        cells
    }

    fn effect_unstable(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let t = self.t();
        for g in &self.glyphs {
            let j = (self.glyphs[((g.index * 17 + 5) % self.glyphs.len().max(1))].x as f64, self.glyphs[((g.index * 17 + 5) % self.glyphs.len().max(1))].y as f64);
            let edge = if hash01(g.index as u64, 151) < 0.5 { (hash01(g.index as u64, 152) * self.width as f64, if hash01(g.index as u64, 153) < 0.5 { -2.0 } else { self.height as f64 + 2.0 }) } else { (if hash01(g.index as u64, 154) < 0.5 { -2.0 } else { self.width as f64 + 2.0 }, hash01(g.index as u64, 155) * self.height as f64) };
            let (x, y, p) = if t < 160.0 {
                let shake = if t > 30.0 { ((self.tick as i32 + g.index as i32) % 3 - 1) as f64 } else { 0.0 };
                (j.0 + shake, j.1 - shake, 0.0)
            } else if t < 360.0 {
                let p = ((t - 160.0) / 200.0).clamp(0.0, 1.0);
                (lerp(j.0, edge.0, EaseKind::OutExpo.apply(p)), lerp(j.1, edge.1, EaseKind::OutExpo.apply(p)), p)
            } else {
                let p = ((t - 360.0) / 220.0).clamp(0.0, 1.0);
                (lerp(edge.0, g.x as f64, EaseKind::OutExpo.apply(p)), lerp(edge.1, g.y as f64, EaseKind::OutExpo.apply(p)), p)
            };
            cells.push(DrawCell::new(x.round() as i32, y.round() as i32, g.ch, gradient(&[Rgb::from_hex("ff9200"), self.final_color(g)], p), 1));
        }
        cells
    }

    fn effect_vhstape(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let mut row_offsets = vec![0_i32; self.height as usize];
        if self.t() < 600.0 {
            for row in 0..self.height as usize {
                if hash01(row as u64, self.tick as u64 / 11) < 0.08 {
                    row_offsets[row] = (hash01(row as u64, self.tick as u64 / 3) * 18.0 - 9.0) as i32;
                }
            }
        }
        for g in &self.glyphs {
            let mut ch = g.ch;
            let mut color = self.final_color(g);
            if self.t() < 600.0 {
                if hash01(g.index as u64, self.tick as u64 / 5) < 0.01 {
                    ch = ['#', '*', '.', ':'][(g.index + self.tick as usize) % 4];
                    color = [Rgb::from_hex("1e1e1f"), Rgb::from_hex("6d6c70"), WHITE][g.index % 3];
                } else if row_offsets[g.y as usize] != 0 {
                    color = [WHITE, Rgb::from_hex("ff0000"), Rgb::from_hex("00ff00"), Rgb::from_hex("0000ff")][(g.index + self.tick as usize / 2) % 4];
                }
            }
            cells.push(DrawCell::new(g.x + row_offsets[g.y as usize], g.y, ch, color, 1));
        }
        cells
    }

    fn effect_overflow(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let rows = self.height as i32;
        let scroll = (self.t() / 4.0) as i32;
        for g in &self.glyphs {
            let cycle_rows = rows + 6;
            for k in 0..3 {
                let y = (g.y + scroll + k * cycle_rows / 2) % cycle_rows - 3;
                if y >= 0 && y < rows {
                    let final_pass = self.t() > 520.0;
                    let color = if final_pass { self.final_color(g) } else { gradient(&[Rgb::from_hex("f2ebc0"), Rgb::from_hex("8dbfb3"), Rgb::from_hex("f2ebc0")], y as f64 / rows.max(1) as f64) };
                    cells.push(DrawCell::new(g.x, if final_pass { g.y } else { y }, g.ch, color, if final_pass { 2 } else { 0 }));
                }
            }
        }
        cells
    }

    fn effect_print(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let mut ordered: Vec<&Glyph> = self.glyphs.iter().collect();
        ordered.sort_by_key(|g| (g.y, g.x));
        let count = ((self.t() / 520.0) * ordered.len() as f64) as usize;
        let mut last = None;
        for g in ordered.iter().take(count.min(ordered.len())) {
            cells.push(DrawCell::new(g.x, g.y, g.ch, self.final_color(g), 1));
            last = Some(*g);
        }
        if let Some(g) = last {
            cells.push(DrawCell::new(g.x + 1, g.y, '█', WHITE, 3));
        }
        cells
    }

    fn effect_orbittingvolley(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        let corners = [(0.0, 0.0), (self.width as f64 - 1.0, 0.0), (self.width as f64 - 1.0, self.height as f64 - 1.0), (0.0, self.height as f64 - 1.0)];
        let launcher_t = self.t() / 70.0;
        for n in 0..4 {
            let a = launcher_t + n as f64 * std::f64::consts::FRAC_PI_2;
            let x = (a.cos() + 1.0) * 0.5 * (self.width as f64 - 1.0);
            let y = (a.sin() + 1.0) * 0.5 * (self.height as f64 - 1.0);
            cells.push(DrawCell::new(x.round() as i32, y.round() as i32, '█', gradient(&[Rgb::from_hex("FFA15C"), Rgb::from_hex("44D492")], n as f64 / 4.0), 3));
        }
        for g in &self.glyphs {
            let start = hash01(g.index as u64, 171) * 420.0;
            let p = ((self.t() - start) / 120.0).clamp(0.0, 1.0);
            if p <= 0.0 { continue; }
            let c = corners[g.index % 4];
            let x = lerp(c.0, g.x as f64, EaseKind::OutSine.apply(p));
            let y = lerp(c.1, g.y as f64, EaseKind::OutSine.apply(p));
            cells.push(DrawCell::new(x.round() as i32, y.round() as i32, g.ch, gradient(&[Rgb::from_hex("FFA15C"), self.final_color(g)], p), 1));
        }
        cells
    }

    fn effect_swarm(&self) -> Vec<DrawCell> {
        let mut cells = Vec::new();
        for g in &self.glyphs {
            let swarm = g.index / 12;
            let start = swarm as f64 * 35.0;
            let p = ((self.t() - start) / 260.0).clamp(0.0, 1.0);
            if p <= 0.0 { continue; }
            let spawn = if swarm % 2 == 0 { (-8.0, hash01(swarm as u64, 181) * self.height as f64) } else { (self.width as f64 + 8.0, hash01(swarm as u64, 182) * self.height as f64) };
            let focus = (hash01(swarm as u64, 183) * self.width as f64, hash01(swarm as u64, 184) * self.height as f64);
            let jitter = ((hash01(g.index as u64, self.tick as u64 / 8) - 0.5) * 10.0, (hash01(g.index as u64, self.tick as u64 / 9) - 0.5) * 5.0);
            let (x, y) = polyline(&[spawn, (focus.0 + jitter.0, focus.1 + jitter.1), (g.x as f64, g.y as f64)], p);
            cells.push(DrawCell::new(x.round() as i32, y.round() as i32, g.ch, gradient(&[Rgb::from_hex("31a0d4"), Rgb::from_hex("f2ea79"), self.final_color(g)], p), 1));
        }
        cells
    }
}

impl App {
    fn move_from<F>(&self, mut origin: F, span: f64, duration: f64, start_color: Rgb, ease: EaseKind) -> Vec<DrawCell>
    where F: FnMut(&App, &Glyph) -> (f64, f64) {
        let mut cells = Vec::new();
        for g in &self.glyphs {
            let start = if span == 0.0 { 0.0 } else { hash01(g.index as u64, 7) * span };
            let p = ((self.t() - start) / duration).clamp(0.0, 1.0);
            if p <= 0.0 && span > 0.0 { continue; }
            let e = ease.apply(p);
            let (x0, y0) = origin(self, g);
            let x = lerp(x0, g.x as f64, e);
            let y = lerp(y0, g.y as f64, e);
            let ch = if p < 0.98 { g.ch } else { g.ch };
            cells.push(DrawCell::new(x.round() as i32, y.round() as i32, ch, gradient(&[start_color, self.final_color(g)], p), 1));
        }
        cells
    }
}

#[derive(Clone, Copy)]
struct Glyph { index: usize, x: i32, y: i32, ch: char }

#[derive(Clone, Copy)]
struct DrawCell { x: i32, y: i32, ch: char, fg: Rgb, layer: i32 }
impl DrawCell { fn new(x: i32, y: i32, ch: char, fg: Rgb, layer: i32) -> Self { Self { x, y, ch, fg, layer } } }

fn layout_text(width: u16, height: u16, input: &str) -> Vec<Glyph> {
    let mut lines: Vec<String> = input.lines().map(|s| s.trim_end().to_string()).collect();
    if lines.is_empty() { lines.push("No Input.".to_string()); }
    let text_w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(1) as i32;
    let text_h = lines.len() as i32;
    let x0 = ((width as i32 - text_w) / 2).max(0);
    let y0 = ((height as i32 - text_h) / 2).max(0);
    let mut out = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == ' ' { continue; }
            let x = x0 + col as i32;
            let y = y0 + row as i32;
            if x >= 0 && y >= 0 && x < width as i32 && y < height as i32 {
                out.push(Glyph { index: out.len(), x, y, ch });
            }
        }
    }
    out
}

struct EffectWidget<'a> { cells: &'a [DrawCell], title: &'a str }
impl<'a> Widget for EffectWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let bg = Style::default().bg(BG.to_tui());
        for y in 0..area.height { for x in 0..area.width { buf.get_mut(area.x + x, area.y + y).set_symbol(" ").set_style(bg); } }
        let mut cells = self.cells.to_vec();
        cells.sort_by_key(|c| c.layer);
        for c in cells {
            if c.x < 0 || c.y < 0 || c.x >= area.width as i32 || c.y >= area.height as i32 { continue; }
            let style = Style::default().fg(c.fg.to_tui()).bg(BG.to_tui());
            let mut encoded = [0_u8; 4];
            let sym = c.ch.encode_utf8(&mut encoded);
            buf.get_mut(area.x + c.x as u16, area.y + c.y as u16).set_symbol(sym).set_style(style);
        }
        let style = Style::default().fg(WHITE.to_tui()).bg(LABEL_BG.to_tui()).add_modifier(Modifier::BOLD);
        for (x, ch) in self.title.chars().enumerate() {
            if x >= area.width as usize { break; }
            let mut encoded = [0_u8; 4];
            let sym = ch.encode_utf8(&mut encoded);
            buf.get_mut(area.x + x as u16, area.y).set_symbol(sym).set_style(style);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rgb { r: u8, g: u8, b: u8 }
impl Rgb {
    const fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
    fn from_hex(s: &str) -> Self {
        let h = s.trim_start_matches('#');
        let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(0);
        Self { r, g, b }
    }
    fn to_tui(self) -> TuiColor { TuiColor::Rgb(self.r, self.g, self.b) }
}

#[derive(Clone, Copy)]
enum EaseKind { Linear, InQuad, InQuart, OutSine, OutExpo, InExpo, InOutSine, InOutQuad, InOutQuart, InOutExpo, InOutBack }
impl EaseKind {
    fn apply(self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::InQuad => t * t,
            Self::InQuart => t.powi(4),
            Self::OutSine => ((t * std::f64::consts::PI) / 2.0).sin(),
            Self::OutExpo => if t >= 1.0 { 1.0 } else { 1.0 - 2_f64.powf(-10.0 * t) },
            Self::InExpo => if t <= 0.0 { 0.0 } else { 2_f64.powf(10.0 * t - 10.0) },
            Self::InOutSine => ease_in_out_sine(t),
            Self::InOutQuad => if t < 0.5 { 2.0 * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(2) / 2.0 },
            Self::InOutQuart => if t < 0.5 { 8.0 * t.powi(4) } else { 1.0 - (-2.0 * t + 2.0).powi(4) / 2.0 },
            Self::InOutExpo => if t == 0.0 { 0.0 } else if t == 1.0 { 1.0 } else if t < 0.5 { 2_f64.powf(20.0 * t - 10.0) / 2.0 } else { (2.0 - 2_f64.powf(-20.0 * t + 10.0)) / 2.0 },
            Self::InOutBack => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if t < 0.5 { ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0 } else { ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0 }
            }
        }
    }
}

fn hexes(values: &[&str]) -> Vec<Rgb> { values.iter().map(|v| Rgb::from_hex(v)).collect() }
fn lerp(a: f64, b: f64, t: f64) -> f64 { a + (b - a) * t.clamp(0.0, 1.0) }
fn distance(a: (f64, f64), b: (f64, f64)) -> f64 { ((a.0 - b.0).powi(2) + ((a.1 - b.1) * 2.0).powi(2)).sqrt() }
fn ease_in_out_sine(t: f64) -> f64 { -((std::f64::consts::PI * t).cos() - 1.0) / 2.0 }
fn ease_out_bounce(t: f64) -> f64 {
    let n1 = 7.5625;
    let d1 = 2.75;
    if t < 1.0 / d1 { n1 * t * t }
    else if t < 2.0 / d1 { n1 * (t - 1.5 / d1).powi(2) + 0.75 }
    else if t < 2.5 / d1 { n1 * (t - 2.25 / d1).powi(2) + 0.9375 }
    else { n1 * (t - 2.625 / d1).powi(2) + 0.984375 }
}

fn hash01(mut x: u64, salt: u64) -> f64 {
    x ^= salt.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    x ^= x >> 30;
    x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 31;
    ((x >> 11) as f64) / ((1_u64 << 53) as f64)
}

fn gradient(stops: &[Rgb], fraction: f64) -> Rgb {
    if stops.is_empty() { return WHITE; }
    if stops.len() == 1 { return stops[0]; }
    let f = fraction.clamp(0.0, 1.0) * (stops.len() - 1) as f64;
    let i = f.floor() as usize;
    let t = f - i as f64;
    let a = stops[i.min(stops.len() - 1)];
    let b = stops[(i + 1).min(stops.len() - 1)];
    Rgb::new(lerp(a.r as f64, b.r as f64, t) as u8, lerp(a.g as f64, b.g as f64, t) as u8, lerp(a.b as f64, b.b as f64, t) as u8)
}

fn brighten(c: Rgb, factor: f64) -> Rgb {
    Rgb::new((c.r as f64 * factor).clamp(0.0, 255.0) as u8, (c.g as f64 * factor).clamp(0.0, 255.0) as u8, (c.b as f64 * factor).clamp(0.0, 255.0) as u8)
}

fn final_color_for(kind: EffectKind, g: &Glyph, width: u16, height: u16) -> Rgb {
    let (stops, dir) = match kind {
        EffectKind::BinaryPath => (hexes(&["00d500", "007500"]), 3),
        EffectKind::BouncyBalls => (hexes(&["f8ffae", "43c6ac"]), 2),
        EffectKind::Bubbles => (hexes(&["d33aff", "02ff7f"]), 2),
        EffectKind::Burn => (hexes(&["00c3ff", "ffff1c"]), 0),
        EffectKind::ColorShift => (hexes(&["e81416", "ffa500", "faeb36", "79c314", "487de7", "4b369d", "70369d"]), 0),
        EffectKind::Crumble => (hexes(&["5CE1FF", "FF8C00"]), 2),
        EffectKind::Decrypt => (hexes(&["eda000"]), 0),
        EffectKind::Matrix => (hexes(&["92be92", "336b33"]), 3),
        EffectKind::OrbittingVolley => (hexes(&["FFA15C", "44D492"]), 3),
        EffectKind::Print => (hexes(&["02b8bd", "c1f0e3", "00ffa0"]), 2),
        EffectKind::Rain => (hexes(&["488bff", "b2e7de", "57eaf7"]), 2),
        EffectKind::Rings | EffectKind::Highlight | EffectKind::Spotlights | EffectKind::VhsTape => (hexes(&["ab48ff", "e7b2b2", "fffebd"]), 0),
        EffectKind::Scattered => (hexes(&["ff9048", "ab9dff", "bdffea"]), 0),
        EffectKind::Slide | EffectKind::Wipe => (hexes(&["833ab4", "fd1d1d", "fcb045"]), 0),
        EffectKind::Swarm => (hexes(&["31b900", "f0ff65"]), 1),
        EffectKind::Waves => (hexes(&["ffb102", "31a0d4", "f0ff65"]), 2),
        _ => (hexes(&["8A008A", "00D1FF", "ffffff"]), 0),
    };
    let fx = if width <= 1 { 0.0 } else { g.x as f64 / (width - 1) as f64 };
    let fy = if height <= 1 { 0.0 } else { g.y as f64 / (height - 1) as f64 };
    let f = match dir { 0 => 1.0 - fy, 1 => fx, 2 => (fx + (1.0 - fy)) / 2.0, _ => (((fx - 0.5).powi(2) + (fy - 0.5).powi(2)).sqrt() * 1.8).clamp(0.0, 1.0) };
    gradient(&stops, f)
}

fn shimmer<const N: usize>(symbols: [char; N], p: f64, tick: u32, offset: usize) -> char {
    if p >= 1.0 { return symbols[N - 1]; }
    let i = ((p.clamp(0.0, 0.999) * (N - 1) as f64) as usize + (tick as usize / 3 + offset) % (N - 1).max(1)) % (N - 1).max(1);
    symbols[i]
}

fn random_cipher(index: u64, step: u32) -> char {
    let blocks = ['█', '▓', '▒', '░', '╬', '╣', '║', '╗', '╝', '╚', '╔', '╩', '╦', '╠', '═', '╫'];
    if hash01(index, step as u64 + 300) < 0.58 {
        (33 + (hash01(index + 9, step as u64 + 301) * 93.0) as u8) as char
    } else {
        blocks[(hash01(index + 19, step as u64 + 302) * blocks.len() as f64) as usize % blocks.len()]
    }
}

fn polyline(points: &[(f64, f64)], p: f64) -> (f64, f64) {
    if points.is_empty() { return (0.0, 0.0); }
    if points.len() == 1 { return points[0]; }
    let total = (points.len() - 1) as f64;
    let scaled = p.clamp(0.0, 1.0) * total;
    let seg = scaled.floor() as usize;
    let t = scaled - seg as f64;
    let a = points[seg.min(points.len() - 1)];
    let b = points[(seg + 1).min(points.len() - 1)];
    (lerp(a.0, b.0, t), lerp(a.1, b.1, t))
}

