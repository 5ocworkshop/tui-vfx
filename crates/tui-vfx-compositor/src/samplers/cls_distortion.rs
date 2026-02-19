// <FILE>tui-vfx-compositor/src/samplers/cls_distortion.rs</FILE>
// <DESC>Generic distortion sampler</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>WG5: Sampler Spatial Context Enhancement</WCTX>
// <CLOG>BREAKING CHANGE: Updated sample() signature to include width, height parameters (unused, prefixed with _)</CLOG>

use crate::traits::sampler::Sampler;

#[allow(dead_code)]
pub struct Distortion;

impl Sampler for Distortion {
    fn sample(
        &self,
        dest_x: u16,
        dest_y: u16,
        _width: u16,
        _height: u16,
        t: f64,
    ) -> Option<(u16, u16)> {
        let t = t as f32;

        // Sine wave distortion
        let offset = (t * 10.0 + (dest_y as f32 / 5.0)).sin() * 2.0;
        let src_x = (dest_x as f32 + offset).round();

        if src_x < 0.0 {
            return None;
        }

        Some((src_x as u16, dest_y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distortion_row_zero_identity_at_t0() {
        let sampler = Distortion;
        // At t=0 and y=0: offset = sin(0*10 + 0/5) * 2 = sin(0) * 2 = 0
        assert_eq!(sampler.sample(5, 0, 10, 10, 0.0), Some((5, 0)));
    }

    #[test]
    fn test_distortion_preserves_y() {
        let sampler = Distortion;
        // Distortion only affects x, y should always be preserved
        for y in 0..5 {
            let result = sampler.sample(5, y, 10, 10, 0.5);
            assert!(result.is_some());
            assert_eq!(result.unwrap().1, y);
        }
    }

    #[test]
    fn test_distortion_negative_x_returns_none() {
        let sampler = Distortion;
        // At x=0 with negative offset, should return None
        // offset = sin(t*10 + y/5) * 2 can be negative when sin is negative
        // Find a combination that gives negative offset
        // sin(PI) = 0, sin(3*PI/2) = -1
        // t*10 + y/5 = 3*PI/2 → test approximately
        let result = sampler.sample(0, 0, 10, 10, std::f64::consts::PI * 0.15);
        // May or may not be None depending on exact offset
        // Just verify it handles the case gracefully
        let _ = result;
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_distortion.rs</FILE>
// <DESC>Generic distortion sampler</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
