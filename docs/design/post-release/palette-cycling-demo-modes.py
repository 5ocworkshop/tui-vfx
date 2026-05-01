# <FILE>docs/design/post-release/palette-cycling-demo-modes.py</FILE> - <DESC>Unified palette-cycling demo combining the three resolution-ladder rungs (full-cell / horizontal half-block / quarter quadrant glyphs) into one program with eight pattern presets and live cell-density / direction / rotation-speed switching. Render mode is selectable via --mode {full,half,quarter} (default quarter); palette size via --palette-size {32,64,128,256} (default 128); frame target via --target-fps (default 60). Runtime keys: 1-8 pattern, m density, r reverse, +/- rotation step, q quit. Field generators take (sub_x, sub_y) cell-coordinate scale factors so on-screen pattern dimensions stay comparable across modes. Frame loop uses deadline-aware sleep that absorbs render cost into the next-frame deadline rather than accumulating drift, and the header shows measured render-ms / effective-fps so the user sees directly when pure-Python compute can't sustain the target. Sibling exemplar files palette-cycling-demo.py (v0.3, half-cell only) and palette-cycling-demo-quarter.py (v0.4, quarter-cell only) remain as pure single-mode references.</DESC>
# <VERS>VERSION: 0.2.1</VERS>
# <WCTX>Surface palette size as a live runtime control (the --palette-size CLI flag only sets the initial value), and extend the +/- speed control past step=1 into frame-skip territory so the cycle can be slowed past the previous N/FPS floor.</WCTX>
# <CLOG>0.2.1: add [/] runtime keys to cycle palette size through {32, 64, 128, 256} with a Palette-size selector line in the header; replace integer step_mag with a signed speed integer where positive values advance |speed| per frame (chunky/fast) and negative values advance once every -speed frames (smooth/slow), removing the previous step≥1 floor on minimum cycle period; swatch line shows speed multiplier and current cycle period in seconds.</CLOG>

"""
Amiga-demoscene palette-cycling demo — full / half / quarter cell modes.

────────────────────────────────────────────────────────────────────────
Run:   python3 palette-cycling-demo-modes.py
       python3 palette-cycling-demo-modes.py --mode full
       python3 palette-cycling-demo-modes.py --palette-size 256
       python3 palette-cycling-demo-modes.py --target-fps 30

Keys:  1..8  — switch pattern preset
       m     — cycle cell density (full → half → quarter → full …)
       [ ]   — cycle palette size (32 → 64 → 128 → 256 → 32 …)
       r     — reverse palette rotation direction
       +/-   — speed up / slow down rotation. Past step 1 the '-' key
               enters frame-skip mode (advance once every N frames),
               so cycle periods of tens of seconds are reachable.
       q     — quit (Ctrl-C also works)
────────────────────────────────────────────────────────────────────────

WHAT THIS DEMONSTRATES

The architectural payoff of palette cycling is that visible motion comes
entirely from rotating a lookup table — the field is computed once and
the palette is regenerated only on preset change. Per frame, only one
integer (`offset`) advances. The eye sees motion because each index
gets bound to a different colour, not because the geometry changed.

This program adds two more axes to explore:

  * how many samples we pack per terminal cell

        full     1 sample/cell      bg + space
        half     2 samples/cell     ▌  fg=left  half, bg=right half
        quarter  4 samples/cell     2×2 quadrant glyphs (▘▝▖▗▀▄▌▐▙▟▜▛▞▚)

  * how big the palette and rotation step are. The two compose:

        per-frame per-cell colour delta  ≈  (step / N) of palette range
        full rotation duration           =  N / (step × FPS) seconds

    Big N + small step at 60 fps gives silky continuous motion (each
    frame moves the colour by a fraction of a per-keyframe interval).
    Small N + big step gives the chunky "marching bands" demoscene
    look. The defaults (N=128, step=1, target=60 fps) sit in the
    smooth-and-slow corner; press `+` repeatedly to feel the chunky
    end of the spectrum without restarting.

PATTERN PRESETS

    1  rings    / water     concentric ripples around the centre
    2  plasma   / fire      classic four-sine demoscene plasma
    3  waves    / neon      diagonal stripes warped by sinusoidal modulation
    4  tunnel   / electric  log-spaced rings, perspective compression toward centre
    5  spiral   / galaxy    Archimedean three-arm spiral
    6  ripples  / water     interference of two radial sources
    7  marble   / fire      turbulence — sines whose phases are perturbed by other sines
    8  vortex   / twilight  angle-modulated radius, a swirling whirlpool

Quarter mode is lossy: a single cell carries one fg slot and one bg
slot, so four sub-cell colours are approximated each frame by the best
2-colour partition. For continuous fields with smoothly-interpolated
palettes the loss is visually mild — a one-sub-cell "dithered" edge at
palette band boundaries.

PERFORMANCE NOTE

This is a pure-Python proof of concept and reference logic for a Rust
implementation. Quarter mode (1404 cells × 4 lookups × 7 partition
evaluations + SGR string assembly per frame) may not sustain 60 fps in
CPython. The header surfaces measured render-ms / effective-fps so the
gap is visible. The frame loop uses a deadline-aware sleep that absorbs
slow frames into the next deadline rather than accumulating drift.
"""

import argparse
import math
import select
import sys
import termios
import time
import tty

# ============================================================================
# Tunables — wired through CLI flags below; constants are scene geometry only.
# ============================================================================

W, H        = 78, 18    # terminal size in cells
CELL_ASPECT = 2.2       # cells are roughly this many times taller than wide
# Palette size and target FPS come from --palette-size and --target-fps.
# Direction (±1) and step magnitude (≥1) are mutable runtime state in main().

# ============================================================================
# Render modes
# ============================================================================
# Each mode declares:
#   sub_w, sub_h   — sub-cell grid the field generator works on
#   sub_x, sub_y   — cell-widths per sub-step (passed to generators so on-screen
#                    pattern dimensions stay comparable across modes)
#   samples        — samples per terminal cell (1, 2, or 4)
#
# Palette size N is decoupled from spatial density and shared across modes
# (set once at startup via --palette-size). The original v0.1.x scheme tied N
# to spatial density, but at the smoothness-limited update rate matching N to
# the display refresh matters far more than matching it to spatial density.
#
# CELL_ASPECT correction is applied INSIDE generators by multiplying the
# y-component by sub_y * CELL_ASPECT, so e.g. quarter-mode rings stay circles
# (sub_y = 0.5 collapses 2.2 to 1.1, which is what we derived by hand for the
# v0.4 file).

MODES = {
    "full":    {"sub_w": W,     "sub_h": H,     "sub_x": 1.0, "sub_y": 1.0, "samples": 1},
    "half":    {"sub_w": W * 2, "sub_h": H,     "sub_x": 0.5, "sub_y": 1.0, "samples": 2},
    "quarter": {"sub_w": W * 2, "sub_h": H * 2, "sub_x": 0.5, "sub_y": 0.5, "samples": 4},
}

MODE_ORDER = ("full", "half", "quarter")  # cycle order for the 'm' key

PALETTE_SIZES = (32, 64, 128, 256)  # cycle order for the '[' / ']' keys

# ============================================================================
# Rotation speed
# ============================================================================
# A single signed integer encodes both rate AND mode of advance:
#
#   speed = +k  (k ≥ 1)  →  advance offset by k each frame   (chunky / fast)
#   speed = -k  (k ≥ 2)  →  advance offset by 1 every k frames (smooth / slow)
#
# The dead-zone {0, -1} is skipped: '+' from -2 jumps to +1, '-' from +1 jumps
# to -2, so the user never has to think about an off-state. Cycle period:
#
#   speed > 0:  cycle_seconds = N        / (speed * fps)
#   speed < 0:  cycle_seconds = N * (-speed) /  fps

MAX_SPEED = 8
MIN_SPEED = -120

def speed_to_motion(speed):
    """Return (step_mag, frames_per_advance) for a given speed setting."""
    return (speed, 1) if speed > 0 else (1, -speed)

def increase_speed(speed):
    if speed == -2:
        return +1
    return min(speed + 1, MAX_SPEED)

def decrease_speed(speed):
    if speed == +1:
        return -2
    return max(speed - 1, MIN_SPEED)

def speed_label(speed):
    return f"{speed}×" if speed > 0 else f"1/{-speed}×"

def cycle_seconds(speed, n, fps):
    step_mag, frames_per_advance = speed_to_motion(speed)
    return n * frames_per_advance / (step_mag * fps)

# ============================================================================
# Terminal escape codes
# ============================================================================

ESC         = "\x1b"
def bg(r, g, b):  return f"{ESC}[48;2;{r};{g};{b}m"
def fgbg(fc, bc): return f"{ESC}[38;2;{fc[0]};{fc[1]};{fc[2]};48;2;{bc[0]};{bc[1]};{bc[2]}m"
HALF        = "▌"
RESET       = f"{ESC}[0m"
HIDE_CURSOR = f"{ESC}[?25l"
SHOW_CURSOR = f"{ESC}[?25h"
HOME        = f"{ESC}[H"
CLEAR       = f"{ESC}[2J"
CLEAR_EOL   = f"{ESC}[K"

# ============================================================================
# Palette construction
# ============================================================================

def interpolate_palette(keyframes, n):
    """Cyclic interpolation between keyframe colours; palette[N-1] interpolates
    back toward palette[0] so rotation is seamless."""
    pal = []
    L = len(keyframes)
    for i in range(n):
        t  = (i / n) * L
        a  = int(t) % L
        b  = (a + 1) % L
        f  = t - int(t)
        ca = keyframes[a]
        cb = keyframes[b]
        pal.append(tuple(int(ca[k] + (cb[k] - ca[k]) * f) for k in range(3)))
    return pal

def water_palette(n):
    """Deep blue → cyan → bright cyan → back. Calm, oceanic."""
    return interpolate_palette(
        [(10, 40, 80), (40, 100, 180), (120, 200, 255), (40, 100, 180)], n)

def fire_palette(n):
    """Dark red → red → orange → yellow → back through orange/red. Hot."""
    return interpolate_palette(
        [(20, 0, 0), (200, 30, 0), (255, 140, 30), (255, 230, 100),
         (255, 140, 30), (200, 30, 0)], n)

def neon_palette(n):
    """Deep purple → magenta → pink → cyan loop. Synthwave."""
    return interpolate_palette(
        [(20, 0, 40), (180, 40, 255), (255, 80, 200), (40, 200, 255),
         (180, 40, 255)], n)

def electric_palette(n):
    """Deep blue → electric cyan → white → violet → back. High-voltage."""
    return interpolate_palette(
        [(0, 20, 60), (0, 120, 200), (180, 240, 255), (255, 255, 255),
         (200, 100, 255), (60, 0, 80), (0, 120, 200)], n)

def galaxy_palette(n):
    """Black → deep purple → blue → starlight white → back. Cosmos."""
    return interpolate_palette(
        [(5, 0, 20), (40, 20, 80), (100, 80, 180), (200, 200, 255),
         (255, 255, 255), (100, 80, 180), (40, 20, 80)], n)

def twilight_palette(n):
    """Indigo → lavender → orange dusk → magenta → back. Sunset."""
    return interpolate_palette(
        [(5, 5, 30), (60, 50, 100), (150, 100, 150), (255, 180, 100),
         (200, 80, 130), (60, 50, 100)], n)

# ============================================================================
# Index-field generators
# ============================================================================
# Each function takes (w, h, n, sub_x, sub_y) and returns a w×h list of ints
# in [0, n). w, h are sub-grid dimensions for the active mode; sub_x, sub_y
# are the cell-width contribution of one sub-step in each axis. Multiplying
# raw x and y by sub_x/sub_y converts to cell-coordinate space, so the same
# frequency/spacing constants give comparable on-screen results in all modes.

def rings_indices(w, h, n, sub_x, sub_y):
    """Concentric circles around the centre."""
    cx, cy = w / 2, h / 2
    field  = []
    for y in range(h):
        row = []
        for x in range(w):
            dx = (x - cx) * sub_x
            dy = (y - cy) * sub_y * CELL_ASPECT
            d  = math.sqrt(dx * dx + dy * dy)
            row.append(int(d / 1.4) % n)
        field.append(row)
    return field

def plasma_indices(w, h, n, sub_x, sub_y):
    """Classic demoscene plasma — sum of four sines at different scales."""
    field = []
    for y in range(h):
        row = []
        for x in range(w):
            xs = x * sub_x
            ys = y * sub_y
            v = (math.sin(xs * 0.30) +
                 math.sin(ys * 0.50) +
                 math.sin((xs + ys) * 0.20) +
                 math.sin(math.sqrt(xs * xs + ys * ys) * 0.25))
            v_norm = (v + 4) / 8
            row.append(int(v_norm * n) % n)
        field.append(row)
    return field

def waves_indices(w, h, n, sub_x, sub_y):
    """Diagonal stripes warped by perpendicular sinusoidal modulation."""
    field = []
    for y in range(h):
        row = []
        for x in range(w):
            xs = x * sub_x
            ys = y * sub_y
            v = ((xs + ys) * 0.4
                 + math.sin(xs * 0.3) * 2.5
                 + math.cos(ys * 0.4) * 1.5)
            row.append(int(v) % n)
        field.append(row)
    return field

def tunnel_indices(w, h, n, sub_x, sub_y):
    """Log-spaced rings — perspective compression toward the centre.

    With the palette rotating, the eye reads this as motion into (or out
    of, with direction = -1) a tunnel mouth. Bands grow exponentially
    wider toward the edges.
    """
    cx, cy = w / 2, h / 2
    field = []
    for y in range(h):
        row = []
        for x in range(w):
            dx = (x - cx) * sub_x
            dy = (y - cy) * sub_y * CELL_ASPECT
            d  = math.sqrt(dx * dx + dy * dy)
            row.append(int(math.log(d + 1.0) * (n * 0.5)) % n)
        field.append(row)
    return field

def spiral_indices(w, h, n, sub_x, sub_y):
    """Three-arm Archimedean spiral. r + k·θ; arms appear to rotate as the
    palette cycles."""
    cx, cy = w / 2, h / 2
    field = []
    arms = 3
    for y in range(h):
        row = []
        for x in range(w):
            dx = (x - cx) * sub_x
            dy = (y - cy) * sub_y * CELL_ASPECT
            r  = math.sqrt(dx * dx + dy * dy)
            theta = math.atan2(dy, dx)
            v = r * 0.7 + theta * (n / (2 * math.pi)) * arms
            row.append(int(v) % n)
        field.append(row)
    return field

def ripples_indices(w, h, n, sub_x, sub_y):
    """Interference of two radial sources — overlapping pond ripples."""
    s1x, s1y = w * 0.30, h * 0.40
    s2x, s2y = w * 0.70, h * 0.60
    field = []
    for y in range(h):
        row = []
        for x in range(w):
            d1x = (x - s1x) * sub_x
            d1y = (y - s1y) * sub_y * CELL_ASPECT
            d2x = (x - s2x) * sub_x
            d2y = (y - s2y) * sub_y * CELL_ASPECT
            d1 = math.sqrt(d1x * d1x + d1y * d1y)
            d2 = math.sqrt(d2x * d2x + d2y * d2y)
            row.append(int((d1 + d2) / 1.4) % n)
        field.append(row)
    return field

def marble_indices(w, h, n, sub_x, sub_y):
    """Turbulence — sines whose phases are perturbed by other sines.

    Each axis's sine is offset by a sine of the other axis, giving an
    irregular swirly pattern reminiscent of marble grain or oil-on-water.
    Cheaper than real Perlin noise and palette-cycle-friendly.
    """
    field = []
    for y in range(h):
        row = []
        for x in range(w):
            xs = x * sub_x
            ys = y * sub_y
            v = (math.sin(xs * 0.14 + math.sin(ys * 0.10) * 3.0) * 5.0
                 + math.sin(ys * 0.18 + math.sin(xs * 0.12) * 3.0) * 5.0
                 + math.sin((xs + ys) * 0.10) * 3.0
                 + math.sin((xs - ys) * 0.08) * 3.0)
            v_norm = (v + 16) / 32
            row.append(int(v_norm * n) % n)
        field.append(row)
    return field

def vortex_indices(w, h, n, sub_x, sub_y):
    """Angle-modulated radius — a swirling whirlpool. Five lobes."""
    cx, cy = w / 2, h / 2
    field = []
    arms = 5
    for y in range(h):
        row = []
        for x in range(w):
            dx = (x - cx) * sub_x
            dy = (y - cy) * sub_y * CELL_ASPECT
            r  = math.sqrt(dx * dx + dy * dy)
            theta = math.atan2(dy, dx)
            v = r + math.sin(theta * arms + r * 0.15) * 4.0
            row.append(int(v) % n)
        field.append(row)
    return field

# ============================================================================
# Presets — bundles of (name, palette-name, pattern, palette)
# ============================================================================

PRESETS = [
    {"name": "rings",   "pal": "water",    "indices": rings_indices,    "palette": water_palette},
    {"name": "plasma",  "pal": "fire",     "indices": plasma_indices,   "palette": fire_palette},
    {"name": "waves",   "pal": "neon",     "indices": waves_indices,    "palette": neon_palette},
    {"name": "tunnel",  "pal": "electric", "indices": tunnel_indices,   "palette": electric_palette},
    {"name": "spiral",  "pal": "galaxy",   "indices": spiral_indices,   "palette": galaxy_palette},
    {"name": "ripples", "pal": "water",    "indices": ripples_indices,  "palette": water_palette},
    {"name": "marble",  "pal": "fire",     "indices": marble_indices,   "palette": fire_palette},
    {"name": "vortex",  "pal": "twilight", "indices": vortex_indices,   "palette": twilight_palette},
]

# ============================================================================
# Quarter-cell partition selection (only used when mode == "quarter")
# ============================================================================
# Given the four quadrant colours (UL, UR, LL, LR), choose the bipartition
# of the four quadrants into a "fg side" and a "bg side" that minimises the
# sum of squared deviations of each quadrant from its side's mean colour.
# Return the matching Block-Elements glyph plus the (fg, bg) cluster means.
#
# Seven canonical non-trivial bipartitions (others are these with fg/bg
# swapped — same cost, complement glyph):
#
#   1-vs-3:  {UL}    ▘ , {UR}    ▝ , {LL}    ▖ , {LR}    ▗
#   2-vs-2:  {UL,UR} ▀ , {UL,LL} ▌ , {UL,LR} ▚    (top, left, anti-diag)

def _mean3(a, b, c):
    return ((a[0] + b[0] + c[0]) // 3,
            (a[1] + b[1] + c[1]) // 3,
            (a[2] + b[2] + c[2]) // 3)

def _mean2(a, b):
    return ((a[0] + b[0]) >> 1,
            (a[1] + b[1]) >> 1,
            (a[2] + b[2]) >> 1)

def _sqd(a, b):
    da = a[0] - b[0]; db = a[1] - b[1]; dc = a[2] - b[2]
    return da * da + db * db + dc * dc

def choose_quadrant(c_ul, c_ur, c_ll, c_lr):
    """Pick the best 2-colour partition; return (glyph, fg_mean, bg_mean)."""
    quads = (c_ul, c_ur, c_ll, c_lr)

    best_cost  = None
    best_glyph = "█"
    best_fg    = c_ul
    best_bg    = c_ul

    for i, glyph in enumerate(("▘", "▝", "▖", "▗")):
        others  = [quads[j] for j in range(4) if j != i]
        bg_mean = _mean3(*others)
        cost    = (_sqd(others[0], bg_mean)
                 + _sqd(others[1], bg_mean)
                 + _sqd(others[2], bg_mean))
        if best_cost is None or cost < best_cost:
            best_cost, best_glyph, best_fg, best_bg = cost, glyph, quads[i], bg_mean

    for (a, b, c, d), glyph in (
        ((0, 1, 2, 3), "▀"),
        ((0, 2, 1, 3), "▌"),
        ((0, 3, 1, 2), "▚"),
    ):
        fg_mean = _mean2(quads[a], quads[b])
        bg_mean = _mean2(quads[c], quads[d])
        cost    = (_sqd(quads[a], fg_mean) + _sqd(quads[b], fg_mean)
                 + _sqd(quads[c], bg_mean) + _sqd(quads[d], bg_mean))
        if cost < best_cost:
            best_cost, best_glyph, best_fg, best_bg = cost, glyph, fg_mean, bg_mean

    return best_glyph, best_fg, best_bg

# ============================================================================
# State cache — (preset_idx, mode_name) → (field, palette)
# ============================================================================
# N is fixed for the run (set by --palette-size at startup), so it doesn't
# need to be in the cache key. Caching makes mode/preset switch-back instant.

_state_cache = {}

def get_state(preset_idx, mode_name, n):
    key = (preset_idx, mode_name)
    if key not in _state_cache:
        params  = MODES[mode_name]
        preset  = PRESETS[preset_idx]
        palette = preset["palette"](n)
        field   = preset["indices"](
            params["sub_w"], params["sub_h"], n,
            params["sub_x"], params["sub_y"])
        _state_cache[key] = (field, palette)
    return _state_cache[key]

# ============================================================================
# Rendering
# ============================================================================
# Three rendering paths share a common header + swatch and differ only in how
# they walk the field and emit cell glyphs. Coalescing key:
#   full     last bg colour
#   half     last (fg, bg) pair
#   quarter  last (glyph, fg, bg) triple

def _render_header(out, mode_name, preset_idx, n, target_fps,
                   measured_ms, measured_fps):
    params = MODES[mode_name]
    out.append("Amiga palette-cycling demo  —  1-8 pattern · m density · [/] palette · "
               "+/- speed · r reverse · q quit\n")
    out.append("Index field is STATIC; only the palette rotates per frame.\n")

    out.append("Mode:    ")
    for m in MODE_ORDER:
        marker = "▶" if m == mode_name else " "
        out.append(f"{marker}{m:<8}")
    samples = params["samples"]
    out.append(f"  ({samples} sample{'s' if samples > 1 else ' '}/cell)\n")

    out.append("Palette: ")
    for size in PALETTE_SIZES:
        marker = "▶" if size == n else " "
        out.append(f"{marker}{size:<5}")
    out.append(f" (entries; step '[' / ']' to cycle)\n")

    fps_str = f"{measured_fps:5.1f}" if measured_fps > 0 else "  -- "
    ms_str  = f"{measured_ms:5.1f}" if measured_ms > 0 else "  -- "
    out.append(f"Timing:  target {target_fps} fps · measured {fps_str} fps "
               f"· render {ms_str} ms/frame{CLEAR_EOL}\n")

    out.append("Pattern: ")
    for i, p in enumerate(PRESETS):
        if i == 4:
            out.append("\n         ")
        marker = "▶" if i == preset_idx else " "
        out.append(f"{marker}{i + 1}:{p['name']:<7}/{p['pal']:<8} ")
    out.append("\n\n")

def _render_swatch(out, palette, offset, direction, speed, target_fps):
    n = len(palette)
    arrow   = "→" if direction > 0 else "←"
    cyc_s   = cycle_seconds(speed, n, target_fps)
    label   = f"{arrow} {speed_label(speed)} · {cyc_s:5.2f}s/cycle"
    out.append(f"Palette {label}{CLEAR_EOL}\n         ")
    if n <= 32:
        for i in range(n):
            c = palette[(i + offset) % n]
            out.append(bg(*c) + "  ")
    else:
        for i in range(0, n, 2):
            cl = palette[(i     + offset) % n]
            cr = palette[(i + 1 + offset) % n]
            out.append(fgbg(cl, cr) + HALF)
    out.append(RESET + "\n\n")

def _render_full(out, field, palette, offset):
    n = len(palette)
    for row in field:
        last = None
        for idx in row:
            c = palette[(idx + offset) % n]
            if c != last:
                out.append(bg(*c))
                last = c
            out.append(" ")
        out.append(RESET + "\n")

def _render_half(out, field, palette, offset):
    n = len(palette)
    for row in field:
        last_pair = None
        for x in range(0, len(row), 2):
            cl = palette[(row[x]     + offset) % n]
            cr = palette[(row[x + 1] + offset) % n]
            if (cl, cr) != last_pair:
                out.append(fgbg(cl, cr))
                last_pair = (cl, cr)
            out.append(HALF)
        out.append(RESET + "\n")

def _render_quarter(out, field, palette, offset):
    n = len(palette)
    sub_h = len(field)
    sub_w = len(field[0])
    for cy in range(0, sub_h, 2):
        row_top = field[cy]
        row_bot = field[cy + 1]
        last = None
        for cx in range(0, sub_w, 2):
            c_ul = palette[(row_top[cx]     + offset) % n]
            c_ur = palette[(row_top[cx + 1] + offset) % n]
            c_ll = palette[(row_bot[cx]     + offset) % n]
            c_lr = palette[(row_bot[cx + 1] + offset) % n]
            glyph, fg_, bg_ = choose_quadrant(c_ul, c_ur, c_ll, c_lr)
            triple = (glyph, fg_, bg_)
            if triple != last:
                out.append(fgbg(fg_, bg_))
                last = triple
            out.append(glyph)
        out.append(RESET + "\n")

RENDERERS = {
    "full":    _render_full,
    "half":    _render_half,
    "quarter": _render_quarter,
}

def render(mode_name, field, palette, offset, preset_idx, direction, n,
           speed, target_fps, measured_ms, measured_fps):
    out = [HOME, RESET]
    _render_header(out, mode_name, preset_idx, n, target_fps,
                   measured_ms, measured_fps)
    _render_swatch(out, palette, offset, direction, speed, target_fps)
    RENDERERS[mode_name](out, field, palette, offset)
    sys.stdout.write("".join(out))
    sys.stdout.flush()

# ============================================================================
# Non-blocking single-key input
# ============================================================================

def read_key():
    if select.select([sys.stdin], [], [], 0)[0]:
        return sys.stdin.read(1)
    return None

# ============================================================================
# CLI
# ============================================================================

def parse_args():
    parser = argparse.ArgumentParser(
        description="Amiga palette-cycling demo with full / half / quarter cell modes.")
    parser.add_argument(
        "--mode", choices=list(MODE_ORDER), default="quarter",
        help="initial cell density (default: quarter). Toggle at runtime with 'm'.")
    parser.add_argument(
        "--palette-size", type=int, choices=[32, 64, 128, 256], default=128,
        metavar="N",
        help="palette entries (default: 128). Bigger N = smoother per-cell rotation, "
             "slower full cycle at fixed step. Choices: 32, 64, 128, 256.")
    parser.add_argument(
        "--target-fps", type=int, default=60, metavar="FPS",
        help="frame pacing target in Hz (default: 60). Pure-Python quarter mode may "
             "not sustain 60; check the measured fps in the header.")
    return parser.parse_args()

# ============================================================================
# Main loop
# ============================================================================

def main():
    args        = parse_args()
    mode_name   = args.mode
    n           = args.palette_size
    target_fps  = args.target_fps
    preset_idx  = 0
    field, palette = get_state(preset_idx, mode_name, n)

    has_tty     = sys.stdin.isatty()
    fd          = sys.stdin.fileno() if has_tty else None
    old_termios = termios.tcgetattr(fd) if has_tty else None
    if has_tty:
        tty.setcbreak(fd)

    sys.stdout.write(HIDE_CURSOR + CLEAR)
    sys.stdout.flush()

    offset       = 0
    direction    = +1
    speed        = +1               # see speed_to_motion() for the encoding
    frame_count  = 0                # used only when speed < 0 (frame-skip mode)
    frame_period = 1.0 / target_fps
    next_deadline = time.perf_counter() + frame_period

    # Effective-fps EMA. Measured render-ms is per-frame, EMA over a few
    # frames to stop the displayed values from twitching.
    ema_render_ms = 0.0
    ema_fps       = 0.0
    EMA_ALPHA     = 0.15
    last_tick     = time.perf_counter()

    try:
        while True:
            if has_tty:
                while True:
                    key = read_key()
                    if key is None:
                        break
                    if key in ("q", "Q"):
                        return
                    if key in ("m", "M"):
                        i = MODE_ORDER.index(mode_name)
                        mode_name = MODE_ORDER[(i + 1) % len(MODE_ORDER)]
                        field, palette = get_state(preset_idx, mode_name, n)
                        sys.stdout.write(CLEAR)
                        continue
                    if key in ("[", "{", "]", "}"):
                        # Cycle palette size. Preserve visible rotation phase
                        # by linearly rescaling offset to the new modulus.
                        old_n = n
                        i = PALETTE_SIZES.index(n)
                        if key in ("[", "{"):
                            n = PALETTE_SIZES[(i - 1) % len(PALETTE_SIZES)]
                        else:
                            n = PALETTE_SIZES[(i + 1) % len(PALETTE_SIZES)]
                        field, palette = get_state(preset_idx, mode_name, n)
                        offset = (offset * n) // old_n
                        sys.stdout.write(CLEAR)
                        continue
                    if key in ("r", "R"):
                        direction = -direction
                        continue
                    if key in ("+", "="):
                        speed = increase_speed(speed)
                        frame_count = 0
                        continue
                    if key in ("-", "_"):
                        speed = decrease_speed(speed)
                        frame_count = 0
                        continue
                    if key.isdigit():
                        idx = int(key) - 1
                        if 0 <= idx < len(PRESETS):
                            preset_idx = idx
                            field, palette = get_state(preset_idx, mode_name, n)
                            sys.stdout.write(CLEAR)

            t0 = time.perf_counter()
            render(mode_name, field, palette, offset, preset_idx, direction,
                   n, speed, target_fps, ema_render_ms, ema_fps)
            t1 = time.perf_counter()

            render_ms     = (t1 - t0) * 1000.0
            ema_render_ms = (ema_render_ms * (1 - EMA_ALPHA)
                             + render_ms * EMA_ALPHA) if ema_render_ms > 0 else render_ms

            step_mag, frames_per_advance = speed_to_motion(speed)
            frame_count += 1
            if frame_count >= frames_per_advance:
                offset = (offset + direction * step_mag) % len(palette)
                frame_count = 0

            # Deadline-aware sleep: sleep until next_deadline, but if we
            # missed it (render ran longer than frame_period) skip the
            # catch-up sleep and reset the deadline so drift doesn't
            # accumulate. Reset when more than one frame behind.
            now = time.perf_counter()
            sleep_for = next_deadline - now
            if sleep_for > 0:
                time.sleep(sleep_for)
                next_deadline += frame_period
            else:
                # Behind schedule. If only slightly, advance the deadline
                # to recover; if very behind, reset to "now + period" so a
                # one-time stall doesn't cause sustained catch-up bursts.
                if -sleep_for > frame_period:
                    next_deadline = time.perf_counter() + frame_period
                else:
                    next_deadline += frame_period

            tick    = time.perf_counter()
            interval = tick - last_tick
            if interval > 0:
                inst_fps = 1.0 / interval
                ema_fps  = (ema_fps * (1 - EMA_ALPHA)
                            + inst_fps * EMA_ALPHA) if ema_fps > 0 else inst_fps
            last_tick = tick

    except KeyboardInterrupt:
        pass
    finally:
        sys.stdout.write(SHOW_CURSOR + RESET + "\n")
        sys.stdout.flush()
        if has_tty and old_termios is not None:
            termios.tcsetattr(fd, termios.TCSADRAIN, old_termios)

if __name__ == "__main__":
    main()

# <FILE>docs/design/post-release/palette-cycling-demo-modes.py</FILE>
# <VERS>END OF VERSION: 0.2.1</VERS>
