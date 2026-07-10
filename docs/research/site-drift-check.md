# Drift-check: `site/` against the canonical markdown

> Resolves wayfinder ticket #18 (map #16, constdecimals0/workflow) — checked 2026-07-09.
>
> Compared: `site/content.js`, `site/index.html`, `site/app.js` against `README.md`,
> `example/tutorial.md`, and the verbatim prompts in `context/example-run-log.md`
> (with `context/run-retrospective.md` and `example/app/src/` as supporting ground truth).

## Verdict

The site is a faithful adaptation in substance: every stage card, both tables, the six
rules, the setup steps, and all run facts spot-checked match README/tutorial, and **eight
of the nine run prompts it tags "you type" match the run log character-for-character**
(including the "sidbar" typo). The drift that exists clusters in two places: (1) the
site's *framing claims about itself* are stronger than what it delivers — "every prompt
was typed", "beats mark every boundary" — and each is falsified somewhere; (2) the
free-play mini-game diverges from the shipped app it claims to be (pad colors permuted,
timing constants off).

Findings: **1 error · 2 drift · 2 judgment-call.**

## Findings

### 1. ERROR — the close-out prompt was never typed, but sits under a word-for-word claim

- **Site says**: `content.js:483` (step `session-15`) presents, tagged "you type":
  *"All tickets under spec #9 are closed — run the loop's close-out: close the spec and
  the map with comments linking the commits, and prune any prototypes and the effort's
  .scratch/."* — under two framing claims: *"Every prompt you'll be shown was typed in
  the real run, captured verbatim in the run log"* (`content.js:36–38`, step
  `what-this-is`) and *"Blocks tagged **you type** are exactly what was typed, word for
  word"* (`content.js:217–218`, step `now-play-it`).
- **Run log says**: Session 18 — *"**User typed** (`/clear`, then): > /wayfinder — No
  further prompts"*. The close-out ran only because the meta-map held a ticket for it;
  the quoted prompt is the *recommended* form from the lesson (run log: *"the close-out
  is a prompt you type yourself — e.g. …"*; retro §5), and the run-log example doesn't
  even include the prune clause.
- **Why it matters**: the walkthrough's core honesty pledge ("everything here happened")
  is falsified by its final prompt. Inherited: `example/tutorial.md` session 15 has the
  same blockquote under the same "every prompt … was typed" claim, so the fix likely
  lands in both renditions (present the close-out as the recommended prompt, not a
  replayed one).

### 2. DRIFT — "beats mark every boundary, and only the boundaries" is false

- **Site says**: `content.js:219–220` (step `now-play-it`): *"the board-flash
  `── /clear ──` beats mark every boundary, and **only** the boundaries"*; also
  `content.js:53–55`: *"that's a session boundary: the real run typed /clear … right
  there."*
- **Canonical says**: the run log records 17 session boundaries (18 sessions). The site
  plays the beat only at card breaks: `grilling-tickets` compresses three sessions
  (run-log 3, 5, 7) under one beat, `graduated-fog` two (8, 9), the implement cards six
  (12–17), and `claim-race` ends with session 5's opening prompt with no beat before the
  next card.
- **Why it matters**: the interstitial is the site's main device for teaching the
  `/clear` rhythm; a reader counting flashes sees ~8 clears standing in for 17. The
  tutorial's equivalent claim is scoped to its `## Session` headings and stays true;
  the site's restatement overreaches.

### 3. DRIFT — the free-play board's colors aren't the game's

- **Site says**: `content.js:573–577` (and the header board, `index.html:15–19`, and the
  favicon): up = green, **left = yellow, right = red, down = blue**, introduced as
  *"You've earned a run of the game this workflow built"* (`content.js:562–563`).
  `app.js:152` also claims *"Arrow keys mirror the game's pad bindings: up/right/down/left
  = green/red/blue/yellow."*
- **App says**: `example/app/src/ui.rs:37–63` — Up = green, **Down = yellow, Left = red,
  Right = blue**. Three of four pads are permuted; the app.js "mirror" comment is wrong
  as stated. (`CONTEXT.md` doesn't pin colors — "four colored quadrants" — so the shipped
  app is the authority.)
- **Why it matters**: the site names the artifact ("the game this workflow built") and
  the prototype card points at the header board as "the very board" the run locked; a
  reader who then runs `example/app/` sees a differently-colored board.

### 4. JUDGMENT-CALL — mini-game timing deviates from the decided numbers it claims to mirror

- **Site says**: `app.js:200–202`: *"The rules mirror the example app"*. Constants:
  watch tempo 600/470/350/250 ms per step (`app.js:264`), uniform 900 ms round break
  (`app.js:319`), 900 ms get-ready (`app.js:283`), "SPEED UP!" shown during the next
  Watch (`app.js:296`).
- **Canonical says**: decided in run-log session 8 and shipped in
  `example/app/src/game.rs:12–22`: tempo **570/430/320/240** ms per step
  (`(450,120)…(180,60)`), round break **800 ms** stretched to ~1.5 s on tier-ups with the
  `SPEED UP! ×n` callout *during the break*, get-ready 1000 ms.
- **Why it matters**: low stakes — the page's visible claims (sudden death, tiers, echo
  timeout, browser-local high score) are all accurate, and 3.0 s timeout / tier rounds
  1-5-9-13 / 10×tier scoring match exactly. But the code comment overclaims; either
  adopt the decided constants or soften the comment.

### 5. JUDGMENT-CALL — "you type" tag on prompts that predate the replay

- **Site says**: `content.js:80` tags `/setup-matt-pocock-skills` "you type" (step
  `setup`), and `content.js:198` tags the `spec #N` close-out *template* "you type"
  (step `stage-close`) — while `what-this-is` (`content.js:36–38`) has already promised
  *"Every prompt you'll be shown was typed in the real run."*
- **Canonical says**: neither appears in the run log (setup isn't captured there; `#N` is
  a placeholder form from README's loop section).
- **Why it matters**: mild — the stronger `now-play-it` claim correctly scopes itself to
  "what follows", but the earlier global claim covers these. A distinct tag (e.g.
  "you'll type") or softening `what-this-is` to match `now-play-it`'s scoping resolves it.

## Observations on the canonical baseline (not site drift)

Surfaced while comparing; relevant to the sibling accuracy-audit ticket:

- Run-log session 1 says the user answered **"nine picker questions"** but enumerates
  eight; tutorial and site both say "eight questions". One of the three is wrong.
- Tutorial's "Sessions 2–7" heading spans **seven** ticket-sessions (run-log sessions
  2–9 minus the interrupted collision), so the range is off by one; the site copies the
  kicker verbatim.
- Tutorial/site session numbering (1, 2–7, 8, 9–14, 15) is the run log's *normalized*
  shape, while both point at "the run log — every prompt of all 18 sessions"; this is
  by design (run-log cleanup note) but a reader can trip on 15-vs-18.

## Checked and clean

- **Prompts** (match the run log exactly): the session-1 `/wayfinder` idea; the pasted
  frontier list; `/wayfinder Grilling: high-score persistence`; bare `/wayfinder`
  (prototype); both prototype reactions (incl. the "sidbar" typo); `/to-spec`;
  `/to-tickets`; `/implement …/issues/10`; `/code-review`.
- **Structure**: all README sections and all tutorial sections are represented; no loop
  stage, friction story, "done when" gate, or rule is dropped; the extra steps (title,
  now-play-it, game-over) are legitimate medium adaptations, and the game-over card
  correctly declares README/tutorial as the source of truth that wins on disagreement.
- **Tables**: the six-stage loop table and the eight-row `/clear` table match
  README/tutorial row-for-row, including the two "never" boundaries and the canonical
  clear.
- **Facts**: five child tickets (1 research / 3 grilling / 1 prototype); 12 CONTEXT.md
  terms; ADRs 0001/0002; 33 user stories, `ready-for-agent`; tickets #10–#15 and their
  chain order; ~12 min research session; 13-second claim race; ~15 min bare-implement
  cost; commit `c730682`; the #11–#15 batch with single review; the unpushed
  `Closes #10` trailer story; close-out results (spec #9, map #1, prototype pruned by
  #15, domain docs never cleaned).
- **Free-play rules that do match the app**: sudden death, tiers entering at rounds
  1/5/9/13 with ×4 plateau, 10 × tier per-step scoring, 3.0 s per-key echo timeout,
  death-freeze reveal of the expected pad, high score persisting (localStorage as the
  disclosed browser analogue of the XDG file).
