// <FILE>crates/tui-vfx-player-ui/src/fnc_render_studio_controls.rs</FILE> - <DESC>Render themed descriptor-derived studio controls</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>Player UI presentation: keep studio control rendering out of the main frame orchestrator.</WCTX>
// <CLOG>0.2.1: PATCH — widen slider tracks so studio controls are easier to target by mouse.
// 0.2.0: MINOR — render local slider, toggle, enum, and color affordances with selected-row windowing.
// 0.1.0: INIT — render studio controls with focused borders and readable value roles.</CLOG>

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{
    PlayerUiApp, PlayerUiFocus, cls_player_ui_control::PlayerUiControl,
    cls_player_ui_theme::PlayerUiTheme,
};

const SLIDER_TRACK_WIDTH: usize = 20;

/// Render descriptor-derived studio controls.
pub(crate) fn render_studio_controls(app: &PlayerUiApp, frame: &mut Frame<'_>, area: Rect) {
    let theme = PlayerUiTheme::eichler();
    let title = if app.focus == PlayerUiFocus::Studio {
        " Studio controls * "
    } else {
        " Studio controls "
    };
    let lines = studio_control_lines(app, area.height.saturating_sub(2), theme);
    frame.render_widget(
        Paragraph::new(lines)
            .style(theme.elevated_panel_style())
            .block(
                Block::default()
                    .title(Line::from(Span::styled(title, theme.title_style())))
                    .borders(Borders::ALL)
                    .border_style(if app.focus == PlayerUiFocus::Studio {
                        theme.focused_border_style()
                    } else {
                        theme.elevated_border_style()
                    })
                    .style(theme.elevated_panel_style()),
            )
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn studio_control_lines(
    app: &PlayerUiApp,
    visible_rows: u16,
    theme: PlayerUiTheme,
) -> Vec<Line<'static>> {
    if app.player.controls.is_empty() {
        return vec![Line::from(Span::styled(
            "no descriptor-derived controls",
            theme.muted_style(),
        ))];
    }

    let visible_control_count = usize::from(visible_rows.max(1)).saturating_div(3).max(1);
    let selected_index = app
        .studio_control_index
        .min(app.player.controls.len().saturating_sub(1));
    let start_index = if app.player.controls.len() <= visible_control_count {
        0
    } else {
        selected_index
            .saturating_sub(visible_control_count / 2)
            .min(
                app.player
                    .controls
                    .len()
                    .saturating_sub(visible_control_count),
            )
    };
    let end_index = (start_index + visible_control_count).min(app.player.controls.len());

    app.player.controls[start_index..end_index]
        .iter()
        .enumerate()
        .map(|(offset, control)| {
            let index = start_index + offset;
            control_line(control, index == selected_index, theme)
        })
        .collect()
}

fn control_line(control: &PlayerUiControl, selected: bool, theme: PlayerUiTheme) -> Line<'static> {
    let marker = if selected { "▶" } else { " " };
    let label_style = if selected {
        theme.selected_row_style()
    } else {
        theme.body_style()
    };
    let value = effective_json_value(control);
    let widget = visual_widget(control, value.as_ref(), theme);
    Line::from(vec![
        Span::styled(marker.to_string(), theme.label_style()),
        Span::styled(format!(" {} ", control.label), label_style),
        Span::styled(format!("[{}] ", control.control_kind), theme.muted_style()),
        widget,
    ])
}

fn visual_widget(
    control: &PlayerUiControl,
    value: Option<&serde_json::Value>,
    theme: PlayerUiTheme,
) -> Span<'static> {
    match control.value_kind.as_str() {
        "integer" | "number" | "duration" if control.range.is_some() => {
            Span::styled(numeric_slider_text(control, value), theme.body_style())
        }
        "boolean" => Span::styled(boolean_toggle_text(value), theme.body_style()),
        "enum" if !control.allowed_values.is_empty() => {
            Span::styled(enum_options_text(control, value), theme.body_style())
        }
        "color" => Span::styled(color_swatch_text(value), color_swatch_style(value, theme)),
        _ => Span::styled(readable_value_text(value), theme.body_style()),
    }
}

fn numeric_slider_text(control: &PlayerUiControl, value: Option<&serde_json::Value>) -> String {
    let current = numeric_value(value).unwrap_or(0.0);
    let Some(range) = control.range else {
        return readable_value_text(value);
    };
    let min_text = range
        .min
        .map(format_number)
        .unwrap_or_else(|| "−∞".to_string());
    let max_text = range
        .max
        .map(format_number)
        .unwrap_or_else(|| "+∞".to_string());
    format!(
        "{} {} range {}..{}",
        slider_track(current, range.min, range.max),
        format_number(current),
        min_text,
        max_text
    )
}

fn slider_track(current: f64, min: Option<f64>, max: Option<f64>) -> String {
    let Some((min, max)) = min.zip(max).filter(|(min, max)| max > min) else {
        return "──────────".to_string();
    };
    let ratio = ((current - min) / (max - min)).clamp(0.0, 1.0);
    let thumb = (ratio * (SLIDER_TRACK_WIDTH.saturating_sub(1) as f64)).round() as usize;
    (0..SLIDER_TRACK_WIDTH)
        .map(|index| {
            if index == thumb {
                '●'
            } else if index < thumb {
                '━'
            } else {
                '─'
            }
        })
        .collect()
}

fn boolean_toggle_text(value: Option<&serde_json::Value>) -> String {
    if boolean_value(value).unwrap_or(false) {
        "toggle [● ON ]".to_string()
    } else {
        "toggle [○ OFF]".to_string()
    }
}

fn enum_options_text(control: &PlayerUiControl, value: Option<&serde_json::Value>) -> String {
    let current = string_value(value).unwrap_or_default();
    let rendered = control
        .allowed_values
        .iter()
        .map(|allowed| {
            if allowed == &current {
                format!("[{allowed}]")
            } else {
                allowed.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    format!("opts {rendered}")
}

fn color_swatch_text(value: Option<&serde_json::Value>) -> String {
    let label = color_label(value).unwrap_or_else(|| readable_value_text(value));
    format!("▣ {label}")
}

fn color_swatch_style(value: Option<&serde_json::Value>, theme: PlayerUiTheme) -> Style {
    color_rgb(value)
        .map(|(red, green, blue)| Style::default().fg(Color::Rgb(red, green, blue)))
        .unwrap_or_else(|| theme.body_style())
}

fn effective_json_value(control: &PlayerUiControl) -> Option<serde_json::Value> {
    control
        .current_value
        .as_ref()
        .and_then(literal_from_json)
        .or_else(|| control.default_value.as_ref().and_then(literal_from_json))
}

fn literal_from_json(value: &serde_json::Value) -> Option<serde_json::Value> {
    if value.get("value").is_some() && value.get("kind").and_then(|kind| kind.as_str()).is_some() {
        if matches!(
            value.get("kind").and_then(|kind| kind.as_str()),
            Some("literal")
        ) {
            return value.get("value").cloned();
        }
        return Some(value.clone());
    }
    value
        .get("fallback")
        .cloned()
        .or_else(|| value.get("value").cloned())
        .or_else(|| Some(value.clone()))
}

fn numeric_value(value: Option<&serde_json::Value>) -> Option<f64> {
    let value = value?;
    value
        .get("value")
        .and_then(|value| value.as_f64())
        .or_else(|| value.as_f64())
}

fn boolean_value(value: Option<&serde_json::Value>) -> Option<bool> {
    let value = value?;
    value
        .get("value")
        .and_then(|value| value.as_bool())
        .or_else(|| value.as_bool())
}

fn string_value(value: Option<&serde_json::Value>) -> Option<String> {
    let value = value?;
    value
        .get("value")
        .and_then(|value| value.as_str())
        .or_else(|| value.as_str())
        .map(ToString::to_string)
}

fn color_label(value: Option<&serde_json::Value>) -> Option<String> {
    if let Some(text) = string_value(value) {
        return Some(text);
    }
    let (red, green, blue) = color_rgb(value)?;
    Some(format!("#{red:02x}{green:02x}{blue:02x}"))
}

fn color_rgb(value: Option<&serde_json::Value>) -> Option<(u8, u8, u8)> {
    let value = value?;
    let payload = value.get("value").unwrap_or(value);
    Some((
        payload.get("r")?.as_u64()?.try_into().ok()?,
        payload.get("g")?.as_u64()?.try_into().ok()?,
        payload.get("b")?.as_u64()?.try_into().ok()?,
    ))
}

fn readable_value_text(value: Option<&serde_json::Value>) -> String {
    match value {
        Some(value) => string_value(Some(value)).unwrap_or_else(|| value.to_string()),
        None => "none".to_string(),
    }
}

fn format_number(value: f64) -> String {
    if (value.fract()).abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.2}")
    }
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_render_studio_controls.rs</FILE> - <DESC>Render themed descriptor-derived studio controls</DESC>
// <VERS>END OF VERSION: 0.2.1</VERS>
