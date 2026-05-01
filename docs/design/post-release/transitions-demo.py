# <FILE>docs/design/post-release/transitions-demo.py</FILE> - <DESC>Six classic transition primitives (crossfade, wipe, iris, push, dissolve/scatter, morph) demonstrated by cycling between two scenes (Day / Night), with six Penner easing functions selectable orthogonally. Both scenes share the same mountain silhouette so the eye can anchor across the transition while the sky gradient, sun-or-moon, and stars-or-empty switch out. Each transition is a single per-cell function over two precomputed half-cell buffers (A and B); compositing back into the output is therefore trivially parallel and cell-local, which is why these are cheap at 60 Hz even with full-screen redraws. Wipe and Push accept a direction parameter (left, right, up, down, diagonal). Iris reveals from a configurable focal point. Morph implements a radial pinch warp combined with crossfade — both buffers are sampled at distorted coordinates that bulge maximally at progress=0.5 then settle back. Easing curves are applied to the raw progress value before it reaches the transition function, so any easing × any transition combination works.</DESC>
# <VERS>VERSION: 0.1.0</VERS>
# <WCTX>Promote transitions to first-class demo content: cross-fade, wipe, iris, push, dissolve, morph as named primitives with knobs (duration, easing, direction), so the difference between linear and ease-in-out feel is visible side-by-side.</WCTX>
# <CLOG>0.1.0: new file; two scenes (sunset Day with sun + warm gradient, starry Night with twinkling stars + moon + cool gradient, both sharing the procedural mountain silhouette); six transition primitives keyed to 1-6, six easings cycled with 'e', four-direction parameter cycled with 'd', auto-cycling A→B→A→B with adjustable transition duration via +/-, manual trigger via space; deadline-aware frame loop; header surfaces transition type marker, easing marker, raw-and-eased progress with a stippled progress bar, direction, phase, and measured fps.</CLOG>

"""
Transitions demo — six classic primitives over two scenes.

────────────────────────────────────────────────────────────────────────
Run:   python3 transitions-demo.py
       python3 transitions-demo.py --transition iris --easing ease-out-back
       python3 transitions-demo.py --duration 1.5 --hold 2.5

Keys:  1..6     transition primitive
                  1 crossfade  · 2 wipe   · 3 iris
                  4 push       · 5 dissolve · 6 morph
       e        cycle easing (linear → in-out-quad → in-out-cubic
                              → in-out-sine → out-back → in-out-elastic)
       d        cycle direction for wipe / push
                  (left → right → up → down → diagonal)
       space    trigger transition NOW (skip the current hold)
       p        pause / resume motion (also pauses the auto-cycle)
       +/-      lengthen / shorten transition duration (250 ms steps)
       0        reset to defaults
       q        quit
────────────────────────────────────────────────────────────────────────

WHAT THIS DEMONSTRATES

The Hypercard / Director / Flash lineage promoted transitions to a
named primitive with knobs (duration, easing, direction). For TUI VFX
they are cheap at 60 Hz because each one is spatially local — a
crossfade or a wipe doesn't re-evaluate a complex shader, it interpolates
or selects between two precomputed buffers per cell.

This program treats each primitive as a pure per-cell function:

    transition(cell_a, cell_b, x, y, progress, params) → output_cell

`progress` is the eased value of (now - transition_start) / duration —
the easing is applied ONCE per frame to the raw progress and the result
is broadcast to every cell. Any easing × any primitive combination
therefore composes; press 'e' to cycle easings while a transition is
in flight and the change applies on the next frame.

THE SIX PRIMITIVES

  crossfade  Per-cell linear interpolation. Trivial; the baseline.
  wipe       Front travels across the screen at progress * width.
             Direction parameter selects the axis. Soft edge ~5% wide.
  iris       Circular reveal from focal point. As progress grows the
             circle radius grows. Same shape as Diablo's torch light,
             animated. Focal point is screen centre by default.
  push       A scrolls off, B scrolls on. Both buffers are sampled at
             shifted coordinates; coordinates outside [0, w) come from
             the other buffer. No fade — a hard mechanical slide.
  dissolve   Per-cell pre-shuffled timing. Each cell has its own
             threshold drawn from a deterministic noise grid; cells
             cross over when progress > their threshold. Looks like
             particles dispersing.
  morph      Radial pinch warp ⊕ crossfade. Both buffers are sampled
             at distorted (x', y') coordinates that bulge inward
             (A) and outward (B) maximally at progress=0.5. The
             samples are then crossfaded by progress. Reads as the
             two scenes pulling toward and through each other.

THE SIX EASINGS (Penner, 1999)

  linear              t
  ease-in-out-quad    quadratic accel + decel; gentle
  ease-in-out-cubic   cubic accel + decel; sharper start, smoother middle
  ease-in-out-sine    half-cosine; the most "natural" feel for free
  ease-out-back       slight overshoot at the end then settles back
  ease-in-out-elastic spring-bounce both ends; comedic, attention-getting

The header shows raw progress vs eased progress as two horizontal bars,
so the difference between linear and ease-in-out reads at a glance.
"""

import argparse
import math
import random
import select
import sys
import termios
import time
import tty

# ============================================================================
# Geometry
# ============================================================================

W, H        = 78, 18
SUB_W       = W * 2
HEADER_H    = 5    # rows reserved for header (transition / easing / progress / timing / blank)
SCENE_H     = H - HEADER_H

# ============================================================================
# Themes (per-scene colour palettes)
# ============================================================================

DAY = {
    "sky_top":     (110,  60,  90),    # warm purple
    "sky_horizon": (255, 160,  60),    # orange
    "sun":         (255, 240, 130),
    "sun_glow":    (255, 200,  90),
    "mountain":    ( 60,  35,  60),
    "ground":      ( 95,  75,  40),
    "ground_dark": ( 70,  55,  30),
}

NIGHT = {
    "sky_top":     (  6,   8,  30),
    "sky_horizon": ( 30,  35,  80),
    "moon":        (235, 240, 255),
    "moon_glow":   (180, 190, 230),
    "star_dim":    (180, 200, 230),
    "star_bright": (255, 255, 255),
    "mountain":    ( 12,  10,  35),
    "ground":      ( 18,  20,  50),
    "ground_dark": ( 10,  12,  35),
}

# ============================================================================
# Terminal escape codes
# ============================================================================

ESC         = "\x1b"
def fgbg(fc, bc): return f"{ESC}[38;2;{fc[0]};{fc[1]};{fc[2]};48;2;{bc[0]};{bc[1]};{bc[2]}m"
def bg(c):        return f"{ESC}[48;2;{c[0]};{c[1]};{c[2]}m"
HALF        = "▌"
RESET       = f"{ESC}[0m"
HIDE_CURSOR = f"{ESC}[?25l"
SHOW_CURSOR = f"{ESC}[?25h"
HOME        = f"{ESC}[H"
CLEAR       = f"{ESC}[2J"
CLEAR_EOL   = f"{ESC}[K"

# ============================================================================
# Shared world geometry — mountain silhouette is the same in both scenes
# ============================================================================

def mountain_top_row(sub_x):
    """Return the SCENE-relative row index where the mountain silhouette starts.

    The mountain occupies all rows >= mountain_top_row(sub_x) within the scene
    band (0..SCENE_H-1).
    """
    # Sum-of-sines silhouette in scene-row units (smaller = taller peak).
    h = 0.0
    h += 4.5 * math.sin(sub_x * 0.045 + 0.7)
    h += 2.0 * math.sin(sub_x * 0.090 + 2.1)
    h += 0.8 * math.cos(sub_x * 0.018 + 0.3)
    base_top = SCENE_H - 6.5  # ground occupies the bottom ~5 rows
    top_row  = int(round(base_top - h - 5.5))
    return max(1, min(SCENE_H - 4, top_row))

GROUND_ROW = SCENE_H - 4   # rows GROUND_ROW..SCENE_H-1 are flat ground

# ============================================================================
# Stars for Night scene — deterministic positions + per-star twinkle phase
# ============================================================================

def _build_stars(n=85):
    rng = random.Random(1337)
    stars = []
    for _ in range(n):
        sx = rng.randint(0, SUB_W - 1)
        sy = rng.randint(0, max(0, GROUND_ROW - 4))   # only in sky band, away from horizon
        phase  = rng.random() * 2 * math.pi
        period = 1.5 + rng.random() * 2.5     # 1.5 .. 4 seconds
        bright = rng.random()                 # 0..1, used to pick star_dim vs star_bright
        stars.append((sx, sy, phase, period, bright))
    return stars

STARS = _build_stars()

# ============================================================================
# Sky gradients — precompute one row per scene
# ============================================================================

def _sky_gradient(theme, scene_h):
    """Vertical interpolation top → horizon over the scene band."""
    z, h = theme["sky_top"], theme["sky_horizon"]
    rows = []
    span = max(1, scene_h - 1)
    for y in range(scene_h):
        t = y / span
        rows.append((
            int(z[0] + (h[0] - z[0]) * t),
            int(z[1] + (h[1] - z[1]) * t),
            int(z[2] + (h[2] - z[2]) * t),
        ))
    return rows

SKY_DAY   = _sky_gradient(DAY,   SCENE_H)
SKY_NIGHT = _sky_gradient(NIGHT, SCENE_H)

# ============================================================================
# Sun and Moon disc rasterisation
# ============================================================================

SUN_CX_SUB,  SUN_CY    = int(SUB_W * 0.78), 3        # right side of sky
SUN_R_SUB              = 7                            # in sub-x units (~3.5 cell-widths)

MOON_CX_SUB, MOON_CY   = int(SUB_W * 0.20), 3        # left side
MOON_R_SUB             = 6

def _disc_intensity(dx_sub, dy_row, r_sub):
    """Anti-aliased disc: 1 inside, 0 outside, smooth boundary.

    dx_sub is in sub-x units (0.5 cell-widths each); dy_row in row units
    (~2.2 cell-widths each). We aspect-correct dy by *2.2/0.5 = 4.4 to
    match sub-x.
    """
    dy_corrected = dy_row * 4.4
    d = math.sqrt(dx_sub * dx_sub + dy_corrected * dy_corrected)
    if d < r_sub - 1.0:
        return 1.0
    if d > r_sub + 0.5:
        return 0.0
    return max(0.0, min(1.0, (r_sub + 0.5 - d) / 1.5))

# ============================================================================
# Scene rendering — half-cell buffer (SCENE_H × SUB_W)
# ============================================================================

def _empty_buf():
    return [[(0, 0, 0)] * SUB_W for _ in range(SCENE_H)]

def render_day(now):
    """Render the Day scene buffer at wall-clock time `now`."""
    buf = _empty_buf()
    sun_pulse = 0.5 + 0.5 * math.sin(now * 0.7)   # 0..1, gentle breathing

    for y in range(SCENE_H):
        sky = SKY_DAY[y]
        m_color = DAY["mountain"]
        for sub_x in range(SUB_W):
            # Mountain silhouette test.
            top = mountain_top_row(sub_x)
            if y >= GROUND_ROW:
                # Ground band — soft horizontal stripes for visual reference.
                if int((sub_x // 6)) & 1:
                    buf[y][sub_x] = DAY["ground"]
                else:
                    buf[y][sub_x] = DAY["ground_dark"]
                continue
            if y >= top:
                buf[y][sub_x] = m_color
                continue
            # Sky cell — composite the sun on top.
            base = sky
            dxs = sub_x - SUN_CX_SUB
            dyr = y - SUN_CY
            sun_alpha = _disc_intensity(dxs, dyr, SUN_R_SUB)
            if sun_alpha > 0:
                glow = DAY["sun"] if sun_alpha > 0.6 else DAY["sun_glow"]
                # Pulse modulates brightness slightly toward sky on dim phase.
                a = sun_alpha * (0.85 + 0.15 * sun_pulse)
                buf[y][sub_x] = (
                    int(base[0] * (1 - a) + glow[0] * a),
                    int(base[1] * (1 - a) + glow[1] * a),
                    int(base[2] * (1 - a) + glow[2] * a),
                )
            else:
                buf[y][sub_x] = base
    return buf

def render_night(now):
    """Render the Night scene buffer at wall-clock time `now`."""
    buf = _empty_buf()

    # Sky + ground + mountains baseline.
    for y in range(SCENE_H):
        sky = SKY_NIGHT[y]
        m_color = NIGHT["mountain"]
        for sub_x in range(SUB_W):
            top = mountain_top_row(sub_x)
            if y >= GROUND_ROW:
                if int((sub_x // 6)) & 1:
                    buf[y][sub_x] = NIGHT["ground"]
                else:
                    buf[y][sub_x] = NIGHT["ground_dark"]
                continue
            if y >= top:
                buf[y][sub_x] = m_color
                continue
            buf[y][sub_x] = sky

    # Stars (sky band only — STARS are pre-filtered to be above GROUND_ROW).
    for sx, sy, phase, period, bright_seed in STARS:
        if sy >= mountain_top_row(sx):
            continue   # star occluded by mountain
        twinkle = 0.5 + 0.5 * math.sin(2 * math.pi * now / period + phase)
        col = NIGHT["star_bright"] if bright_seed > 0.65 else NIGHT["star_dim"]
        a = 0.5 + 0.5 * twinkle
        base = buf[sy][sx]
        buf[sy][sx] = (
            int(base[0] * (1 - a) + col[0] * a),
            int(base[1] * (1 - a) + col[1] * a),
            int(base[2] * (1 - a) + col[2] * a),
        )

    # Moon overlay (composite over sky cells; not over mountain).
    for y in range(min(GROUND_ROW, SCENE_H)):
        for sub_x in range(SUB_W):
            if y >= mountain_top_row(sub_x):
                continue
            dxs = sub_x - MOON_CX_SUB
            dyr = y - MOON_CY
            a = _disc_intensity(dxs, dyr, MOON_R_SUB)
            if a > 0:
                col = NIGHT["moon"] if a > 0.6 else NIGHT["moon_glow"]
                base = buf[y][sub_x]
                buf[y][sub_x] = (
                    int(base[0] * (1 - a) + col[0] * a),
                    int(base[1] * (1 - a) + col[1] * a),
                    int(base[2] * (1 - a) + col[2] * a),
                )
    return buf

# ============================================================================
# Easing functions (Penner)
# ============================================================================

def ease_linear(t): return t

def ease_in_out_quad(t):
    return 2 * t * t if t < 0.5 else 1 - 2 * (1 - t) ** 2

def ease_in_out_cubic(t):
    return 4 * t ** 3 if t < 0.5 else 1 - (-2 * t + 2) ** 3 / 2

def ease_in_out_sine(t):
    return 0.5 * (1 - math.cos(math.pi * t))

def ease_out_back(t):
    c1 = 1.70158; c3 = c1 + 1
    return 1 + c3 * (t - 1) ** 3 + c1 * (t - 1) ** 2

def ease_in_out_elastic(t):
    if t == 0: return 0.0
    if t == 1: return 1.0
    c5 = (2 * math.pi) / 4.5
    if t < 0.5:
        return -(2 ** ( 20 * t - 10) * math.sin((20 * t - 11.125) * c5)) / 2
    return  (2 ** (-20 * t + 10) * math.sin((20 * t - 11.125) * c5)) / 2 + 1

EASINGS = {
    "linear":              ease_linear,
    "in-out-quad":         ease_in_out_quad,
    "in-out-cubic":        ease_in_out_cubic,
    "in-out-sine":         ease_in_out_sine,
    "out-back":            ease_out_back,
    "in-out-elastic":      ease_in_out_elastic,
}

EASING_ORDER = ("linear", "in-out-quad", "in-out-cubic",
                "in-out-sine", "out-back", "in-out-elastic")

# ============================================================================
# Transition primitives — per-cell functions of (a, b, x, y, p)
# ============================================================================
# All work over the same SCENE_H × SUB_W half-cell buffers. `p` is the
# already-eased progress in [0, 1]. `params` carries direction / focal etc.

def _lerp(a, b, t):
    # Round-half-up via +0.5 before int(); without it, t = 0.999… gives
    # int(254.97) = 254 instead of 255 at the wipe / iris soft boundary.
    return (
        int(a[0] + (b[0] - a[0]) * t + 0.5),
        int(a[1] + (b[1] - a[1]) * t + 0.5),
        int(a[2] + (b[2] - a[2]) * t + 0.5),
    )

def _clamp01(v):
    return 0.0 if v < 0 else 1.0 if v > 1 else v

def trans_crossfade(a_buf, b_buf, p, params):
    out = _empty_buf()
    for y in range(SCENE_H):
        a_row, b_row, o_row = a_buf[y], b_buf[y], out[y]
        for x in range(SUB_W):
            o_row[x] = _lerp(a_row[x], b_row[x], p)
    return out

# Wipe directions express as a unit vector (dx, dy) and a softness band.
# We compute, per cell, "how far along the wipe axis is this cell?", and
# blend A→B inside the soft window around the wipe front.
WIPE_DIRECTIONS = {
    "left":     ( 1.0,  0.0),    # B reveals from the left
    "right":    (-1.0,  0.0),    # B reveals from the right
    "up":       ( 0.0,  1.0),    # B reveals from the top
    "down":     ( 0.0, -1.0),    # B reveals from the bottom
    "diagonal": ( 1.0,  1.0),    # B reveals from upper-left
}

def trans_wipe(a_buf, b_buf, p, params):
    direction = params.get("direction", "left")
    dx, dy    = WIPE_DIRECTIONS[direction]
    # Aspect-correct dy: a sub-x is 0.5 cw, a row is ~2.2 cw, so 1 row ≈ 4.4 sub-x.
    dy_eff    = dy * 4.4
    # Length of the projection axis through the rectangle.
    # (Worst-case max projection over corners minus min projection.)
    corners = ((0, 0), (SUB_W - 1, 0), (0, SCENE_H - 1), (SUB_W - 1, SCENE_H - 1))
    projs   = [c[0] * dx + c[1] * dy_eff for c in corners]
    p_min, p_max = min(projs), max(projs)
    span    = p_max - p_min
    softness = span * 0.06
    front    = p_min + p * (span + 2 * softness) - softness

    out = _empty_buf()
    for y in range(SCENE_H):
        a_row, b_row, o_row = a_buf[y], b_buf[y], out[y]
        for x in range(SUB_W):
            proj = x * dx + y * dy_eff
            local_p = _clamp01(0.5 + (front - proj) / (2 * softness))
            o_row[x] = _lerp(a_row[x], b_row[x], local_p)
    return out

def trans_iris(a_buf, b_buf, p, params):
    fx, fy = params.get("focal", (SUB_W / 2, SCENE_H / 2))
    # Aspect-correct y so the iris is a screen-circle.
    max_r = max(
        math.hypot(fx,                 (fy)              * 4.4),
        math.hypot(SUB_W - 1 - fx,     (fy)              * 4.4),
        math.hypot(fx,                 (SCENE_H - 1 - fy)* 4.4),
        math.hypot(SUB_W - 1 - fx,     (SCENE_H - 1 - fy)* 4.4),
    )
    softness = max_r * 0.06
    front    = -softness + p * (max_r + 2 * softness)

    out = _empty_buf()
    for y in range(SCENE_H):
        dy_eff = (y - fy) * 4.4
        a_row, b_row, o_row = a_buf[y], b_buf[y], out[y]
        for x in range(SUB_W):
            r = math.sqrt((x - fx) ** 2 + dy_eff * dy_eff)
            local_p = _clamp01(0.5 + (front - r) / (2 * softness))
            o_row[x] = _lerp(a_row[x], b_row[x], local_p)
    return out

PUSH_DIRECTIONS = {
    "left":  ( 1, 0),     # A pushes off to the left, B comes in from the right
    "right": (-1, 0),
    "up":    ( 0, 1),     # row units
    "down":  ( 0,-1),
    "diagonal": ( 1, 1),
}

def trans_push(a_buf, b_buf, p, params):
    direction = params.get("direction", "left")
    dx_dir, dy_dir = PUSH_DIRECTIONS[direction]
    shift_x = int(round(p * SUB_W   * dx_dir))
    shift_y = int(round(p * SCENE_H * dy_dir))

    out = _empty_buf()
    for y in range(SCENE_H):
        for x in range(SUB_W):
            # A is shifted by (shift_x, shift_y); the cell at output (x, y)
            # came from A position (x + shift_x, y + shift_y) — if in bounds,
            # else from B position (x + shift_x - SUB_W*sign, ...) etc.
            ax = x + shift_x
            ay = y + shift_y
            if 0 <= ax < SUB_W and 0 <= ay < SCENE_H:
                out[y][x] = a_buf[ay][ax]
            else:
                # B comes in from the opposite side, so its sample position
                # is offset by the WHOLE buffer in the move direction.
                bx = ax - SUB_W   * (1 if dx_dir > 0 else -1 if dx_dir < 0 else 0)
                by = ay - SCENE_H * (1 if dy_dir > 0 else -1 if dy_dir < 0 else 0)
                if 0 <= bx < SUB_W and 0 <= by < SCENE_H:
                    out[y][x] = b_buf[by][bx]
                else:
                    out[y][x] = (0, 0, 0)
    return out

# Pre-shuffled per-cell thresholds for dissolve.
def _build_dissolve_grid():
    rng = random.Random(2024)
    return [[rng.random() for _ in range(SUB_W)] for _ in range(SCENE_H)]

DISSOLVE_GRID = _build_dissolve_grid()

def trans_dissolve(a_buf, b_buf, p, params):
    softness = 0.06   # cells take 6% of total progress to cross over individually
    # Stretch p slightly past [0, 1] so cells with extreme thresholds (near 0
    # or near 1) still fully cross over by the global endpoints. Otherwise
    # ~40 cells remain mid-blend at p=0 and p=1.
    extended_p = -softness * 0.5 + p * (1.0 + softness)
    out = _empty_buf()
    for y in range(SCENE_H):
        a_row, b_row, o_row, t_row = a_buf[y], b_buf[y], out[y], DISSOLVE_GRID[y]
        for x in range(SUB_W):
            local_p = _clamp01((extended_p - t_row[x]) / softness + 0.5)
            o_row[x] = _lerp(a_row[x], b_row[x], local_p)
    return out

def _sample_nn(buf, x, y):
    ix = 0 if x < 0 else SUB_W - 1   if x >= SUB_W   else int(x)
    iy = 0 if y < 0 else SCENE_H - 1 if y >= SCENE_H else int(y)
    return buf[iy][ix]

def trans_morph(a_buf, b_buf, p, params):
    """Radial pinch warp + crossfade.

    Both buffers are sampled at distorted (x', y') coordinates whose
    radial offset from the screen centre is modulated by sin(πp). At
    p=0 and p=1 the warp vanishes; at p=0.5 it reaches maximum amplitude
    and the two scenes are 50/50 crossfaded — so the visible mid-frame
    is a maximally-distorted blend of both scenes.
    """
    cx_sub = SUB_W   / 2.0
    cy_row = SCENE_H / 2.0
    bulge  = math.sin(math.pi * p) * 0.45     # 0..0.45
    out = _empty_buf()
    for y in range(SCENE_H):
        dy_row = y - cy_row
        dy_eff = dy_row * 4.4
        for x in range(SUB_W):
            dx_sub = x - cx_sub
            r = math.sqrt(dx_sub * dx_sub + dy_eff * dy_eff)
            if r < 1e-6:
                a_sample = a_buf[y][x]
                b_sample = b_buf[y][x]
            else:
                # A pulls inward (smaller radius), B pushes outward.
                ux, uy = dx_sub / r, dy_eff / r
                a_r = r * (1 - bulge)
                b_r = r * (1 + bulge * 0.5)
                a_x = cx_sub + ux * a_r
                a_y = cy_row + uy * a_r / 4.4
                b_x = cx_sub + ux * b_r
                b_y = cy_row + uy * b_r / 4.4
                a_sample = _sample_nn(a_buf, a_x, a_y)
                b_sample = _sample_nn(b_buf, b_x, b_y)
            out[y][x] = _lerp(a_sample, b_sample, p)
    return out

TRANSITIONS = {
    "crossfade": trans_crossfade,
    "wipe":      trans_wipe,
    "iris":      trans_iris,
    "push":      trans_push,
    "dissolve":  trans_dissolve,
    "morph":     trans_morph,
}

TRANSITION_ORDER = ("crossfade", "wipe", "iris", "push", "dissolve", "morph")

# Direction is meaningful for wipe and push.
DIRECTION_ORDER = ("left", "right", "up", "down", "diagonal")

# ============================================================================
# Header rendering
# ============================================================================

def _progress_bar(value, width):
    """Stippled progress bar with a partial-block tip for sub-cell precision."""
    frac = max(0.0, min(1.0, value)) * width
    full = int(frac)
    parts = " ▏▎▍▌▋▊▉█"
    tip = parts[int((frac - full) * (len(parts) - 1) + 0.5)]
    cells = "█" * full + (tip if full < width else "")
    return cells.ljust(width, " ")

def _render_header(out, state):
    out.append("Transitions  —  1-6 type · e easing · d direction · space trigger · "
               "p pause · +/- duration · 0 reset · q quit\n")
    out.append("Type:    ")
    for i, name in enumerate(TRANSITION_ORDER, start=1):
        marker = "▶" if state['transition'] == name else " "
        out.append(f"{marker}{i}:{name:<10}")
    out.append("\n")

    out.append("Easing:  ")
    for name in EASING_ORDER:
        marker = "▶" if state['easing'] == name else " "
        out.append(f"{marker}{name:<14}")
    out.append("\n")

    raw = state['raw_progress']
    e   = state['eased_progress']
    bar_raw   = _progress_bar(raw, 20)
    bar_eased = _progress_bar(e,   20)
    direction = state['direction'] if state['transition'] in ("wipe", "push") else "—"
    phase = state['phase']
    out.append(f"Progress: raw   |{bar_raw}| {raw:5.3f}    direction: {direction:<8}  phase: {phase:<14}\n")
    fps_str = f"{state['ema_fps']:5.1f}" if state['ema_fps'] > 0 else "  -- "
    ms_str  = f"{state['ema_ms']:5.1f}" if state['ema_ms'] > 0 else "  -- "
    out.append(f"          eased |{bar_eased}| {e:5.3f}    duration:  {state['duration']:4.2f}s  "
               f"target {state['target_fps']} · {fps_str} fps · {ms_str} ms{CLEAR_EOL}\n")

# ============================================================================
# Buffer emission
# ============================================================================

def _emit_buf(out, buf):
    for row in buf:
        last_pair = None
        for x in range(0, SUB_W, 2):
            cl = row[x]
            cr = row[x + 1]
            if (cl, cr) != last_pair:
                out.append(fgbg(cl, cr))
                last_pair = (cl, cr)
            out.append(HALF)
        out.append(RESET + "\n")

def render(state, a_buf, b_buf):
    out = [HOME, RESET]
    _render_header(out, state)

    if state['phase'] == 'hold_a':
        scene = a_buf
    elif state['phase'] == 'hold_b':
        scene = b_buf
    elif state['phase'] in ('a_to_b', 'b_to_a'):
        # Resolve direction: A→B uses (a_buf, b_buf); B→A uses (b_buf, a_buf).
        from_buf, to_buf = (a_buf, b_buf) if state['phase'] == 'a_to_b' else (b_buf, a_buf)
        params = {
            'direction': state['direction'],
            'focal':     (SUB_W / 2.0, SCENE_H / 2.0),
        }
        scene = TRANSITIONS[state['transition']](from_buf, to_buf, state['eased_progress'], params)
    else:
        scene = a_buf  # safety net

    _emit_buf(out, scene)
    sys.stdout.write("".join(out))
    sys.stdout.flush()

# ============================================================================
# Phase machine
# ============================================================================

def _advance_phase(state, now):
    """When the current phase elapses, move to the next one and update timestamps."""
    if state['paused']:
        state['phase_start'] = now - state['phase_elapsed_at_pause']
        return

    elapsed = now - state['phase_start']
    if state['phase'] in ('hold_a', 'hold_b'):
        if elapsed >= state['hold']:
            state['phase']       = 'a_to_b' if state['phase'] == 'hold_a' else 'b_to_a'
            state['phase_start'] = now
    elif state['phase'] in ('a_to_b', 'b_to_a'):
        if elapsed >= state['duration']:
            state['phase']       = 'hold_b' if state['phase'] == 'a_to_b' else 'hold_a'
            state['phase_start'] = now

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
        description="Six-primitive transition demo over Day / Night scenes.")
    parser.add_argument("--transition", choices=TRANSITION_ORDER, default="crossfade",
                        help="initial transition primitive (default: crossfade).")
    parser.add_argument("--easing", choices=EASING_ORDER, default="in-out-cubic",
                        help="initial easing curve (default: in-out-cubic).")
    parser.add_argument("--direction", choices=DIRECTION_ORDER, default="left",
                        help="initial direction for wipe/push (default: left).")
    parser.add_argument("--duration", type=float, default=1.0, metavar="SEC",
                        help="transition duration in seconds (default: 1.0).")
    parser.add_argument("--hold", type=float, default=1.5, metavar="SEC",
                        help="hold time on each scene between transitions (default: 1.5).")
    parser.add_argument("--target-fps", type=int, default=60, metavar="FPS",
                        help="frame pacing target in Hz (default: 60).")
    return parser.parse_args()

# ============================================================================
# Main loop
# ============================================================================

def main():
    args = parse_args()

    state = {
        'transition':   args.transition,
        'easing':       args.easing,
        'direction':    args.direction,
        'duration':     args.duration,
        'hold':         args.hold,
        'paused':       False,

        'phase':        'hold_a',
        'phase_start':  time.perf_counter(),
        'phase_elapsed_at_pause': 0.0,

        'raw_progress':   0.0,
        'eased_progress': 0.0,

        'target_fps':   args.target_fps,
        'ema_ms':       0.0,
        'ema_fps':      0.0,
    }

    has_tty     = sys.stdin.isatty()
    fd          = sys.stdin.fileno() if has_tty else None
    old_termios = termios.tcgetattr(fd) if has_tty else None
    if has_tty:
        tty.setcbreak(fd)

    sys.stdout.write(HIDE_CURSOR + CLEAR)
    sys.stdout.flush()

    target_fps   = args.target_fps
    frame_period = 1.0 / target_fps
    next_deadline = time.perf_counter() + frame_period

    EMA_ALPHA  = 0.15
    last_tick  = time.perf_counter()

    try:
        while True:
            if has_tty:
                while True:
                    key = read_key()
                    if key is None:
                        break
                    if key in ("q", "Q"):
                        return
                    if key in ("1", "2", "3", "4", "5", "6"):
                        state['transition'] = TRANSITION_ORDER[int(key) - 1]
                        continue
                    if key in ("e", "E"):
                        i = EASING_ORDER.index(state['easing'])
                        state['easing'] = EASING_ORDER[(i + 1) % len(EASING_ORDER)]
                        continue
                    if key in ("d", "D"):
                        i = DIRECTION_ORDER.index(state['direction'])
                        state['direction'] = DIRECTION_ORDER[(i + 1) % len(DIRECTION_ORDER)]
                        continue
                    if key in (" ",):
                        # Trigger now: jump to the next transition phase from
                        # whichever hold we're in, or restart the current
                        # transition.
                        now = time.perf_counter()
                        if state['phase'] == 'hold_a':
                            state['phase'] = 'a_to_b'
                        elif state['phase'] == 'hold_b':
                            state['phase'] = 'b_to_a'
                        else:
                            # Already transitioning: restart it.
                            pass
                        state['phase_start'] = now
                        continue
                    if key in ("p", "P"):
                        # Pause toggles. When pausing we record elapsed; when
                        # resuming we shift phase_start so progress continues.
                        now = time.perf_counter()
                        if not state['paused']:
                            state['phase_elapsed_at_pause'] = now - state['phase_start']
                            state['paused'] = True
                        else:
                            state['phase_start'] = now - state['phase_elapsed_at_pause']
                            state['paused'] = False
                        continue
                    if key in ("+", "="):
                        state['duration'] = min(5.0, state['duration'] + 0.25)
                        continue
                    if key in ("-", "_"):
                        state['duration'] = max(0.10, state['duration'] - 0.25)
                        continue
                    if key == "0":
                        state['transition'] = args.transition
                        state['easing']     = args.easing
                        state['direction']  = args.direction
                        state['duration']   = args.duration
                        state['hold']       = args.hold
                        state['phase']      = 'hold_a'
                        state['phase_start']= time.perf_counter()
                        state['paused']     = False
                        continue

            now = time.perf_counter()
            _advance_phase(state, now)

            # Compute raw and eased progress within the current phase.
            elapsed = (now - state['phase_start']) if not state['paused'] \
                      else state['phase_elapsed_at_pause']
            if state['phase'] in ('hold_a', 'hold_b'):
                state['raw_progress']   = 0.0
                state['eased_progress'] = 0.0
            else:
                raw = max(0.0, min(1.0, elapsed / max(1e-6, state['duration'])))
                state['raw_progress']   = raw
                state['eased_progress'] = EASINGS[state['easing']](raw)

            # Render scenes.
            a_buf = render_day(now)
            b_buf = render_night(now)

            t0 = time.perf_counter()
            render(state, a_buf, b_buf)
            t1 = time.perf_counter()

            render_ms       = (t1 - t0) * 1000.0
            state['ema_ms'] = (state['ema_ms'] * (1 - EMA_ALPHA)
                               + render_ms * EMA_ALPHA) if state['ema_ms'] > 0 else render_ms

            now2      = time.perf_counter()
            sleep_for = next_deadline - now2
            if sleep_for > 0:
                time.sleep(sleep_for)
                next_deadline += frame_period
            else:
                if -sleep_for > frame_period:
                    next_deadline = time.perf_counter() + frame_period
                else:
                    next_deadline += frame_period

            tick     = time.perf_counter()
            interval = tick - last_tick
            if interval > 0:
                inst_fps         = 1.0 / interval
                state['ema_fps'] = (state['ema_fps'] * (1 - EMA_ALPHA)
                                    + inst_fps * EMA_ALPHA) if state['ema_fps'] > 0 else inst_fps
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

# <FILE>docs/design/post-release/transitions-demo.py</FILE>
# <VERS>END OF VERSION: 0.1.0</VERS>
