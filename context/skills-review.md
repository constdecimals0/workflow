# Skills review: what the installed v1.1 "Matt Pocock flow" skills actually say

Primary-source review of the skill files installed under `/Users/coachchucksol/.claude/skills/` (each entry is a symlink into `~/.agents/skills/`), checked against the v1.1 changelog at `context/v1.1.md`. Every hand-off / session-boundary claim quotes the skill's own text.

## Summary

- The only **explicit** context-clearing instruction in the whole v1.1 set is the last line of `to-tickets`: *"Work the frontier one ticket at a time with `/implement`, clearing context between tickets."* Everything else about session boundaries is either in `wayfinder` (one ticket per session; charting is its own session) or only in the changelog, not the skills.
- Two hand-offs are **same-session by construction**: grilling → `to-spec` (to-spec synthesizes "the current conversation" and must not interview), and `implement` → `code-review` (implement invokes it before committing).
- One hand-off exists **only in the changelog**: wayfinder → spec. The `wayfinder` skill text never names `/to-spec`; the changelog says "take this map and turn it into a spec the regular way."
- No skill says what happens **after** `code-review` — the loop restart (next ticket, fresh session) is implied by `to-tickets`' closing line, not stated anywhere as a cycle.
- The changelog instructs removing lingering old skills after migration (`context/v1.1.md:17`). Three pre-1.1 leftovers were still installed when this review ran: `to-prd`, `to-issues`, and — found during this review — `review`, a pre-smell-baseline copy of `code-review` that was **model-invocable with a description identical to `code-review`'s**, making it the only leftover that could actually be invoked by mistake. All three have since been deleted, per the setup step this review motivated. `grill-me` turns out to be v1.1-current, not a leftover.

---

## Flow skills

### grilling

Source: `/Users/coachchucksol/.claude/skills/grilling/SKILL.md` (7 lines of body; model-invocable — no `disable-model-invocation` flag).

- **Inputs**: a plan or design in conversation, plus a live human. Facts come from the codebase, decisions from the user: "If a *fact* can be found by exploring the codebase, look it up rather than asking me. The *decisions*, though, are mine — put each one to me and wait for my answer." (SKILL.md:10)
- **Artifacts**: none. Conversation-only — the sharpened shared understanding *is* the output, which is why `to-spec` must run in the same session.
- **Session boundaries / hand-offs**: no `/clear`, no next skill named. The one gate is the confirmation gate: "Do not enact the plan until I confirm we have reached a shared understanding." (SKILL.md:12) Also "Ask the questions one at a time, waiting for feedback on each question before continuing. Asking multiple questions at once is bewildering." (SKILL.md:8)
- **vs. changelog**: matches all three v1.1 fixes described in `context/v1.1.md:51-68` — one-question-at-a-time with the "why" (bewildering), the confirmation gate, and the facts/decisions split that prevents self-grilling.

### grill-me

Source: `/Users/coachchucksol/.claude/skills/grill-me/SKILL.md`. Entire body: "Run a `/grilling` session." (SKILL.md:7). `disable-model-invocation: true` (SKILL.md:4) — user-typed entry point only.

- **Inputs/artifacts/hand-offs**: entirely delegated to `grilling`.
- **vs. changelog**: **not** a pre-1.1 leftover. The changelog describes exactly this architecture: "Both [`/grill-me` and `/grill-with-docs`] rely on a central reference grilling skill that shows the LLM how to grill a person effectively." (`context/v1.1.md:47-48`) The installed thin-wrapper form is the v1.1 form.

### grill-with-docs

Source: `/Users/coachchucksol/.claude/skills/grill-with-docs/SKILL.md`. Entire body: "Run a `/grilling` session, using the `/domain-modeling` skill." (SKILL.md:7). `disable-model-invocation: true`.

- **Artifacts**: via `/domain-modeling` — ADRs and glossary entries (`CONTEXT.md`, `docs/adr/`) created as decisions get resolved (see `docs/agents/domain.md`: domain files are created "lazily when terms or decisions actually get resolved").
- **Hand-offs**: none named. The changelog positions `wayfinder` as its replacement for large work: "Try using `/wayfinder` instead of `/grill-with-docs` for larger planning tasks" (`context/v1.1.md:234`).

### wayfinder

Source: `/Users/coachchucksol/.claude/skills/wayfinder/SKILL.md` (the longest flow skill). `disable-model-invocation: true`.

- **Inputs**: chart mode — "a loose idea" (SKILL.md:109); work mode — "a map (URL or number). A ticket is **optional**" (SKILL.md:119). Requires tracker config: "The issue tracker should have been provided to you — run `/setup-matt-pocock-skills` if not. … If no tracker has been provided, default to the local-markdown tracker." (SKILL.md:25)
- **Artifacts**: a map issue labelled `wayfinder:map` with child ticket issues labelled `wayfinder:<type>` (`research`/`prototype`/`grilling`/`task`), native blocking edges, resolution comments, and a Decisions-so-far index on the map (SKILL.md:21-25, 57-71). Tracker mechanics live in the setup skill's tracker docs ("Wayfinding operations" sections).
- **Session boundaries — the most explicit in the suite**:
  - "Two modes. Either way, **never resolve more than one ticket per session.**" (SKILL.md:105)
  - Chart mode step 5: "Stop — charting the map is one session's work; do not also resolve tickets." (SKILL.md:115)
  - Tickets are context-budgeted: "Its body is the question, sized to one 100K token agent session" (SKILL.md:57).
  - The end-of-map signal: "The pull to just do the work is usually the signal you've reached the edge of the map and it's time to hand off." (SKILL.md:13)
  - Escape hatch: "**If this surfaces no fog** — the way to the destination is already clear, the whole journey small enough for one session — you don't need a map. Stop and ask the user how they'd like to proceed." (SKILL.md:112)
- **Skills it invokes**: `/grilling` + `/domain-modeling` for charting and as the default resolution tools (SKILL.md:111, 123), `/prototype` for prototype tickets (SKILL.md:78), `/setup-matt-pocock-skills` as prerequisite (SKILL.md:25).
- **Hand-off gap**: the skill never names `/to-spec`. The destination "might be a spec to hand off and iterate on" (SKILL.md:9), but only the changelog says what to do when the map is done: "You can then take this map and turn it into a spec the regular way." (`context/v1.1.md:179`) and "You just close a session and open the next Wayfinder ticket." (`context/v1.1.md:181`)
- **vs. changelog**: otherwise matches (map on tracker, four ticket types, blocking edges, session-sized decisions).

### to-spec

Source: `/Users/coachchucksol/.claude/skills/to-spec/SKILL.md`. `disable-model-invocation: true`.

- **Inputs**: the current session's conversation — "This skill takes the current conversation context and codebase understanding and produces a spec (you may know this document as a PRD). Do NOT interview the user — just synthesize what you already know." (SKILL.md:7) Requires tracker config (SKILL.md:9). One mid-flight checkpoint: sketch test seams and "Check with the user that these seams match their expectations." (SKILL.md:17)
- **Artifacts**: a spec issue on the project tracker (Problem Statement / Solution / User Stories / Implementation Decisions / Testing Decisions / Out of Scope / Further Notes), pre-triaged: "publish it to the project issue tracker. Apply the `ready-for-agent` triage label - no need for additional triage." (SKILL.md:19) No file paths or code snippets, except decision-encoding prototype snippets (SKILL.md:55-57).
- **Session boundaries / hand-offs**: none stated — no `/clear`, and `/to-tickets` is never named as the next step. The same-session requirement with grilling is structural: with no interview allowed, an empty context would produce an empty spec.
- **vs. changelog**: matches the `/to-prd` → `/to-spec` rename story, including the "you may know this document as a PRD" bridge (`context/v1.1.md:30-32`).

### to-tickets

Source: `/Users/coachchucksol/.claude/skills/to-tickets/SKILL.md`. `disable-model-invocation: true`.

- **Inputs**: either the live session or a reference argument — "Work from whatever is already in the conversation context. If the user passes a reference (a spec path, an issue number or URL) as an argument, fetch it and read its full body and comments." (SKILL.md:17) So this is the first flow skill that can start a **fresh** session. Requires tracker config (SKILL.md:11). Human approval gate: "Iterate until the user approves the breakdown." (SKILL.md:56)
- **Artifacts**: tracer-bullet vertical-slice tickets with blocking edges, in one of two shapes (SKILL.md:62-63): a `tickets.md` at the repo root (local tracker) or one issue per ticket with native blocking links and "the `ready-for-agent` triage label unless instructed otherwise — the tickets are agent-grabbable by construction." Plus: "Do NOT close or modify any parent issue." (SKILL.md:65) Wide refactors get expand–contract sequencing instead of vertical slices (SKILL.md:40).
- **Session boundaries / hand-offs — the canonical `/clear` line of the whole suite**: "Work the frontier one ticket at a time with `/implement`, clearing context between tickets." (SKILL.md:113) Ticket sizing bakes the boundary in: "Each slice is sized to fit in a single fresh context window" (SKILL.md:33).
- **vs. changelog**: matches the merge of `/to-plan` + `/to-issues` and the two-shape artifact story (`context/v1.1.md:36-42`).

### implement

Source: `/Users/coachchucksol/.claude/skills/implement/SKILL.md` (5 lines of body). `disable-model-invocation: true`.

Full body:

> "Implement the work described by the user in the spec or tickets.
> Use /tdd where possible, at pre-agreed seams.
> Run typechecking regularly, single test files regularly, and the full test suite once at the end.
> Once done, use /code-review to review the work.
> Commit your work to the current branch." (SKILL.md:7-15)

- **Inputs**: "the spec or tickets" supplied by the user — the skill assumes the ticket/spec is handed to it (typically a fresh session per `to-tickets`' closing line). "Pre-agreed seams" points back at the seams agreed during `to-spec`/`tdd`.
- **Artifacts**: code + tests, committed to the current branch.
- **Session boundaries / hand-offs**: invokes `/tdd` and then `/code-review` **in the same session**, and commits after review. No `/clear` language of its own — the per-ticket fresh session lives in `to-tickets` (skill) and the changelog ("implement each one in a separate coding session", `context/v1.1.md:105`).
- **vs. changelog**: matches, including "After implementation, the `/implement` skill calls `/code-review` to review the work before committing." (`context/v1.1.md:109`)

### code-review

Source: `/Users/coachchucksol/.claude/skills/code-review/SKILL.md`. Model-invocable (no disable flag).

- **Inputs**: a fixed point for the diff — "If they didn't specify one, ask for it." (SKILL.md:19); the originating spec, found in order: issue references in commit messages (fetched via `docs/agents/issue-tracker.md`), a path argument, "A PRD/spec file under `docs/`, `specs/`, or `.scratch/`", else ask the user (SKILL.md:27-32); standards files (`CODING_STANDARDS.md`, `CONTRIBUTING.md`) plus a built-in 12-smell Fowler baseline (SKILL.md:36-56). Tracker prerequisite: "run `/setup-matt-pocock-skills` if `docs/agents/issue-tracker.md` is missing." (SKILL.md:13)
- **Artifacts**: conversation-only — two verbatim sub-agent reports under `## Standards` and `## Spec` plus a one-line per-axis summary (SKILL.md:76-80). Nothing is written or labelled.
- **Session boundaries / hand-offs**: none. It neither commits, closes tickets, nor names a next step — `implement` owns the commit, and no skill text closes the loop back to the next ticket.
- **vs. changelog**: matches on two axes / parallel sub-agents / smell baseline. Minor drift: the changelog's smell list (`context/v1.1.md:124-133`) has 10 smells; the installed skill has 12 (adds **Shotgun Surgery** and **Refused Bequest**), each with a fix direction. The installed skill is a superset.

---

## Supporting skills

### research

Source: `/Users/coachchucksol/.claude/skills/research/SKILL.md` (whole skill is 13 lines). Model-invocable.

- **Inputs**: a question. **Artifacts**: "Write the findings to a single Markdown file, citing each claim's source. Save it where the repo already keeps such notes" (SKILL.md:11-13).
- **Session boundaries**: none — the opposite, it's designed to avoid breaking the session: "Spin up a **background agent** to do the research, so you keep working while it reads." (SKILL.md:7)
- **vs. changelog**: matches `context/v1.1.md:189-196` point for point. In the flow it's the engine of wayfinder's AFK "Research" ticket type.

### prototype

Sources: `/Users/coachchucksol/.claude/skills/prototype/SKILL.md`, `LOGIC.md`, `UI.md`. Model-invocable — consistent with "The `/prototype` skill is now model-invoked so that Wayfinder can invoke it itself." (`context/v1.1.md:200`)

- **Inputs**: a design question, branched: logic/state → LOGIC.md (pure reducer/state-machine module + throwaway TUI shell); looks → UI.md (3–5 structurally different variants on one route behind `?variant=`, floating switcher, hidden in production).
- **Artifacts**: throwaway code near its future home, one-command runnable; then "**Delete or absorb when done.**" (SKILL.md:26) The durable output is the *answer*: "Capture it somewhere durable (commit message, ADR, issue, or a `NOTES.md` next to the prototype) along with the question it was answering." (SKILL.md:30)
- **Session boundaries / hand-offs**: none of its own; wayfinder's HITL "Prototype" ticket type invokes it and "Links the prototype as an asset" (wayfinder SKILL.md:78). `to-spec`/`to-tickets` both carry the matching receiving rule: prototype-derived snippets are the one exception to "no code in specs/tickets".
- **vs. changelog**: matches (`context/v1.1.md:198-207`).

### tdd

Sources: `/Users/coachchucksol/.claude/skills/tdd/SKILL.md`, `tests.md`, `mocking.md`. Model-invocable.

- **Inputs**: seams agreed with the user — "**Test only at pre-agreed seams.** Before writing any test, write down the seams under test and confirm them with the user." (SKILL.md:22) In the flow those seams were already agreed in `to-spec` step 2, which is what `implement`'s "pre-agreed seams" refers to.
- **Artifacts**: tests + code via red → green vertical slices. Reference material, not a step list: "This skill is the reference that makes that loop produce tests worth keeping" (SKILL.md:8).
- **Session boundaries / hand-offs**: one, pointing at review: "**Refactoring is not part of the loop.** It belongs to the review stage (see the `code-review` skill), not the red → green implementation cycle." (SKILL.md:36)
- **vs. changelog**: matches the v1.1 rewrite — "reference material only … refactoring is no longer part of the TDD loop. It's now handled in the code review phase" (`context/v1.1.md:219-227`).

### setup-matt-pocock-skills

Sources: `/Users/coachchucksol/.claude/skills/setup-matt-pocock-skills/SKILL.md` + five seed templates (`issue-tracker-github.md`, `issue-tracker-gitlab.md`, `issue-tracker-local.md`, `triage-labels.md`, `domain.md`). `disable-model-invocation: true`.

- **Position in the flow**: strictly first — "Run once before first use of the other engineering skills." (SKILL.md:3, description) Five downstream skills name it as their fallback (`wayfinder`:25, `to-spec`:9, `to-tickets`:11, `code-review`:13, plus pre-1.1 `to-prd`:9 and `to-issues`:11).
- **Artifacts**: `docs/agents/issue-tracker.md`, `docs/agents/triage-labels.md`, `docs/agents/domain.md`, and an `## Agent skills` block in `CLAUDE.md`/`AGENTS.md` (SKILL.md:97-123). The tracker docs carry the "Wayfinding operations" section wayfinder depends on (e.g. `issue-tracker-github.md:36-45`: map = issue labelled `wayfinder:map`, claim = `--add-assignee @me` as "the session's first write", native issue dependencies for blocking). This repo already has the GitHub output in `docs/agents/`.
- **Re-run policy**: "re-running this skill is only necessary if they want to switch issue trackers or restart from scratch." (SKILL.md:127)
- **vs. changelog**: not covered by the v1.1 changelog (predates it / orthogonal); no conflicts.

---

## Pre-1.1 leftovers: shadowing, duplication, conflicts

The changelog's migration instruction, verbatim: "This gives you the freedom to pick and choose which skills you want. Once done, go through your skills folder and make sure no old skills are lingering." (`context/v1.1.md:17`) And in What to Watch For: "**`/to-spec` replaces `/to-prd`** – Update any processes or documentation" and "**`/to-tickets` replaces `/to-plan` and `/to-issues`** – These are now unified" (`context/v1.1.md:231-232`).

### to-prd (leftover — since removed)

`/Users/coachchucksol/.claude/skills/to-prd/SKILL.md` was `to-spec` verbatim with "spec" → "PRD" (same seam step, same template, same "Apply the `ready-for-agent` triage label" at line 19). `disable-model-invocation: true` on both, so the model couldn't auto-pick the wrong one — the risk was a user typing `/to-prd` from habit. The artifact space **fully overlapped**: both published a `ready-for-agent` document to the same tracker, and `code-review` step 2 searches for "A PRD/spec file" and issue references without distinguishing, so a stray PRD issue would be indistinguishable from a spec. Duplication, not contradiction.

### to-issues (leftover — since removed)

`/Users/coachchucksol/.claude/skills/to-issues/SKILL.md` was `to-tickets` minus three v1.1 additions: no local `tickets.md` mode, no wide-refactor expand–contract exception (to-tickets SKILL.md:40), and — most important for the loop — **no closing hand-off line**. `to-issues` ended at "Do NOT close or modify any parent issue." (SKILL.md:84); it never mentioned `/implement` or clearing context. Both were `disable-model-invocation: true` and both published `ready-for-agent` issues to the same tracker, so a habitual `/to-issues` produced near-identical artifacts but silently dropped the per-ticket session-boundary instruction. Duplication plus a lost hand-off, not contradiction.

### grill-me (NOT a leftover — keep)

Despite the suspicion, the installed `grill-me` is the v1.1 thin wrapper ("Run a `/grilling` session.") matching the changelog's architecture note that `/grill-me` and `/grill-with-docs` "rely on a central reference grilling skill" (`context/v1.1.md:47-48`). It's `disable-model-invocation: true`, so it can't collide with the model-invocable `grilling` on trigger descriptions. No shadowing.

### review (undocumented fourth leftover — since removed; found during this audit)

`/Users/coachchucksol/.claude/skills/review/SKILL.md` was a pre-smell-baseline copy of `code-review`: `diff` showed it identical except the name and the entire Fowler smell baseline (code-review SKILL.md:38-57) plus the smell-aware sub-agent brief, which `review` lacked. Unlike `to-prd`/`to-issues`, **both `review` and `code-review` were model-invocable and their `description` frontmatter was word-for-word identical**, so on any "review this branch" request the model could invoke the stale `review` and silently skip the smell baseline — the one v1.1 feature the changelog calls "outrageously useful" (`context/v1.1.md:137`). `implement` says "use /code-review" by name, which disambiguated that path, but direct user requests were a coin flip. This was the only leftover with genuine trigger shadowing.

---

## The implied loop, as the skills themselves describe it

Assembled only from skill text (changelog-only links marked):

```
/setup-matt-pocock-skills          once per repo ("Run once before first use")
        |
        v
ENTRY — pick one:
  small/medium: /grill-me or /grill-with-docs  ──one session──┐
  large/foggy:  /wayfinder                                    │
     chart session ("Stop — charting the map is one           │
       session's work; do not also resolve tickets")          │
     then N ticket sessions ("never resolve more than         │
       one ticket per session") — /grilling, /research,       │
       /prototype, /domain-modeling per ticket type           │
     map done ──[changelog only: "turn it into a spec         │
       the regular way" — no skill names /to-spec]──┐         │
        v                                           v         v
/to-spec — SAME session as the grilling conversation
  ("takes the current conversation context … Do NOT interview")
  → spec issue on tracker, ready-for-agent
        |
        v   (same session, or fresh session passing the spec as argument —
        |    "If the user passes a reference (a spec path, an issue number
        |    or URL) as an argument, fetch it")
/to-tickets → ticket issues with blocking edges, ready-for-agent
        |
        v   *** the canonical /clear point ***
        |   "Work the frontier one ticket at a time with /implement,
        |    clearing context between tickets."
per ticket, fresh session:
  /implement → /tdd ("pre-agreed seams") → /code-review (same
    session: "Once done, use /code-review") → "Commit your work
    to the current branch."
        |
        v
next frontier ticket, fresh session — the restart is implied by
to-tickets' line; no skill states the loop explicitly, and nothing
names what follows /code-review.
```

Where sessions break, per the texts:

| Boundary | Stated where | Language |
| --- | --- | --- |
| setup → everything | setup SKILL.md:3 | "Run once before first use of the other engineering skills" |
| wayfinder chart → ticket work | wayfinder SKILL.md:115 | "Stop — charting the map is one session's work" |
| wayfinder ticket → ticket | wayfinder SKILL.md:105 | "never resolve more than one ticket per session" |
| wayfinder map → spec | changelog only (`v1.1.md:179`) | "turn this map … into a spec the regular way" |
| grilling → to-spec | **no break** (to-spec SKILL.md:7) | to-spec consumes "the current conversation context" |
| to-spec → to-tickets | optional break (to-tickets SKILL.md:17) | conversation context *or* a passed reference |
| to-tickets → implement | to-tickets SKILL.md:113 | "clearing context between tickets" |
| implement → code-review | **no break** (implement SKILL.md:13) | "Once done, use /code-review" |
| code-review → next ticket | unstated in skills; changelog `v1.1.md:105` | "implement each one in a separate coding session" |
