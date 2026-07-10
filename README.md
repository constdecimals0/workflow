# Workflow

How to run a development workflow with the [Matt Pocock skills](https://www.aihero.dev/skills) —
which skill to run when, where sessions break (`/clear`), when to run the whole loop again, and
what to clean up.

Everything here comes from a real run: the [Simon Says terminal game](example/app/) in this repo
was built in one full pass of the loop, captured prompt-by-prompt in
**[the tutorial](example/tutorial.md)**. This page is the two-minute index; the tutorial is the
prompt-along walkthrough.

## Setup

1. Install the v1.1 skills: `npx skills add mattpocock/skills`
2. Delete lingering pre-1.1 skills — `review`, `to-prd`, `to-issues` (keep `grill-me`). The stale
   `review` has the same trigger description as `code-review`, so a generic "review this branch"
   can silently run the old skill.
3. Once per repo, before any loop: **`/setup-matt-pocock-skills`** — wires up the issue tracker,
   triage labels, and domain-docs layout.

## The loop

**One loop = one feature** — not a ticket, not a project. The per-ticket implement cycle is a
sub-loop inside it. Run the whole loop again when the next feature arrives.

| Stage | You type | Session shape |
| --- | --- | --- |
| 1. Chart | `/wayfinder <your idea>` | one session; stops once the map exists |
| 2. Decide | `/wayfinder <ticket name>`, per map ticket | one ticket per session |
| 3. Plan | `/to-spec` then `/to-tickets` | one session, back-to-back |
| 4. Build | `/implement <ticket-url>` | one ticket per session |
| 5. Review | `/code-review` | same session as Build — never `/clear` between |
| 6. Close | a close-out prompt you type yourself | one session |

**1. Chart — entry is always `/wayfinder`, even for work you suspect is small.** Its no-fog
hatch detects the small case and drops straight into `/to-spec` in the same session — don't
pre-judge size yourself. (`/grill-with-docs` is a shortcut for work you *already know* is small.)
Anything with real open questions gets a **map**: an issue whose child tickets are the decisions
to make. Charting is one session; it stops when the map and its tickets exist.

**2. Decide — work the map's frontier, one ticket per session.** The frontier is the map's open,
unblocked, unclaimed tickets. Research tickets run alone; grilling tickets interview you one
question at a time and write the durable domain docs (`CONTEXT.md`, `docs/adr/`); prototype
tickets build something cheap and throwaway to react to. Parallel terminals are fine, but each
must be told its ticket by name — two bare sessions will race for the same claim.

**3. Plan — map done, fresh session.** `/to-spec` rebuilds from the completed map and publishes
the spec as `ready-for-agent`; `/to-tickets` in the **same session** breaks it into
implementation tickets chained with blocking edges. When you approve the breakdown, planning is
over — `/clear`. **This is the canonical clear**, the one boundary the skills themselves name.

**4. Build — per ticket, fresh session.** `/implement <ticket-url>` builds the ticket test-first
— TDD at the seams the spec agreed. Always pass the ticket URL; a bare `/implement` will guess
its scope. When the build lands, don't commit and don't `/clear` — the review comes first, in
the same session.

**5. Review — same session as Build, never `/clear` between.** Type `/code-review`: spec axis
against the ticket, standards axis against the ADRs and `CONTEXT.md`. Fix the real findings
pre-commit, commit green. Then **close the ticket with a comment linking the commit and verify
it shows closed** — an unpushed `Closes` trailer closes nothing — and `/clear`. As each ticket
closes, the next unblocks.

**6. Close out — frontier empty means the feature is done.** No skill does this; you type it:

> All tickets under spec #N are closed — run the loop's close-out: close the spec and the map
> with comments linking the commits, and prune any prototypes and the effort's `.scratch/`.

That's also the between-loops cleanup: spec and map closed, prototypes and the effort's scratch
space deleted. `CONTEXT.md` and `docs/adr/` are **never** cleaned up — domain docs accumulate
across loops by design.

## Where `/clear` happens

A **session** is one agent context window; `/clear` (or a new terminal) ends it. The settled
boundaries:

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

## The one rule that outranks the rest

**Pass the argument.** Every serious friction in the real run traced to a bare invocation
letting the agent guess its scope: bare `/implement` grabbed the entire backlog (twice), parallel
bare `/wayfinder` sessions raced for the same ticket. Give `/implement` the ticket URL,
`/wayfinder` the ticket name when sessions run in parallel, `/to-tickets` the spec URL if you
cleared. Bare is safe only when the repo state leaves exactly one thing to do. The full six
rules, with the receipts, are in the [run retrospective](context/run-retrospective.md).

## Learn more

- **[The tutorial](example/tutorial.md)** — recreate the Simon Says app prompt-by-prompt.
- **[The run retrospective](context/run-retrospective.md)** — every divergence in the real run
  and what it teaches.
- **[The run log](context/example-run-log.md)** — every prompt of all 18 sessions, session by session.
