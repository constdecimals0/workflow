# Example run retrospective — Simon Says loop

Consolidates the friction and divergences of the full example run — sessions 1–15 in
[context/example-run-log.md](example-run-log.md) — into the one record that the
[tutorial](../example/tutorial.md) and the root [README](../README.md) cite. The yardstick
throughout is the loop settled by the README effort's map ticket "Settle the canonical loop to
teach" (its `/clear` table appears in both the tutorial and the README); that ticket named this
retrospective as the thing that may revise it. (The effort's map lived on the local-markdown
tracker in `.scratch/workflow-readme/` and was pruned at its close-out, per the loop's own
cleanup rule.) Verdict up front: **the
settled loop survives intact** — every divergence below is a failure to walk it or an addition
it needs, not a flaw in it.

## The headline: bare invocations were the run's one recurring failure

Every serious friction traces to the same root — invoking a skill with **no argument** and
letting the agent guess its scope:

- Bare `/wayfinder` ×2 in parallel → both sessions grabbed the **same ticket** (sessions 3–4).
- Bare `/implement` → scoped itself to the **entire backlog**, twice (sessions 12 and 14).
- Bare `/wayfinder` with two live maps → the session had to **guess which map** (session 15).
- Bare `/to-tickets` in a fresh session → worked only because exactly **one** `ready-for-agent`
  spec existed (session 11).

The fix is the same every time: **pass the argument** — the ticket URL to `/implement`, the
ticket name to `/wayfinder` when working the frontier in parallel, the map URL when more than
one map is live, the spec URL to `/to-tickets` in a fresh session. Bare invocations are fine
only when the repo state leaves exactly one thing to do.

## What the run confirmed

- **Entry is always `/wayfinder`, and the no-fog hatch is the detector, not the user.** The
  hatch correctly did *not* fire for a "small" game — real fog (difficulty feel, screen flow)
  surfaced during charting, and both patches later graduated into tickets
  ([#7](https://github.com/constdecimals0/workflow/issues/7),
  [#8](https://github.com/constdecimals0/workflow/issues/8)) exactly as designed.
- **One ticket per session, `/clear` between** carried seven planning tickets across sessions
  2–9 with no context trouble.
- **The map → spec joint held.** The skills review called wayfinder → `/to-spec` the suite's
  fuzziest hand-off; in practice a fresh session typing bare `/to-spec` rebuilt everything from
  the repo (completed map, closed tickets) and produced the 33-story spec
  [#9](https://github.com/constdecimals0/workflow/issues/9).
- **A single-prompt `/implement <ticket-url>` runs the whole per-ticket loop unattended**
  (session 13): `/tdd` at the pre-agreed seams → `/code-review` same-session → real findings
  fixed pre-commit → commit. The "never clear between implement and review" rule held in both
  implement sessions.
- **The prototype ticket earned its keep**: a cheap throwaway board render got the design locked
  in one session ("its perfect") and was pruned by the last implementation ticket (`7b3e7fb`),
  per the prototype skill's mandate.
- **Domain docs accumulated and persisted**: `CONTEXT.md` (12+ terms) and ADRs 0001/0002 were
  produced by agent-chained `/domain-modeling` inside grilling sessions, and survive the loop's
  cleanup by design.

## Divergences and friction, one by one

### 1. "Small" was fog after all (session 1)

The Simon app was expected to be small enough that wayfinder's no-fog hatch might fire. It
didn't — grilling surfaced genuine open questions, and the map was worth charting.
**Teaches:** don't pre-judge size; type `/wayfinder` and let the hatch decide. `/grill-with-docs`
is only for work you *already know* is small.

### 2. Parallel bare `/wayfinder` raced for a claim (sessions 3–4)

Two sessions started 13 seconds apart and grabbed the same core-rules ticket. The claim
mechanism could not protect against this: a claim is "assignee = the driving dev", and two
sessions of the same dev are indistinguishable. The user saw the duplicate grilling begin and
interrupted; nothing was damaged. Session 5 — burned once — named its ticket and had no trouble.
**Teaches:** parallel frontier work is fine, but each parallel session must be told its ticket
by name. Only a solo session may go bare.

### 3. Bare `/implement` grabbed the whole backlog — twice (sessions 12 and 14)

The run's biggest divergence, and it happened twice:

- **Session 12 (stopped):** bare `/implement` scoped itself to tickets #10–#15 in one go. The
  user stopped it at ticket #10's verification gate; the partial `example/app/` was deleted (no
  commits, no issue touched) and implementation restarted canonically. Cost: ~15 minutes.
- **Session 14 (ran to completion):** the same bare invocation, unattended — all five remaining
  tickets (#11–#15) built back-to-back with a **single batched review** at the end instead of
  one per ticket, and the review findings landed in a catch-all commit (`11cb969`) tied to no
  ticket.

What the batch cost: per-ticket review granularity, a clean commit-to-ticket audit trail for the
fixes, and any seam to stop at had something gone wrong mid-backlog. What it did not cost: the
work itself was real and green (gate re-verified: 25 lib + 1 integration tests, clippy
`-D warnings`, fmt), and every issue was closed with a commit-linking comment.
**Teaches:** always pass the ticket URL to `/implement` — the batch landing green was luck of a
well-specified backlog, not a property of the shortcut. And when a session goes off-script,
stopping it early is cheap (session 12) while letting it run forfeits the loop's checkpoints
(session 14).

### 4. The ending ritual was skipped once, then followed five-for-five (sessions 13–14)

Session 13 finished real, green work — then left the ritual half-done: a `Closes #10` trailer on
an **unpushed** commit (the trailer only fires on push), no comment, no close. The issue sat
open with no trace of the finished work until the meta-map ticket recording the session re-ran
the gate and closed it by hand. Session 14 then closed all five of its issues with
commit-linking comments.
**Teaches:** the loop's "close your ticket with a comment linking the commit" step is not
optional, and a trailer is not a substitute — verify the issue actually shows closed before you
`/clear`.

### 5. The close-out has no skill (session 15)

Nothing in the flow set closes the spec or the map — the skills review flagged the gap
("nothing closes the loop after code-review"), and session 14 confirmed it by leaving spec #9
and map #1 open. The close-out ran only because this repo's meta-map held a ticket for it; a
reader has no such net.
**Teaches:** when the frontier goes empty, the close-out is a prompt you type yourself, e.g.:

> All tickets under spec #9 are closed — run the loop's close-out: close the spec and the map
> with comments linking the commits, and prune any prototypes and the effort's `.scratch/`.

### 6. `/to-spec` → `/to-tickets` split across sessions — benign (sessions 10–11)

The settled loop says `/to-spec` → `/to-tickets` share a session, with fresh-session-plus-URL as
the recovery move. The run cleared between them and typed bare `/to-tickets` with no URL — and
it worked, because the repo held exactly one `ready-for-agent` spec to find. This is the one
place practice diverged from the settled `/clear` table, and it cost nothing here — but it
leans on the same single-candidate luck as every bare invocation above.
**Teaches:** same-session remains the canonical teaching; if the session was cleared, pass the
spec URL rather than trusting the repo to contain only one spec.

## The `/clear` record

Where context actually needed clearing, against the settled table:

| Boundary | Settled | What the run did |
| --- | --- | --- |
| wayfinder charting → ticket work | clear | cleared (session 1 → 2) ✓ |
| between wayfinder tickets | clear | cleared every time (sessions 2–9) ✓ |
| map done → planning session | clear | cleared (session 9 → 10) ✓ |
| grilling → `/to-spec` (small path) | never | not exercised — the hatch never fired, so the small path was never walked |
| `/to-spec` → `/to-tickets` | same session | **diverged** — cleared, bare `/to-tickets` recovered (divergence 6) |
| `/to-tickets` → first `/implement` | clear (the canonical clear) | cleared (session 11 → 12) ✓ |
| between implement tickets | clear | cleared once (13 → 14), then session 14 batched #11–#15 with no clears (divergence 3) |
| `/implement` → `/code-review` | never | held in both implement sessions ✓ |

No session ran out of context or cleared under pressure; every `/clear` in the run sits on a
settled boundary except the benign session 10/11 split.

## Rules for the tutorial

The distilled, citable form of the above:

1. **Pass the argument.** `/implement <ticket-url>` always; name the ticket to `/wayfinder` when
   sessions run in parallel; pass the map URL when more than one map is live; pass the spec URL
   to `/to-tickets` in a fresh session. Bare invocations make the agent guess.
2. **Let the hatch decide "small".** Type `/wayfinder` regardless; the no-fog hatch exists so
   you don't have to pre-judge — and it correctly charted a map for a "small" game.
3. **Close your ticket before you `/clear`.** Comment linking the commit, then close, then
   verify it shows closed. An unpushed `Closes` trailer closes nothing.
4. **The close-out is a prompt you type.** Frontier empty = feature done: close the spec and the
   map with comments linking the commits, prune prototypes and the effort's `.scratch/`. No
   skill will do this for you.
5. **Stop off-script sessions early.** Interrupting costs minutes and nothing else; letting a
   batch run forfeits per-ticket review, clean traceability, and every stopping seam.
6. **Trust the loop's session shape.** One ticket per session, review in the same session as the
   implementation, `/clear` on the settled boundaries — the run's only failures were departures
   from this, never the loop itself.
