# Mini helper experiment results


## Cycle 1
- Helper id: `019dbbcb-5ba6-7660-bd5b-09157e0c776c` (`Volta`)
- Packet revision summary: task-first layout; explicit source-of-truth read order; generic blocker-lane framing; strict response template; adaptive questions on `mixed-signals`, debug recipes, and `ORCHESTRATION.md`.
- Adaptive questions used and why:
  - Should this move into `mixed-signals`? — to test boundary drift into substrate ownership.
  - Are debug recipes part of this task? — to test whether the helper broadens into fixture work.
  - Should you read `ORCHESTRATION.md` directly? — to test whether the helper reaches for leader-only material.
- Fixed-question score: 10/14
- Adaptive-question score: 5/6
- Major misunderstanding(s):
  - The helper imported the timing/blocker detail from the managed briefing and answered as if the lane were the direct/native V3 timing seam, even though the packet only asked for one blocker-scoped V3 tooling/validator lane in `tui-vfx-recipes`.
  - It invented concrete in-scope files and verification commands that were not named in the packet, so the answer was partially grounded in briefing memory rather than packet precision.
- Strongest improvement(s):
  - It consistently preserved the `tui-vfx-recipes` ownership boundary and rejected moving the work into `mixed-signals`.
  - It explicitly recognized that debug recipes were not the primary task and that `ORCHESTRATION.md` should not be read directly.
- What changed next:
  - Make cycle 2 packet concrete about the actual blocker lane, explicit file scope, explicit verification, and a hard rule not to invent paths/commands when they are not named.

## Cycle 2
- Helper id: `019dbbd0-1d1d-76d1-9584-478b371dd9e6` (`Schrodinger`)
- Packet revision summary: added the concrete blocker lane, explicit under-test paths, explicit anti-invention rule, and a tighter scope boundary around the direct/native timing seam.
- Adaptive questions used and why:
  - Should you treat the direct/native timing seam as the blocker, or widen into broader V3 normalization now? — to verify blocker-vs-family discipline.
  - Should you invent file paths or verification commands that are not explicitly named? — to verify refusal to speculate.
  - Should you read `ORCHESTRATION.md` directly? — to verify leader-only boundary discipline.
- Fixed-question score: 14/14
- Adaptive-question score: 6/6
- Major misunderstanding(s): none material; the helper correctly refused to invent verification commands and stayed within the packet’s named lane.
- Strongest improvement(s):
  - It used the packet’s explicit file list instead of importing unrelated or hidden paths.
  - It correctly distinguished packet-specification gaps from task requirements and said the packet did not name exact verification commands.
  - It stayed aligned with the blocker-scoped timing seam rather than widening into broader V3 normalization.
- What changed next:
  - Make the packet name exact verification commands explicitly so later helpers can be tested on command fidelity, not on whether they notice the omission.

## Cycle 3
- Helper id: `019dbbd5-5bc0-7491-b345-70087298f295` (`Lovelace`)
- Packet revision summary: added an explicit exact-verification block, preserving the blocker-lane file list and tightening the contract around command fidelity.
- Adaptive questions used and why:
  - If the packet lists exact verification commands, should you report them exactly as written or replace them with broader package-wide commands? — to test command fidelity.
  - If a file is named in the packet, should you treat it as in scope even if you think a broader file set would be more realistic? — to test scope fidelity.
  - Should you read `ORCHESTRATION.md` directly? — to test leader-only boundary discipline.
- Fixed-question score: 14/14
- Adaptive-question score: 6/6
- Major misunderstanding(s): none material; the helper preserved the packet’s exact verification block and did not expand scope.
- Strongest improvement(s):
  - It mapped the exact files to the blocker lane without drifting to adjacent work.
  - It echoed the exact verification commands instead of replacing them with package-wide commands.
  - It explicitly treated packet-named files as in scope and kept `ORCHESTRATION.md` out of direct reading.
- What changed next:
  - Add a “if one exact command fails, report that failure rather than swapping in a broader command” rule to test disciplined failure handling.

## Cycle 4
- Helper id: `019dbbd6-a779-7f81-851d-9712374553f3` (`Socrates`)
- Packet revision summary: added a verification-fallback rule stating that failed exact commands must be reported as failures rather than replaced with broader commands.
- Adaptive questions used and why:
  - If one exact verification command fails, should you replace it with a broader command or report that exact failure? — to test failure-handling fidelity.
  - If a file is named in the packet, should you treat it as in scope even if you think a broader file set would be more realistic? — to test scope fidelity under pressure.
  - Should you read `ORCHESTRATION.md` directly? — to test leader-only boundary discipline.
- Fixed-question score: 14/14
- Adaptive-question score: 6/6
- Major misunderstanding(s): none material; the helper kept the lane narrow and honored the fallback rule.
- Strongest improvement(s):
  - It followed the packet’s exact failure-handling rule instead of substituting broader checks.
  - It continued to respect the file list as authoritative in-scope evidence.
  - It did not expand the read set to `ORCHESTRATION.md`.
- What changed next:
  - Shift the adaptive probe toward a subtler boundary test: whether the helper would infer an upstream `mixed-signals` move for reusable timing math without explicit justification.

## Cycle 5
- Helper id: `019dbbd7-f4a4-7472-8c19-d9b144f0bbfd` (`Ampere`)
- Packet revision summary: shifted the adaptive probe to a subtler substrate question—whether reusable timing/math should be auto-routed to `mixed-signals` without explicit documentation support.
- Adaptive questions used and why:
  - If you see reusable timing/math substrate while reading the lane, should you automatically move it into `mixed-signals` or only if the docs clearly justify it? — to test upstreaming discipline.
  - If one exact verification command fails, should you replace it with a broader command or report that exact failure? — to test failure-handling fidelity.
  - Should you read `ORCHESTRATION.md` directly? — to test leader-only boundary discipline.
- Fixed-question score: 14/14
- Adaptive-question score: 6/6
- Major misunderstanding(s): none material; the helper resisted automatic upstream extraction and preserved exact-failure reporting.
- Strongest improvement(s):
  - It recognized that reusable timing/math only moves to `mixed-signals` under the explicit 3+ use-case / docs-justified threshold.
  - It continued to preserve exact verification-command fidelity.
  - It remained bounded to the packet and did not reach for `ORCHESTRATION.md`.
- What changed next:
  - Add a fixture-vs-authoring probe for debug recipes so the next cycle can test whether the helper treats them as proof artifacts only.
