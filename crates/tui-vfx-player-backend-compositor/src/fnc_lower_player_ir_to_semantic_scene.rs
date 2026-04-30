// <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_lower_player_ir_to_semantic_scene.rs</FILE> - <DESC>Lower player render IR into compositor semantic scene inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 player backend playback: turn backend-neutral player rows/styled cells into OwnedGrid and RoleMap.</WCTX>
// <CLOG>0.1.0: INIT — map rows, styled-cell colors/modifiers, and semantic roles without importing UI state.</CLOG>

use tui_vfx_player::{PlayerRenderBackendDiagnostic, PlayerRenderCell, PlayerRenderIrReport};
use tui_vfx_types::{Cell, Color, Grid, Modifiers, OwnedGrid, RoleMap, RoleTag, SemanticScene};

/// Compositor-ready scene inputs lowered from one player render IR report.
#[derive(Clone, Debug)]
pub struct LoweredPlayerRenderIr {
    /// Source grid consumed by the compositor pipeline.
    pub source_grid: OwnedGrid,
    /// Source roles consumed by the compositor pipeline.
    pub source_roles: RoleMap,
    /// Empty destination scene the compositor writes into.
    pub destination_scene: SemanticScene,
    /// Non-fatal lowering diagnostics.
    pub diagnostics: Vec<PlayerRenderBackendDiagnostic>,
}

/// Lower player-owned render IR into compositor-ready grid and role data.
pub fn lower_player_ir_to_semantic_scene(input: &PlayerRenderIrReport) -> LoweredPlayerRenderIr {
    let width = render_width(input);
    let height = render_height(input);
    let mut grid = OwnedGrid::new(width, height);
    let mut roles = RoleMap::new_with_default(width as u16, height as u16, RoleTag::Background);
    let mut diagnostics = Vec::new();

    for (y, row) in input.rows.iter().enumerate().take(height) {
        for (x, ch) in row.chars().enumerate().take(width) {
            grid.set(x, y, Cell::new(ch));
            if ch != ' ' {
                roles.set((x as u16, y as u16), RoleTag::Text);
            }
        }
    }

    for styled_cell in &input.styled_cells {
        if styled_cell.x >= width || styled_cell.y >= height {
            diagnostics.push(PlayerRenderBackendDiagnostic {
                code: "styledCellOutOfBounds".to_string(),
                path: format!("styledCells[{},{}]", styled_cell.x, styled_cell.y),
                message: "Styled cell was outside the compositor source grid and was skipped."
                    .to_string(),
            });
            continue;
        }
        grid.set(
            styled_cell.x,
            styled_cell.y,
            cell_from_player_cell(styled_cell),
        );
        roles.set(
            (styled_cell.x as u16, styled_cell.y as u16),
            role_from_player_cell(styled_cell),
        );
    }

    let destination_grid = OwnedGrid::new(width, height);
    let destination_scene =
        SemanticScene::from_grid_with_default_role(destination_grid, RoleTag::Background);

    LoweredPlayerRenderIr {
        source_grid: grid,
        source_roles: roles,
        destination_scene,
        diagnostics,
    }
}

fn render_width(input: &PlayerRenderIrReport) -> usize {
    input
        .width
        .max(
            input
                .rows
                .iter()
                .map(|row| row.chars().count())
                .max()
                .unwrap_or(0),
        )
        .max(
            input
                .styled_cells
                .iter()
                .map(|cell| cell.x + 1)
                .max()
                .unwrap_or(0),
        )
}

fn render_height(input: &PlayerRenderIrReport) -> usize {
    input.height.max(input.rows.len()).max(
        input
            .styled_cells
            .iter()
            .map(|cell| cell.y + 1)
            .max()
            .unwrap_or(0),
    )
}

fn cell_from_player_cell(styled_cell: &PlayerRenderCell) -> Cell {
    Cell::new(glyph_char(&styled_cell.glyph))
        .with_fg(color_from_label(&styled_cell.foreground))
        .with_bg(color_from_label(&styled_cell.background))
        .with_mods(modifiers_from_labels(&styled_cell.modifiers))
}

fn glyph_char(glyph: &str) -> char {
    glyph.chars().next().unwrap_or(' ')
}

fn color_from_label(label: &str) -> Color {
    let normalized = label.trim();
    if normalized.eq_ignore_ascii_case("transparent")
        || normalized.eq_ignore_ascii_case("default")
        || normalized.eq_ignore_ascii_case("defaultForeground")
        || normalized.eq_ignore_ascii_case("defaultBackground")
        || normalized.is_empty()
    {
        return Color::TRANSPARENT;
    }
    match normalized.to_ascii_lowercase().as_str() {
        "black" => return Color::BLACK,
        "white" => return Color::WHITE,
        "red" => return Color::RED,
        "green" => return Color::GREEN,
        "blue" => return Color::BLUE,
        "yellow" => return Color::YELLOW,
        "cyan" => return Color::CYAN,
        "magenta" => return Color::MAGENTA,
        _ => {}
    }
    parse_rgba(normalized).unwrap_or(Color::TRANSPARENT)
}

fn parse_rgba(label: &str) -> Option<Color> {
    let inner = label.strip_prefix("rgba(")?.strip_suffix(')')?;
    let mut parts = inner.split(',').map(str::trim);
    let r = parts.next()?.parse::<u8>().ok()?;
    let g = parts.next()?.parse::<u8>().ok()?;
    let b = parts.next()?.parse::<u8>().ok()?;
    let a = parts.next()?.parse::<u8>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(Color::new(r, g, b, a))
}

fn modifiers_from_labels(labels: &[String]) -> Modifiers {
    labels.iter().fold(Modifiers::NONE, |mods, label| {
        match label.to_ascii_lowercase().as_str() {
            "bold" => mods.with_bold(),
            "italic" => mods.with_italic(),
            "underline" => mods.with_underline(),
            "dim" => mods.with_dim(),
            "reverse" => mods.with_reverse(),
            "strikethrough" => mods.with_strikethrough(),
            "slowblink" | "slow_blink" | "slow-blink" => mods.with_slow_blink(),
            "rapidblink" | "rapid_blink" | "rapid-blink" => mods.with_rapid_blink(),
            "hidden" => mods.with_hidden(),
            _ => mods,
        }
    })
}

fn role_from_player_cell(styled_cell: &PlayerRenderCell) -> RoleTag {
    styled_cell
        .role
        .as_deref()
        .map(str::trim)
        .filter(|role| !role.is_empty())
        .map(|role| RoleTag::from_shorthand(&role.to_ascii_lowercase()))
        .unwrap_or_else(|| {
            if styled_cell.glyph.trim().is_empty() {
                RoleTag::Background
            } else {
                RoleTag::Text
            }
        })
}

/// Convert a compositor cell back to player backend evidence labels.
pub fn player_cell_from_compositor_cell(
    x: usize,
    y: usize,
    cell: &Cell,
    role: Option<RoleTag>,
) -> PlayerRenderCell {
    PlayerRenderCell {
        x,
        y,
        glyph: cell.ch.to_string(),
        foreground: color_label(cell.fg),
        background: color_label(cell.bg),
        modifiers: modifier_labels(cell.mods),
        role: role.map(|role| role.shorthand_name()),
    }
}

fn color_label(color: Color) -> String {
    if color.a == 0 {
        "transparent".to_string()
    } else {
        format!("rgba({},{},{},{})", color.r, color.g, color.b, color.a)
    }
}

fn modifier_labels(modifiers: Modifiers) -> Vec<String> {
    let mut labels = Vec::new();
    if modifiers.bold {
        labels.push("bold".to_string());
    }
    if modifiers.italic {
        labels.push("italic".to_string());
    }
    if modifiers.underline {
        labels.push("underline".to_string());
    }
    if modifiers.dim {
        labels.push("dim".to_string());
    }
    if modifiers.reverse {
        labels.push("reverse".to_string());
    }
    if modifiers.strikethrough {
        labels.push("strikethrough".to_string());
    }
    if modifiers.slow_blink {
        labels.push("slowBlink".to_string());
    }
    if modifiers.rapid_blink {
        labels.push("rapidBlink".to_string());
    }
    if modifiers.hidden {
        labels.push("hidden".to_string());
    }
    labels
}

#[cfg(test)]
mod tests {
    use tui_vfx_contract::LifecyclePhase;
    use tui_vfx_player::{PlayerRenderIrReport, PlayerStatus};
    use tui_vfx_types::Grid;

    use super::*;

    #[test]
    fn lowers_rows_styled_colors_and_roles() {
        let report = PlayerRenderIrReport {
            schema_version: "v3.1.player.renderIr.1",
            recipe_id: "demo".to_string(),
            path: None,
            status: PlayerStatus::Rendered,
            phase: LifecyclePhase::Dwell,
            phase_t: 1.0,
            loop_t: None,
            width: 2,
            height: 1,
            render_hash: 1,
            non_empty_cells: 2,
            rows: vec!["AB".to_string()],
            styled_cells: vec![PlayerRenderCell {
                x: 1,
                y: 0,
                glyph: "B".to_string(),
                foreground: "rgba(1,2,3,255)".to_string(),
                background: "rgba(4,5,6,255)".to_string(),
                modifiers: vec!["bold".to_string()],
                role: Some("border".to_string()),
            }],
            provenance: vec![],
            layers: vec![],
            graph_values: vec![],
            errors: vec![],
            warnings: vec![],
        };

        let lowered = lower_player_ir_to_semantic_scene(&report);
        let cell = lowered.source_grid.get(1, 0).expect("cell");
        assert_eq!(cell.ch, 'B');
        assert_eq!(cell.fg, Color::new(1, 2, 3, 255));
        assert_eq!(cell.bg, Color::new(4, 5, 6, 255));
        assert!(cell.mods.bold);
        assert_eq!(lowered.source_roles.get((1, 0)), Some(RoleTag::Border));
    }
}

// <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_lower_player_ir_to_semantic_scene.rs</FILE> - <DESC>Lower player render IR into compositor semantic scene inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
