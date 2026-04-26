// <FILE>crates/tui-vfx-types/src/glyph/test_fnc_sample_eight_subcells.rs</FILE> - <DESC>TDD peer tests for sample_eight_subcells and sample_eight_subcells_with_slope helpers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Glyph rendering framework Phase 3: TDD tests for subcell sampling helpers for water/fire/future field-effect glyph encoding</WCTX>
// <CLOG>0.1.0: initial TDD coverage with dot-order pinning, no-mutation check, slope interpolation verification, and single-call counter test</CLOG>

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use mixed_signals::traits::{Signal, SignalContext, SignalTime};
    use mixed_signals::traits::{SignalWithSlope, SlopeSample};

    use crate::glyph::fnc_sample_eight_subcells::{
        SUBCELL_OFFSETS, sample_eight_subcells, sample_eight_subcells_with_slope,
    };

    // ─── Counting signal that returns call-index-based values ────────────────

    struct CountingSignal {
        counter: Arc<AtomicUsize>,
    }

    impl Signal for CountingSignal {
        fn sample(&self, _t: SignalTime) -> f32 {
            0.0
        }

        fn sample_with_context(&self, _t: SignalTime, _ctx: &SignalContext) -> f32 {
            let idx = self.counter.fetch_add(1, Ordering::SeqCst);
            // Return the call index as a fraction so each subcell gets a distinct value
            idx as f32 / 10.0
        }
    }

    // ─── Slope signal that counts calls ─────────────────────────────────────

    struct SlopeCountingSignal {
        slope_calls: Arc<AtomicUsize>,
        /// Return value: constant field with given slope
        value: f32,
        dx: f32,
        dy: f32,
    }

    impl Signal for SlopeCountingSignal {
        fn sample(&self, _t: SignalTime) -> f32 {
            self.value
        }

        fn sample_with_context(&self, _t: SignalTime, _ctx: &SignalContext) -> f32 {
            self.value
        }
    }

    impl SignalWithSlope for SlopeCountingSignal {
        fn sample_with_slope(&self, _t: SignalTime, _ctx: &SignalContext) -> SlopeSample {
            self.slope_calls.fetch_add(1, Ordering::SeqCst);
            SlopeSample {
                value: self.value,
                dx: self.dx,
                dy: self.dy,
            }
        }
    }

    // ─── Tests ───────────────────────────────────────────────────────────────

    /// Pins the SUBCELL_OFFSETS constant to the expected dot order.
    #[test]
    fn test_subcell_offsets_constant() {
        assert_eq!(
            SUBCELL_OFFSETS,
            [
                (0.25, 0.125), // dot 1
                (0.25, 0.375), // dot 2
                (0.25, 0.625), // dot 3
                (0.75, 0.125), // dot 4
                (0.75, 0.375), // dot 5
                (0.75, 0.625), // dot 6
                (0.25, 0.875), // dot 7
                (0.75, 0.875), // dot 8
            ]
        );
    }

    /// Verifies that sample_eight_subcells calls the signal exactly eight times
    /// and that results are ordered by call index (matching SUBCELL_OFFSETS traversal).
    #[test]
    fn test_sample_eight_returns_eight_scalars_in_dot_order() {
        let counter = Arc::new(AtomicUsize::new(0));
        let signal = CountingSignal {
            counter: counter.clone(),
        };
        let ctx = SignalContext::default();
        let results = sample_eight_subcells(&signal, &ctx, 0.0);

        // Eight calls made
        assert_eq!(counter.load(Ordering::SeqCst), 8);
        // Results are in call order: 0/10, 1/10, ..., 7/10
        for i in 0..8 {
            assert!(
                (results[i] - i as f32 / 10.0).abs() < 1e-6,
                "results[{i}] = {} expected {}",
                results[i],
                i as f32 / 10.0
            );
        }
    }

    /// Verifies that the input ctx is not mutated by sample_eight_subcells.
    ///
    /// Because `subcell_offset` is pub(crate), we verify non-mutation
    /// indirectly: the same ctx passed to a plain signal before and after
    /// sample_eight_subcells produces the same output.
    #[test]
    fn test_sample_eight_does_not_mutate_input_ctx() {
        struct ConstantSignal;
        impl Signal for ConstantSignal {
            fn sample(&self, _t: SignalTime) -> f32 {
                0.42
            }
        }

        let signal = ConstantSignal;
        let ctx = SignalContext::default();
        // Sample before
        let before = signal.sample_with_context(0.0, &ctx);
        // Run sample_eight_subcells (uses ctx clones internally)
        let _ = sample_eight_subcells(&signal, &ctx, 0.0);
        // Sample after — should be identical
        let after = signal.sample_with_context(0.0, &ctx);
        assert_eq!(before, after, "ctx must not be mutated");
        // Also verify ctx still works: frame should still be 0
        assert_eq!(ctx.frame, 0);
    }

    /// sample_eight_subcells_with_slope must call sample_with_slope exactly once.
    #[test]
    fn test_sample_eight_with_slope_uses_one_sample_call() {
        let slope_calls = Arc::new(AtomicUsize::new(0));
        let signal = SlopeCountingSignal {
            slope_calls: slope_calls.clone(),
            value: 0.5,
            dx: 0.0,
            dy: 0.0,
        };
        let ctx = SignalContext::default();
        let _ = sample_eight_subcells_with_slope(&signal, &ctx, 0.0);
        assert_eq!(
            slope_calls.load(Ordering::SeqCst),
            1,
            "sample_with_slope should be called exactly once"
        );
    }

    /// Linear interpolation correctness: value=0.0, dx=2.0, dy=4.0.
    ///
    /// For each subcell offset (ox, oy):
    ///   expected = value + (ox - 0.5) * dx + (oy - 0.5) * dy
    ///            = 0.0 + (ox - 0.5) * 2.0 + (oy - 0.5) * 4.0
    #[test]
    fn test_sample_eight_with_slope_linear_interpolates_correctly() {
        let signal = SlopeCountingSignal {
            slope_calls: Arc::new(AtomicUsize::new(0)),
            value: 0.0,
            dx: 2.0,
            dy: 4.0,
        };
        let ctx = SignalContext::default();
        let results = sample_eight_subcells_with_slope(&signal, &ctx, 0.0);

        for (i, &(ox, oy)) in SUBCELL_OFFSETS.iter().enumerate() {
            let expected = (ox - 0.5) * 2.0 + (oy - 0.5) * 4.0;
            assert!(
                (results[i] - expected).abs() < 1e-5,
                "subcell[{i}]: got {}, expected {}",
                results[i],
                expected
            );
        }
    }

    /// A plain Signal wrapped with the default SignalWithSlope impl produces finite outputs.
    #[test]
    fn test_sample_eight_default_signal_with_slope_falls_back_to_numeric_diff() {
        struct LinearSignal;
        impl Signal for LinearSignal {
            fn sample(&self, _t: SignalTime) -> f32 {
                0.5
            }

            fn sample_with_context(&self, _t: SignalTime, ctx: &SignalContext) -> f32 {
                // Simple spatial signal
                ctx.cell_x.unwrap_or(0) as f32 * 0.1
            }
        }
        impl SignalWithSlope for LinearSignal {}

        let signal = LinearSignal;
        let ctx = SignalContext::default().with_cell_position(4, 2);
        let results = sample_eight_subcells_with_slope(&signal, &ctx, 0.0);
        // All outputs should be finite (no NaN/Inf)
        for (i, &r) in results.iter().enumerate() {
            assert!(r.is_finite(), "result[{i}] = {r} is not finite");
        }
    }
}

// <FILE>crates/tui-vfx-types/src/glyph/test_fnc_sample_eight_subcells.rs</FILE> - <DESC>TDD peer tests for sample_eight_subcells and sample_eight_subcells_with_slope helpers</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
