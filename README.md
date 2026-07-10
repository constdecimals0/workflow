# Workflow

How to run a development workflow with the [Matt Pocock skills](https://www.aihero.dev/skills) —
which skill to run when, where sessions break (`/clear`), when to run the whole loop again, and
what to clean up.

Everything here comes from a real run: the [Simon Says terminal game](example/app/) in this repo
was built in one full pass of the loop, captured prompt-by-prompt in
**[the tutorial](example/tutorial.md)**.

**Read in this order:**

1. **This README** — the two-minute index of the loop.
2. **[The walkthrough site](https://constdecimals0.github.io/workflow/)** — the real run replayed
   step by step in your browser. (Offline or already cloned? `./serve.sh` serves the same site
   locally.)
3. **[The tutorial](example/tutorial.md)** — the same run as a prompt-along read; typing along and
   rebuilding the game is optional reps, not a prerequisite.
4. **[On a work repo](#on-a-work-repo)** — what changes when the loop runs on a real work repo.
   Reading ends there; the next step is running it.

## Setup

1. Install the v1.1 skills: `npx skills@latest add mattpocock/skills`. The installer is an
   interactive picker — make sure you select `/setup-matt-pocock-skills`.
2. Only if this machine had a pre-1.1 install: delete the lingering skills — `review`, `to-prd`,
   `to-issues` (keep `grill-me`). The stale `review` has the same trigger description as
   `code-review`, so a generic "review this branch" can silently run the old skill.
3. Once per repo, before any loop: **`/setup-matt-pocock-skills`** — wires up the issue tracker,
   triage labels, and domain-docs layout.

## The loop

**One loop = one feature** — not a ticket, not a project. The per-ticket implement cycle is a
sub-loop inside it. Run the whole loop again when the next feature arrives.

| Stage | You type | Session shape |
| --- | --- | --- |
| 1. Chart | `/wayfinder <your idea>` | one session; stops once the map exists |
| 2. Decide | `/wayfinder <ticket name>`, per map ticket | one ticket per session |
| 3. Plan | `/to-spec <map-url>` then `/to-tickets` | one session, back-to-back |
| 4. Build | `/implement <ticket-url>` | one ticket per session |
| 5. Review | `/code-review` | same session as Build — never `/clear` between |
| 6. Close | a close-out prompt you type yourself | one session |

**1. Chart — entry is always `/wayfinder`, even for work you suspect is small.** Its no-fog
hatch detects the small case, then **stops and asks** how you'd like to proceed — the move is
answering "go straight to `/to-spec`", which runs on the grilling still in context; don't
pre-judge size yourself. (`/grill-with-docs` is a shortcut for work you *already know* is small.)
Anything with real open questions gets a **map**: an issue whose child tickets are the decisions
to make. Charting is one session; it stops when the map and its tickets exist. *This entry rule
deliberately inverts upstream's framing — [v1.1](context/v1.1.md) settles wayfinder as a
situational on-ramp, with the grill-led chain as the front door — because one entry rule beats
pre-judging effort size.*

**2. Decide — work the map's frontier, one ticket per session.** The frontier is the map's open,
unblocked, unclaimed tickets. Research tickets run alone; grilling tickets interview you one
question at a time and write the durable domain docs (`CONTEXT.md`, `docs/adr/`); prototype
tickets build something cheap and throwaway to react to. Parallel terminals are fine, but each
must be told its ticket by name — two bare sessions will race for the same claim.

**3. Plan — map done, fresh session.** `/to-spec <map-url>` builds the spec from the completed
map and publishes it as `ready-for-agent`; `/to-tickets` in the **same session** breaks it into
implementation tickets chained with blocking edges. When you approve the breakdown, planning is
over — `/clear`. **This is the canonical clear**, the one boundary the skills themselves name.

**4. Build — per ticket, fresh session.** `/implement <ticket-url>` builds the ticket test-first
— TDD at the seams the spec agreed. Always pass the ticket URL; a bare `/implement` will guess
its scope. When the build lands, don't commit and don't `/clear` — the review comes first, in
the same session.

**5. Review — same session as Build, never `/clear` between.** Type `/code-review`: spec axis
against the ticket, standards axis against whatever documents how code should be written — here,
the ADRs and `CONTEXT.md` — plus an always-on baseline of twelve classic Fowler code smells. Fix
the real findings
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
`/wayfinder` the ticket name when sessions run in parallel (the skill's own convention is
map-first — pass the map URL when more than one map is live), `/to-tickets` the spec URL if you
cleared. Bare is safe only when the repo state leaves exactly one thing to do. The full six
rules, with the receipts, are in the [run retrospective](context/run-retrospective.md).

## On a work repo

The tutorial's run had the luxury of committing straight to `main` in a repo built to be looked
at. On a real work repo, three things change:

**One PR per feature.** Branch at the canonical clear — between `/to-tickets` and the first
`/implement`. Each implement session commits to the feature branch and still closes its ticket
with a comment linking the commit (rule 3 unchanged — `Closes` trailers only fire at merge). The
close-out gains three steps: push, open the PR, merge — then close the spec and the map.

**Tracker choice.** GitHub Issues is the default — maps and tickets live where the team already
looks. If the effort must leave no trace, pick the local-markdown tracker at setup instead:
GitHub-tracked maps can't be gitignored, so zero footprint fully applies only on the markdown
tracker. Both are native to `/setup-matt-pocock-skills`.

**Zero footprint.** Gitignore everything the workflow writes — scratch space, prototype dirs,
research notes, domain docs (`CONTEXT.md`, `docs/adr/`), agent config — and at setup's
confirm-and-edit step, steer its agent-skills block into a gitignored local memory file. Complete
cleanup is then deleting the ignored paths. The accepted cost: domain docs become local-only,
per-machine.

## Learn more

- **[The tutorial](example/tutorial.md)** — the Simon Says run as a prompt-along read;
  rebuilding the game yourself is optional reps.
- **[The run retrospective](context/run-retrospective.md)** — every divergence in the real run
  and what it teaches.
- **[The run log](context/example-run-log.md)** — every prompt of all 18 sessions, session by session.
- **[The skills review](context/skills-review.md)** — the read of the skill texts that motivated
  the setup steps.
