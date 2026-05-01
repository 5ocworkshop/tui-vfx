# <FILE>docs/design/post-release/palette-cycling-demo-quarter.py</FILE> - <DESC>Quarter-cell variant of palette-cycling-demo.py that packs FOUR samples into each terminal cell using the Unicode 2×2 quadrant glyphs (U+2580..U+259F). Because a terminal cell has only one fg slot and one bg slot, the four sample colours are approximated each frame by the best 2-colour partition of the four quadrants — the partition with minimum sum-of-squared deviations from the two cluster means. For palette-cycling content (continuous index field + smoothly-interpolated palette), the four quadrants of any 2×2 block are nearly-adjacent palette entries and cluster cleanly, so the lossy approximation is visually mild. Sibling: palette-cycling-demo.py (v0.3, lossless 2× horizontal half-block variant).</DESC>
# <VERS>VERSION: 0.1.0</VERS>
# <WCTX>Push the resolution further by encoding 4 samples per cell via 2×2 quadrant glyphs with per-cell 2-colour partition selection; document the lossiness honestly so the v0.3 vs v0.4 trade-off is legible.</WCTX>
# <CLOG>0.1.0: new file derived from palette-cycling-demo.py v0.3; field generated 2W × 2H wide; per-cell render picks best 2-of-4 colour partition over the seven non-trivial 2×2 bipartitions and emits the matching Block-Elements quadrant glyph; PALETTE_SIZE 32→64; rings aspect correction rescaled (dx*0.5, dy*1.1) for the new sub-cell grid; plasma/waves multipliers halved to preserve on-screen pattern dimensions vs v0.3.</CLOG>

"""
Amiga-demoscene palette-cycling demo — quarter-cell variant.

────────────────────────────────────────────────────────────────────────
Run:   python3 palette-cycling-demo-quarter.py
Keys:  1, 2, 3  — switch pattern preset
       q        — quit (Ctrl-C also works)
────────────────────────────────────────────────────────────────────────

WHAT THIS DEMONSTRATES

This is the v0.4 step on the resolution ladder:

    v0.2  one sample per cell  (bg + space)            78 × 18 = 1404 samples
    v0.3  two samples per cell (▌ fg=left, bg=right)   78 × 18 × 2 = 2808
    v0.4  four samples per cell (2×2 quadrant glyphs)  78 × 18 × 4 = 5616

Each preset still bundles two pure-data inputs:

  1. an *index field* — now a (2W × 2H) grid of integer palette indices,
     computed ONCE when the preset is selected and never modified;
  2. a *palette*      — a list of N (R,G,B) tuples, also computed once.

Per-frame work for each terminal cell:

       quadrant indices  = field[2y..2y+1][2x..2x+1]  (4 ints; static)
       quadrant colours  = palette[(idx + offset) % N] for each idx
       (glyph, fg, bg)   = best_2colour_partition(four quadrant colours)

`offset` advancing one slot per frame remains the only per-frame state.
The visible motion is still the eye seeing each index get bound to a
different colour each frame — not the field changing, not the palette
being recomputed.

THE 2-COLOUR CONSTRAINT (and how we work around it)

A terminal cell carries exactly one foreground colour and one background
colour. The 2×2 quadrant glyphs (▘▝▖▗▀▄▌▐▙▟▜▛▞▚█) let us choose WHICH
quadrants get the fg colour and which get the bg colour, but every
"fg quadrant" within a cell shares one RGB triple, and every "bg
quadrant" shares another. A 2×2 block of four arbitrary colours cannot
be rendered exactly — we have to approximate.

The approximation, per cell per frame:

  1. Look up the four quadrant colours c_UL, c_UR, c_LL, c_LR.
  2. Score each of the seven non-trivial bipartitions of {UL,UR,LL,LR}
     into a "side A" subset and a "side B" subset. For each partition
     the optimal two colours are the componentwise means of the two
     sides; the cost is the sum of squared deviations of each quadrant
     from its side's mean.
  3. Pick the lowest-cost partition. Emit the matching quadrant glyph
     with fg = mean of fg-side, bg = mean of bg-side.

WHY THIS WORKS WELL FOR PALETTE-CYCLING SPECIFICALLY

The field generators are continuous functions, so the four indices in
any 2×2 block are usually adjacent integers like (k, k, k+1, k+1). The
palette is keyframe-interpolated, so adjacent indices map to nearly-
adjacent colours. Most 2×2 blocks therefore contain only ~2 distinct
colour clusters by construction — the partition cost is near zero and
the rendered cell is almost a faithful reproduction. The lossiness
shows up only at boundaries between palette bands, where it appears as
a one-sub-cell-wide "dithered" edge.

HOW THIS DIFFERS FROM v0.3

v0.3 (palette-cycling-demo.py) is lossless: every sample's colour is
exactly palette[(idx + offset) % N]. v0.4 is lossy: pairs of adjacent
samples are averaged into one fg or one bg. The architectural payoff
(field static, palette rotates, lookup binds at render time) still
holds — there's just one extra pure function in the lookup chain.

OTHER LADDER RUNGS NOT TAKEN

  • Sextants (U+1FB00..U+1FB3B), 2×3 = 6 samples — same 2-colour
    partition constraint, sparser font support.
  • Octants (U+1CD00.., Unicode 16, 2024), 2×4 = 8 — same constraint,
    bleeding-edge font support.
  • Braille (U+2800..U+28FF), 2×4 dot bitmask — ALL eight dots share
    one foreground colour. Wrong tool for palette cycling, where
    colour-per-sample IS the point.
  • Sixel / Kitty graphics protocol / iTerm inline images — true per-
    pixel colour, bypassing the cell grid; terminal-specific.
"""

import math
import select
import sys
import termios
import time
import tty

# ============================================================================
# Tunables — change any of these and rerun
# ============================================================================

W, H         = 78, 18    # terminal size in cells
SUB_W        = W * 2     # sub-columns — two horizontal samples per cell
SUB_H        = H * 2     # sub-rows    — two vertical   samples per cell
PALETTE_SIZE = 64        # palette entries; doubled again from v0.3 (32) to keep gradients smooth at 4× spatial density
FPS          = 14        # frames per second; 60 also works on modern terms
DIRECTION    = +1        # +1 = palette rotates one way, -1 = the other

# ============================================================================
# Terminal escape codes
# ============================================================================

ESC         = "\x1b"
def bg(r, g, b):  return f"{ESC}[48;2;{r};{g};{b}m"
def fgbg(fc, bc): return f"{ESC}[38;2;{fc[0]};{fc[1]};{fc[2]};48;2;{bc[0]};{bc[1]};{bc[2]}m"
HALF        = "▌"        # used in the swatch only — fg=left, bg=right
RESET       = f"{ESC}[0m"
HIDE_CURSOR = f"{ESC}[?25l"
SHOW_CURSOR = f"{ESC}[?25h"
HOME        = f"{ESC}[H"
CLEAR       = f"{ESC}[2J"

# ============================================================================
# Palette construction
# ============================================================================
# A palette is a list of (R, G, B) tuples. We build palettes by interpolating
# between named "keyframe" colours and looping back to the first keyframe so
# rotation is seamless: when the offset wraps around past N, the colour at
# entry 0 matches the colour at entry N, with no visible jump.

def interpolate_palette(keyframes, n):
    """Build an n-entry palette by interpolating cyclically between keyframes."""
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
    keys = [(10, 40, 80), (40, 100, 180), (120, 200, 255), (40, 100, 180)]
    return interpolate_palette(keys, n)

def fire_palette(n):
    """Dark red → red → orange → yellow → back through orange/red. Hot."""
    keys = [(20, 0, 0), (200, 30, 0), (255, 140, 30), (255, 230, 100),
            (255, 140, 30), (200, 30, 0)]
    return interpolate_palette(keys, n)

def neon_palette(n):
    """Deep purple → magenta → pink → cyan loop. Synthwave."""
    keys = [(20, 0, 40), (180, 40, 255), (255, 80, 200), (40, 200, 255),
            (180, 40, 255)]
    return interpolate_palette(keys, n)

# ============================================================================
# Index-field generators — the *spatial pattern* axis
# ============================================================================
# Each function returns a 2D list of ints in [0, n), at the SUB-cell grid
# (SUB_W × SUB_H). Coefficients on x and y are halved relative to the v0.3
# (horizontal-only) demo so on-screen pattern dimensions stay comparable —
# the visible difference between v0.3 and v0.4 is then about resolution and
# edge smoothness, not pattern density.

def rings_indices(w, h, n):
    """Concentric circles around the centre.

    Aspect terms convert (sub-x, sub-y) to a screen-isotropic distance:
    each sub-x is half a cell wide → multiply dx by 0.5; each sub-y is
    half a cell tall × ~2.2 cell-widths-per-cell-height → multiply dy
    by 1.1. Without these the rings would render as ellipses.
    """
    cx, cy = w / 2, h / 2
    field  = []
    for y in range(h):
        row = []
        for x in range(w):
            dx = (x - cx) * 0.5
            dy = (y - cy) * 1.1
            d  = math.sqrt(dx * dx + dy * dy)
            row.append(int(d / 1.4) % n)
        field.append(row)
    return field

def plasma_indices(w, h, n):
    """Classic demoscene plasma: sum of multiple sine waves.

    Frequency multipliers are halved vs v0.3 because x and y now span
    twice as many sub-cells; halving keeps the on-screen blob size
    comparable to v0.3's horizontal-only doubling.
    """
    field = []
    for y in range(h):
        row = []
        for x in range(w):
            v = (math.sin(x * 0.15) +
                 math.sin(y * 0.25) +
                 math.sin((x + y) * 0.10) +
                 math.sin(math.sqrt(x * x + y * y) * 0.125))
            v_norm = (v + 4) / 8
            row.append(int(v_norm * n) % n)
        field.append(row)
    return field

def waves_indices(w, h, n):
    """Diagonal stripes warped by perpendicular sinusoidal modulation.

    Multipliers halved as for plasma; same rationale.
    """
    field = []
    for y in range(h):
        row = []
        for x in range(w):
            v = ((x + y) * 0.2
                 + math.sin(x * 0.15) * 2.5
                 + math.cos(y * 0.20) * 1.5)
            row.append(int(v) % n)
        field.append(row)
    return field

# ============================================================================
# Presets — bundles of (name, pattern, palette)
# ============================================================================

PRESETS = [
    {"name": "Rings (water)",   "indices": rings_indices,   "palette": water_palette},
    {"name": "Plasma (fire)",   "indices": plasma_indices,  "palette": fire_palette},
    {"name": "Waves (neon)",    "indices": waves_indices,   "palette": neon_palette},
]

# ============================================================================
# Quadrant partition selection
# ============================================================================
# Given the four quadrant colours (UL, UR, LL, LR), choose the bipartition of
# the four quadrants into a "fg side" and a "bg side" that minimises the sum
# of squared deviations of each quadrant from its side's mean colour. Return
# the matching Unicode 2×2 quadrant glyph plus the (fg, bg) cluster means.
#
# We enumerate the seven canonical non-trivial bipartitions (the other seven
# are these with fg/bg labels swapped — same cost, just complement glyph):
#
#   1-vs-3:  {UL}     ▘ , {UR}     ▝ , {LL}     ▖ , {LR}     ▗
#   2-vs-2:  {UL,UR}  ▀ , {UL,LL}  ▌ , {UL,LR}  ▚    (top, left, anti-diag)
#
# All other 2×2 partitions reduce to one of the above by relabelling.

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

    # Four 1-vs-3 partitions: one quadrant is the fg side, the other three
    # are the bg side. Cost = variance of the three bg-side colours.
    for i, glyph in enumerate(("▘", "▝", "▖", "▗")):
        others = [quads[j] for j in range(4) if j != i]
        bg_mean = _mean3(*others)
        cost    = (_sqd(others[0], bg_mean)
                 + _sqd(others[1], bg_mean)
                 + _sqd(others[2], bg_mean))
        if best_cost is None or cost < best_cost:
            best_cost, best_glyph, best_fg, best_bg = cost, glyph, quads[i], bg_mean

    # Three 2-vs-2 partitions. Cost = variance of each pair around its mean.
    for (a, b, c, d), glyph in (
        ((0, 1, 2, 3), "▀"),   # top {UL,UR} vs bottom {LL,LR}
        ((0, 2, 1, 3), "▌"),   # left {UL,LL} vs right {UR,LR}
        ((0, 3, 1, 2), "▚"),   # diagonal {UL,LR} vs anti-diagonal {UR,LL}
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

def render(indices, palette, offset, preset_idx):
    """Paint one frame to stdout, overwriting the previous frame in place.

    indices is SUB_H rows × SUB_W cols. Each terminal cell consumes a 2×2
    sub-block: rows 2y, 2y+1 × cols 2x, 2x+1. Per cell we look up four
    palette colours, pick the best 2-colour partition, and emit one
    quadrant glyph with the chosen (fg, bg).
    """
    out = [HOME, RESET]

    # Header / menu.
    out.append("Amiga palette-cycling demo (quarter cells)  —  press 1/2/3 to switch, q to quit\n")
    out.append("Index field is STATIC; only the palette rotates per frame. Quadrant glyph chosen per cell per frame.\n")
    out.append("Menu: ")
    for i, p in enumerate(PRESETS):
        marker = "▶" if i == preset_idx else " "
        out.append(f"{marker}{i + 1}:{p['name']}  ")
    out.append("\n\n")

    # Live palette swatch — uses the v0.3 half-block trick to fit 64 entries
    # in 32 cells without losing per-entry visibility as offset rotates.
    out.append("Palette → ")
    n = len(palette)
    for i in range(0, n, 2):
        cl = palette[(i     + offset) % n]
        cr = palette[(i + 1 + offset) % n]
        out.append(fgbg(cl, cr) + HALF)
    out.append(RESET + "\n\n")

    # The field, rendered with per-cell quadrant selection. Coalescing on
    # (glyph, fg, bg) triples is rarer than on the v0.3 (fg, bg) pair, so
    # we coalesce only on identical-triple runs.
    last = None
    for cy in range(0, SUB_H, 2):
        row_top = indices[cy]
        row_bot = indices[cy + 1]
        for cx in range(0, SUB_W, 2):
            c_ul = palette[(row_top[cx]     + offset) % n]
            c_ur = palette[(row_top[cx + 1] + offset) % n]
            c_ll = palette[(row_bot[cx]     + offset) % n]
            c_lr = palette[(row_bot[cx + 1] + offset) % n]
            glyph, fg, bgc = choose_quadrant(c_ul, c_ur, c_ll, c_lr)
            triple = (glyph, fg, bgc)
            if triple != last:
                out.append(fgbg(fg, bgc))
                last = triple
            out.append(glyph)
        out.append(RESET + "\n")
        last = None  # SGR resets at line end

    sys.stdout.write("".join(out))
    sys.stdout.flush()

# ============================================================================
# Non-blocking single-key input
# ============================================================================

def read_key():
    """Return the next pending keypress (one char) or None if nothing waiting."""
    if select.select([sys.stdin], [], [], 0)[0]:
        return sys.stdin.read(1)
    return None

# ============================================================================
# Main loop
# ============================================================================

def main():
    preset_idx = 0
    indices = PRESETS[0]["indices"](SUB_W, SUB_H, PALETTE_SIZE)
    palette = PRESETS[0]["palette"](PALETTE_SIZE)

    has_tty = sys.stdin.isatty()
    fd = sys.stdin.fileno() if has_tty else None
    old_termios = termios.tcgetattr(fd) if has_tty else None
    if has_tty:
        tty.setcbreak(fd)

    sys.stdout.write(HIDE_CURSOR + CLEAR)
    sys.stdout.flush()

    offset = 0
    delay  = 1.0 / FPS

    try:
        while True:
            if has_tty:
                while True:
                    key = read_key()
                    if key is None:
                        break
                    if key in ("q", "Q"):
                        return
                    if key.isdigit():
                        n = int(key) - 1
                        if 0 <= n < len(PRESETS):
                            preset_idx = n
                            indices = PRESETS[n]["indices"](SUB_W, SUB_H, PALETTE_SIZE)
                            palette = PRESETS[n]["palette"](PALETTE_SIZE)
                            sys.stdout.write(CLEAR)

            render(indices, palette, offset, preset_idx)
            offset = (offset + DIRECTION) % len(palette)
            time.sleep(delay)
    except KeyboardInterrupt:
        pass
    finally:
        sys.stdout.write(SHOW_CURSOR + RESET + "\n")
        sys.stdout.flush()
        if has_tty and old_termios is not None:
            termios.tcsetattr(fd, termios.TCSADRAIN, old_termios)

if __name__ == "__main__":
    main()

# <FILE>docs/design/post-release/palette-cycling-demo-quarter.py</FILE>
# <VERS>END OF VERSION: 0.1.0</VERS>
