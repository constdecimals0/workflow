# Tutorial: build Simon Says with the skills workflow

This is a prompt-along tutorial. It walks you through one full loop of the Matt Pocock
skills workflow — grill/wayfind → spec → tickets → implement → review → close — by
recreating a real run: the Simon Says terminal game (Rust) that lives in [`app/`](app/).

Everything here happened. Every prompt you're told to type was typed in the real run,
captured verbatim in the [run log](../context/example-run-log.md). Where the run hit
friction, the tutorial says so instead of hiding it — the consolidated lessons live in
the [run retrospective](../context/run-retrospective.md), cited throughout as
*Retro rule N*.

## What you'll practice

- **One loop = one feature.** The whole tutorial is one loop. The per-ticket implement
  cycle is a sub-loop inside it.
- **Sessions and `/clear`.** A session is one agent context window. The loop has settled
  `/clear` boundaries — this tutorial marks every one, and the two boundaries where you
  must *never* clear.
- **Passing arguments.** The run's one recurring failure was invoking a skill bare and
  letting the agent guess its scope (*Retro rule 1*). You'll see exactly where that bit.

## Before you start

1. **Install the v1.1 skills:**

   ```bash
   npx skills add mattpocock/skills
   ```

2. **Clean out lingering pre-1.1 skills.** After installing, check your skills folder
   for leftovers — delete `review`, `to-prd`, and `to-issues` (both the
   `~/.claude/skills/` symlink and its `~/.agents/skills/` target); keep `grill-me`,
   which is the current v1.1 wrapper. The stale `review` skill is the dangerous one: its
   trigger description is identical to `code-review`'s, so a generic "review this
   branch" can silently run the old skill.

3. **Rust toolchain**, stable ≥ 1.88 (the example's manifest pins `rust-version = "1.88"`).

4. **`gh` authenticated** against a GitHub repo with issues enabled — the workflow uses
   issues as its tracker: the map, its tickets, the spec, and the implementation tickets
   all live there.

5. **Once per repo, before any loop:** run `/setup-matt-pocock-skills` so the repo knows
   its tracker, triage labels, and domain-docs layout.

## How to read this

- Blockquoted prompts are what you type, verbatim from the run.
- Each `## Session` heading is a fresh context — type `/clear` (or open a new terminal)
  at every session boundary, and *only* at session boundaries.
- **What our run did** callouts report the real outcome, including the divergences.

---

## Session 1 — chart the map

Entry is always `/wayfinder` — even for work you suspect is small. Its no-fog escape
hatch detects the small case for you and drops straight into `/to-spec`; don't pre-judge
size yourself (*Retro rule 2*). Type your loose idea as the argument:

> /wayfinder I want a small, fun Simon Says game that runs in the terminal, written in Rust, living in example/app/ of this repo

The agent chains into `/grilling` and asks one question at a time — mostly option
pickers — to pin down the **destination** and then fan out across the open decisions.
In our run it took eight questions: what "done" looks like (design locked, ready for
`/to-spec`), the dependency stance (go rich — ratatui), how much game "small, fun" means
(polished arcade mini-game), what's out of scope up front (real audio, multiple game
modes, CI/packaging, online leaderboards), and which fundamentals to lock now (classic
4 pads).

**What our run did:** the no-fog hatch did **not** fire — even this "small" game had
real fog (difficulty feel, screen flow), so a full map was charted:
[Wayfinder map: Simon Says terminal game](https://github.com/constdecimals0/workflow/issues/1)
with five child tickets wired as sub-issues with native blocking — one research, three
grilling, one prototype — plus the fog noted on the map. The session stops after
charting; that's wayfinder's own rule.

**Done when:** the map issue exists with its tickets, and the session tells you it has
stopped. `/clear`.

---

## Sessions 2–7 — work the frontier, one ticket per session

The **frontier** is the map's open, unblocked, unclaimed tickets. Work them one per
session, `/clear` between — our run carried seven tickets this way with no context
trouble (*Retro rule 6*). You can run frontier tickets in parallel terminals, but read
the friction note below first.

### The research ticket (AFK)

> /wayfinder - Research: ratatui patterns for a tick-driven mini-game (AFK)
> - Grilling: core game rules
> - Grilling: high-score persistence

(The run pasted the frontier list from session 1's closing summary — naming what's
takeable.) The agent chains into `/research` and works alone (~12 min in our run).

**Produced:** [`docs/research/ratatui-tick-driven-game.md`](../docs/research/ratatui-tick-driven-game.md)
— crate versions, the tick + poll loop shape, flash timing — and the ticket closed.

### ⚠ Friction: the parallel claim race

Our run opened two more terminals 13 seconds apart, both typing bare `/wayfinder`. Both
grabbed the **same** ticket — a claim is "assignee = the driving dev", and two sessions
of the same dev are indistinguishable. The user saw the duplicate grilling start and
interrupted it; nothing was damaged.

**The lesson (Retro rule 1):** a bare `/wayfinder` is fine only when you're solo and the
frontier has an obvious next ticket. Parallel sessions must each be told their ticket by
name, like the very next session did:

> /wayfinder Grilling: high-score persistence

### The grilling tickets (HITL)

Each grilling ticket is a conversation — one question at a time, you answer, and when
you confirm shared understanding the agent chains into `/domain-modeling` to make the
decisions durable. Our run's grilling sessions produced:

- **Core game rules** — arrow keys, append-one sequences, sudden death, per-key timeout,
  tiered speed-ups, steps × tier scoring. Chained `/domain-modeling` created
  [`CONTEXT.md`](../CONTEXT.md) at the repo root with 12 domain terms (Pad, Step,
  Sequence, Round, Watch, Echo, …). Resolving this ticket also **graduated fog**: the
  difficulty-feel question was now sharp enough to become a new ticket.
- **High-score persistence** — single best score, plain-text file, XDG data dir,
  best-effort silent failure. Produced [ADR 0001](../docs/adr/0001-std-only-plaintext-high-score.md).
- **Structure & testing** — bin crate + lib target, ratatui-free core with time and PRNG
  injected, core unit tests + one full-run integration test, gate = `cargo test` +
  `clippy -D warnings` + `fmt --check`. Produced
  [ADR 0002](../docs/adr/0002-bin-lib-crate-with-pure-core.md).

**Done when (each):** *you* confirm the understanding is shared — grilling's only gate —
the ticket closes, and any domain docs are written. `/clear` between tickets.

### The prototype ticket (HITL)

> /wayfinder

(Solo session by now, so bare was safe.) The agent chains into `/prototype` and builds a
cheap, throwaway board render — explicitly outside `app/` — with layout, color, and
flash variants for you to react to. Our run's reactions, verbatim:

> I like a mix of 2/3. I like the hud sidebar ( take the sidbar as is ), and I like the arrow cross pad the most

> its perfect

**Produced:** the board design locked in one session (arrow-cross pad, HUD sidebar, hub
in the middle), plus timing numbers that seeded the difficulty ticket. The prototype
directory is *throwaway* — the last implementation ticket prunes it, per the prototype
skill's own mandate.

### The graduated-fog tickets

Two more grilling sessions worked the tickets that graduated out of the fog — difficulty
tuning (4 speed tiers, tempo ramp, 3.0 s per-key timeout, ×1/×2/×3/×4 scoring) and
screen flow (Get Ready, Round Break, Death Freeze, Game Over overlay). After the last
one: **frontier empty — map complete.**

---

## Session 8 — plan: `/to-spec` → `/to-tickets`, one session

Map done → `/clear` → a fresh session that loads the map and turns it into a spec and
tickets. Type:

> /to-spec

With no argument and a cleared context, the agent rebuilds from the repo: it finds the
completed map and its closed tickets, and synthesizes the spec. Our run produced
[Spec: Simon Says terminal game](https://github.com/constdecimals0/workflow/issues/9) —
33 user stories, labelled `ready-for-agent`.

Then, **in the same session**:

> /to-tickets

One approval picker later, our run had six implementation tickets
([#10](https://github.com/constdecimals0/workflow/issues/10)–[#15](https://github.com/constdecimals0/workflow/issues/15))
in a linear native-dependency chain: walking skeleton → first playable → timing feel →
speed tiers & scoring → high-score persistence → integration test + prototype pruning.

**⚠ What our run actually did:** it `/clear`ed between `/to-spec` and `/to-tickets` and
got away with it — bare `/to-tickets` found the spec only because the repo held exactly
**one** `ready-for-agent` spec. Same-session is the canonical teaching; if you did
clear, pass the spec URL rather than trusting the repo to contain only one spec
(*Retro rule 1*).

**Done when:** you approve the breakdown and the tickets land with blocking edges.
Planning ends here. **`/clear` — this is the canonical clear** between planning and
implementation.

---

## Sessions 9–14 — implement, one ticket per session

The per-ticket ritual, which each session runs start to finish:

**`/implement <ticket-url>` → TDD → `/code-review` (same session) → fix findings →
commit → close the ticket with a comment linking the commit → verify it shows closed →
`/clear`.**

> /implement https://github.com/constdecimals0/workflow/issues/10

Always pass the ticket URL (*Retro rule 1* — see the friction below). Our run's session
for #10 ran the whole ritual unattended from that single prompt: it chained `/tdd` at
the pre-agreed seams (lib public API, injected time, seeded PRNG), then `/code-review`
over the staged work — spec axis against the ticket, standards axis against the ADRs and
`CONTEXT.md` — fixed the real findings the review produced, and committed green
(`cargo test`, `clippy -D warnings`, `fmt --check`).

Repeat for each ticket on the chain: as each closes, the next becomes unblocked.

### ⚠ Friction: bare `/implement` grabs the whole backlog

Our run typed a bare `/implement` twice, and both times the session scoped itself to the
**entire backlog**:

- The first time, the user stopped it at the first verification gate, deleted the
  partial work (no commits had landed), and restarted canonically with the ticket URL.
  Cost: ~15 minutes. Stopping off-script sessions early is cheap (*Retro rule 5*).
- The second time nobody stopped it, and it built all five remaining tickets (#11–#15)
  back-to-back with a **single batched review** at the end. The work landed real and
  green — but the batch forfeited per-ticket review granularity, a clean
  commit-to-ticket audit trail for the review fixes, and any seam to stop at had
  something gone wrong mid-backlog. That it landed green was the luck of a
  well-specified backlog, not a property of the shortcut.

### ⚠ Friction: the ending ritual is not optional

Our run's #10 session finished real, green work — then left a `Closes #10` trailer on an
**unpushed** commit. The trailer only fires on push, so the issue sat open with no trace
of the finished work until it was closed by hand later. Close your ticket with a comment
linking the commit, and **verify it actually shows closed** before you `/clear`
(*Retro rule 3*).

**Done when (each ticket):** gate green, commit made, ticket closed with a
commit-linking comment. `/clear` between tickets — but **never** between `/implement`
and `/code-review`: the review reads the session's context, same session, pre-commit.

---

## Session 15 — the close-out

Frontier empty = feature done. No skill closes the loop for you — the close-out is a
prompt you type yourself (*Retro rule 4*):

> All tickets under spec #9 are closed — run the loop's close-out: close the spec and the map with comments linking the commits, and prune any prototypes and the effort's .scratch/.

**What our run did:** the session closed spec #9 with a closing comment listing the
per-ticket commits, then closed map #1 linking the spec. The board prototype had already
been pruned by ticket #15, so there was nothing else to delete. `CONTEXT.md` and
`docs/adr/` are **never** cleaned up — domain docs persist across loops by design.

**Done when:** spec and map both show closed. The loop is complete — run it again when
the next feature arrives.

---

## The `/clear` table

The settled boundaries, and what the run confirmed:

| Boundary | Clear? |
| --- | --- |
| wayfinder charting → ticket work | yes |
| between wayfinder tickets | yes |
| map done → planning session | yes (the fresh session loads the map) |
| grilling → `/to-spec` (small path, no map) | **never** — it destroys to-spec's only input |
| `/to-spec` → `/to-tickets` | no — same session; fresh-session + spec URL is the recovery move |
| `/to-tickets` → first `/implement` | **yes — the canonical clear** |
| between implement tickets | yes |
| `/implement` → `/code-review` | **never** — same session, pre-commit |

No session in the run ran out of context or cleared under pressure.

## The rules

The [retrospective](../context/run-retrospective.md)'s distilled form — every serious
friction in the run traces to breaking one of these:

1. **Pass the argument.** `/implement <ticket-url>` always; name the ticket to
   `/wayfinder` when sessions run in parallel; pass the map URL when more than one map
   is live; pass the spec URL to `/to-tickets` in a fresh session.
2. **Let the hatch decide "small".** Type `/wayfinder` regardless; don't pre-judge size.
3. **Close your ticket before you `/clear`.** Comment linking the commit, then verify it
   shows closed. An unpushed `Closes` trailer closes nothing.
4. **The close-out is a prompt you type.** Frontier empty: close the spec and the map,
   prune prototypes and the effort's `.scratch/`.
5. **Stop off-script sessions early.** Interrupting costs minutes; letting a batch run
   forfeits the loop's checkpoints.
6. **Trust the loop's session shape.** One ticket per session, review in the same
   session as the implementation, `/clear` on the settled boundaries. The run's only
   failures were departures from this — never the loop itself.
