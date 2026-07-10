// Step content, adapted from README.md and example/tutorial.md — those files are the
// source of truth. Prompts tagged "you type" are verbatim from the real run; prompts
// tagged "you'll type" are recommended forms the run never typed as shown. Prose is
// adapted for the click-through medium.
//
// color: which pad accents the step — green (Chart), red (Decide), yellow (Plan),
// blue (Build), all (Close), hub (neutral).
// clearAfter: advancing past this step plays the /clear board flash.

const GH = "https://github.com/constdecimals0/workflow";

const STEPS = [

// ─── Start ────────────────────────────────────────────────────────────────────
{
  id: "title",
  kicker: "",
  title: "The Workflow",
  color: "hub",
  body: `
    <p class="lede">How to run a development loop with the
    <a href="https://www.aihero.dev/skills">Matt Pocock skills</a> — which skill to run when,
    where sessions break, and what to clean up.</p>
    <p>Taught by the run that built a real game: the <strong>Simon Says terminal game</strong>
    in this repo was built in one full pass of the loop, captured prompt-by-prompt.</p>
    <p class="hint">Click <strong>press start</strong>, or use the arrow keys — this is a
    Simon Says walkthrough, after all.</p>
  `,
  nextLabel: "▶ press start"
},
{
  id: "what-this-is",
  kicker: "START HERE",
  title: "Everything here happened",
  color: "hub",
  body: `
    <p>This is a <strong>prompt-along walkthrough</strong>. Every prompt tagged
    <span class="prompt-tag inline">you type</span> was typed in the real run, captured
    verbatim in the <a href="${GH}/blob/main/context/example-run-log.md">run log</a> (the log
    presents the run normalized to 18 sessions; this walkthrough's 15 are the run's real
    shape). Prompts tagged <span class="prompt-tag inline will">you'll type</span> are the
    recommended form — the run never typed them as shown. Where the run hit friction, the
    walkthrough says so instead of hiding it — the consolidated lessons live in
    the <a href="${GH}/blob/main/context/run-retrospective.md">run retrospective</a>, cited
    throughout as <em>Retro rule N</em>.</p>
    <p>Three things you'll practice:</p>
    <ul>
      <li><strong>One loop = one feature.</strong> The whole walkthrough is one loop. The
      per-ticket implement cycle is a sub-loop inside it.</li>
      <li><strong>Sessions and <code>/clear</code>.</strong> A session is one agent context
      window. The loop has settled <code>/clear</code> boundaries — every one is marked here,
      including the two boundaries where you must <em>never</em> clear.</li>
      <li><strong>Passing arguments.</strong> The run's one recurring failure was invoking a
      skill bare and letting the agent guess its scope (<em>Retro rule 1</em>). You'll see
      exactly where that bit.</li>
    </ul>
    <p class="hint">When you see a board flash and <code>── /clear ──</code> between steps,
    that's a session boundary: the real run typed <code>/clear</code> (or opened a new
    terminal) right there.</p>
  `
},
{
  id: "setup",
  kicker: "START HERE",
  title: "Setup, once",
  color: "hub",
  body: `
    <ol class="setup">
      <li><strong>Install the v1.1 skills:</strong>
        <div class="prompt shell"><span class="prompt-tag">shell</span><button class="copy">copy</button><pre><code>npx skills@latest add mattpocock/skills</code></pre></div>
        The installer is an interactive picker — make sure you select
        <code>/setup-matt-pocock-skills</code>.
      </li>
      <li><strong>Only if this machine had a pre-1.1 install:</strong> delete the lingering
      skills — <code>review</code>, <code>to-prd</code>, <code>to-issues</code> (keep
      <code>grill-me</code>, the current v1.1 wrapper). Remove both the
      <code>~/.claude/skills/</code> symlink and its <code>~/.agents/skills/</code> target.
      The stale <code>review</code> is the dangerous one: its trigger description is
      identical to <code>code-review</code>'s, so a generic "review this branch" can silently
      run the old skill. On a fresh machine there is nothing to delete.</li>
      <li><strong>Rust toolchain</strong>, stable ≥ 1.88, if you're following the Simon Says
      example (its manifest pins <code>rust-version = "1.88"</code>).</li>
      <li><strong><code>gh</code> authenticated</strong> against a GitHub repo with issues
      enabled — the workflow uses issues as its tracker: the map, its tickets, the spec, and
      the implementation tickets all live there.</li>
      <li><strong>Once per repo, before any loop:</strong>
        <div class="prompt"><span class="prompt-tag will">you'll type</span><button class="copy">copy</button><pre><code>/setup-matt-pocock-skills</code></pre></div>
        wires up the issue tracker, triage labels, and domain-docs layout.</li>
    </ol>
  `
},

// ─── The loop ─────────────────────────────────────────────────────────────────
{
  id: "the-loop",
  kicker: "THE LOOP",
  title: "One loop = one feature",
  color: "hub",
  body: `
    <p>Not a ticket, not a project — <strong>one feature</strong>. The per-ticket implement
    cycle is a sub-loop inside it. Run the whole loop again when the next feature arrives.</p>
    <table class="loop-table">
      <thead><tr><th>Stage</th><th>You type</th><th>Session shape</th></tr></thead>
      <tbody>
        <tr class="tint-green"><td>1. Chart</td><td><code>/wayfinder &lt;your idea&gt;</code></td><td>one session; stops once the map exists</td></tr>
        <tr class="tint-red"><td>2. Decide</td><td><code>/wayfinder &lt;ticket name&gt;</code>, per map ticket</td><td>one ticket per session</td></tr>
        <tr class="tint-yellow"><td>3. Plan</td><td><code>/to-spec &lt;map-url&gt;</code> then <code>/to-tickets</code></td><td>one session, back-to-back</td></tr>
        <tr class="tint-blue"><td>4. Build</td><td><code>/implement &lt;ticket-url&gt;</code></td><td>one ticket per session</td></tr>
        <tr class="tint-blue"><td>5. Review</td><td><code>/code-review</code></td><td>same session as Build — never <code>/clear</code> between</td></tr>
        <tr class="tint-all"><td>6. Close</td><td>a close-out prompt you type yourself</td><td>one session</td></tr>
      </tbody>
    </table>
    <p class="hint">The next six steps walk each stage. Each stage owns a pad color — Review
    shares Build's blue because they're one session.</p>
  `
},
{
  id: "stage-chart",
  kicker: "THE LOOP · STAGE 1",
  title: "Chart",
  color: "green",
  body: `
    <p><strong>Entry is always <code>/wayfinder</code> — even for work you suspect is
    small.</strong> Its no-fog hatch detects the small case, then <strong>stops and
    asks</strong> how you'd like to proceed — the move is answering "go straight to
    <code>/to-spec</code>", which runs on the grilling still in context; don't pre-judge
    size yourself. (<code>/grill-with-docs</code> is a shortcut for work you
    <em>already know</em> is small.)</p>
    <p>Anything with real open questions gets a <strong>map</strong>: an issue whose child
    tickets are the decisions to make. Charting is one session; it stops when the map and its
    tickets exist.</p>
  `
},
{
  id: "stage-decide",
  kicker: "THE LOOP · STAGE 2",
  title: "Decide",
  color: "red",
  body: `
    <p><strong>Work the map's frontier, one ticket per session.</strong> The frontier is the
    map's open, unblocked, unclaimed tickets.</p>
    <ul>
      <li><strong>Research tickets</strong> run alone.</li>
      <li><strong>Grilling tickets</strong> interview you one question at a time and write the
      durable domain docs (<code>CONTEXT.md</code>, <code>docs/adr/</code>).</li>
      <li><strong>Prototype tickets</strong> build something cheap and throwaway to react
      to.</li>
    </ul>
    <p>Parallel terminals are fine, but each must be told its ticket by name — two bare
    sessions will race for the same claim.</p>
  `
},
{
  id: "stage-plan",
  kicker: "THE LOOP · STAGE 3",
  title: "Plan",
  color: "yellow",
  body: `
    <p><strong>Map done → fresh session.</strong> <code>/to-spec &lt;map-url&gt;</code>
    builds the spec from the completed map and publishes it as <code>ready-for-agent</code>;
    <code>/to-tickets</code> in the <strong>same session</strong> breaks it into
    implementation tickets chained with blocking edges.</p>
    <p>When you approve the breakdown, planning is over — <code>/clear</code>.
    <strong>This is the canonical clear</strong>, the one boundary the skills themselves
    name.</p>
  `
},
{
  id: "stage-build",
  kicker: "THE LOOP · STAGE 4",
  title: "Build",
  color: "blue",
  body: `
    <p><strong>Per ticket, fresh session.</strong> <code>/implement &lt;ticket-url&gt;</code>
    builds the ticket test-first — TDD at the seams the spec agreed. Always pass the ticket
    URL; a bare <code>/implement</code> will guess its scope.</p>
    <p>When the build lands, don't commit and don't <code>/clear</code> — the review comes
    first, in the same session.</p>
  `
},
{
  id: "stage-review",
  kicker: "THE LOOP · STAGE 5",
  title: "Review",
  color: "blue",
  body: `
    <p><strong>Same session as Build — never <code>/clear</code> between implement and
    review.</strong> Type <code>/code-review</code>: two review axes run as parallel agents,
    <strong>spec</strong> against the ticket and <strong>standards</strong> against whatever
    documents how code should be written — here, the ADRs and <code>CONTEXT.md</code> — plus
    an always-on baseline of twelve classic Fowler code smells.</p>
    <p>Verify the findings rather than blindly accepting them, fix the real ones
    <strong>pre-commit</strong>, and commit green. Then <strong>close the ticket with a
    comment linking the commit and verify it shows closed</strong> — an unpushed
    <code>Closes</code> trailer closes nothing — and <code>/clear</code>. As each ticket
    closes, the next unblocks.</p>
  `
},
{
  id: "stage-close",
  kicker: "THE LOOP · STAGE 6",
  title: "Close out",
  color: "all",
  body: `
    <p><strong>Frontier empty means the feature is done.</strong> No skill does this; you
    type it:</p>
    <div class="prompt"><span class="prompt-tag will">you'll type</span><button class="copy">copy</button><pre><code>All tickets under spec #N are closed — run the loop's close-out: close the spec and the map with comments linking the commits, and prune any prototypes and the effort's .scratch/.</code></pre></div>
    <p>That's also the between-loops cleanup: spec and map closed, prototypes and the
    effort's scratch space deleted. <code>CONTEXT.md</code> and <code>docs/adr/</code> are
    <strong>never</strong> cleaned up — domain docs accumulate across loops by design.</p>
  `
},

// ─── The tutorial ─────────────────────────────────────────────────────────────
{
  id: "now-play-it",
  kicker: "THE TUTORIAL",
  title: "Now watch the real run",
  color: "hub",
  body: `
    <p>Theory done. The rest of this walkthrough replays the run that built the
    <a href="${GH}/tree/main/example/app">Simon Says terminal game</a> — one full loop,
    session by session, replayed prompts verbatim.</p>
    <p>How to read what follows:</p>
    <ul>
      <li>Blocks tagged <span class="prompt-tag inline">you type</span> are exactly what was
      typed, word for word. Copy them freely.</li>
      <li>Blocks tagged <span class="prompt-tag inline will">you'll type</span> are the
      recommended form — the run never typed them as shown.</li>
      <li>Each session is a fresh context — a board-flash <code>── /clear ──</code> beat
      marks each session boundary at a card break; cards that compress several sessions say
      so.</li>
      <li><strong>What our run did</strong> callouts report the real outcome, including the
      divergences.</li>
    </ul>
  `
},
{
  id: "session-1",
  kicker: "THE TUTORIAL · SESSION 1",
  title: "Chart the map",
  color: "green",
  clearAfter: true,
  body: `
    <p>Entry is always <code>/wayfinder</code>, even for work you suspect is small — the
    no-fog hatch decides for you (<em>Retro rule 2</em>). Type your loose idea as the
    argument:</p>
    <div class="prompt"><span class="prompt-tag">you type</span><button class="copy">copy</button><pre><code>/wayfinder I want a small, fun Simon Says game that runs in the terminal, written in Rust, living in example/app/ of this repo</code></pre></div>
    <p>The agent chains into <code>/grilling</code> and asks one question at a time — mostly
    option pickers. In our run it took eight questions: what "done" looks like (design locked,
    ready for <code>/to-spec</code>), the dependency stance (go rich — ratatui), how much game
    "small, fun" means (polished arcade mini-game), what's out of scope up front (real audio,
    multiple game modes, CI/packaging, online leaderboards), and which fundamentals to lock
    now (classic 4 pads).</p>
    <div class="callout note"><div class="callout-tag">▶ what our run did</div>
    <p>The no-fog hatch did <strong>not</strong> fire — even this "small" game had real fog
    (difficulty feel, screen flow), so a full map was charted:
    <a href="${GH}/issues/1">Wayfinder map: Simon Says terminal game</a> with five child
    tickets wired as sub-issues with native blocking — one research, three grilling, one
    prototype — plus the fog noted on the map. The session stops after charting; that's
    wayfinder's own rule.</p></div>
    <div class="callout done"><div class="callout-tag">✓ done when</div>
    <p>The map issue exists with its tickets, and the session tells you it has stopped.
    <code>/clear</code>.</p></div>
  `
},
{
  id: "frontier",
  kicker: "THE TUTORIAL · SESSIONS 2–7",
  title: "Work the frontier",
  color: "red",
  body: `
    <p>The <strong>frontier</strong> is the map's open, unblocked, unclaimed tickets. Work
    them one per session, <code>/clear</code> between — our run carried seven tickets this
    way with no context trouble (<em>Retro rule 6</em>), packing them into six numbered
    slots by running terminals in parallel: the AFK research ticket worked alongside the
    first grilling.</p>
    <p>You can run frontier tickets in parallel too, but there's a friction story coming
    first.</p>
    <p class="hint">The map dealt five tickets: 1 research · 3 grilling · 1 prototype — plus
    two more that graduated out of the fog along the way.</p>
  `
},
{
  id: "research-ticket",
  kicker: "THE TUTORIAL · SESSIONS 2–7",
  title: "The research ticket (AFK)",
  color: "red",
  clearAfter: true,
  body: `
    <div class="prompt"><span class="prompt-tag">you type</span><button class="copy">copy</button><pre><code>/wayfinder - Research: ratatui patterns for a tick-driven mini-game (AFK)
- Grilling: core game rules
- Grilling: high-score persistence</code></pre></div>
    <p>(The run pasted the frontier list from session 1's closing summary — naming what's
    takeable.) The agent reaches for <code>/research</code> and works alone — about 12
    minutes in our run.</p>
    <div class="callout note"><div class="callout-tag">▶ produced</div>
    <p><a href="${GH}/blob/main/docs/research/ratatui-tick-driven-game.md">docs/research/ratatui-tick-driven-game.md</a>
    — crate versions, the tick + poll loop shape, flash timing — and the ticket closed.</p></div>
  `
},
{
  id: "claim-race",
  kicker: "THE TUTORIAL · SESSIONS 2–7",
  title: "⚠ Friction: the parallel claim race",
  color: "red",
  clearAfter: true,
  clearLabel: "fresh terminal, ticket named",
  body: `
    <div class="callout friction"><div class="callout-tag">⚠ what our run did</div>
    <p>Our run opened two more terminals 13 seconds apart, both typing bare
    <code>/wayfinder</code>. Both grabbed the <strong>same</strong> ticket — a claim is
    "assignee = the driving dev", and two sessions of the same dev are indistinguishable. The
    user saw the duplicate grilling start and interrupted it; nothing was damaged.</p></div>
    <p><strong>The lesson (Retro rule 1):</strong> a bare <code>/wayfinder</code> is fine only
    when you're solo and the frontier has an obvious next ticket. Parallel sessions must each
    be told their ticket by name, like the very next session did:</p>
    <div class="prompt"><span class="prompt-tag">you type</span><button class="copy">copy</button><pre><code>/wayfinder Grilling: high-score persistence</code></pre></div>
  `
},
{
  id: "grilling-tickets",
  kicker: "THE TUTORIAL · SESSIONS 2–7",
  title: "The grilling tickets (HITL)",
  color: "red",
  clearAfter: true,
  body: `
    <p>Each grilling ticket is a conversation — one question at a time, you answer, and when
    you confirm shared understanding the agent chains into <code>/domain-modeling</code> to
    make the decisions durable. Three sessions, compressed into one card — our run's
    grilling sessions produced:</p>
    <ul>
      <li><strong>Core game rules</strong> — arrow keys, append-one sequences, sudden death,
      per-key timeout, tiered speed-ups, steps × tier scoring. Chained
      <code>/domain-modeling</code> created <a href="${GH}/blob/main/CONTEXT.md">CONTEXT.md</a>
      at the repo root with 12 domain terms (Pad, Step, Sequence, Round, Watch, Echo, …).
      Resolving this ticket also <strong>graduated fog</strong>: the difficulty-feel question
      was now sharp enough to become a new ticket.</li>
      <li><strong>High-score persistence</strong> — single best score, plain-text file, XDG
      data dir, best-effort silent failure. Produced
      <a href="${GH}/blob/main/docs/adr/0001-std-only-plaintext-high-score.md">ADR 0001</a>.</li>
      <li><strong>Structure &amp; testing</strong> — bin crate + lib target, ratatui-free core
      with time and PRNG injected, core unit tests + one full-run integration test, gate =
      <code>cargo test</code> + <code>clippy -D warnings</code> + <code>fmt --check</code>.
      Produced <a href="${GH}/blob/main/docs/adr/0002-bin-lib-crate-with-pure-core.md">ADR 0002</a>.</li>
    </ul>
    <div class="callout done"><div class="callout-tag">✓ done when (each)</div>
    <p><em>You</em> confirm the understanding is shared — grilling's only gate — the ticket
    closes, and any domain docs are written. <code>/clear</code> between tickets.</p></div>
  `
},
{
  id: "prototype-ticket",
  kicker: "THE TUTORIAL · SESSIONS 2–7",
  title: "The prototype ticket (HITL)",
  color: "red",
  clearAfter: true,
  body: `
    <div class="prompt"><span class="prompt-tag">you type</span><button class="copy">copy</button><pre><code>/wayfinder</code></pre></div>
    <p>(Solo session by now, so bare was safe.) The agent chains into <code>/prototype</code>
    and builds a cheap, throwaway board render — explicitly outside <code>app/</code> — with
    layout, color, and flash variants for you to react to. Our run's reactions, verbatim:</p>
    <div class="prompt"><span class="prompt-tag">you type</span><button class="copy">copy</button><pre><code>I like a mix of 2/3. I like the hud sidebar ( take the sidbar as is ), and I like the arrow cross pad the most</code></pre></div>
    <div class="prompt"><span class="prompt-tag">you type</span><button class="copy">copy</button><pre><code>its perfect</code></pre></div>
    <div class="callout note"><div class="callout-tag">▶ produced</div>
    <p>The board design locked in one session (arrow-cross pad, HUD sidebar, hub in the
    middle — the very board blinking above this text), plus timing numbers that seeded the
    difficulty ticket. The prototype directory is <em>throwaway</em> — the last implementation
    ticket prunes it, per the prototype skill's own mandate.</p></div>
  `
},
{
  id: "graduated-fog",
  kicker: "THE TUTORIAL · SESSIONS 2–7",
  title: "The graduated-fog tickets",
  color: "red",
  clearAfter: true,
  body: `
    <p>Two more grilling sessions worked the tickets that graduated out of the fog:</p>
    <ul>
      <li><strong>Difficulty tuning</strong> — 4 speed tiers, tempo ramp, 3.0 s per-key
      timeout, ×1/×2/×3/×4 scoring.</li>
      <li><strong>Screen flow</strong> — Get Ready, Round Break, Death Freeze, Game Over
      overlay.</li>
    </ul>
    <p>After the last one: <strong>frontier empty — map complete.</strong></p>
  `
},
{
  id: "session-8",
  kicker: "THE TUTORIAL · SESSION 8",
  title: "Plan: /to-spec → /to-tickets",
  color: "yellow",
  clearAfter: true,
  clearLabel: "the canonical clear",
  body: `
    <p>Map done → <code>/clear</code> → a fresh session that turns the map into a spec and
    tickets. The reliable form passes the argument (<em>Retro rule 1</em>):
    <code>/to-spec &lt;map-url&gt;</code>, so the fresh session builds the spec from the map
    by instruction. Our run improvised instead and typed it bare:</p>
    <div class="prompt"><span class="prompt-tag">you type</span><button class="copy">copy</button><pre><code>/to-spec</code></pre></div>
    <p>With no argument and a cleared context, the agent rebuilt from the repo — found the
    completed map and its closed tickets, and synthesized the spec. Improvisation that held,
    not designed behavior: pass the map URL. Our run produced
    <a href="${GH}/issues/9">Spec: Simon Says terminal game</a> — 33 user stories, labelled
    <code>ready-for-agent</code>.</p>
    <p>Then, <strong>in the same session</strong>:</p>
    <div class="prompt"><span class="prompt-tag">you type</span><button class="copy">copy</button><pre><code>/to-tickets</code></pre></div>
    <p>One approval picker later, our run had six implementation tickets
    (<a href="${GH}/issues/10">#10</a>–<a href="${GH}/issues/15">#15</a>) in a linear
    native-dependency chain: walking skeleton → first playable → timing feel → speed tiers
    &amp; scoring → high-score persistence → integration test + prototype pruning.</p>
    <div class="callout friction"><div class="callout-tag">⚠ what our run actually did</div>
    <p>It <code>/clear</code>ed between <code>/to-spec</code> and <code>/to-tickets</code> and
    got away with it — bare <code>/to-tickets</code> found the spec only because the repo held
    exactly <strong>one</strong> <code>ready-for-agent</code> spec. Same-session is the
    canonical teaching; if you did clear, pass the spec URL rather than trusting the repo to
    contain only one spec (<em>Retro rule 1</em>).</p></div>
    <div class="callout done"><div class="callout-tag">✓ done when</div>
    <p>You approve the breakdown and the tickets land with blocking edges. Planning ends
    here. <code>/clear</code> — <strong>this is the canonical clear</strong> between planning
    and implementation.</p></div>
  `
},
{
  id: "implement-ritual",
  kicker: "THE TUTORIAL · SESSIONS 9–14",
  title: "Implement, one ticket per session",
  color: "blue",
  body: `
    <p>The per-ticket ritual, which each session runs start to finish:</p>
    <p class="ritual"><code>/implement &lt;ticket-url&gt;</code> → TDD →
    <code>/code-review</code> (same session) → fix findings → commit → close the ticket with
    a comment linking the commit → verify it shows closed → <code>/clear</code></p>
    <div class="prompt"><span class="prompt-tag">you type</span><button class="copy">copy</button><pre><code>/implement https://github.com/constdecimals0/workflow/issues/10</code></pre></div>
    <p>Always pass the ticket URL (<em>Retro rule 1</em> — friction story next). The agent
    chains <code>/tdd</code> at the pre-agreed seams (lib public API, injected time, seeded
    PRNG) and builds the ticket test-first.</p>
    <p>Then, <strong>in the same session</strong> — never <code>/clear</code> between
    implement and review:</p>
    <div class="prompt"><span class="prompt-tag">you type</span><button class="copy">copy</button><pre><code>/code-review</code></pre></div>
    <p>Two review axes run as parallel agents — spec against the ticket, standards against
    whatever documents how code should be written (here, the ADRs and
    <code>CONTEXT.md</code>), plus an always-on baseline of twelve classic Fowler code
    smells. Fix the real findings pre-commit and commit green
    (<code>cargo test</code>, <code>clippy -D warnings</code>, <code>fmt --check</code>). Our
    run's session for <a href="${GH}/issues/10">#10</a> did exactly this: the reviews' real
    findings were verified, fixed, and folded into commit <code>c730682</code>.</p>
    <p>Repeat for each ticket on the chain: as each closes, the next becomes unblocked.</p>
  `
},
{
  id: "bare-implement",
  kicker: "THE TUTORIAL · SESSIONS 9–14",
  title: "⚠ Friction: bare /implement grabs the whole backlog",
  color: "blue",
  body: `
    <div class="callout friction"><div class="callout-tag">⚠ what our run did</div>
    <p>Our run typed a bare <code>/implement</code> twice, and both times the session scoped
    itself to the <strong>entire backlog</strong>.</p></div>
    <ul>
      <li><strong>The first time</strong>, the user stopped it at the first verification
      gate, deleted the partial work (no commits had landed), and restarted canonically with
      the ticket URL. Cost: ~15 minutes. Stopping off-script sessions early is cheap
      (<em>Retro rule 5</em>).</li>
      <li><strong>The second time</strong> nobody stopped it, and it built all five remaining
      tickets (#11–#15) back-to-back with a <strong>single batched review</strong> at the end.
      The work landed real and green — but the batch forfeited per-ticket review granularity,
      a clean commit-to-ticket audit trail for the review fixes, and any seam to stop at had
      something gone wrong mid-backlog. That it landed green was the luck of a well-specified
      backlog, not a property of the shortcut.</li>
    </ul>
  `
},
{
  id: "ending-ritual",
  kicker: "THE TUTORIAL · SESSIONS 9–14",
  title: "⚠ Friction: the ending ritual is not optional",
  color: "blue",
  clearAfter: true,
  body: `
    <div class="callout friction"><div class="callout-tag">⚠ what our run did</div>
    <p>Our run's #10 session finished real, green work — then left a <code>Closes #10</code>
    trailer on an <strong>unpushed</strong> commit. The trailer only fires on push, so the
    issue sat open with no trace of the finished work until it was closed by hand later.</p></div>
    <p>Close your ticket with a comment linking the commit, and <strong>verify it actually
    shows closed</strong> before you <code>/clear</code> (<em>Retro rule 3</em>).</p>
    <div class="callout done"><div class="callout-tag">✓ done when (each ticket)</div>
    <p>Gate green, commit made, ticket closed with a commit-linking comment.
    <code>/clear</code> between tickets — but <strong>never</strong> between
    <code>/implement</code> and <code>/code-review</code>: the review reads the session's
    context, same session, pre-commit.</p></div>
  `
},
{
  id: "session-15",
  kicker: "THE TUTORIAL · SESSION 15",
  title: "The close-out",
  color: "all",
  clearAfter: true,
  clearLabel: "loop complete",
  body: `
    <p>Frontier empty = feature done. No skill closes the loop for you — the close-out is a
    prompt you type yourself (<em>Retro rule 4</em>). The recommended form:</p>
    <div class="prompt"><span class="prompt-tag will">you'll type</span><button class="copy">copy</button><pre><code>All tickets under spec #9 are closed — run the loop's close-out: close the spec and the map with comments linking the commits, and prune any prototypes and the effort's .scratch/.</code></pre></div>
    <p>One honest note: that block is the recommended form, not replayed history — the real
    run typed bare <code>/wayfinder</code>, and the close-out ran because a map ticket drove
    it.</p>
    <div class="callout note"><div class="callout-tag">▶ what our run did</div>
    <p>The session closed spec <a href="${GH}/issues/9">#9</a> with a closing comment listing
    the per-ticket commits, then closed map <a href="${GH}/issues/1">#1</a> linking the spec.
    The board prototype had already been pruned by ticket #15, so there was nothing else to
    delete. <code>CONTEXT.md</code> and <code>docs/adr/</code> are <strong>never</strong>
    cleaned up — domain docs persist across loops by design.</p></div>
    <div class="callout done"><div class="callout-tag">✓ done when</div>
    <p>Spec and map both show closed. The loop is complete — run it again when the next
    feature arrives.</p></div>
  `
},

// ─── Reference ────────────────────────────────────────────────────────────────
{
  id: "clear-table",
  kicker: "REFERENCE",
  title: "The /clear table",
  color: "hub",
  body: `
    <p>A <strong>session</strong> is one agent context window; <code>/clear</code> (or a new
    terminal) ends it. The settled boundaries, and what the run confirmed:</p>
    <table class="clear-table">
      <thead><tr><th>Boundary</th><th>Clear?</th></tr></thead>
      <tbody>
        <tr><td>wayfinder charting → ticket work</td><td><span class="chip yes">yes</span></td></tr>
        <tr><td>between wayfinder tickets</td><td><span class="chip yes">yes</span></td></tr>
        <tr><td>map done → planning session</td><td><span class="chip yes">yes</span> <span class="chip-note">the fresh session loads the map</span></td></tr>
        <tr><td>grilling → <code>/to-spec</code> (small path, no map)</td><td><span class="chip never">never</span> <span class="chip-note">it destroys to-spec's only input</span></td></tr>
        <tr><td><code>/to-spec</code> → <code>/to-tickets</code></td><td><span class="chip no">no</span> <span class="chip-note">same session; fresh-session + spec URL is the recovery move</span></td></tr>
        <tr><td><code>/to-tickets</code> → first <code>/implement</code></td><td><span class="chip yes">yes</span> <span class="chip-note"><strong>the canonical clear</strong></span></td></tr>
        <tr><td>between implement tickets</td><td><span class="chip yes">yes</span></td></tr>
        <tr><td><code>/implement</code> → <code>/code-review</code></td><td><span class="chip never">never</span> <span class="chip-note">same session, pre-commit</span></td></tr>
      </tbody>
    </table>
    <p class="hint">No session in the run ran out of context or cleared under pressure.</p>
  `
},
{
  id: "the-rules",
  kicker: "REFERENCE",
  title: "The rules",
  color: "hub",
  body: `
    <p>The one rule that outranks the rest: <strong>pass the argument.</strong> Every serious
    friction in the real run traced to a bare invocation letting the agent guess its scope —
    bare <code>/implement</code> grabbed the entire backlog (twice), parallel bare
    <code>/wayfinder</code> sessions raced for the same ticket (the skill's own convention is
    map-first — pass the map URL when more than one map is live). Bare is safe only when the
    repo state leaves exactly one thing to do.</p>
    <p>The <a href="${GH}/blob/main/context/run-retrospective.md">retrospective</a>'s
    distilled form — every serious friction in the run traces to breaking one of these:</p>
    <ol class="rules">
      <li><strong>Pass the argument.</strong> <code>/implement &lt;ticket-url&gt;</code>
      always; name the ticket to <code>/wayfinder</code> when sessions run in parallel; pass
      the map URL when more than one map is live; pass the spec URL to
      <code>/to-tickets</code> in a fresh session.</li>
      <li><strong>Let the hatch decide "small".</strong> Type <code>/wayfinder</code>
      regardless; don't pre-judge size.</li>
      <li><strong>Close your ticket before you <code>/clear</code>.</strong> Comment linking
      the commit, then verify it shows closed. An unpushed <code>Closes</code> trailer closes
      nothing.</li>
      <li><strong>The close-out is a prompt you type.</strong> Frontier empty: close the spec
      and the map, prune prototypes and the effort's <code>.scratch/</code>.</li>
      <li><strong>Stop off-script sessions early.</strong> Interrupting costs minutes;
      letting a batch run forfeits the loop's checkpoints.</li>
      <li><strong>Trust the loop's session shape.</strong> One ticket per session, review in
      the same session as the implementation, <code>/clear</code> on the settled boundaries.
      The run's only failures were departures from this — never the loop itself.</li>
    </ol>
  `
},
{
  id: "work-repo",
  kicker: "REFERENCE",
  title: "On a work repo",
  color: "hub",
  body: `
    <p>This run had the luxury of committing straight to <code>main</code> in a repo built
    to be looked at. On a real work repo, three things change:</p>
    <ul>
      <li><strong>One PR per feature.</strong> Branch at the canonical clear — between
      <code>/to-tickets</code> and the first <code>/implement</code>. Each implement session
      commits to the feature branch and still closes its ticket with a comment linking the
      commit (rule 3 unchanged — <code>Closes</code> trailers only fire at merge). The
      close-out gains three steps: push, open the PR, merge — then close the spec and the
      map.</li>
      <li><strong>Tracker choice.</strong> GitHub Issues is the default — maps and tickets
      live where the team already looks. If the effort must leave no trace, pick the
      local-markdown tracker at setup instead: GitHub-tracked maps can't be gitignored, so
      zero footprint fully applies only on the markdown tracker. Both are native to
      <code>/setup-matt-pocock-skills</code>.</li>
      <li><strong>Zero footprint.</strong> Gitignore everything the workflow writes — scratch
      space, prototype dirs, research notes, domain docs, agent config — and at setup's
      confirm-and-edit step, steer its agent-skills block into a gitignored local memory
      file. Complete cleanup is then deleting the ignored paths. The accepted cost: domain
      docs become local-only, per-machine.</li>
    </ul>
  `
},
{
  id: "game-over",
  kicker: "GAME OVER",
  title: "Free play",
  color: "all",
  body: `
    <p class="lede">Frontier empty. Spec closed. Map closed. <strong>The loop is
    complete.</strong></p>
    <p>You've earned a run of the game this workflow built. Watch the sequence, then echo it
    with the arrow keys (or click the pads). Sudden death, speed tiers, and your high score
    stays in this browser.</p>
    <div class="simon" id="simon">
      <div class="simon-stats">
        <div class="stat"><span class="stat-label">score</span><span class="stat-val" id="simon-score">0</span></div>
        <div class="stat"><span class="stat-label">high score</span><span class="stat-val" id="simon-high">0</span></div>
        <div class="stat"><span class="stat-label">round</span><span class="stat-val" id="simon-round">–</span></div>
        <div class="stat"><span class="stat-label">tier</span><span class="stat-val" id="simon-tier">×1</span></div>
      </div>
      <div class="simon-board">
        <button class="spad spad-up" data-color="green" aria-label="green pad — up arrow">▲</button>
        <button class="spad spad-left" data-color="red" aria-label="red pad — left arrow">◀</button>
        <div class="simon-hub" id="simon-hub">▶</div>
        <button class="spad spad-right" data-color="blue" aria-label="blue pad — right arrow">▶</button>
        <button class="spad spad-down" data-color="yellow" aria-label="yellow pad — down arrow">▼</button>
      </div>
      <div class="simon-msg" id="simon-msg">press start — watch the sequence, then echo it</div>
      <div class="simon-actions"><button class="btn next" id="simon-start">▶ start run</button></div>
    </div>
    <p><strong>Next stop: <a href="${GH}/blob/main/example/tutorial.md">the tutorial</a></strong>
    — the same run as a prompt-along read; typing along and rebuilding the game is optional
    reps, not a prerequisite. Reading ends at the README's
    <a href="${GH}/blob/main/README.md#on-a-work-repo">On a work repo</a> section (mirrored
    one card back) — after that, the next step is running the loop on a repo of your own.</p>
    <p>Go deeper whenever you want:</p>
    <ul>
      <li><a href="${GH}/blob/main/context/run-retrospective.md">The run retrospective</a> —
      every divergence in the real run and what it teaches.</li>
      <li><a href="${GH}/blob/main/context/example-run-log.md">The run log</a> — every prompt
      of all 18 sessions, session by session.</li>
      <li><a href="${GH}/blob/main/context/skills-review.md">The skills review</a> — the read
      of the skill texts that motivated the setup steps.</li>
    </ul>
    <p class="provenance">This walkthrough is adapted from
    <a href="${GH}/blob/main/README.md">README.md</a> and
    <a href="${GH}/blob/main/example/tutorial.md">example/tutorial.md</a> — those files are
    the source of truth. If they disagree with this page, they win.</p>
  `,
  nextLabel: "↺ replay walkthrough"
}

];
