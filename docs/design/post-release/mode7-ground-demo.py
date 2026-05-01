# <FILE>docs/design/post-release/mode7-ground-demo.py</FILE> - <DESC>SNES-Mode-7-style perspective ground plane rendered to terminal cells. A 2D world-space texture (checkerboard / grid / stripes) is sampled per output column with a different scale and offset per output row, producing a tilted ground plane that recedes into a fog-shaded horizon. Camera has forward velocity and steerable yaw — pressing 'a'/'d' adds angular velocity (continuous turn until cancelled), 'w'/'s' adjusts forward speed, '1'/'2'/'3' switches texture, '`' cycles colour scheme. Renders at half-cell horizontal density (▌ glyph, fg=left half, bg=right half) for clean perspective lines without needing the quarter-cell partition selector. Per-row constants (depth, scale, fog) are precomputed once at startup since they depend only on viewer height, focal length, and horizon position; the per-frame work is the (cos α, sin α) rotation of view-relative coordinates plus the texture sample.</DESC>
# <VERS>VERSION: 0.1.0</VERS>
# <WCTX>Bring the SNES Mode 7 perspective trick into the terminal demo set: the same per-scanline-affine idea (HDMA-loaded scale + offset per row in F-Zero / Mario Kart / FFVI overworld) translates directly to per-row sampling in a TUI compositor and gives a cheap, atmospheric ground plane.</WCTX>
# <CLOG>0.1.0: new file; half-cell rendered perspective ground with steerable camera; per-row depth/scale/fog precomputed at startup; checker / grid / stripes textures × grid / vaporwave / f-zero / sunset colour schemes; runtime keys w/s speed, a/d turn, 1-3 texture, ` scheme, p pause, 0 reset, q quit; deadline-aware frame loop; header surfaces camera state and measured fps.</CLOG>

"""
Mode-7 perspective ground demo — fly over a textured plane in the terminal.

────────────────────────────────────────────────────────────────────────
Run:   python3 mode7-ground-demo.py
       python3 mode7-ground-demo.py --scheme vaporwave
       python3 mode7-ground-demo.py --texture checker --speed 12

Keys:  w / s   — forward speed +/- 2 cell-widths/sec
       a / d   — angular velocity -/+ 0.25 rad/sec (continuous turn)
       1 2 3   — switch texture (1 grid · 2 checker · 3 stripes)
       `       — cycle colour scheme (grid → vaporwave → f-zero → sunset)
       p       — pause / resume motion
       0       — reset position, heading, and velocities
       q       — quit (Ctrl-C also works)
────────────────────────────────────────────────────────────────────────

WHAT THIS DEMONSTRATES

The SNES PPU Mode 7 (1990) drew a 2D background tile-map but applied a
DIFFERENT 2×2 affine transform (scale + rotation + translation) on every
scanline. The transforms were loaded from RAM by HDMA mid-line, so each
horizontal row of output sampled the source plane at a different rate
and offset. Choose the per-row transforms to follow a ground-plane
perspective formula and you get F-Zero / Mario Kart / FFVI overworld:
a flat 2D map that LOOKS three-dimensional, no triangle rasterisation
needed.

This demo is Mode 7 translated to terminal cells:

  * The "source plane" is a procedurally-evaluated texture in 2D world
    space — checker, grid, or stripes — at any (world_x, world_z).
  * The "scanline" is one terminal row.
  * The "per-scanline transform" is the depth z and screen-to-world
    scale for that row, derived from viewer height and focal length.
    Rows nearer the bottom of the screen sample close-up territory at
    a fine scale; rows just below the horizon sample distant territory
    at a coarse scale.
  * Camera rotation rotates ALL the per-row sample directions through a
    single (cos α, sin α) basis, exactly the per-frame cost the SNES
    paid: two trig values plus the per-row affine coefficients.

Per-row precomputation. Depth z, screen-x → world-side scale, and fog
factor depend only on the row index and the geometry constants
(viewer_h, focal_length, horizon position, fog_near/far), not on the
camera's position or heading. We compute them once at startup. The
per-frame work is then just (cos α, sin α) and one rotated view sample
per output sub-cell.

PERSPECTIVE GEOMETRY

For a row at screen_y_below_horizon = y_screen - horizon_y (in cell-widths,
positive going down):

    z          = viewer_h * focal_length / screen_y_below_horizon
    scale_x    = z / focal_length            (screen-x → world-side)
    fog_factor = clamp01((z - FOG_NEAR) / (FOG_FAR - FOG_NEAR))

Then per output column at screen_x = x_cw - center_x:

    world_side    = screen_x * scale_x
    world_forward = z
    world_dx = world_side * cos(α) + world_forward * sin(α)
    world_dz = world_forward * cos(α) - world_side * sin(α)
    world_x  = camera_x + world_dx
    world_z  = camera_z + world_dz
    base_color = texture(world_x, world_z)
    final      = lerp(base_color, horizon_color, fog_factor)

This is the entire Mode 7 program.

CHEAP HALF-CELL RENDER

The terminal cell is split horizontally with U+258C ▌ (fg = left half,
bg = right half). This doubles horizontal resolution losslessly without
needing the quarter-cell partition selector — perspective edges look
notably cleaner than full-cell. Vertical density stays at H rows; with
the perspective compression toward the horizon, sub-row precision near
the horizon would help, but quarter-cell rendering would also force the
2-colour partition approximation, which is a poor match for sharp grid
lines. Half-cell is the sweet spot for this scene.
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
SUB_W       = W * 2          # half-cell horizontal sub-columns
CELL_ASPECT = 2.2            # cells are this many cell-widths tall

# All distances below are in cell-widths (the screen-isotropic unit).
SCREEN_W_CW  = W * 1.0
SCREEN_H_CW  = H * CELL_ASPECT
CENTER_X_CW  = SCREEN_W_CW / 2.0
ROW_H_CW     = CELL_ASPECT       # one terminal row = 2.2 cell-widths tall

# Camera / projection — abstract, tuned by feel for this aspect ratio.
VIEWER_HEIGHT      = 1.0
FOCAL_LENGTH_CW    = 30.0
DEFAULT_HORIZON_FRAC = 0.45
FOG_NEAR           = 5.0
FOG_FAR            = 80.0

# Defaults
DEFAULT_TILE_SIZE  = 4.0     # world cell-widths per tile
DEFAULT_FORWARD    = 8.0     # cell-widths / second
TURN_STEP          = 0.25    # rad / second per 'a' or 'd' press
SPEED_STEP         = 2.0     # cell-widths / sec per 'w' or 's' press

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
# Colour schemes — each defines ground tones plus a sky gradient
# ============================================================================
# horizon: colour at the horizon (also the fog destination)
# zenith:  colour at the top of the sky
# tone_a, tone_b: two ground texture colours (used as a/b in checker/stripes,
#                 or as fill/line in grid)

SCHEMES = {
    "grid": {
        "tone_a":  ( 12,  12,  20),     # near-black
        "tone_b":  (  0, 220, 200),     # cyan grid lines
        "horizon": ( 30,  60,  90),     # dim teal
        "zenith":  (  4,   6,  18),
    },
    "vaporwave": {
        "tone_a":  ( 30,  10,  50),     # deep purple
        "tone_b":  (255, 100, 200),     # hot pink
        "horizon": (255, 130, 180),
        "zenith":  ( 60,  10,  90),
    },
    "f-zero": {
        "tone_a":  ( 18,  22,  60),     # navy
        "tone_b":  (255, 220,  60),     # yellow
        "horizon": (255, 130,  40),     # sunset orange
        "zenith":  (  8,  10,  40),
    },
    "sunset": {
        "tone_a":  ( 60,  20,  30),     # ground rust
        "tone_b":  (160,  50,  60),     # ground brick
        "horizon": (255, 180, 120),     # peach
        "zenith":  ( 50,  20,  60),     # purple top
    },
}

SCHEME_ORDER = ("grid", "vaporwave", "f-zero", "sunset")

# ============================================================================
# Textures — (world_x, world_z, scheme, tile_size) → RGB
# ============================================================================

def tex_checker(wx, wz, scheme, tile):
    """Classic two-tone Mode 7 checkerboard."""
    if (int(math.floor(wx / tile)) + int(math.floor(wz / tile))) & 1:
        return scheme["tone_a"]
    return scheme["tone_b"]

def tex_grid(wx, wz, scheme, tile):
    """Tile-fill (tone_a) with bright lines (tone_b) at tile boundaries."""
    fx = (wx / tile) % 1.0
    fz = (wz / tile) % 1.0
    near = 0.08          # line thickness, fraction of a tile
    if min(fx, 1.0 - fx) < near or min(fz, 1.0 - fz) < near:
        return scheme["tone_b"]
    return scheme["tone_a"]

def tex_stripes(wx, wz, scheme, tile):
    """Bands perpendicular to forward motion — road / runway feel."""
    if int(math.floor(wz / tile)) & 1:
        return scheme["tone_a"]
    return scheme["tone_b"]

TEXTURES = {
    "grid":    tex_grid,
    "checker": tex_checker,
    "stripes": tex_stripes,
}

TEXTURE_ORDER = ("grid", "checker", "stripes")  # for the 1/2/3 keys

# ============================================================================
# Sky gradient — precomputed once per scheme
# ============================================================================

def build_sky(scheme, horizon_row):
    """Return a list of H RGB tuples; rows above horizon_row are the gradient,
    rows from horizon_row down are unused (overwritten by ground)."""
    h    = scheme["horizon"]
    z    = scheme["zenith"]
    rows = []
    for y in range(H):
        if horizon_row <= 0:
            rows.append(h)
            continue
        # Linear interpolation top (zenith) → bottom (horizon).
        t = y / max(1, horizon_row - 1) if horizon_row > 1 else 0.0
        t = max(0.0, min(1.0, t))
        rows.append((
            int(z[0] + (h[0] - z[0]) * t),
            int(z[1] + (h[1] - z[1]) * t),
            int(z[2] + (h[2] - z[2]) * t),
        ))
    return rows

# ============================================================================
# Per-row geometry — depth z, x-scale, fog factor, all functions of row only
# ============================================================================

def build_row_constants(horizon_frac):
    """Return (horizon_row_inclusive, z_per_row, scale_per_row, fog_per_row).

    horizon_row_inclusive is the first row drawn from the ground-plane path.
    The geometry above is sky, below is ground.
    """
    horizon_y_cw = horizon_frac * SCREEN_H_CW

    z_per_row     = [0.0] * H
    scale_per_row = [0.0] * H
    fog_per_row   = [0.0] * H
    horizon_row   = H  # default: no ground rows

    for y in range(H):
        # Use mid-row centre for the sample (y + 0.5 cell-rows from top).
        y_cw = (y + 0.5) * ROW_H_CW
        below = y_cw - horizon_y_cw
        if below <= 0:
            continue
        if y < horizon_row:
            horizon_row = y
        z = VIEWER_HEIGHT * FOCAL_LENGTH_CW / below
        z_per_row[y]     = z
        scale_per_row[y] = z / FOCAL_LENGTH_CW
        fog = (z - FOG_NEAR) / max(1e-6, (FOG_FAR - FOG_NEAR))
        fog_per_row[y]   = max(0.0, min(1.0, fog))

    return horizon_row, z_per_row, scale_per_row, fog_per_row

# ============================================================================
# Rendering
# ============================================================================

def _render_header(out, state):
    out.append("Mode-7 ground demo  —  w/s speed · a/d turn · 1-3 texture · ` scheme · p pause · 0 reset · q quit\n")
    out.append(f"Camera: x={state['cam_x']:+7.2f}  z={state['cam_z']:+7.2f}  "
               f"heading={math.degrees(state['cam_angle']) % 360:6.1f}°  "
               f"speed={state['fwd_speed']:+5.1f} cw/s  "
               f"turn={state['turn_rate']:+5.2f} rad/s"
               f"{'   PAUSED' if state['paused'] else '         '}{CLEAR_EOL}\n")
    out.append(f"Scene:  texture=▶{state['texture']:<8}  scheme=▶{state['scheme']:<10}  "
               f"horizon row {state['horizon_row']}/{H}{CLEAR_EOL}\n")
    fps_str = f"{state['ema_fps']:5.1f}" if state['ema_fps'] > 0 else "  -- "
    ms_str  = f"{state['ema_ms']:5.1f}" if state['ema_ms'] > 0 else "  -- "
    out.append(f"Timing: target {state['target_fps']} fps · measured {fps_str} fps "
               f"· render {ms_str} ms/frame{CLEAR_EOL}\n\n")

def _emit_sky_row(out, sky_color):
    """One full-width terminal row painted with a single sky color."""
    out.append(bg(sky_color))
    out.append(" " * W)
    out.append(RESET + "\n")

def _emit_ground_row(out, y, state, sky_horizon, scheme):
    """Half-cell ground row using per-row constants and current camera."""
    z          = state['z_per_row'][y]
    scale      = state['scale_per_row'][y]
    fog        = state['fog_per_row'][y]
    cos_a      = state['cos_a']
    sin_a      = state['sin_a']
    cam_x      = state['cam_x']
    cam_z      = state['cam_z']
    tex_fn     = TEXTURES[state['texture']]
    tile       = state['tile_size']

    # Fog destination is the horizon sky colour (darkens / tints distance).
    hr, hg, hb  = sky_horizon
    inv_fog     = 1.0 - fog

    last_pair = None
    for cell_x in range(W):
        # Two sub-cells per output cell (left half = fg, right half = bg).
        fg_color = bg_color = (0, 0, 0)
        for half_idx in range(2):
            sub_x = cell_x * 2 + half_idx
            # Sub-cell screen-x at its centre (each sub-cell = 0.5 cell-wide).
            screen_x_cw = (sub_x + 0.5) * 0.5
            rel_side    = (screen_x_cw - CENTER_X_CW) * scale
            rel_fwd     = z
            world_dx    = rel_side * cos_a + rel_fwd * sin_a
            world_dz    = rel_fwd  * cos_a - rel_side * sin_a
            tone        = tex_fn(cam_x + world_dx, cam_z + world_dz, scheme, tile)
            r = int(tone[0] * inv_fog + hr * fog)
            g = int(tone[1] * inv_fog + hg * fog)
            b = int(tone[2] * inv_fog + hb * fog)
            if half_idx == 0:
                fg_color = (r, g, b)
            else:
                bg_color = (r, g, b)
        if (fg_color, bg_color) != last_pair:
            out.append(fgbg(fg_color, bg_color))
            last_pair = (fg_color, bg_color)
        out.append(HALF)
    out.append(RESET + "\n")

def render(state):
    out = [HOME, RESET]
    _render_header(out, state)

    sky    = state['sky']
    scheme = SCHEMES[state['scheme']]
    horizon_row = state['horizon_row']
    sky_horizon = scheme['horizon']

    # Sky rows.
    for y in range(min(horizon_row, H)):
        _emit_sky_row(out, sky[y])

    # Ground rows.
    for y in range(horizon_row, H):
        _emit_ground_row(out, y, state, sky_horizon, scheme)

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
        description="SNES Mode 7 perspective ground plane in a terminal.")
    parser.add_argument("--scheme", choices=SCHEME_ORDER, default="grid",
                        help="colour scheme (default: grid).")
    parser.add_argument("--texture", choices=TEXTURE_ORDER, default="grid",
                        help="ground texture pattern (default: grid).")
    parser.add_argument("--tile-size", type=float, default=DEFAULT_TILE_SIZE,
                        metavar="CW",
                        help=f"world cell-widths per texture tile (default: {DEFAULT_TILE_SIZE}).")
    parser.add_argument("--speed", type=float, default=DEFAULT_FORWARD,
                        metavar="CW/S",
                        help=f"initial forward speed in cell-widths/sec (default: {DEFAULT_FORWARD}).")
    parser.add_argument("--horizon", type=float, default=DEFAULT_HORIZON_FRAC,
                        metavar="FRAC",
                        help=f"horizon position as fraction down screen (default: {DEFAULT_HORIZON_FRAC}).")
    parser.add_argument("--target-fps", type=int, default=60, metavar="FPS",
                        help="frame pacing target in Hz (default: 60).")
    return parser.parse_args()

# ============================================================================
# Main loop
# ============================================================================

def main():
    args = parse_args()

    horizon_row, z_per_row, scale_per_row, fog_per_row = build_row_constants(args.horizon)
    sky = build_sky(SCHEMES[args.scheme], horizon_row)

    state = {
        'cam_x':     0.0,
        'cam_z':     0.0,
        'cam_angle': 0.0,
        'fwd_speed': args.speed,
        'turn_rate': 0.0,
        'paused':    False,

        'scheme':       args.scheme,
        'texture':      args.texture,
        'tile_size':    args.tile_size,
        'horizon_row':  horizon_row,
        'z_per_row':    z_per_row,
        'scale_per_row': scale_per_row,
        'fog_per_row':  fog_per_row,
        'sky':          sky,

        'cos_a': 1.0,
        'sin_a': 0.0,

        'target_fps': args.target_fps,
        'ema_ms':  0.0,
        'ema_fps': 0.0,
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
    last_frame = time.perf_counter()

    try:
        while True:
            if has_tty:
                while True:
                    key = read_key()
                    if key is None:
                        break
                    if key in ("q", "Q"):
                        return
                    if key in ("w", "W"):
                        state['fwd_speed'] += SPEED_STEP
                        continue
                    if key in ("s", "S"):
                        state['fwd_speed'] -= SPEED_STEP
                        continue
                    if key in ("a", "A"):
                        state['turn_rate'] -= TURN_STEP
                        continue
                    if key in ("d", "D"):
                        state['turn_rate'] += TURN_STEP
                        continue
                    if key in ("p", "P", " "):
                        state['paused'] = not state['paused']
                        continue
                    if key == "0":
                        state['cam_x'] = 0.0
                        state['cam_z'] = 0.0
                        state['cam_angle'] = 0.0
                        state['turn_rate'] = 0.0
                        state['fwd_speed'] = args.speed
                        sys.stdout.write(CLEAR)
                        continue
                    if key in ("1", "2", "3"):
                        state['texture'] = TEXTURE_ORDER[int(key) - 1]
                        continue
                    if key == "`":
                        i = SCHEME_ORDER.index(state['scheme'])
                        state['scheme'] = SCHEME_ORDER[(i + 1) % len(SCHEME_ORDER)]
                        state['sky'] = build_sky(SCHEMES[state['scheme']], state['horizon_row'])
                        sys.stdout.write(CLEAR)
                        continue

            # Advance simulation by elapsed wall-clock dt (axis 3).
            now = time.perf_counter()
            dt  = now - last_frame
            last_frame = now
            if not state['paused']:
                state['cam_angle'] += state['turn_rate'] * dt
                cos_a = math.cos(state['cam_angle'])
                sin_a = math.sin(state['cam_angle'])
                state['cam_x'] += state['fwd_speed'] * sin_a * dt
                state['cam_z'] += state['fwd_speed'] * cos_a * dt
                state['cos_a']  = cos_a
                state['sin_a']  = sin_a
            else:
                # Keep cos_a / sin_a in sync with current angle even when paused.
                state['cos_a'] = math.cos(state['cam_angle'])
                state['sin_a'] = math.sin(state['cam_angle'])

            t0 = time.perf_counter()
            render(state)
            t1 = time.perf_counter()

            render_ms       = (t1 - t0) * 1000.0
            state['ema_ms'] = (state['ema_ms'] * (1 - EMA_ALPHA)
                               + render_ms * EMA_ALPHA) if state['ema_ms'] > 0 else render_ms

            now2     = time.perf_counter()
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

# <FILE>docs/design/post-release/mode7-ground-demo.py</FILE>
# <VERS>END OF VERSION: 0.1.0</VERS>
