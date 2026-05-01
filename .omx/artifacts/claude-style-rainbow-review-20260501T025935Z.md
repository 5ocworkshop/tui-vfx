# Claude review: style.rainbow compositor lowering

Started: 2026-05-01T02:59:35Z

## Claude response

## Hard blockers

**1. V2 oracle parity is broken across backends.** `crates/tui-vfx-player/src/fnc_apply_style_primitive.rs:359` still emits constant `rgb(0,255,254)` (cyan-teal) for `style.rainbow` — the V2 player oracle. The deleted compositor stub matched that constant. The new `RainbowCycleShader::style_at` (`crates/tui-vfx-style/src/models/cls_rainbow_cycle_shader.rs:33-36`) produces real hue rotation `((t * rotation_speed) * 360°).rem_euclid(360)`. Result: at `phase_t=0.5, rotation_speed=1.0` the new path lands near hue=180° (≈cyan), so the new test happens to coincide with the old stub. At any other `phase_t` (or any other `rotation_speed`), compositor-native and player-native now produce different colors for the **same V2-oracle recipe**. Either update the player path in lockstep, or treat this as a deliberate behavior change and rename/retire `debugStyleRainbowV2Oracle`.

**2. The new CLI test does not lock the contract.** `test_fnc_cli_native_style_rainbow_lowers_to_compositor_shader_json` only asserts:
- `foreground != "rgba(255,255,255,255)"`
- `background == "rgba(30,30,30,255)"`

The deleted constant-cyan stub would have passed both assertions. The test pins neither V2 parity nor the new hue formula. Pin the exact rgba at `phase_t=0.5` and a second sample at a different `phase_t` to demonstrate rotation, otherwise the migration is unverified.

## Correctness risks (not blockers)

**3. Silent semantic upgrade `number_input` → `resolved_number_input` for `rotationSpeed`.** Consistent with sibling lowerers (neon flicker, etc.), so likely intended, but it changes binding-resolution behavior for this field — call it out in the CLOG if it was intentional.

**4. f64 → `as f32` cast** on `rotation_speed` is fine for sane inputs but the field type changed (legacy `NativeStyleStage::Rainbow` carried `f64`); recipe authors who relied on f64 precision now silently truncate. Low risk given the formula.

## Layer boundaries / V3 hierarchy

- `VfxRainbowCycleShader` is a `pub type` alias of `RainbowCycleShader` — round-trip via `shader.clone()` is sound, naming `rainbow_cycle` is consistent with sibling composed primitives (`stripe_motion`, `modifier_window`).
- `StyleRegion::All` for `style.rainbow` is correct (V2 was global).
- No leftover `NativeStyleStage::Rainbow` references — deletion is clean.
- Composed-primitive enum, kind/description/parameter tables, and lowering round-trip test all updated symmetrically.

## Bottom line

Migration scaffolding is sound. The single substantive blocker is **cross-backend V2 parity** (#1) plus a **too-weak verification test** (#2) that hides it. Either tighten the test to pin exact rgba at multiple phase_t values *and* port the player-path stub to the same shader, or document that this recipe is no longer a V2 oracle.

Claude exit: 0 at 2026-05-01T03:00:58Z

## Follow-up actions after review

- Updated `tui-vfx-player` style.rainbow evidence path to use the same normalized `phase_t * rotationSpeed * 360°` foreground hue rotation as `RainbowCycleShader`, preserving background.
- Strengthened the CLI test to pin exact deterministic foreground colors at `phase_t=0.5` (`rgba(0,255,254,255)`) and `phase_t=0.25` (`rgba(127,255,0,255)`), so the deleted constant-cyan backend stub would no longer pass.
- Kept `resolved_number_input` intentionally for compositor lowering, matching the migration direction toward resolved graph values.
