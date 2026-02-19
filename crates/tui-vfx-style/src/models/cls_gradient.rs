// <FILE>tui-vfx-style/src/models/cls_gradient.rs</FILE> - <DESC>Multi-stop color gradient</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-18T10:16:09Z</VERS>
// <WCTX>Fixing ConfigSchema gap for external recipes</WCTX>
// <CLOG>Derived ConfigSchema and marked stops as opaque</CLOG>

use crate::models::ColorConfig;
use crate::models::ColorSpace;
use crate::utils::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::Color;
#[derive(Debug, Clone, PartialEq, tui_vfx_core::ConfigSchema)]
pub struct Gradient {
    /// List of (position, color) tuples.
    /// Position must be 0.0 to 1.0.
    /// Must be sorted by position.
    #[config(opaque)]
    pub stops: Vec<(f32, Color)>,
    pub space: ColorSpace,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GradientSerde {
    stops: Vec<(f32, ColorConfig)>,
    space: ColorSpace,
}
impl From<&Gradient> for GradientSerde {
    fn from(value: &Gradient) -> Self {
        Self {
            stops: value
                .stops
                .iter()
                .map(|(t, c)| (*t, ColorConfig::from(*c)))
                .collect(),
            space: value.space,
        }
    }
}
impl From<GradientSerde> for Gradient {
    fn from(value: GradientSerde) -> Self {
        Self {
            stops: normalize_stops(
                value
                    .stops
                    .into_iter()
                    .map(|(t, c)| (t, Color::from(c)))
                    .collect(),
            ),
            space: value.space,
        }
    }
}
impl Serialize for Gradient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        GradientSerde::from(self).serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Gradient {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = GradientSerde::deserialize(deserializer)?;
        Ok(Self::from(value))
    }
}
impl Default for Gradient {
    fn default() -> Self {
        Self {
            stops: vec![(0.0, Color::BLACK), (1.0, Color::WHITE)],
            space: ColorSpace::Rgb,
        }
    }
}
impl Gradient {
    pub fn new(stops: Vec<(f32, Color)>) -> Self {
        Self {
            stops: normalize_stops(stops),
            space: ColorSpace::Rgb,
        }
    }
    pub fn sample(&self, t: f32) -> Color {
        if self.stops.is_empty() {
            return Color::TRANSPARENT;
        }
        let t = t.clamp(0.0, 1.0);
        if t <= self.stops.first().unwrap().0 {
            return self.stops.first().unwrap().1;
        }
        if t >= self.stops.last().unwrap().0 {
            return self.stops.last().unwrap().1;
        }
        for i in 0..self.stops.len() - 1 {
            let (p1, c1) = self.stops[i];
            let (p2, c2) = self.stops[i + 1];
            if t >= p1 && t <= p2 {
                let segment_t = (t - p1) / (p2 - p1);
                return blend_colors(c1, c2, segment_t, self.space);
            }
        }
        self.stops.last().unwrap().1
    }
}

fn normalize_stops(stops: Vec<(f32, Color)>) -> Vec<(f32, Color)> {
    let mut normalized: Vec<(f32, Color)> = stops
        .into_iter()
        .filter(|(t, _)| t.is_finite())
        .map(|(t, c)| (t.clamp(0.0, 1.0), c))
        .collect();
    normalized.sort_by(|a, b| a.0.total_cmp(&b.0));
    normalized.dedup_by(|a, b| a.0 == b.0);
    normalized
}

// <FILE>tui-vfx-style/src/models/cls_gradient.rs</FILE> - <DESC>Multi-stop color gradient</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-18T10:16:09Z</VERS>
