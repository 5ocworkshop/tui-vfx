# <FILE>docs/design/post-release/ansi-techniques-demo.py</FILE> - <DESC>Generator for ansi-techniques-demo.ans — emits a truecolor + ▓▒░ block-shade demonstration of (1) block-character anti-aliasing at a color boundary and (2) stipple-pattern intermediate colors, the two textmode-scene techniques described in historical-graphics-techniques-addendum.md §1. Tweak A, B, or W and rerun to regenerate.</DESC>
# <VERS>VERSION: 0.1.0</VERS>
# <WCTX>Provide a runnable, viewable demonstration of the addendum's two block-character textmode techniques so they can be `cat`-viewed in any truecolor terminal.</WCTX>
# <CLOG>0.1.0: initial generator — A/B truecolor pair, hard-edge vs density-transition AA, vertical density fade, stipple intermediate-color comparison against an interpolated truecolor midpoint, bonus 5-step stipple gradient vs RGB-lerp gradient.</CLOG>

ESC = "\x1b"

def fg(r, g, b): return f"{ESC}[38;2;{r};{g};{b}m"
def bg(r, g, b): return f"{ESC}[48;2;{r};{g};{b}m"

RESET = f"{ESC}[0m"

A = (255, 140,  50)   # warm orange
B = ( 40,  80, 200)   # cool blue
M = ((A[0] + B[0]) // 2, (A[1] + B[1]) // 2, (A[2] + B[2]) // 2)
W = 12                # color-block width in cells

def lerp(a, b, t):
    return tuple(int(a[i] + (b[i] - a[i]) * t) for i in range(3))

L = []
def p(s=""): L.append(s)

RULE = "═" * 72

p(RESET + RULE)
p("  ANSI techniques demo — block-character AA + stipple intermediate colors")
p("  A = warm orange (255, 140,  50)    B = cool blue ( 40,  80, 200)")
p(RULE)
p()
p("─── 1. Block-character anti-aliasing at a color boundary ─────────────────")
p()
p("Hard edge (raw color jump, no anti-aliasing):")
for _ in range(3):
    p("  " + bg(*A) + " " * W + bg(*B) + " " * W + RESET)
p()
p("Smoothed edge with ▓▒░ density transition (FG=A, BG=B):")
for _ in range(3):
    p("  " + bg(*A) + " " * W + bg(*B) + fg(*A) + "▓▒░" + " " * (W - 3) + RESET)
p()
p("Vertical fade — pure A · ▓ · ▒ · ░ · pure B (FG=A, BG=B):")
p("  " + bg(*A) + " " * W + RESET)
p("  " + bg(*B) + fg(*A) + "▓" * W + RESET)
p("  " + bg(*B) + fg(*A) + "▒" * W + RESET)
p("  " + bg(*B) + fg(*A) + "░" * W + RESET)
p("  " + bg(*B) + " " * W + RESET)
p()
p("─── 2. Stipple-pattern intermediate colors ───────────────────────────────")
p()
p("  Pure A               " + bg(*A) + " " * W + RESET)
p("  ▓  (≈75% A, 25% B)   " + bg(*B) + fg(*A) + "▓" * W + RESET)
p("  ▒  (≈50% A, 50% B)   " + bg(*B) + fg(*A) + "▒" * W + RESET)
p("  ░  (≈25% A, 75% B)   " + bg(*B) + fg(*A) + "░" * W + RESET)
p("  Pure B               " + bg(*B) + " " * W + RESET)
p()
p("  Truecolor midpoint reference (computed (A+B)/2):")
p("  Midpoint             " + bg(*M) + " " * W + RESET)
p()
p("  Compare the ▒ row above to the midpoint row — perceptually similar,")
p("  but the ▒ version uses ONLY the two source colors A and B. 16-color")
p("  terminals faked colors they didn't have this way; with truecolor, the")
p("  same trick adds 1–2 effective bits of perceived range by spatially")
p("  dithering between exactly-displayable colors.")
p()
p("─── Bonus: stipple gradient vs. truecolor gradient ───────────────────────")
p()
p("  5-step gradient using only A, B, and density (no color interpolation):")
p("  " + bg(*A) + " " * 4 + RESET
       + bg(*B) + fg(*A) + "▓" * 4 + RESET
       + bg(*B) + fg(*A) + "▒" * 4 + RESET
       + bg(*B) + fg(*A) + "░" * 4 + RESET
       + bg(*B) + " " * 4 + RESET)
p()
p("  5-step gradient using truecolor RGB interpolation:")
chunks = []
for t in (0.0, 0.25, 0.5, 0.75, 1.0):
    chunks.append(bg(*lerp(A, B, t)) + " " * 4 + RESET)
p("  " + "".join(chunks))
p()
p("  At normal viewing distance the two rows should look very close.")
p("  Up close, the stipple version reveals its character-density pattern;")
p("  from a foot or two away, the eye averages each cell into an apparent")
p("  intermediate color for free.")
p()

OUT = "/usr/projects/tui-vfx/docs/design/post-release/ansi-techniques-demo.ans"
with open(OUT, "w", encoding="utf-8") as f:
    f.write("\n".join(L) + "\n")

print(f"wrote {OUT} ({sum(len(s) for s in L)} chars across {len(L)} lines)")

# <FILE>docs/design/post-release/ansi-techniques-demo.py</FILE>
# <VERS>END OF VERSION: 0.1.0</VERS>
