<!-- <FILE>docs/DESIGN_EXCELLENCE_USAGE_GUIDE.md</FILE> - <DESC>Research-backed guidance for when and how to use subtle polish effects</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Document usage guidance for newly added design-excellence shader families and adjacent subtle polish effects</WCTX> -->
<!-- <CLOG>NEW: Add guidance on deterministic core feedback, stochastic fringe delight, fatigue management, and recommended use cases for ConcealedLight, Diffusion, AffordanceWake, and WayfindingNode</CLOG> -->

# Design Excellence Usage Guide

This guide captures the research-backed recommendations for **when** subtle
polish effects should be used, **how often** they should appear, and where they
work best in a truecolor terminal UI.

The goal is not to add more motion for its own sake. The goal is to create
interfaces that feel more considered, premium, clear, and alive without becoming
noisy or fatiguing.

## Core principles

1. **Deterministic core, stochastic fringe**
   - Core feedback must be deterministic.
   - Peripheral delight may be probabilistic.

2. **Hierarchy by light, not by noise**
   - Prefer changes in depth, contrast, and restrained illumination over broad
     animation or repeated glyph changes.

3. **Structure should often be felt more than seen**
   - The best polish details often help people orient and trust the interface
     without demanding attention.

4. **The terminal is a cell grid**
   - Use shell-owned regions — borders, padding, margins, empty states, and
     chrome — as the primary canvas for richer effects.
   - Avoid overworking dense text cells.

5. **60 FPS is for continuity, not spectacle**
   - Use smooth frame cadence to make subtle transitions feel continuous.
   - Do not use it as a reason to animate everything.

## Frequency tiers

### Tier A — Always-on structural cues
Use for cues that must support orientation and hierarchy without drawing focus.

Examples:
- grade-underlying shadows
- subtle elevation tint
- contact darkening
- `ConcealedLight`

Guidance:
- Should feel calm under constant use.
- Should work even if frozen.

### Tier B — Contextual interaction cues
Use when an action, focus change, or navigation state should become clearer.

Examples:
- `AffordanceWake`
- focus rings
- hover indicators
- `WayfindingNode`

Guidance:
- Fire only when context makes them useful.
- Keep durations short and intensity low.

### Tier C — Occasional expressive cues
Use sparingly for meaningful moments, not frequent micro-actions.

Examples:
- rare completion twinkle
- seasonal nods
- milestone shimmer

Guidance:
- Never attach to every repeated click or row navigation.
- Rate-limit with cooldowns or milestone gating.

## New shader guidance

### ConcealedLight
Best for:
- panel headers/footers
- drawers and command palettes
- thresholds between interface regions
- sidebar and shell hierarchy

Avoid:
- dense text-only content
- celebratory or attention-grabbing moments

Default posture:
- static-first
- background-heavy application
- low intensity

### Diffusion
Best for:
- supportive/companion interfaces
- empty states
- calm loading/skeleton surfaces
- ambient shell surfaces with negative space

Avoid:
- strict cockpit-style precision views
- terse warnings and operational alerts
- large bright fields behind dense text

Default posture:
- background-focused
- very subtle drift only when the theme justifies it

### FocusField
Best for:
- active pane emphasis
- subtle hotspot guidance
- modal and command palette focus shaping
- drawers and inspectors that should feel current without glowing loudly

Avoid:
- turning every focus change into a theatrical spotlight
- large high-intensity fields behind dense copy
- using pulse by default in power-user-heavy flows

Default posture:
- background-focused
- rect/pane mode before hotspot mode in application UI
- static by default; pulse only when the theme strongly justifies it

### AffordanceWake
Best for:
- reveal-on-need secondary affordances
- corners, edges, and rails that should resolve into visibility
- focus/hover-adjacent shell cues

Avoid:
- replacing explicit hover/focus indicators everywhere
- high baseline intensity

Default posture:
- `rest_intensity = 0.0`
- progress-driven
- contextual only

### WayfindingNode
Best for:
- breadcrumbs
- progress steps
- node/junction emphasis
- onboarding or route hints

Avoid:
- turning routine application flows into animated signal-trace demos
- using motion where static current-position emphasis would be enough

Default posture:
- calm, local emphasis
- static current node first, optional low-amplitude pulse second

## Fatigue management

Repeated polish becomes annoying faster than teams expect.

Use one or more of:
- milestone gating
- cooldown windows
- rotating variants
- probabilistic optional delight
- reduced-motion suppression

Good candidates for occasional randomness:
- rare success sparkle
- small seasonal nod
- tiny ambient variation in non-critical surfaces

Bad candidates for randomness:
- focus state
- error confirmation
- destructive warnings
- orientation and wayfinding essentials

## Theme-level suggestions

- **Harbor / Rams / Stuttgart**
  - favor structural hierarchy and precise support cues
  - keep diffusion rare and restrained

- **Hygge / Grimoire / Eichler**
  - allow more warmth and ambient emotionality
  - still keep core navigation and task cues deterministic

## Final rule of thumb

If a user repeats an interaction dozens or hundreds of times an hour, the
effect should be:
- nearly instant,
- low amplitude,
- or occasionally withheld.

If the effect does not still make sense when motion is removed, it is probably
too dependent on animation to be a trustworthy default for terminal UI.

<!-- <FILE>docs/DESIGN_EXCELLENCE_USAGE_GUIDE.md</FILE> - <DESC>Research-backed guidance for when and how to use subtle polish effects</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
