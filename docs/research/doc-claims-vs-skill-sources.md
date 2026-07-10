# Audit: teaching-doc claims vs the skill sources installed today

> Resolves wayfinder ticket #17 (map #16, constdecimals0/workflow) — audited 2026-07-09.
>
> Compared: every behavioral claim in `README.md`, `example/tutorial.md`, and `context/*.md`
> against the skill sources `npx skills add mattpocock/skills` installs today
> (github.com/mattpocock/skills @ `d574778`, 2026-07-08).

## Ground truth: there is no drift since v1.1

The drift half of the ticket has the cleanest possible answer:

- Upstream `HEAD` **is** the `v1.1.0` release — the GitHub compare `v1.1.0...HEAD` reports
  `ahead_by: 0`, `package.json` says `1.1.0`, and `.changeset/` holds no pending changesets.
- All 15 skills the docs teach are **byte-identical** between the installed set
  (`~/.claude/skills/`) and today's clone: `wayfinder`, `grilling`, `grill-me`,
  `grill-with-docs`, `domain-modeling`, `to-spec`, `to-tickets`, `implement`, `code-review`,
  `tdd`, `prototype`, `research`, `setup-matt-pocock-skills`, `triage`, `ask-matt`.

Consequences:

- **Nothing in the worked example needs re-capturing** — every run-log step reproduces against
  today's sources. (This answers the map's "Not yet specified" question about drift forcing a
  re-capture: it doesn't.)
- Every inaccuracy below is a mismatch with **v1.1 as it always was**, not something that
  rotted after the docs were written.
- `context/skills-review.md`'s quotes and line-number citations of the skill texts all still
  resolve correctly (spot-checked: wayfinder 105/112/115, to-spec 7/19, to-tickets 17/56/113,
  implement 7–15, code-review smell baseline).

Peripheral upstream changes that are *presentation*, not skill text:

- The upstream README's quickstart is now `npx skills@latest add mattpocock/skills` and stresses
  the installer is an interactive picker: "**Make sure you select `/setup-matt-pocock-skills`**."
  Our docs give the bare command and never mention the selection step.
- `review`, `to-prd`, `to-issues` do not exist anywhere upstream (renamed/deleted *in* 1.1.0), so
  a fresh install today cannot produce them. The "delete lingering pre-1.1 skills" setup step
  (README step 2, tutorial step 2) only applies to a machine that had a pre-1.1 install — for a
  new teammate it is a no-op. (On this machine all three are already gone.)
- A `skills/deprecated/` category exists upstream (`qa`, `design-an-interface`,
  `request-refactor-plan`, `ubiquitous-language`) but predates v1.1 (last touched 2026-04-28)
  and is referenced by no teaching doc. No action.

## Confirmed inaccuracies

Ordered by severity. "Source" quotes the skill text installed today.

### 1. The no-fog hatch does not drop into `/to-spec` — it stops and asks

- **Claim** — `README.md:35-37`: "Its no-fog hatch detects the small case and drops straight
  into `/to-spec` in the same session"; `example/tutorial.md:58-60`: "drops straight into
  `/to-spec`; don't pre-judge size yourself".
- **Source** — `wayfinder/SKILL.md:112`: "**If this surfaces no fog** — the way to the
  destination is already clear, the whole journey small enough for one session — you don't need
  a map. **Stop and ask the user how they'd like to proceed.**" The 1.1.0 changelog says the
  same: "it stops and asks how you'd like to proceed rather than building a map nobody needs."
- **Why it matters** — this is the README's stage-1 teaching. A reader on the small path will
  wait for an auto-chain into `/to-spec` that never happens; the correct teaching is that the
  hatch detects the small case and *you* choose (typically answering "go straight to
  `/to-spec`", which then runs on the grilling conversation still in context).
- Note: the retrospective phrases it correctly ("let the hatch decide" whether a map is needed,
  `run-retrospective.md:150-151`) — only README and tutorial over-claim the chain.

### 2. `/to-spec` has no "rebuilds from the completed map" behavior

- **Claim** — `README.md:47-48`: "`/to-spec` rebuilds from the completed map and publishes the
  spec"; `example/tutorial.md:170-172`: "With no argument and a cleared context, the agent
  rebuilds from the repo: it finds the completed map and its closed tickets, and synthesizes
  the spec."
- **Source** — `to-spec/SKILL.md:7`: "This skill takes the **current conversation context** and
  codebase understanding and produces a spec … Do NOT interview the user — just synthesize what
  you already know." The only repo-reading it prescribes is step 1's "Explore the repo to
  understand the current state of the codebase." It never mentions a map, closed tickets, or a
  fresh-session mode — and `wayfinder` never names `/to-spec` (the joint exists only in the
  v1.1 release notes: "take this map and turn it into a spec the regular way").
- **Why it matters** — the run's rebuild *worked*, but it was agent improvisation on top of a
  skill whose stated input is the live conversation. The retrospective reports it accurately as
  the suite's "fuzziest hand-off" that "held" in practice (`run-retrospective.md:39-42`);
  README and tutorial upgrade that observation into designed behavior. Safer teaching: pass the
  map URL to `/to-spec` in the fresh session (consistent with Retro rule 1, "pass the
  argument"), or present the bare-invocation rebuild explicitly as what-the-run-showed.

### 3. `code-review`'s standards axis never names ADRs or `CONTEXT.md`

- **Claim** — `README.md:57-58`: "standards axis against the ADRs and `CONTEXT.md`";
  `example/tutorial.md:212-213`: same.
- **Source** — `code-review/SKILL.md:36`: standards sources are "Anything in the repo that
  documents how code should be written, such as `CODING_STANDARDS.md` or `CONTRIBUTING.md`",
  plus the always-on 12-smell Fowler baseline (SKILL.md:38-56).
- **Why it matters** — in *this* repo the standards sub-agent did read ADR 0001/0002 and
  `CONTEXT.md` (they qualify under "anything…", and the run log session 12 records it), so the
  claim is accurate-in-effect here. But stated as skill behavior it promises something the
  skill doesn't: in a repo without domain docs the axis reads whatever standards files exist
  plus the smell baseline. Related gap: **no teaching doc mentions the Fowler smell baseline at
  all** — the one v1.1 code-review feature the release notes call "outrageously useful" — nor
  that `/code-review` asks for a fixed point when invoked without one outside a fresh
  implement session.

### 4. Wayfinder work-mode invocation is map-first in the skill, ticket-first in the docs

- **Claim** — `README.md:92-93` and the tutorial teach `/wayfinder <ticket name>` for parallel
  sessions, with no map reference.
- **Source** — `wayfinder/SKILL.md:119`: "User invokes with a map (URL or number). A ticket is
  **optional**."
- **Why it matters** — barely. Passing only a ticket name works whenever one map is live (the
  session finds the map itself), and the retro already teaches "pass the map URL when more than
  one map is live." Worth at most a clause acknowledging the skill's own calling convention.

### 5. "The agent chains into `/research`" is emergent, not specified

- **Claim** — `example/tutorial.md:98-99`: "The agent chains into `/research` and works alone."
- **Source** — wayfinder's Research ticket type (`SKILL.md:74`) never names the `/research`
  skill; it says the ticket "Creates a markdown summary as a linked asset." `/research` is
  model-invocable and did fire in the run — but by trigger-matching, not by wayfinder's text.
  (Contrast: `implement` *does* name `/tdd` and `/code-review`, and wayfinder *does* name
  `/grilling`, `/domain-modeling`, `/prototype` — those chain-claims are all sourced.)
- **Why it matters** — low; a reader whose research ticket doesn't auto-chain shouldn't think
  the run is broken. One word ("the agent reaches for `/research`") would defuse it.

## Framing divergence worth an explicit decision (not an inaccuracy)

**"Entry is always `/wayfinder`" is this repo's doctrine, not upstream's.** The 1.1.0 release
notes deliberately settle wayfinder as "a **situational on-ramp**, not the new main entry flow —
the grill-led *idea → ship* chain stays the front door," with `grill-me`/`grill-with-docs`
signposting *up* to wayfinder only when the effort exceeds one session. The README inverts this
(entry always `/wayfinder`, `/grill-with-docs` demoted to "a shortcut for work you *already
know* is small"). Nothing in the skill texts contradicts the local doctrine — wayfinder's own
hatch makes it workable — but the review should decide consciously: keep the inversion (it is
simpler to teach and the run validated it) and perhaps note it diverges from upstream's framing,
or align with upstream.

## Verified accurate (the load-bearing claims that check out)

- **Setup** — `/setup-matt-pocock-skills` wires tracker, triage labels, domain-docs layout;
  once per repo; re-run only to switch trackers (`SKILL.md:3,127`).
- **Wayfinder session shape** — never resolve more than one ticket per session (105); charting
  stops when the map exists (115); claim = assignee (skill + tracker doc); frontier =
  open/unblocked/unclaimed; tickets sized to one 100K-token session (57); fog graduates on
  resolution; HITL/AFK split (research AFK; grilling/prototype HITL).
- **Grilling** — one question at a time (8); facts looked up, decisions put to the human (10);
  confirmation gate before enacting (12). `grill-me` is the current v1.1 thin wrapper — the
  "keep `grill-me`" advice is right.
- **to-spec** — seam sketch checked with the user (17); publishes to the tracker with
  `ready-for-agent`, no further triage (19); no file paths/code except decision-encoding
  prototype snippets (55-57).
- **to-tickets** — works from conversation *or* a passed reference, so fresh-session + spec URL
  is a sourced recovery move (17); approval gate (56); native sub-issues/blocking preferred,
  `ready-for-agent` applied (63); its last line is the suite's only explicit `/clear`
  instruction — "the canonical clear" teaching is exactly sourced (113).
- **implement** — scope is "the work described by the user in the spec or tickets" (so a bare
  invocation genuinely guesses); `/tdd` at pre-agreed seams; `/code-review` then commit, same
  session (7-15).
- **code-review** — two axes as parallel sub-agents; spec axis locates the ticket via
  commit-message issue references (29); reports are kept separate, never reranked (78-80).
- **prototype** — throwaway from day one, near its future home, one command to run,
  delete-or-absorb mandate (21-26); the durable output is the captured answer (30).
- **tdd** — reference-only; red → green with refactoring moved to review; seams confirmed
  before any test.
- **The close-out gap** — confirmed: no skill text closes the spec, the map, or anything after
  `code-review`. The "close-out is a prompt you type" teaching is sound.
- **The whole `/clear` table** — every row consistent with the sources: grilling → to-spec never
  (to-spec:7), to-spec → to-tickets same-session with URL recovery (to-tickets:17), the
  canonical clear (to-tickets:113), implement → code-review never (implement:13).
- **Install command** — `npx skills add mattpocock/skills` still resolves and still gets v1.1
  (upstream now writes `skills@latest` and stresses picking `setup-matt-pocock-skills` in the
  interactive picker — worth mirroring).

## Cross-doc inconsistencies (found in passing; internal, not vs sources)

- **Session numbering disagrees three ways.** The run log has **18** sessions (implementation
  normalized to 12–17, close-out 18); the retrospective cites "sessions 1–15 in
  [example-run-log.md]" and numbers its divergences in the *real* 15-session numbering
  (close-out = "session 15"); the tutorial renumbers to its own 15 (implement 9–14, close-out
  15). README:102 says "all 18 sessions". A reader cross-referencing retro "session 14" against
  run-log "session 14" lands on different sessions. One numbering (or an explicit mapping note)
  should win.
- **Question count wobble** — run log session 1 says "nine picker questions" then lists eight;
  the tutorial says "eight questions".
- **`context/skills-review.md` snapshot claims are stale on purpose** — it reports `review`,
  `to-prd`, `to-issues` as "still installed", which was true when written and false now (they
  were deleted per the setup step it motivated). Its skill-text analysis remains fully valid. A
  one-line dateline note would keep it honest as an exhibit without rewriting it.

## What this feeds

The map's remaining decisions (per-file edit decisions, reading path, fate of `context/`)
should treat findings 1–2 as must-fix in README + tutorial, finding 3 as a two-word softening
plus a decision on whether to introduce the smell baseline, findings 4–5 as optional clauses,
and the framing divergence as an explicit keep/align decision. No re-capture of the worked
example is needed anywhere.
