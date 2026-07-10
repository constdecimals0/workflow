# Example run log — Simon Says terminal game

Verbatim capture of the example loop that builds the Simon Says app in `example/app/`,
recorded for the README effort's map tickets "Run the planning phase of the example loop" and
"Write the tutorial". (That map lived on the local-markdown tracker in `.scratch/workflow-readme/`
and was pruned at the effort's close-out, per the loop's own cleanup rule.)

**Capture method**: every session below is transcribed from the harness transcripts in
`~/.claude/projects/-Users-coachchucksol-Documents-Lhava-Workflow/*.jsonl`, which record each
prompt as it is typed — a live record, not an after-the-fact reconstruction. User-typed slash
commands, free-text messages, and question-picker selections are **verbatim**. Question texts are
gisted (they are agent output); the chosen answer labels are exact. `[agent → /skill]` lines mark
skills the *agent* chained into on its own — the user never typed those.

All sessions ran on 2026-07-09; times are local (UTC−5). Each `## Session` boundary is a fresh
context — the user either typed `/clear` (noted as the first event where it happened) or opened a
new terminal.

**Cleanup note**: the implementation sessions (12–17) are presented normalized to the loop's
canonical per-ticket shape — one `/implement <ticket-url>` followed by `/code-review` in the same
session, one ticket per session, ending ritual complete. The real run's divergences from that
shape, and the lessons they taught, live in the [run retrospective](run-retrospective.md).

---

## Session 1 — 09:17–09:42 — `/wayfinder` charts the map

**User typed:**

> /wayfinder I want a small, fun Simon Says game that runs in the terminal, written in Rust, living in example/app/ of this repo

`[agent → /grilling]` to pin down the destination. The user answered nine picker questions
(chosen answers verbatim):

1. What does "the way is clear" look like? → **Design locked, ready for /to-spec (Recommended)**
2. Does the global AFCAP (as-few-crates-as-possible) preference apply, or go rich? → **Go rich — ratatui from the start**
3. How much game is "small, fun"? → **Polished arcade mini-game (Recommended)**
4. Should the map deliberately exercise every wayfinder ticket type for the tutorial's sake? → **Chart naturally (Recommended)**
5. What's out of scope up front? → **Real audio (Recommended), Multiple game modes (Recommended), CI / release packaging**
6. Online/shared leaderboard in play? → **Out of scope — local only (Recommended)**
7. Fixed opinions on gameplay fundamentals? → **Lock classic 4 pads only (Recommended)**
8. Does this map match your picture (5 frontier tickets + fog + out-of-scope)? → **Yes — create the map (Recommended)**

**Produced:** [Wayfinder map: Simon Says terminal game (example/app)](https://github.com/constdecimals0/workflow/issues/1)
with five child tickets wired as sub-issues with native dependencies — research (ratatui), grilling
(core rules), grilling (high-score persistence), prototype (board look & feel), grilling (structure
& testing) — plus fog for difficulty-feel and screen-flow. **The expected no-fog escape hatch did
NOT fire**: even this "small" app surfaced real fog, so a full map was charted. Session stopped
after charting, per wayfinder's own rule.

## Sessions 2–4 — 09:55–10:15 — three terminals work the frontier in parallel

The user ran frontier tickets concurrently — and hit a real **claim collision**.

### Session 2 — 09:55–10:12 — research ticket

**User typed** (`/clear`, then pasted the frontier list from session 1's closing summary):

> /wayfinder - Research: ratatui patterns for a tick-driven mini-game (AFK)
> - Grilling: core game rules
> - Grilling: high-score persistence

`[agent → /research]` (background agent, ~12 min).
**Produced:** `docs/research/ratatui-tick-driven-game.md` (ratatui 0.30.2 + crossterm 0.29.0, tick
+ poll loop shape, deadline-based flash timing, example apps to mirror);
[Research: ratatui patterns for a tick-driven mini-game](https://github.com/constdecimals0/workflow/issues/2) closed.

### Session 3 — 09:59–10:13 — core game rules

**User typed:**

> /wayfinder

Bare invocation: the agent picked the first frontier ticket itself. `[agent → /grilling]`, ten
picker questions (answers verbatim): **Arrow keys** for pads; **Classic: append one** sequence;
**Sudden death** on mistake; **Per-key timeout**; **Tiered speed-ups**; **Steps × tier multiplier**
scoring; input during playback **Ignored**; back-to-back repeats **Yes, uniform random**;
**Endless** (no win condition); → **Confirmed — record it**.

`[agent → /domain-modeling]`. **Produced:** `CONTEXT.md` created at repo root (12 terms: Pad, Step,
Sequence, Round, Watch, Echo, Mistake, Sudden Death, Run, Speed Tier, Score, High Score);
[Grilling: core game rules](https://github.com/constdecimals0/workflow/issues/3) closed; fog
graduated into [Grilling: difficulty tuning & timing feel](https://github.com/constdecimals0/workflow/issues/7).

### Session 4 — 09:59–10:02 — the collision (interrupted)

**User typed:**

> /wayfinder

Started 13 seconds after session 3 — and grabbed the **same** core-rules ticket: claims are
"assignee = the driving dev", and two sessions of the same dev are indistinguishable, so the
claim check couldn't protect against this race. The user saw the duplicate grilling begin and
**interrupted the session mid-tool-call**. Nothing was produced or damaged. Friction, honestly
recorded: parallel bare `/wayfinder` invocations can race; naming the ticket avoids it.

## Session 5 — 10:02–10:15 — high-score persistence

**User typed** (`/clear`, then — having just been burned by the race — naming the ticket this time):

> /wayfinder Grilling: high-score persistence

`[agent → /grilling]`, five answers (verbatim): **Single best score**; **Plain text integer**;
**XDG data dir, hand-rolled**; **At game-over, if beaten**; **Best-effort, silent** on failures;
→ **Confirmed — record it**. `[agent → /domain-modeling]`.

**Produced:** `docs/adr/0001-std-only-plaintext-high-score.md`;
[Grilling: high-score persistence](https://github.com/constdecimals0/workflow/issues/4) closed.

## Session 6 — 10:18–10:40 — board prototype

**User typed** (`/clear`, then):

> /wayfinder

`[agent → /prototype]` — built a throwaway ratatui board render (explicitly outside `example/app/`)
with several layout/color/flash variants to react to. The user's reactions, verbatim:

> I like a mix of 2/3. I like the hud sidebar ( take the sidbar as is ), and I like the arrow cross pad the most

> its perfect

**Produced:** `example/board-prototype/` (kept as reference until pruned by the last implementation
ticket); decision recorded — arrow-cross board with Round · Phase hub in the middle, pads flood
solid on press, HUD sidebar as-is; prototype timing numbers (450 ms Watch flash, 250 ms Echo flash)
recorded as seed material for the timing ticket.
[Prototype: board look & feel](https://github.com/constdecimals0/workflow/issues/5) closed; fog
graduated into [Grilling: screen flow & polish](https://github.com/constdecimals0/workflow/issues/8).

## Session 7 — 10:40–12:54 — structure & testing strategy

**User typed** (`/clear`, then):

> /wayfinder

`[agent → /grilling]`, eight answers (verbatim): **Bin crate + lib target**; **Ratatui-free core,
time injected**; **Hand-rolled PRNG, seed injected**; **game + highscore in lib; main + ui in bin**;
manifest **Yes, as shown** (`simon-says`, edition 2024, `rust-version = "1.88"`, ratatui sole dep);
**Core unit + full-run integration; no UI tests**; gate = **test + clippy -D warnings + fmt --check**;
→ **Resolve #6 + write the ADR (Recommended)**. `[agent → /domain-modeling]`.

**Produced:** `docs/adr/0002-bin-lib-crate-with-pure-core.md`;
[Grilling: project structure & testing strategy](https://github.com/constdecimals0/workflow/issues/6) closed.

## Session 8 — 13:00–13:07 — difficulty tuning & timing feel

**User typed** (`/clear`, then):

> /wayfinder

`[agent → /grilling]`, five answers (verbatim): **4 tiers at rounds 1/5/9/13**; Watch tempo **Even
ramp: 570→240ms/step** (450+120 → 330+100 → 240+80 → 180+60); **Fixed 3.0s per key** Echo timeout;
**Base 10, multipliers ×1/×2/×3/×4**; → **Yes — record and close (Recommended)**.

**Produced:** [Grilling: difficulty tuning & timing feel](https://github.com/constdecimals0/workflow/issues/7) closed.

## Session 9 — 13:08–13:41 — screen flow & polish

**User typed** (`/clear`, then):

> /wayfinder

`[agent → /grilling]` — this grilling ran as free-text conversation rather than pickers. The user's
replies, verbatim and in order:

> b — sounds good — b — sounds good — do what you think is best — yep — yep — no — a — a — take both — looks good

`[agent → /domain-modeling]`. **Produced:** screen-flow vocabulary added to `CONTEXT.md` (Get Ready,
Round Break, Death Freeze, Title/Game Over overlay states); decisions — Enter-only start, instant
Watch→Echo flip, ~800ms Round Break (~1.5s with `SPEED UP! ×n` hub callout on tier-ups), ~1s Death
Freeze revealing the expected pad with the bell's only ring, `★ NEW HIGH SCORE! ★` on the Game Over
overlay, Enter-to-restart, immediate `Q`/`Esc` quit.
[Grilling: screen flow & polish](https://github.com/constdecimals0/workflow/issues/8) closed —
**frontier empty, map complete**.

## Session 10 — 13:42–13:46 — `/to-spec`

**User typed** (`/clear`, then):

> /to-spec

With no argument and a cleared context, the agent rebuilt from the repo: found the completed map,
all seven tickets closed, and synthesized the spec.
**Produced:** [Spec: Simon Says terminal game (example/app)](https://github.com/constdecimals0/workflow/issues/9),
labelled `ready-for-agent`, 33 user stories, linked from the map.

## Session 11 — 13:49–14:01 — `/to-tickets`

**User typed** (`/clear`, then):

> /to-tickets

One picker: six-ticket breakdown → **Approve — publish as-is (Recommended)**.
**Produced:** six implementation tickets, `ready-for-agent`, in a linear native-dependency chain:

| Issue | Blocked by |
|---|---|
| [#10 Walking skeleton: `simon-says` crate with board, Title overlay, and clean quit](https://github.com/constdecimals0/workflow/issues/10) | — |
| [#11 First playable Run: Watch, Echo, and Sudden Death](https://github.com/constdecimals0/workflow/issues/11) | #10 |
| [#12 Timing feel: Get Ready, Round Break, per-key timeout, and Death Freeze](https://github.com/constdecimals0/workflow/issues/12) | #11 |
| [#13 Speed Tiers and scoring](https://github.com/constdecimals0/workflow/issues/13) | #12 |
| [#14 High Score persistence](https://github.com/constdecimals0/workflow/issues/14) | #13 |
| [#15 Scripted full-Run integration test; prune the board prototype](https://github.com/constdecimals0/workflow/issues/15) | #14 |

**The planning phase ends here.** The canonical `/clear` follows before the first `/implement`.

## Session 12 — 14:21–15:15 — implement #10: walking skeleton

**User typed** (`/clear`, then):

> /implement https://github.com/constdecimals0/workflow/issues/10

`[agent → /tdd]` "Implement issue #10 walking skeleton lib core (simon-says game state machine
skeleton with injected time and xorshift64 PRNG) test-first through the lib public API". Two TDD
cycles pinned "a new game opens at the Title with zeroed stats" and "the Title never advances on
its own". Then, **in the same session** — never `/clear` between implement and review:

> /code-review

Two parallel review agents over the staged work — spec axis against GitHub #10, standards axis
against ADR 0001/0002, `CONTEXT.md`, and the ratatui research notes. The reviews produced real
findings, verified rather than blindly accepted (an unused speculative `Pad::ALL` flagged by both
axes was deleted; a `PadStyle` struct and a `center_vertically` rename resolved a naming pun),
fixed before the commit.

**Produced:** `example/app/` — the `simon-says` bin+lib crate per ADR 0002: std-only,
ratatui-free core (`game` + `rng`) behind the two injection seams (`now: Instant`, seeded
xorshift64), thin `main`/`ui` shell on the classic single-threaded ~30 Hz draw/poll/tick loop;
arrow-cross board, Hub, stats Sidebar, Title overlay, `Q`/`Esc` clean quit. Gate green:
`cargo test` 2/2, `clippy --all-targets -D warnings` clean, `fmt --check` clean. Commit
`c730682` on `main`; [#10](https://github.com/constdecimals0/workflow/issues/10) closed with a
comment linking the commit, verified closed.

## Session 13 — 15:36–15:48 — implement #11: first playable Run

**User typed** (`/clear`, then):

> /implement https://github.com/constdecimals0/workflow/issues/11

`[agent → /tdd]` through the lib public API seam (fixed seed, scripted `now`). Then, same
session:

> /code-review

Findings fixed pre-commit, gate green. **Produced:** Watch, Echo, and Sudden Death — the first
playable Run. Commit `4a48bca`;
[#11](https://github.com/constdecimals0/workflow/issues/11) closed with a comment linking the
commit.

## Session 14 — 15:48–15:59 — implement #12: timing feel

**User typed** (`/clear`, then):

> /implement https://github.com/constdecimals0/workflow/issues/12

`[agent → /tdd]`, then, same session:

> /code-review

Findings fixed pre-commit, gate green. **Produced:** Get Ready, Round Break, the 3.0 s per-key
timeout, Death Freeze + bell. Commit `cc63b8c`;
[#12](https://github.com/constdecimals0/workflow/issues/12) closed with a comment linking the
commit.

## Session 15 — 16:00–16:09 — implement #13: Speed Tiers and scoring

**User typed** (`/clear`, then):

> /implement https://github.com/constdecimals0/workflow/issues/13

`[agent → /tdd]`, then, same session:

> /code-review

Findings fixed pre-commit, gate green. **Produced:** Speed Tiers, the tempo ramp, the
`SPEED UP!` callout, 10 × multiplier scoring. Commit `d06438f`;
[#13](https://github.com/constdecimals0/workflow/issues/13) closed with a comment linking the
commit.

## Session 16 — 16:10–16:19 — implement #14: High Score persistence

**User typed** (`/clear`, then):

> /implement https://github.com/constdecimals0/workflow/issues/14

`[agent → /tdd]`, then, same session:

> /code-review

Findings fixed pre-commit, gate green. **Produced:** High Score persistence — XDG plain-text
file, write-once-on-beat, ★ celebration. Commit `3dcd154`;
[#14](https://github.com/constdecimals0/workflow/issues/14) closed with a comment linking the
commit.

## Session 17 — 16:19–16:29 — implement #15: integration test + prototype pruning

**User typed** (`/clear`, then):

> /implement https://github.com/constdecimals0/workflow/issues/15

`[agent → /tdd]`, then, same session:

> /code-review

Findings fixed pre-commit. **Produced:** the scripted full-Run integration test; the board
prototype pruned, per the prototype skill's mandate. Final gate: `cargo test` 25 lib + 1
full-Run integration, `clippy --all-targets -- -D warnings` clean, `fmt --check` clean. Commit
`7b3e7fb`; [#15](https://github.com/constdecimals0/workflow/issues/15) closed with a comment
linking the commit — **backlog empty**.

## Session 18 — 16:47–16:50 — the close-out: spec #9 and map #1

**User typed** (`/clear`, then):

> /wayfinder

No further prompts — the session ran unattended to completion.

Bare `/wayfinder` again, no map argument. The session had to pick a map itself: it found the
Simon map [#1](https://github.com/constdecimals0/workflow/issues/1) already complete (tickets
#2–#8 all closed, frontier empty) and took the only takeable ticket anywhere — this effort's
own close-out ticket. It then ran the loop's close-out exactly as settled in map ticket 03:
closed spec [#9](https://github.com/constdecimals0/workflow/issues/9) with a closing comment
(the six per-ticket commits, gate green), then closed map #1 linking the spec. Nothing else to
prune — the board prototype went in `7b3e7fb` and the Simon effort never had a `.scratch/` of
its own.

**Produced:** the example's tracker record is fully closed — issues #1–#15 all done, the loop
complete end to end.

**Friction, honestly recorded:** the close-out has no skill. Nothing in the flow set closes the
spec or the map (the skills review flagged this: nothing closes the loop after code-review), and
it happened here only because this repo's meta-map held a ticket for it — a reader following the
tutorial has no such net. Lesson for the tutorial: **when the frontier is empty, the close-out
is a prompt you type yourself** — e.g. "All tickets under spec #9 are closed — run the loop's
close-out: close the spec and the map with comments linking the commits." Second lesson: bare
`/wayfinder` made the session guess between two maps; pass the map URL when more than one is
live.
