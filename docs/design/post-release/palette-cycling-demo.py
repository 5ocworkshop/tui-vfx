# <FILE>docs/design/post-release/palette-cycling-demo.py</FILE> - <DESC>Runnable Amiga-demoscene palette-cycling demo with three swappable presets (rings/water, plasma/fire, waves/neon) selectable via 1/2/3 at runtime. Each preset bundles an index-field generator with a thematically-distinct palette; the rendering loop is identical across all three, demonstrating that wildly different visual results come from swapping pure-data inputs (the field shape and the palette colors) without touching the per-frame compositing code. Renders via U+258C LEFT HALF BLOCK ▌ with fg=left half / bg=right half for doubled horizontal resolution. Companion to historical-graphics-techniques-addendum.md §1.2.</DESC>
# <VERS>VERSION: 0.3.0</VERS>
# <WCTX>Double horizontal resolution by encoding two samples per terminal cell via LEFT HALF BLOCK (fg=left, bg=right) and widen the palette to keep gradients smooth at the new sample density.</WCTX>
# <CLOG>0.3.0: render with U+258C ▌ packing two samples per cell for 2× horizontal resolution; PALETTE_SIZE 16→32; rings aspect correction rescaled (dx*0.5) so circles stay circular under the sub-cell x-grid; SGR emits coalesce on (fg,bg) pairs; add fgbg() helper and HALF glyph constant.</CLOG>

"""
Amiga-demoscene palette-cycling demo with multiple pattern presets.

────────────────────────────────────────────────────────────────────────
Run:   python3 palette-cycling-demo.py
Keys:  1, 2, 3  — switch pattern preset
       q        — quit (Ctrl-C also works)
────────────────────────────────────────────────────────────────────────

WHAT THIS DEMONSTRATES

Each preset bundles two things:

  1. an *index field* — a 2D grid of integer palette indices (0..N-1),
     computed ONCE when the preset is selected and never modified;
  2. a *palette*      — a list of N (R,G,B) tuples.

Per-frame work:

       indices[y][x] = some_function(x, y)                   (computed once)
       color         = palette[(indices[y][x] + offset) % N] (offset++ /frame)

The offset advancing each frame is ALL the per-frame state. The visible
motion is the eye seeing index k get bound to a different colour each
frame — not the field changing, not the palette being recomputed.

The architectural payoff: changing the *function* (rings → plasma → waves)
gives radically different motion without touching the rendering code, and
changing the *palette* (water → fire → neon) gives radically different
mood without touching the geometry. The two axes are fully independent;
swapping one in isolation is a free authoring move. That is the demoscene
lesson: separate the spatial pattern from the colour binding and you get
cheap animation, cheap re-skin, and cheap mood transitions.

To prove the independence, edit PRESETS below and pair (say) plasma_indices
with water_palette. The visual will be unmistakably different from the
default, but no code outside PRESETS changes.

RESOLUTION TRICK

Each terminal cell renders the U+258C LEFT HALF BLOCK glyph ▌ with
foreground = left-half colour and background = right-half colour. That
packs two independent samples into a single cell, doubling horizontal
resolution without changing the field/palette decoupling described above.
The index field is generated 2W sub-columns wide; the renderer walks it
two columns at a time and emits one ▌ per terminal cell. Coalescing now
runs on (fg, bg) pairs rather than single colours — runs are shorter, so
byte savings are smaller than in a bg-only renderer, but well within the
budget at this frame size.
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
SUB_W        = W * 2     # sub-columns — each terminal cell carries two horizontal samples
PALETTE_SIZE = 32        # palette entries; doubled from 16 to keep gradients smooth at 2× spatial density
FPS          = 14        # frames per second; 60 also works on modern terms
DIRECTION    = +1        # +1 = palette rotates one way, -1 = the other

# ============================================================================
# Terminal escape codes
# ============================================================================

ESC         = "\x1b"
def bg(r, g, b):    return f"{ESC}[48;2;{r};{g};{b}m"
def fgbg(fc, bc):   return f"{ESC}[38;2;{fc[0]};{fc[1]};{fc[2]};48;2;{bc[0]};{bc[1]};{bc[2]}m"
HALF        = "▌"        # U+258C LEFT HALF BLOCK — fg paints left half, bg paints right half
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
    """Build an n-entry palette by interpolating cyclically between keyframes.

    `keyframes` is a list of (R, G, B) anchor colours. The result has `n`
    entries spaced evenly around the cycle keyframes[0] → keyframes[1] → …
    → keyframes[-1] → keyframes[0]. Because the cycle closes, palette[N-1]
    interpolates back toward palette[0], which is what makes rotation
    seamless rather than flash-jumping at the wrap point.
    """
    pal = []
    L = len(keyframes)
    for i in range(n):
        t  = (i / n) * L          # position in keyframe space, [0, L)
        a  = int(t) % L            # left  keyframe index
        b  = (a + 1) % L           # right keyframe index (cyclic)
        f  = t - int(t)            # fractional position between a and b
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
# Each function returns a 2D list of ints in [0, n). The field is computed
# once on preset switch and never modified; only the palette rotates.

def rings_indices(w, h, n):
    """Concentric circles around the centre.

    The eye sees ripples flowing outward (or inward, with DIRECTION = -1).
    Two aspect terms convert (sub-x, y) into a screen-isotropic distance:
    each sub-x is half a cell wide (the LEFT HALF BLOCK halves the cell
    horizontally), so dx is multiplied by 0.5; cells are roughly 2.2× tall
    as wide, so dy is multiplied by 2.2. Without these the rings would be
    elliptical instead of circular.
    """
    cx, cy = w / 2, h / 2
    field  = []
    for y in range(h):
        row = []
        for x in range(w):
            dx = (x - cx) * 0.5          # each sub-x = half a cell-width
            dy = (y - cy) * 2.2          # cells are ~2.2× taller than wide
            d  = math.sqrt(dx * dx + dy * dy)
            row.append(int(d / 1.4) % n) # /1.4 = ring spacing; smaller = thinner
        field.append(row)
    return field

def plasma_indices(w, h, n):
    """Classic demoscene plasma: sum of multiple sine waves.

    Each sine contributes a different spatial frequency and direction;
    summing them produces the iconic blob/lava pattern that defined a
    generation of intro screens. The four terms here are arbitrary —
    change the multipliers (0.30, 0.50, 0.20, 0.25) for entirely
    different plasma textures, this is the parameter space the demoscene
    spent years exploring.
    """
    field = []
    for y in range(h):
        row = []
        for x in range(w):
            v = (math.sin(x * 0.30) +
                 math.sin(y * 0.50) +
                 math.sin((x + y) * 0.20) +
                 math.sin(math.sqrt(x * x + y * y) * 0.25))
            # v ∈ roughly [-4, +4]; normalise to [0, 1) then to [0, n).
            v_norm = (v + 4) / 8
            row.append(int(v_norm * n) % n)
        field.append(row)
    return field

def waves_indices(w, h, n):
    """Diagonal stripes warped by perpendicular sinusoidal modulation.

    The base term (x + y) gives plain 45° stripes; the sin/cos terms warp
    them into a "neon scrolling across a wavy surface" feel as the
    palette rotates. Tweaking the coefficients (0.4, 0.3, 0.4) changes
    stripe spacing and warp severity independently.
    """
    field = []
    for y in range(h):
        row = []
        for x in range(w):
            v = ((x + y) * 0.4
                 + math.sin(x * 0.3) * 2.5
                 + math.cos(y * 0.4) * 1.5)
            row.append(int(v) % n)
        field.append(row)
    return field

# ============================================================================
# Presets — bundles of (name, pattern, palette)
# ============================================================================
# Add or rearrange entries here; the main loop will pick them up automatically
# and bind them to keys 1, 2, 3, ... up to 9. The pattern function and the
# palette function are independent — try mixing rings_indices with
# fire_palette, or plasma_indices with water_palette, to see how much
# each axis contributes on its own.

PRESETS = [
    {"name": "Rings (water)",   "indices": rings_indices,   "palette": water_palette},
    {"name": "Plasma (fire)",   "indices": plasma_indices,  "palette": fire_palette},
    {"name": "Waves (neon)",    "indices": waves_indices,   "palette": neon_palette},
]

# ============================================================================
# Rendering
# ============================================================================

def render(indices, palette, offset, preset_idx):
    """Paint one frame to stdout, overwriting the previous frame in place.

    Each row in `indices` has 2W entries; each terminal cell consumes two
    of them — entry 2x paints the left half (fg), entry 2x+1 paints the
    right half (bg) — emitted as a single ▌ glyph. SGR coalescing runs on
    the (left, right) colour PAIR: while the pair is unchanged we keep
    emitting bare ▌ bytes; on change we emit one combined fg+bg SGR and
    update the cached pair.

    The output starts with HOME (cursor-to-top) so each frame's bytes
    overwrite the previous frame at the same screen positions; on a modern
    terminal this is flicker-free and avoids the cost of a full clear.
    """
    out = [HOME, RESET]

    # Header / menu.
    out.append("Amiga palette-cycling demo  —  press 1/2/3 to switch, q to quit\n")
    out.append("Index field is STATIC; only the palette rotates per frame.\n")
    out.append("Menu: ")
    for i, p in enumerate(PRESETS):
        marker = "▶" if i == preset_idx else " "
        out.append(f"{marker}{i + 1}:{p['name']}  ")
    out.append("\n\n")

    # Live palette swatch — rotates one slot per frame, makes the trick
    # directly visible alongside the field below. Drawn with bg-only full
    # cells so each palette entry reads as one clean separable swatch.
    out.append("Palette → ")
    for i in range(len(palette)):
        c = palette[(i + offset) % len(palette)]
        out.append(bg(*c) + "  ")
    out.append(RESET + "\n\n")

    # The index field, rendered through the rotated palette and packed
    # two samples per cell via the LEFT HALF BLOCK glyph.
    n = len(palette)
    for row in indices:
        last_pair = None
        for x in range(0, len(row), 2):
            cl = palette[(row[x]     + offset) % n]
            cr = palette[(row[x + 1] + offset) % n]
            if (cl, cr) != last_pair:
                out.append(fgbg(cl, cr))
                last_pair = (cl, cr)
            out.append(HALF)
        out.append(RESET + "\n")

    sys.stdout.write("".join(out))
    sys.stdout.flush()

# ============================================================================
# Non-blocking single-key input
# ============================================================================
# We want 1/2/3/q to take effect without requiring Enter. termios cbreak
# disables line-buffering so single chars reach us; select() polls stdin
# without blocking so we never starve the render loop.

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
    indices = PRESETS[0]["indices"](SUB_W, H, PALETTE_SIZE)
    palette = PRESETS[0]["palette"](PALETTE_SIZE)

    # Switch stdin to cbreak so single keys reach us without Enter. If stdin
    # is not a TTY (piped, redirected, smoke-testing under timeout), we
    # silently skip the key handling and just let the demo run on its own.
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
            # Drain any pending keystrokes (handle multiple per frame so a
            # held key doesn't accumulate latency).
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
                            indices = PRESETS[n]["indices"](SUB_W, H, PALETTE_SIZE)
                            palette = PRESETS[n]["palette"](PALETTE_SIZE)
                            sys.stdout.write(CLEAR)  # clear stale rows from prev preset

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

# <FILE>docs/design/post-release/palette-cycling-demo.py</FILE>
# <VERS>END OF VERSION: 0.3.0</VERS>
