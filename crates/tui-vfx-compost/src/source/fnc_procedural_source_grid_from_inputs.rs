// <FILE>crates/tui-vfx-compost/src/source/fnc_procedural_source_grid_from_inputs.rs</FILE> - <DESC>Materialize registered procedural sources</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>source.procedural dispatches named deterministic generators without legacy compositor DTOs.</WCTX>
// <CLOG>0.1.0: INIT — render braille_flag_field from v3.1 braille dotfield assets.</CLOG>

use std::collections::BTreeMap;
use std::fs;

use serde::Deserialize;
use tui_vfx_contract::{AssetLocator, AssetSpec, SourceInputId, StructuredValue, ValueSource};
use tui_vfx_types::{Cell, Color, Grid, Modifiers, OwnedGrid, RoleMap, RoleTag, SemanticScene};

use crate::RenderError;
use crate::runtime::RuntimeContext;

use super::col_literal_source_input::{literal_integer, literal_structured, literal_text};

pub(crate) fn procedural_source_grid_from_inputs(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    assets: &BTreeMap<tui_vfx_contract::AssetId, AssetSpec>,
    context: &RuntimeContext,
) -> Result<SemanticScene, RenderError> {
    let generator = literal_text(inputs, "generator", context)?;
    match generator.as_str() {
        "braille_flag_field" => render_braille_flag_field(inputs, assets, context),
        other => Err(RenderError::Unsupported(format!(
            "source.procedural generator `{other}` is not materializable"
        ))),
    }
}

fn render_braille_flag_field(
    inputs: &BTreeMap<SourceInputId, ValueSource>,
    assets: &BTreeMap<tui_vfx_contract::AssetId, AssetSpec>,
    context: &RuntimeContext,
) -> Result<SemanticScene, RenderError> {
    let width = literal_integer(inputs, "width", context)?;
    let height = literal_integer(inputs, "height", context)?;
    let params = literal_structured(inputs, "params", context)?;
    let asset = asset_for_params(&params, assets)?;
    let dotfield = load_dotfield(asset)?;
    let mut grid = OwnedGrid::new(width, height);
    let mut roles = RoleMap::new_with_default(width as u16, height as u16, RoleTag::Background);
    let scale_x = dotfield
        .width_dots
        .saturating_div(width.saturating_mul(2))
        .max(1);
    let scale_y = dotfield
        .height_dots
        .saturating_div(height.saturating_mul(4))
        .max(1);

    for y in 0..height {
        for x in 0..width {
            if let Some((glyph, color)) = dotfield.braille_cell(x, y, scale_x, scale_y) {
                grid.set(
                    x,
                    y,
                    Cell::styled(glyph, color, Color::TRANSPARENT, Modifiers::NONE),
                );
                roles.set((x as u16, y as u16), RoleTag::Procedural);
            }
        }
    }
    Ok(SemanticScene::new(grid, roles))
}

fn asset_for_params<'a>(
    params: &StructuredValue,
    assets: &'a BTreeMap<tui_vfx_contract::AssetId, AssetSpec>,
) -> Result<&'a AssetSpec, RenderError> {
    let StructuredValue::Object(values) = params else {
        return Err(RenderError::Unsupported(
            "source.procedural params must be an object".to_string(),
        ));
    };
    let Some(StructuredValue::String(reference)) = values.get("asset") else {
        return Err(RenderError::Unsupported(
            "braille_flag_field params must include asset reference".to_string(),
        ));
    };
    let asset_id = reference.strip_prefix("$asset:").ok_or_else(|| {
        RenderError::Unsupported("braille_flag_field asset must use $asset:<id>".to_string())
    })?;
    assets
        .get(&tui_vfx_contract::AssetId::new(asset_id))
        .ok_or_else(|| RenderError::Unsupported(format!("unknown procedural asset `{asset_id}`")))
}

fn load_dotfield(asset: &AssetSpec) -> Result<BrailleDotfieldAsset, RenderError> {
    let AssetLocator::Path { path } = &asset.locator else {
        return Err(RenderError::Unsupported(
            "braille_flag_field requires a path asset locator".to_string(),
        ));
    };
    let text = fs::read_to_string(path).map_err(|error| {
        RenderError::Unsupported(format!(
            "could not read braille dotfield asset `{path}`: {error}"
        ))
    })?;
    serde_json::from_str(&text).map_err(|error| {
        RenderError::Unsupported(format!(
            "could not parse braille dotfield asset `{path}`: {error}"
        ))
    })
}

#[derive(Deserialize)]
struct BrailleDotfieldAsset {
    width_dots: usize,
    height_dots: usize,
    transparent: char,
    palette: BTreeMap<char, PaletteColor>,
    rows: Vec<String>,
}

#[derive(Deserialize)]
struct PaletteColor {
    r: u8,
    g: u8,
    b: u8,
}

impl BrailleDotfieldAsset {
    fn braille_cell(
        &self,
        cell_x: usize,
        cell_y: usize,
        scale_x: usize,
        scale_y: usize,
    ) -> Option<(char, Color)> {
        let mut bits = 0u8;
        let mut color_counts: BTreeMap<char, u8> = BTreeMap::new();
        for dot_y in 0..4 {
            for dot_x in 0..2 {
                let source_x = cell_x * 2 * scale_x + dot_x * scale_x;
                let source_y = cell_y * 4 * scale_y + dot_y * scale_y;
                let Some(symbol) = self.symbol_at(source_x, source_y) else {
                    continue;
                };
                if symbol == self.transparent {
                    continue;
                }
                bits |= braille_bit(dot_x, dot_y);
                *color_counts.entry(symbol).or_default() += 1;
            }
        }
        if bits == 0 {
            return None;
        }
        let color_symbol = color_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(symbol, _)| symbol)?;
        let color = self.palette.get(&color_symbol)?;
        Some((char::from_u32(0x2800 + u32::from(bits))?, color.to_color()))
    }

    fn symbol_at(&self, x: usize, y: usize) -> Option<char> {
        self.rows.get(y)?.chars().nth(x)
    }
}

impl PaletteColor {
    fn to_color(&self) -> Color {
        Color::rgb(self.r, self.g, self.b)
    }
}

fn braille_bit(dot_x: usize, dot_y: usize) -> u8 {
    match (dot_x, dot_y) {
        (0, 0) => 0x01,
        (0, 1) => 0x02,
        (0, 2) => 0x04,
        (0, 3) => 0x40,
        (1, 0) => 0x08,
        (1, 1) => 0x10,
        (1, 2) => 0x20,
        (1, 3) => 0x80,
        _ => 0,
    }
}

// <FILE>crates/tui-vfx-compost/src/source/fnc_procedural_source_grid_from_inputs.rs</FILE> - <DESC>Materialize registered procedural sources</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
