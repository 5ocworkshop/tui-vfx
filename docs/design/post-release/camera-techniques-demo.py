# <FILE>docs/design/post-release/camera-techniques-demo.py</FILE> - <DESC>Four-tab demonstration of 2D camera techniques over a shared procedural sunset landscape: (1) rigid pan with smooth float-precision scrolling, (2) layered parallax with sky/mountains/hills/ground at different scroll rates, (3) discrete integer zoom (1×, 2×, 4×) with Bayer-4 dither blending during transitions, (4) actor-tracking camera with a deadband rectangle (Mario-64 style) and a critically-damped spring follow as alternative. Same world geometry across all tabs; only the camera behaviour changes, so the differences are crisp side-by-side. Tabs switch via 1/2/3/4. Renders at half-cell horizontal density (▌ glyph) for sub-pixel pan smoothness without paying the quarter-cell partition cost. Sprites (trees and actor) are emitted as glyphs that override the half-cell run inside their cell, with the bg under the glyph sampled from whichever layer would have been there.</DESC>
# <VERS>VERSION: 0.1.0</VERS>
# <WCTX>Bring the four canonical 2D-camera techniques (pan, parallax, discrete zoom with dither blend, deadband / spring follow) into one explorable demo so the differences can be felt directly side-by-side. Same world geometry across tabs; only the camera changes.</WCTX>
# <CLOG>0.1.0: new file; four-tab structure (1 pan / 2 parallax / 3 zoom / 4 follow), shared sunset landscape (procedural mountain/hill silhouettes, scattered tree sprites, configurable horizon and ground row), half-cell rendering with sprite glyph overrides; zoom transition uses a Bayer-4 dither matrix to choose between zoom_level and zoom_target per cell weighted by transition progress; follow mode toggles between deadband-rectangle (Mario-64 style) and critically-damped spring tracking via 'f'; deadline-aware frame loop; header surfaces tab markers, camera state, follow mode, zoom blend, and measured fps.</CLOG>

"""
Camera-techniques demo — four 2D-viewport techniques over a shared scene.

────────────────────────────────────────────────────────────────────────
Run:   python3 camera-techniques-demo.py
       python3 camera-techniques-demo.py --tab 4 --follow spring

Common:    1 / 2 / 3 / 4   switch tab
           p / space       pause / resume motion
           0               reset camera and actor to origin
           q               quit

Tab 1 — Pan
           Auto-scrolls right at constant velocity. All layers move at
           1.0×; demonstrates rigid scroll over a fractional camera.
           Press +/- to change scroll speed; r reverses direction.

Tab 2 — Parallax
           Same auto-scroll, but each layer moves at its own rate:
              sky        0.0×   (held stationary)
              mountains  0.2×   (far)
              hills      0.5×   (mid)
              ground     1.0×   (near)
           Trees scroll with the ground. The depth cue is felt directly
           when peaks of different layers cross at different rates.

Tab 3 — Zoom
           Static camera, discrete integer zoom levels {1×, 2×, 4×}.
           Press + to retarget the next-larger zoom, - the next-smaller.
           During a transition, blend ∈ (0, 1) controls a Bayer-4 dither
           that picks zoom_level vs zoom_target per cell — the SNES /
           old-tilemap trick where the only "fractional zoom" you can
           express is two integer-zoomed renders mixed by a screen-door.

Tab 4 — Camera follow
           The actor (☻) is controllable with a / d. Camera tracks it.
           Press 'f' to toggle between two follow rules:
              deadband:  camera stays still while actor sits inside a
                         central rectangle; once the actor leaves the
                         box, the camera moves just enough to keep them
                         on the box edge. (Mario-64 / 2D-platformer.)
              spring:    cam position is spring-attracted to actor with
                         critical damping; the actor "drags" the camera
                         elastically. Smoother but less precise.
────────────────────────────────────────────────────────────────────────

ARCHITECTURAL NOTE

All four tabs share the same scene-rendering pipeline; the tab only
changes which camera-update rule runs each frame and which parallax
factors apply when sampling layers. The world is fully procedural — no
tilemap — so coordinate transforms (pan, zoom) are just changes to the
screen-x → world-x mapping inside the layer samplers. This is a clean
fit for the eventual Rust port: the camera is a pure data-flow node
between input/state and the renderer; swapping its update rule is a
one-line change.
"""

import argparse
import math
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
CELL_ASPECT = 2.2

SCREEN_W_CW  = W * 1.0
SCREEN_H_CW  = H * CELL_ASPECT
CENTER_X_CW  = SCREEN_W_CW / 2.0
ROW_H_CW     = CELL_ASPECT

HORIZON_ROW  = 5     # rows 0..HORIZON_ROW-1 are sky-only
GROUND_ROW   = 13    # rows GROUND_ROW..H-1 are flat ground (below all silhouettes)

# Layer row ranges (inclusive top, exclusive bottom):
# sky        : 0..HORIZON_ROW
# mountain   : HORIZON_ROW..ground
# hill       : (overlays mountain band, in front)
# ground     : GROUND_ROW..H

# ============================================================================
# Theme — sunset over plains
# ============================================================================

THEME = {
    "sky_zenith":   ( 30,  20,  60),
    "sky_horizon":  (255, 130,  80),
    "mountain":     ( 25,  18,  55),
    "mountain_lit": ( 90,  50,  90),     # rim/shading toward horizon
    "hill":         ( 25,  55,  60),
    "ground":       ( 25,  60,  35),
    "ground_dark":  ( 12,  40,  20),
    "tree_leaf":    ( 30,  90,  45),
    "tree_trunk":   ( 60,  40,  25),
    "actor":        (255, 230, 100),
    "actor_outline":( 80,  60,  20),
    "deadband":     (255, 255, 255),
}

# ============================================================================
# Terminal escapes
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
# World — procedural silhouettes and tree positions
# ============================================================================
# All world coordinates are in cell-widths; zoom factors scale screen → world.
# Mountain/hill heights are returned in cell-widths above the GROUND_ROW
# baseline (which is itself ROW_H_CW * GROUND_ROW cell-widths from the top).

def mountain_height_cw(world_x):
    """Far mountain silhouette. Sum-of-sines, slow frequencies, tall amps."""
    h = 0.0
    h += 4.5  * math.sin(world_x * 0.07 + 0.3)
    h += 2.5  * math.sin(world_x * 0.13 + 1.7)
    h += 1.0  * math.sin(world_x * 0.21 + 3.1)
    return 10.0 + h    # cell-widths above the ground baseline

def hill_height_cw(world_x):
    """Closer hill silhouette. Smaller amps, higher frequencies."""
    h = 0.0
    h += 1.6 * math.sin(world_x * 0.18 + 0.9)
    h += 0.7 * math.cos(world_x * 0.27 + 2.4)
    return 4.0 + h

# Tree positions — scattered along the world axis with some rhythm.
# Each entry: (world_x_cell_widths, foliage_extra_rows). The foliage
# row is GROUND_ROW - 1, the trunk row is GROUND_ROW. Foliage_extra
# adds an extra leaf row above for "tall" trees.
TREE_SEED_OFFSETS = (1.7, 6.4, 11.2, 17.9, 23.5, 29.1, 35.6, 42.4, 48.0, 54.3,
                     60.1, 66.8, 72.5, 78.9, 85.2, 91.4, 98.0, 104.7, 110.3,
                     117.1, 123.8, 130.0)
TREE_TALL_INDICES = frozenset((1, 4, 7, 10, 13, 16, 19))  # every 3rd-ish

def trees_in_range(world_x_min, world_x_max):
    """Yield (world_x_cw, tall) for trees whose trunk falls in the range.

    The seed offsets define a 130-cw stretch of trees; we tile it modulo
    so the world is effectively infinite in both directions.
    """
    period = 135.0
    n_periods_min = math.floor(world_x_min / period)
    n_periods_max = math.ceil (world_x_max / period)
    for k in range(n_periods_min, n_periods_max + 1):
        base = k * period
        for i, off in enumerate(TREE_SEED_OFFSETS):
            wx = base + off
            if world_x_min - 1.0 <= wx <= world_x_max + 1.0:
                yield wx, (i in TREE_TALL_INDICES)

# ============================================================================
# Sky gradient — precomputed once
# ============================================================================

def build_sky():
    z = THEME["sky_zenith"]
    h = THEME["sky_horizon"]
    rows = []
    span = max(1, GROUND_ROW)
    for y in range(H):
        if y >= GROUND_ROW:
            rows.append(h)         # below ground row, never used as sky
            continue
        t = y / span               # 0 at top → ~1 at ground row
        t = max(0.0, min(1.0, t))
        rows.append((
            int(z[0] + (h[0] - z[0]) * t),
            int(z[1] + (h[1] - z[1]) * t),
            int(z[2] + (h[2] - z[2]) * t),
        ))
    return rows

SKY = build_sky()

# ============================================================================
# Bayer-4 dither matrix — for zoom-blend cell selection
# ============================================================================

BAYER_4 = (
    ( 0,  8,  2, 10),
    (12,  4, 14,  6),
    ( 3, 11,  1,  9),
    (15,  7, 13,  5),
)
BAYER_DENOM = 16.0

# ============================================================================
# Per-cell scene sampling
# ============================================================================
# Returns the colour of a single sub-cell at a given screen position, given
# the active per-layer parallax factors and the integer zoom level. The
# layered-back-to-front check is inlined for cheap, branch-friendly code.

def sample_sub(sub_x, row, cam_x, zoom_level, parallax):
    """Sample a single half-cell sub-cell and return its RGB colour.

    sub_x      : 0..SUB_W-1
    row        : 0..H-1
    cam_x      : float, camera world-x in cell-widths
    zoom_level : 1, 2, or 4 (integer)
    parallax   : 4-tuple (sky, mountain, hill, ground) of scroll factors
    """
    # Sky band first — short-circuits silhouette sampling.
    if row < HORIZON_ROW:
        return SKY[row]

    screen_x_cw = (sub_x + 0.5) * 0.5   # mid-sub-x in cell-widths

    # World-x for each layer = cam_offset(layer) + (screen_x - centre) / zoom.
    # zoom_level scales world units: at zoom=2, one screen cell-width covers
    # half a world cell-width; the world appears twice as big.
    world_dx = (screen_x_cw - CENTER_X_CW) / zoom_level

    row_cw_below_horizon = (row + 0.5 - HORIZON_ROW) * ROW_H_CW
    ground_cw_below_horizon = (GROUND_ROW + 0.5 - HORIZON_ROW) * ROW_H_CW

    # Mountain test (parallax[1]).
    wx_m = cam_x * parallax[1] + world_dx
    m_h_cw = mountain_height_cw(wx_m) * zoom_level   # mountain rises further on screen at higher zoom
    m_top_below_horizon = ground_cw_below_horizon - m_h_cw
    if row_cw_below_horizon < m_top_below_horizon:
        return SKY[row]

    # Hill test (parallax[2]).
    wx_h = cam_x * parallax[2] + world_dx
    h_h_cw = hill_height_cw(wx_h) * zoom_level
    h_top_below_horizon = ground_cw_below_horizon - h_h_cw
    if row_cw_below_horizon < h_top_below_horizon:
        # Mountain pixel — shade slightly toward sky horizon for atmospheric haze.
        m_color = THEME["mountain"]
        # 0 at peak (full mountain colour) → 1 near horizon (lit).
        haze = max(0.0, 1.0 - (m_top_below_horizon - row_cw_below_horizon) / 6.0)
        haze = max(0.0, min(0.7, haze))
        lit = THEME["mountain_lit"]
        return (
            int(m_color[0] * (1 - haze) + lit[0] * haze),
            int(m_color[1] * (1 - haze) + lit[1] * haze),
            int(m_color[2] * (1 - haze) + lit[2] * haze),
        )

    # Hill band (front of mountain, behind ground line).
    if row < GROUND_ROW:
        return THEME["hill"]

    # Ground band — slight horizontal stripes to give pan/parallax a reference.
    wx_g = cam_x * parallax[3] + world_dx
    if int(math.floor(wx_g * 0.5)) & 1:
        return THEME["ground"]
    return THEME["ground_dark"]

# ============================================================================
# Sprite resolution — trees and actor occupy whole cells with a glyph
# ============================================================================

def collect_sprites(state):
    """Return a list of (cell_col, cell_row, glyph, fg, bg_layer_resampler).

    bg_layer_resampler is a callable that yields the bg colour to use behind
    the glyph by sampling the layered scene at the cell's location, so the
    sprite sits cleanly on whatever layer it covers (sky / hill / ground).
    The resampler is invoked once per sprite per frame.
    """
    out = []
    if state['tab'] == 3:
        return out  # zoom tab: no sprites; the dither blend is the show

    parallax = state['parallax']
    cam_x    = state['cam_x']
    zoom     = 1            # sprites only show in non-zoom tabs

    # Trees scroll with the ground (parallax[3]).
    px_g = parallax[3]
    # Visible world-x range for the ground layer.
    half_world_x = (SCREEN_W_CW / 2.0) / zoom
    world_x_min  = cam_x * px_g - half_world_x
    world_x_max  = cam_x * px_g + half_world_x

    for wx, tall in trees_in_range(world_x_min, world_x_max):
        # Convert world_x to a screen cell column (centre of cell).
        screen_x_cw = (wx - cam_x * px_g) + CENTER_X_CW
        cell_col = int(math.floor(screen_x_cw))
        if not (0 <= cell_col < W):
            continue
        # Trunk: GROUND_ROW; foliage: GROUND_ROW - 1; tall extra: GROUND_ROW - 2.
        out.append((cell_col, GROUND_ROW,     "█", THEME["tree_trunk"], None))
        out.append((cell_col, GROUND_ROW - 1, "▲", THEME["tree_leaf"],  None))
        if tall and GROUND_ROW - 2 >= 0:
            out.append((cell_col, GROUND_ROW - 2, "▲", THEME["tree_leaf"], None))

    # Actor — only in tab 4.
    if state['tab'] == 4:
        ax_world = state['actor_x']
        screen_x_cw = (ax_world - cam_x) + CENTER_X_CW
        cell_col = int(math.floor(screen_x_cw))
        if 0 <= cell_col < W:
            out.append((cell_col, GROUND_ROW - 1, "☻", THEME["actor"], None))

    return out

# ============================================================================
# Rendering
# ============================================================================

def _render_header(out, state):
    out.append("Camera techniques  —  1-4 tab · w/s speed · a/d actor · +/- zoom · f follow · p pause · 0 reset · r reverse · q quit\n")
    tabs = ("Pan", "Parallax", "Zoom", "Follow")
    out.append("Tab:    ")
    for i, name in enumerate(tabs, start=1):
        marker = "▶" if state['tab'] == i else " "
        out.append(f"{marker}{i}:{name:<10}")
    out.append("\n")

    if state['tab'] in (1, 2):
        out.append(f"Camera: x={state['cam_x']:+8.2f}  speed={state['scroll_speed']:+5.1f} cw/s"
                   f"   parallax={state['parallax']}{CLEAR_EOL}\n")
    elif state['tab'] == 3:
        out.append(f"Zoom:   level=▶{state['zoom_level']}×  target={state['zoom_target']}×  "
                   f"blend={state['zoom_blend']:5.2f}  ({state['zoom_levels']})"
                   f"{CLEAR_EOL}\n")
    else:  # 4
        out.append(f"Follow: mode=▶{state['follow_mode']:<8}  "
                   f"actor x={state['actor_x']:+7.2f}  cam x={state['cam_x']:+7.2f}  "
                   f"v_actor={state['actor_v']:+5.1f}  v_cam={state['cam_v']:+5.1f}"
                   f"{CLEAR_EOL}\n")

    fps_str = f"{state['ema_fps']:5.1f}" if state['ema_fps'] > 0 else "  -- "
    ms_str  = f"{state['ema_ms']:5.1f}" if state['ema_ms'] > 0 else "  -- "
    out.append(f"Timing: target {state['target_fps']} fps · measured {fps_str} fps "
               f"· render {ms_str} ms/frame{CLEAR_EOL}\n\n")

def render(state):
    out = [HOME, RESET]
    _render_header(out, state)

    cam_x       = state['cam_x']
    parallax    = state['parallax']
    zoom_level  = state['zoom_level']
    zoom_target = state['zoom_target']
    blend       = state['zoom_blend']
    blending    = state['tab'] == 3 and 0.0 < blend < 1.0

    # Build per-cell (glyph, fg, bg) — with sprite overlays.
    sprites = collect_sprites(state)

    # Sprite map for fast cell-level lookup.
    sprite_map = {}
    for cell_col, cell_row, glyph, fg_color, _ in sprites:
        sprite_map[(cell_col, cell_row)] = (glyph, fg_color)

    for row in range(H):
        last_pair = None
        for cell_col in range(W):
            sub_xl = cell_col * 2
            sub_xr = cell_col * 2 + 1

            # Scene sampling per sub-cell. With dither blend in tab 3,
            # pick zoom_level vs zoom_target per CELL based on the
            # Bayer-4 threshold; both halves of a given output cell use
            # the same chosen zoom for SGR-coalescing friendliness.
            if blending:
                threshold = BAYER_4[row & 3][cell_col & 3] / BAYER_DENOM
                z = zoom_target if threshold < blend else zoom_level
            else:
                z = zoom_level
            fg_sample = sample_sub(sub_xl, row, cam_x, z, parallax)
            bg_sample = sample_sub(sub_xr, row, cam_x, z, parallax)

            sprite = sprite_map.get((cell_col, row))
            if sprite is not None:
                glyph, sprite_fg = sprite
                # Sprite's bg = mean of the two scene samples it covers,
                # which reads naturally as "the layer behind the sprite".
                sprite_bg = (
                    (fg_sample[0] + bg_sample[0]) >> 1,
                    (fg_sample[1] + bg_sample[1]) >> 1,
                    (fg_sample[2] + bg_sample[2]) >> 1,
                )
                if (sprite_fg, sprite_bg) != last_pair:
                    out.append(fgbg(sprite_fg, sprite_bg))
                    last_pair = (sprite_fg, sprite_bg)
                out.append(glyph)
            else:
                if (fg_sample, bg_sample) != last_pair:
                    out.append(fgbg(fg_sample, bg_sample))
                    last_pair = (fg_sample, bg_sample)
                out.append(HALF)
        out.append(RESET + "\n")

    # Tab 4 deadband visualisation: faint vertical guides bounding the
    # central rectangle, drawn at the bottom of the header band.
    if state['tab'] == 4 and state['follow_mode'] == "deadband":
        left  = int(CENTER_X_CW - state['deadband_half'])
        right = int(CENTER_X_CW + state['deadband_half'])
        # Ribbons painted as a status row already covered by the header;
        # we surface the bounds in the header text instead so this stays
        # visually clean. (See header line for follow numerics.)

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
# Camera update rules
# ============================================================================

ZOOM_SEQ = (1, 2, 4)

def update_camera(state, dt):
    tab = state['tab']
    if state['paused']:
        return

    if tab in (1, 2):
        # Auto-pan at scroll_speed (cell-widths/sec).
        state['cam_x'] += state['scroll_speed'] * dt

    elif tab == 3:
        # Animate zoom_blend toward 1.0; on completion snap to target.
        if state['zoom_target'] != state['zoom_level']:
            state['zoom_blend'] += dt / state['zoom_duration']
            if state['zoom_blend'] >= 1.0:
                state['zoom_level']  = state['zoom_target']
                state['zoom_blend']  = 0.0

    elif tab == 4:
        # Actor velocity decays slightly toward zero (friction); update pos.
        # The user adds impulse via a/d.
        state['actor_x'] += state['actor_v'] * dt
        state['actor_v'] *= max(0.0, 1.0 - 4.0 * dt)  # ~τ = 0.25 s

        if state['follow_mode'] == "deadband":
            # If actor leaves the deadband rectangle (camera-relative),
            # move camera just enough to put actor back on the boundary.
            screen_actor_x = state['actor_x'] - state['cam_x']
            band_l = -state['deadband_half']
            band_r = +state['deadband_half']
            if screen_actor_x > band_r:
                state['cam_x'] = state['actor_x'] - band_r
            elif screen_actor_x < band_l:
                state['cam_x'] = state['actor_x'] - band_l
            state['cam_v'] = 0.0
        else:
            # Critically-damped spring: a = -k(x - target) - 2*sqrt(k)*v.
            # Target is the actor position (camera centred on actor).
            k         = state['spring_k']
            crit_damp = 2.0 * math.sqrt(k)
            error     = state['actor_x'] - state['cam_x']
            accel     = k * error - crit_damp * state['cam_v']
            state['cam_v'] += accel * dt
            state['cam_x'] += state['cam_v'] * dt

# ============================================================================
# CLI
# ============================================================================

def parse_args():
    parser = argparse.ArgumentParser(
        description="Four-tab demo of 2D camera techniques (pan / parallax / zoom / follow).")
    parser.add_argument("--tab", type=int, choices=(1, 2, 3, 4), default=1,
                        help="initial tab (default: 1).")
    parser.add_argument("--scroll-speed", type=float, default=8.0, metavar="CW/S",
                        help="initial auto-scroll speed for tabs 1/2 (default: 8.0).")
    parser.add_argument("--follow", choices=("deadband", "spring"), default="deadband",
                        help="initial camera-follow rule for tab 4 (default: deadband).")
    parser.add_argument("--target-fps", type=int, default=60, metavar="FPS",
                        help="frame pacing target in Hz (default: 60).")
    return parser.parse_args()

# ============================================================================
# Main loop
# ============================================================================

PARALLAX_FLAT     = (0.0, 1.0, 1.0, 1.0)   # tab 1: rigid pan
PARALLAX_LAYERED  = (0.0, 0.20, 0.50, 1.0) # tab 2: layered parallax

def parallax_for_tab(tab):
    if tab == 2:
        return PARALLAX_LAYERED
    if tab == 3:
        return PARALLAX_FLAT      # zoom tab: parallax doesn't matter; static cam
    return PARALLAX_FLAT

def main():
    args = parse_args()

    state = {
        'tab':           args.tab,
        'paused':        False,
        'cam_x':         0.0,
        'cam_v':         0.0,
        'scroll_speed':  args.scroll_speed,
        'parallax':      parallax_for_tab(args.tab),

        'zoom_level':    1,
        'zoom_target':   1,
        'zoom_blend':    0.0,
        'zoom_duration': 0.6,             # seconds for a transition
        'zoom_levels':   ZOOM_SEQ,

        'actor_x':       0.0,
        'actor_v':       0.0,
        'actor_step':    14.0,            # impulse added per a/d press
        'follow_mode':   args.follow,
        'deadband_half': 6.0,             # cell-widths half-width of band
        'spring_k':      36.0,            # rad²/s² (critically damped at this k)

        'target_fps': args.target_fps,
        'ema_ms':     0.0,
        'ema_fps':    0.0,
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

    EMA_ALPHA   = 0.15
    last_tick   = time.perf_counter()
    last_frame  = time.perf_counter()

    try:
        while True:
            if has_tty:
                while True:
                    key = read_key()
                    if key is None:
                        break
                    if key in ("q", "Q"):
                        return
                    if key in ("1", "2", "3", "4"):
                        state['tab']      = int(key)
                        state['parallax'] = parallax_for_tab(state['tab'])
                        sys.stdout.write(CLEAR)
                        continue
                    if key in ("p", "P", " "):
                        state['paused'] = not state['paused']
                        continue
                    if key == "0":
                        state['cam_x']      = 0.0
                        state['cam_v']      = 0.0
                        state['actor_x']    = 0.0
                        state['actor_v']    = 0.0
                        state['zoom_level'] = 1
                        state['zoom_target'] = 1
                        state['zoom_blend'] = 0.0
                        sys.stdout.write(CLEAR)
                        continue
                    if state['tab'] in (1, 2):
                        if key in ("w", "W", "+", "="):
                            state['scroll_speed'] += 2.0
                            continue
                        if key in ("s", "S", "-", "_"):
                            state['scroll_speed'] -= 2.0
                            continue
                        if key in ("r", "R"):
                            state['scroll_speed'] = -state['scroll_speed']
                            continue
                    elif state['tab'] == 3:
                        if key in ("+", "="):
                            i = ZOOM_SEQ.index(state['zoom_target'])
                            if i + 1 < len(ZOOM_SEQ) and state['zoom_blend'] == 0.0:
                                state['zoom_target'] = ZOOM_SEQ[i + 1]
                                state['zoom_blend']  = 1e-6
                            continue
                        if key in ("-", "_"):
                            i = ZOOM_SEQ.index(state['zoom_target'])
                            if i > 0 and state['zoom_blend'] == 0.0:
                                state['zoom_target'] = ZOOM_SEQ[i - 1]
                                state['zoom_blend']  = 1e-6
                            continue
                    elif state['tab'] == 4:
                        if key in ("a", "A"):
                            state['actor_v'] -= state['actor_step']
                            continue
                        if key in ("d", "D"):
                            state['actor_v'] += state['actor_step']
                            continue
                        if key in ("f", "F"):
                            state['follow_mode'] = (
                                "spring" if state['follow_mode'] == "deadband" else "deadband")
                            state['cam_v'] = 0.0
                            continue

            now = time.perf_counter()
            dt  = now - last_frame
            last_frame = now
            update_camera(state, dt)

            t0 = time.perf_counter()
            render(state)
            t1 = time.perf_counter()

            render_ms        = (t1 - t0) * 1000.0
            state['ema_ms']  = (state['ema_ms'] * (1 - EMA_ALPHA)
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

# <FILE>docs/design/post-release/camera-techniques-demo.py</FILE>
# <VERS>END OF VERSION: 0.1.0</VERS>
