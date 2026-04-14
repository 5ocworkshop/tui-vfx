<!-- <FILE>docs/RECIPE_VISUAL_QA.md</FILE> - <DESC>Canonical visual QA checklist for the probe-validation recipe corpus</DESC> -->
<!-- <VERS>VERSION: 1.1.0</VERS> -->
<!-- <WCTX>Recipe visual QA guide aligned with the restored lighthouse motion model</WCTX> -->
<!-- <CLOG>MINOR: Update the lighthouse checklist to describe the one-way wrap beam behavior and the continuous offscreen half-cycle that now define the intended look</CLOG> -->

# Recipe Visual QA

This checklist is the canonical human-eye companion to the structured probe and
validator workflow. Use it during manual preview runs to confirm that the final
recipe **looks** the way the structured tooling says it behaves.

## alarm_lighthouse.json
- **Must see:** center bloom, stable readable `PANIC BEACON // HOLD`, a wide red beam that sweeps one direction, disappears for roughly half the loop, then re-enters from the opposite side like a rotating lighthouse lamp, plus urgent orange-red pulse and perimeter glow.
- **Bad sign:** border warps, sweep snaps back instead of wrapping, or the beam never spends any time offscreen.
- **Likely tuning knob:** `filter.dwell.motion_mode`, `filter.dwell.band_width`, `filter.dwell.bps`, `style.dwell_effect.frequency`.

## cathedral_of_static.json
- **Must see:** noisy old-screen entrance, stable `STATIC` title, CRT bend/jitter, mostly desaturated body, edge unease.
- **Bad sign:** title fragments, looks clean/digital, no flicker tension, no sense of broken broadcast hardware.
- **Likely tuning knob:** `sampler.*.scanline_strength`, `style.dwell_effect.frequency`, `filter.dwell.strength`.

## ghost_orchard.json
- **Must see:** gentle orchard whisper text reveal, tiny floating motes, cool silvery edge sheen, soft spectral color drift.
- **Bad sign:** dust is absent, text gets lost in the mask, sheen overpowers the orchard mood.
- **Likely tuning knob:** `filter.dwell.density`, `style.dwell_effect.hue_shift`, `style.spatial_shader.speed`.

## gravity_gospel.json
- **Must see:** center-out arrival, stable readable `ASCEND / DESCEND`, glowing underline meter above the floor, dusky devotional palette.
- **Bad sign:** underline overwrites the border, text truncates, mood is too playful instead of solemn.
- **Likely tuning knob:** `filter.dwell.progress`, `filter.dwell.row_offset`, `style.dwell_effect.frequency`.

## ketchup_oracle.json
- **Must see:** fast arrival, twitchy text on enter, convincing rigid-body shake during dwell, italic cadence synced to shake, quick metallic shine.
- **Bad sign:** shake feels mushy, italics do not sync with motion, first character sits visibly offset, or margins clip the rigid shake.
- **Likely tuning knob:** `filter.dwell.max_eighths`, `filter.dwell.damping`, `style.dwell_effect.shake_period`.

## midnight_switchboard.json
- **Must see:** spiral reveal, stable readable `PATCH DREAMS TO CH 9`, active underline one row above the floor, border sweep, pulsing current through text.
- **Bad sign:** underline lands on the border, border sweep invisible, no sense of electrical motion in the body.
- **Likely tuning knob:** `filter.dwell.progress`, `filter.dwell.row_offset`, `styles[1].spatial_shader.speed`.

## neon_quarantine.json
- **Must see:** redacted warning body, hazard-badge silhouette, jittery contamination vibe, chromatic fringing, controlled but unstable neon surface.
- **Bad sign:** reads as a normal button, no contamination energy, chromatic edge too subtle to register.
- **Likely tuning knob:** `filter.dwell.progress`, `style.dwell_effect.intensity`, `style.spatial_shader.intensity`.

## reef_of_receipts.json
- **Must see:** underwater slide-in, top-down surf reveal, suspended particulate shimmer, cool cyan halo, readable ledger line.
- **Bad sign:** text dissolves into noise, no aquatic shimmer, panel looks dry/static.
- **Likely tuning knob:** `filter.dwell.density`, `filter.dwell.drift`, `style.spatial_shader.pulse_speed`.

## velvet_faultline.json
- **Must see:** stylish morph into the final title, spiral entrance, runway underline one row above the floor, velvet-magenta luxury.
- **Bad sign:** morph feels invisible, underline lands on the border, glam palette collapses into muddy purple.
- **Likely tuning knob:** `filter.dwell.progress`, `filter.dwell.row_offset`, `style.dwell_effect.hue_shift`.

## wormhole_pageant.json
- **Must see:** iris reveal, stable `WORMHOLE PARADE` title, orbiting dots, diagonal synthetic sheen, loud celebratory color drift.
- **Bad sign:** orbit too faint, title gets mangled, whole piece feels flat instead of cosmic.
- **Likely tuning knob:** `style.spatial_shader.dot_count`, `style.spatial_shader.speed`, `filter.dwell.boost`.

<!-- <FILE>docs/RECIPE_VISUAL_QA.md</FILE> - <DESC>Canonical visual QA checklist for the probe-validation recipe corpus</DESC> -->
<!-- <VERS>END OF VERSION: 1.1.0</VERS> -->
