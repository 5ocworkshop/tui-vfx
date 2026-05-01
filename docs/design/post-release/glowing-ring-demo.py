# <FILE>docs/design/post-release/glowing-ring-demo.py</FILE> - <DESC>Stationary glowing ring rendered at quarter-cell density that fades in and out to black on a time-driven envelope. Uses the same quarter-cell partition technique as palette-cycling-demo-quarter.py, but inverts the architecture: the index field is a STATIC Gaussian-falloff distance from a target ring radius (each cell holds an integer brightness bin), and the PALETTE is rebuilt each frame from absolute wall-clock time so a sine / triangle / square / pulse envelope can scale the peak colour from 0 (black) to base_colour. Counterpoint to the palette-cycling demos: those rotate a fixed palette (LUT cycling); this fades a varying palette (LUT scaling). Together they exercise two of the three timing axes — presentation cadence (paint rate) and sample time (absolute_t_ms drives the envelope phase) — without needing the semantic update cadence axis at all.</DESC>
# <VERS>VERSION: 0.1.0</VERS>
# <WCTX>Demonstrate a complementary use of the quarter-cell rendering substrate where motion comes from a wall-clock-driven palette envelope rather than from LUT rotation, so the same field/palette decoupling teaches both styles of cheap animation.</WCTX>
# <CLOG>0.1.0: new file; static Gaussian-ring intensity field at the quarter-cell sub-grid; time-driven palette built each frame from envelope(phase) × base_colour with sine/triangle/square/pulse envelope shapes; CLI flags for --color, --period, --envelope, --radius, --target-fps; runtime keys 1-4 to switch envelope, +/- to adjust period, q to quit; deadline-aware frame loop; header surfaces phase, envelope value, measured fps and render-ms.</CLOG>

"""
Glowing-ring demo — stationary ring that fades to black and back.

────────────────────────────────────────────────────────────────────────
Run:   python3 glowing-ring-demo.py
       python3 glowing-ring-demo.py --color cyan --period 5
       python3 glowing-ring-demo.py --envelope pulse --color #ff5050

Keys:  1..4  — switch envelope shape (1 sine · 2 triangle · 3 square · 4 pulse)
       +/-   — shorten / lengthen the cycle period (250 ms steps)
       q     — quit (Ctrl-C also works)
────────────────────────────────────────────────────────────────────────

WHAT THIS DEMONSTRATES

Where palette-cycling-demo*.py rotates a fixed palette to create motion
(field static, offset advances, palette unchanged), this demo SHAPES the
palette every frame from wall-clock time (field static, palette varies,
no offset). Same architectural decoupling, opposite axis of variation.

Two pure-data inputs:

  1. an *intensity field* — a 2D grid of integer brightness bins (0..N-1),
     computed ONCE at startup. Each cell's bin is set by a Gaussian
     falloff from the target ring radius:

         intensity(x, y) = exp(-(d - target_r)^2 / (2 * width^2))
         field[y][x]     = int(intensity * (N - 1))

     Cells right on the ring get the highest bin (N-1); cells well off
     the ring get bin 0. The field never changes after startup.

  2. a *time-driven palette* — rebuilt every frame from absolute time:

         phase    = (now_ms % period_ms) / period_ms       in [0, 1)
         env      = ENVELOPE(phase)                        in [0, 1]
         peak     = base_colour * env                      RGB triple
         palette[i] = (i / (N-1)) * peak                   linear ramp

     At env = 0 the entire palette is black, so every cell renders
     black — the ring vanishes. At env = 1 the palette is the full
     black→base_colour ramp, so the ring (highest field bins) glows at
     base_colour and the outer halo fades to black through the ramp.

Per-frame work is just lookup: cell_colour = palette[field[y][x]]. The
quarter-cell partition selector still picks the best 2-of-4 colour
representation per terminal cell, the same as in the LUT-cycling demo.

TIMING AXES

This demo uses two of the three axes the runtime team is carving out
for the Rust port:

  axis 1  presentation cadence  →  --target-fps drives the deadline-aware
                                   sleep loop. Each frame we paint.
  axis 2  semantic update rate  →  not used. We don't have a separate
                                   per-source tick rate; the palette is
                                   recomputed every frame.
  axis 3  sample / elapsed time →  absolute_t_ms (time.perf_counter())
                                   drives envelope phase. The renderer
                                   does NOT count frames to advance the
                                   animation; it asks the wall clock.

Reading wall-clock time means the animation stays correct even if the
presentation rate changes mid-run, which is exactly the property the
runtime team wants for Madeira and similar continuous procedurals.

ENVELOPE SHAPES

  sine      smooth in, smooth out — symmetric pulse, the default
  triangle  linear in, linear out — sharper turn at the peak
  square    instant on / instant off — strobe
  pulse     fast attack, slow decay — heartbeat-like

Press 1, 2, 3, 4 to swap them at runtime.
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

W, H        = 78, 18    # terminal cells
SUB_W       = W * 2     # quarter-cell sub-columns (▌-style horizontal halving)
SUB_H       = H * 2     # quarter-cell sub-rows
CELL_ASPECT = 2.2       # cells are ~2.2× taller than wide
PALETTE_SIZE = 64       # brightness bins; gradient from black to peak

# After aspect correction, sub-cell dimensions in screen-isotropic cell-widths:
SUB_X       = 0.5                       # 1 sub-x-column = 0.5 cell-widths
SUB_Y       = 0.5 * CELL_ASPECT         # 1 sub-y-row    = 1.1 cell-widths
HALF_V_CW   = (SUB_H / 2) * SUB_Y       # vertical half-extent in cell-widths

# ============================================================================
# Terminal escape codes
# ============================================================================

ESC         = "\x1b"
def fgbg(fc, bc): return f"{ESC}[38;2;{fc[0]};{fc[1]};{fc[2]};48;2;{bc[0]};{bc[1]};{bc[2]}m"
RESET       = f"{ESC}[0m"
HIDE_CURSOR = f"{ESC}[?25l"
SHOW_CURSOR = f"{ESC}[?25h"
HOME        = f"{ESC}[H"
CLEAR       = f"{ESC}[2J"
CLEAR_EOL   = f"{ESC}[K"

# ============================================================================
# Colour presets and parsing
# ============================================================================

COLOURS = {
    "amber":   (255, 180,  60),
    "yellow":  (255, 220, 100),
    "cyan":    (100, 220, 255),
    "magenta": (255, 100, 200),
    "white":   (255, 255, 255),
    "green":   (120, 255, 140),
    "red":     (255,  80,  60),
    "violet":  (180, 100, 255),
}

def parse_colour(s):
    """Accept a named colour or a #RRGGBB hex string."""
    if s in COLOURS:
        return COLOURS[s]
    if s.startswith("#") and len(s) == 7:
        try:
            return (int(s[1:3], 16), int(s[3:5], 16), int(s[5:7], 16))
        except ValueError:
            pass
    raise argparse.ArgumentTypeError(
        f"colour must be one of {sorted(COLOURS)} or #RRGGBB; got {s!r}")

# ============================================================================
# Envelopes — phase ∈ [0, 1) → intensity ∈ [0, 1]
# ============================================================================

def env_sine(p):
    """Smooth in, smooth out. Symmetric. Period 1."""
    return 0.5 * (1.0 - math.cos(p * 2 * math.pi))

def env_triangle(p):
    """Linear up to mid-cycle, linear down. Sharper peak than sine."""
    return 1.0 - abs(2 * p - 1)

def env_square(p):
    """Instant on for the first half of the cycle, off for the rest."""
    return 1.0 if p < 0.5 else 0.0

def env_pulse(p):
    """Fast attack (10% of cycle), slow decay (90%). Heartbeat-like."""
    if p < 0.10:
        return p / 0.10
    return max(0.0, 1.0 - (p - 0.10) / 0.90)

ENVELOPES = {
    "sine":     env_sine,
    "triangle": env_triangle,
    "square":   env_square,
    "pulse":    env_pulse,
}

ENVELOPE_ORDER = ("sine", "triangle", "square", "pulse")  # for the 1..4 keys

# ============================================================================
# Intensity field — static, computed once at startup
# ============================================================================

def ring_field(target_r_cw, width_cw):
    """Build the SUB_H × SUB_W intensity field for a Gaussian-glowing ring.

    target_r_cw, width_cw are in screen-isotropic cell-width units. The
    field's integer values are PALETTE_SIZE-1 right on the ring radius and
    fall off smoothly in both directions.
    """
    cx, cy = SUB_W / 2, SUB_H / 2
    two_w_sq = 2.0 * width_cw * width_cw
    field = []
    for y in range(SUB_H):
        row = []
        for x in range(SUB_W):
            dx = (x - cx) * SUB_X
            dy = (y - cy) * SUB_Y
            d  = math.sqrt(dx * dx + dy * dy)
            offset_from_ring = abs(d - target_r_cw)
            intensity = math.exp(-(offset_from_ring * offset_from_ring) / two_w_sq)
            row.append(int(intensity * (PALETTE_SIZE - 1)))
        field.append(row)
    return field

# ============================================================================
# Time-driven palette — rebuilt every frame
# ============================================================================

def build_palette(base_colour, env_value):
    """Linear ramp from black to (base_colour × env_value) over PALETTE_SIZE
    entries. At env_value = 0 the whole palette is black; at env_value = 1
    it spans the full black → base_colour gradient."""
    br, bg_, bb = base_colour
    peak_r = br  * env_value
    peak_g = bg_ * env_value
    peak_b = bb  * env_value
    pal = []
    last = PALETTE_SIZE - 1
    for i in range(PALETTE_SIZE):
        t = i / last
        pal.append((int(peak_r * t), int(peak_g * t), int(peak_b * t)))
    return pal

# ============================================================================
# Quarter-cell partition selection (identical to palette-cycling-demo-quarter)
# ============================================================================

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
# Rendering
# ============================================================================

INTENSITY_BAR = " ▏▎▍▌▋▊▉█"  # 9 levels for the env-value visualiser

def _intensity_bar(value, width=20):
    """ASCII bar of `width` cells where `value` ∈ [0, 1] fills proportionally."""
    levels = len(INTENSITY_BAR) - 1
    full   = value * width
    cells  = []
    for i in range(width):
        if full >= i + 1:
            cells.append(INTENSITY_BAR[-1])
        elif full > i:
            cells.append(INTENSITY_BAR[int((full - i) * levels)])
        else:
            cells.append(INTENSITY_BAR[0])
    return "".join(cells)

def _render_header(out, period_ms, env_name, colour_name, phase, env_value,
                   target_fps, measured_ms, measured_fps):
    out.append("Glowing ring demo  —  q quit · +/- period · 1-4 envelope shape\n")
    out.append("Static Gaussian-ring field × time-driven palette (env(t) × base ↦ black)\n")

    out.append(f"Period:  {period_ms / 1000:5.2f}s   ")
    for i, name in enumerate(ENVELOPE_ORDER):
        marker = "▶" if name == env_name else " "
        out.append(f"{marker}{i+1}:{name:<8}")
    out.append(f"  Colour: {colour_name}\n")

    fps_str = f"{measured_fps:5.1f}" if measured_fps > 0 else "  -- "
    ms_str  = f"{measured_ms:5.1f}" if measured_ms > 0 else "  -- "
    out.append(f"Timing:  target {target_fps} fps · measured {fps_str} fps "
               f"· render {ms_str} ms/frame{CLEAR_EOL}\n")

    bar = _intensity_bar(env_value, width=24)
    out.append(f"Phase:   {phase:5.3f}   "
               f"intensity {env_value:5.3f}  |{bar}|\n\n")

def render(field, palette, period_ms, env_name, colour_name, phase, env_value,
           target_fps, measured_ms, measured_fps):
    out = [HOME, RESET]
    _render_header(out, period_ms, env_name, colour_name, phase, env_value,
                   target_fps, measured_ms, measured_fps)

    for cy in range(0, SUB_H, 2):
        row_top = field[cy]
        row_bot = field[cy + 1]
        last = None
        for cx in range(0, SUB_W, 2):
            c_ul = palette[row_top[cx]]
            c_ur = palette[row_top[cx + 1]]
            c_ll = palette[row_bot[cx]]
            c_lr = palette[row_bot[cx + 1]]
            glyph, fg_, bg_ = choose_quadrant(c_ul, c_ur, c_ll, c_lr)
            triple = (glyph, fg_, bg_)
            if triple != last:
                out.append(fgbg(fg_, bg_))
                last = triple
            out.append(glyph)
        out.append(RESET + "\n")

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
        description="Stationary glowing ring that fades to black and back.")
    parser.add_argument(
        "--color", "--colour", dest="colour", type=parse_colour, default="amber",
        metavar="NAME_OR_HEX",
        help=f"ring base colour: {sorted(COLOURS)} or #RRGGBB (default: amber).")
    parser.add_argument(
        "--period", type=float, default=3.0, metavar="SEC",
        help="full envelope cycle in seconds (default: 3.0). Adjustable at runtime with +/-.")
    parser.add_argument(
        "--envelope", choices=ENVELOPE_ORDER, default="sine",
        help="envelope shape (default: sine). Switch at runtime with 1..4.")
    parser.add_argument(
        "--radius", type=float, default=0.55, metavar="FRAC",
        help="ring radius as fraction of vertical half-height in cell-widths "
             "(default: 0.55, ~11 cell-widths at 78×18).")
    parser.add_argument(
        "--width", type=float, default=0.05, metavar="FRAC",
        help="Gaussian ring width as fraction of vertical half-height "
             "(default: 0.05, ~1 cell-width — soft glow).")
    parser.add_argument(
        "--target-fps", type=int, default=60, metavar="FPS",
        help="frame pacing target in Hz (default: 60).")
    return parser.parse_args()

# ============================================================================
# Main loop
# ============================================================================

def colour_label(colour):
    """Reverse-lookup a name for the colour, fall back to #RRGGBB."""
    for name, rgb in COLOURS.items():
        if rgb == colour:
            return name
    return f"#{colour[0]:02x}{colour[1]:02x}{colour[2]:02x}"

def main():
    args = parse_args()

    target_r = HALF_V_CW * args.radius
    width    = HALF_V_CW * args.width
    field    = ring_field(target_r, width)

    base_colour   = args.colour
    colour_name   = colour_label(base_colour)
    period_ms     = args.period * 1000.0
    env_name      = args.envelope
    target_fps    = args.target_fps
    frame_period  = 1.0 / target_fps

    has_tty     = sys.stdin.isatty()
    fd          = sys.stdin.fileno() if has_tty else None
    old_termios = termios.tcgetattr(fd) if has_tty else None
    if has_tty:
        tty.setcbreak(fd)

    sys.stdout.write(HIDE_CURSOR + CLEAR)
    sys.stdout.flush()

    start          = time.perf_counter()
    next_deadline  = start + frame_period

    ema_render_ms  = 0.0
    ema_fps        = 0.0
    EMA_ALPHA      = 0.15
    last_tick      = start

    try:
        while True:
            if has_tty:
                while True:
                    key = read_key()
                    if key is None:
                        break
                    if key in ("q", "Q"):
                        return
                    if key in ("+", "="):
                        period_ms = max(period_ms - 250.0, 250.0)
                        continue
                    if key in ("-", "_"):
                        period_ms = min(period_ms + 250.0, 30000.0)
                        continue
                    if key in ("1", "2", "3", "4"):
                        env_name = ENVELOPE_ORDER[int(key) - 1]
                        continue

            now        = time.perf_counter()
            elapsed_ms = (now - start) * 1000.0
            phase      = (elapsed_ms % period_ms) / period_ms
            env_value  = ENVELOPES[env_name](phase)
            palette    = build_palette(base_colour, env_value)

            t0 = time.perf_counter()
            render(field, palette, period_ms, env_name, colour_name, phase,
                   env_value, target_fps, ema_render_ms, ema_fps)
            t1 = time.perf_counter()

            render_ms     = (t1 - t0) * 1000.0
            ema_render_ms = (ema_render_ms * (1 - EMA_ALPHA)
                             + render_ms * EMA_ALPHA) if ema_render_ms > 0 else render_ms

            now       = time.perf_counter()
            sleep_for = next_deadline - now
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

# <FILE>docs/design/post-release/glowing-ring-demo.py</FILE>
# <VERS>END OF VERSION: 0.1.0</VERS>
