// <FILE>tui-vfx-geometry/src/types/path_type.rs</FILE> - <DESC>Motion path types with physics integration</DESC>
// <VERS>VERSION: 3.0.0 - 2025-12-31</VERS>
// <WCTX>Architectural roadmap: Physics PathType variants</WCTX>
// <CLOG>Added Projectile, Friction, Orbit, Pendulum variants using mixed-signals physics</CLOG>

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum PathType {
    Linear,
    Arc {
        bulge: f32,
    },
    /// Quadratic Bezier curve with explicit control point.
    ///
    /// Unlike `Arc` which computes the control point from a bulge factor,
    /// `Bezier` allows specifying an arbitrary control point for precise
    /// curve shaping. The control point is typically resolved from a
    /// `via` PlacementSpec at animation setup time.
    ///
    /// Note: This is a spatial quadratic Bezier (single control point) used
    /// for motion paths. It is distinct from `EasingCurve::Bezier`, which is
    /// a cubic Bezier defined in normalized time/value space for easing.
    ///
    /// Example: An arc from mid-left to mid-right via top-center would
    /// have control_x at center and control_y near the top.
    Bezier {
        #[config(help = "X coordinate of the control point", default = 0.0)]
        control_x: f32,
        #[config(help = "Y coordinate of the control point", default = 0.0)]
        control_y: f32,
    },
    Spring {
        stiffness: f32,
        damping: f32,
    },
    Bounce {
        #[config(
            help = "Number of bounces (oscillations)",
            default = 3,
            min = 0,
            max = 12
        )]
        bounces: u8,
        #[config(
            help = "Decay factor (higher settles faster)",
            default = 6.0,
            min = 0.0,
            max = 50.0
        )]
        decay: f32,
    },
    Squash,
    Hover,
    Rectilinear {
        x_first: bool,
    },
    /// Spirals outward from start to end.
    Spiral {
        #[config(default = 2.0)]
        rotations: f32,
    },
    /// Quantized movement (stop-motion).
    Step {
        #[config(default = 5)]
        steps: u8,
    },

    // =========================================================================
    // Physics-based paths (using mixed-signals solvers)
    // =========================================================================
    /// Ballistic/projectile motion with gravity.
    ///
    /// Creates a parabolic arc like throwing or tossing an object.
    /// The element follows a realistic physics trajectory from start to end.
    ///
    /// # Parameters
    /// - `arc_height`: Peak height above the straight line (in cells). Negative for upward arc.
    /// - `gravity`: Gravity strength (higher = faster fall). Default: 500.0
    ///
    /// # Example
    /// A toast notification that arcs in from the side would use a negative arc_height
    /// to create an upward-then-down trajectory.
    Projectile {
        #[config(
            help = "Peak height of arc above straight line (negative for upward)",
            default = -20.0
        )]
        arc_height: f32,
        #[config(
            help = "Gravity strength (higher = faster fall)",
            default = 500.0,
            min = 0.0
        )]
        gravity: f32,
    },

    /// Friction/momentum decay for scroll-like motion.
    ///
    /// Element moves with initial momentum and decelerates due to friction.
    /// Creates natural-feeling scroll inertia or slide-to-stop effects.
    ///
    /// # Parameters
    /// - `drag`: Friction coefficient (higher = faster stop). Default: 4.0
    ///
    /// # Note
    /// The path travels from start toward end, decelerating naturally.
    /// At t=1.0, the element will be at or very near the end position.
    Friction {
        #[config(
            help = "Friction coefficient (higher = faster stop)",
            default = 4.0,
            min = 0.1,
            max = 20.0
        )]
        drag: f32,
    },

    /// Orbital/circular motion around a center point.
    ///
    /// Element orbits around the midpoint between start and end positions.
    /// Useful for spinner effects or circular reveal animations.
    ///
    /// # Parameters
    /// - `revolutions`: Number of complete orbits (can be fractional). Default: 1.0
    /// - `direction`: 1.0 for counter-clockwise, -1.0 for clockwise. Default: 1.0
    ///
    /// # Example
    /// A loading spinner that orbits once from start to end position.
    Orbit {
        #[config(
            help = "Number of complete revolutions",
            default = 1.0,
            min = 0.0,
            max = 10.0
        )]
        revolutions: f32,
        #[config(
            help = "Direction: 1.0 = counter-clockwise, -1.0 = clockwise",
            default = 1.0
        )]
        direction: f32,
    },

    /// Pendulum/swinging motion.
    ///
    /// Element swings like a pendulum, oscillating around the end position.
    /// Useful for hanging notifications or "swing in" effects.
    ///
    /// # Parameters
    /// - `amplitude`: Initial swing amplitude in cells. Default: 30.0
    /// - `oscillations`: Number of back-and-forth swings. Default: 3.0
    /// - `damping`: How quickly oscillations decay (0 = no decay). Default: 2.0
    ///
    /// # Note
    /// The pendulum swings around the end position, settling there at t=1.0.
    Pendulum {
        #[config(help = "Initial swing amplitude in cells", default = 30.0, min = 0.0)]
        amplitude: f32,
        #[config(help = "Number of oscillations", default = 3.0, min = 0.0, max = 20.0)]
        oscillations: f32,
        #[config(
            help = "Damping factor (higher = faster settling)",
            default = 2.0,
            min = 0.0
        )]
        damping: f32,
    },
}

// <FILE>tui-vfx-geometry/src/types/path_type.rs</FILE> - <DESC>Motion path types with physics integration</DESC>
// <VERS>END OF VERSION: 3.0.0 - 2025-12-31</VERS>
