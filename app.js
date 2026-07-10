// Stepper for the walkthrough. STEPS comes from content.js.

const card = document.getElementById("card");
const kickerEl = document.getElementById("kicker");
const titleEl = document.getElementById("title");
const bodyEl = document.getElementById("body");
const hubEl = document.getElementById("hub");
const dotsEl = document.getElementById("dots");
const countEl = document.getElementById("step-count");
const backBtn = document.getElementById("back");
const nextBtn = document.getElementById("next");
const overlay = document.getElementById("clear-overlay");
const clearSub = document.getElementById("clear-sub");
const pads = {
  green: document.querySelector(".pad-up"),
  red: document.querySelector(".pad-right"),
  yellow: document.querySelector(".pad-left"),
  blue: document.querySelector(".pad-down"),
};

const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
const STORE_KEY = "workflow-walkthrough";

let state = { i: 0, seen: 0 };
let transitioning = false;

// ── Persistence ───────────────────────────────────────────────────────────────

function loadState() {
  try {
    const saved = JSON.parse(localStorage.getItem(STORE_KEY) || "{}");
    if (Number.isInteger(saved.seen)) state.seen = Math.min(saved.seen, STEPS.length - 1);
    if (Number.isInteger(saved.i)) state.i = Math.min(saved.i, state.seen);
  } catch (_) { /* fresh start */ }
  // A hash is a deliberate deep link — honor it and unlock the steps up to it.
  const hashIndex = STEPS.findIndex((s) => "#" + s.id === location.hash);
  if (hashIndex >= 0) {
    state.i = hashIndex;
    state.seen = Math.max(state.seen, hashIndex);
  }
}

function saveState() {
  try { localStorage.setItem(STORE_KEY, JSON.stringify(state)); } catch (_) { /* private mode */ }
}

// ── Board ─────────────────────────────────────────────────────────────────────

function flashPad(color, ms = 180) {
  const pad = pads[color];
  if (!pad || reducedMotion) return;
  pad.classList.add("lit");
  setTimeout(() => pad.classList.remove("lit"), ms);
}

function flashSequence(colors, gap = 140) {
  colors.forEach((c, n) => setTimeout(() => flashPad(c), n * gap));
}

// ── Render ────────────────────────────────────────────────────────────────────

function render() {
  const step = STEPS[state.i];
  stopSimon();

  card.className = "card c-" + step.color;
  if (!reducedMotion) {
    void card.offsetWidth; // restart the enter animation
    card.classList.add("enter");
  }

  kickerEl.textContent = step.kicker;
  titleEl.textContent = step.title;
  bodyEl.innerHTML = step.body;
  bodyEl.querySelectorAll("table").forEach((t) => {
    const wrap = document.createElement("div");
    wrap.className = "table-wrap";
    t.parentNode.insertBefore(wrap, t);
    wrap.appendChild(t);
  });

  hubEl.textContent = state.i + 1;
  countEl.textContent = "step " + (state.i + 1) + " / " + STEPS.length;

  dotsEl.innerHTML = "";
  STEPS.forEach((s, n) => {
    const dot = document.createElement("button");
    dot.className = "dot" + (n <= state.seen ? " seen c-" + s.color : "") + (n === state.i ? " current" : "");
    dot.title = s.title;
    dot.setAttribute("aria-label", "Step " + (n + 1) + ": " + s.title);
    if (n <= state.seen) dot.addEventListener("click", () => go(n));
    dotsEl.appendChild(dot);
  });

  backBtn.disabled = state.i === 0;
  nextBtn.textContent = step.nextLabel || "continue ▶";

  if (step.id === "game-over") initSimon();

  history.replaceState(null, "", "#" + step.id);
  window.scrollTo({ top: 0 });
}

function go(n) {
  if (n < 0 || n >= STEPS.length) return;
  state.i = n;
  state.seen = Math.max(state.seen, n);
  saveState();
  render();
}

// ── /clear interstitial ───────────────────────────────────────────────────────

function playClear(label, done) {
  if (reducedMotion) { done(); return; }
  transitioning = true;
  clearSub.textContent = label || "fresh context";
  overlay.classList.add("show");
  flashSequence(["green", "red", "blue", "yellow", "green", "red", "blue", "yellow"], 90);
  const finish = () => {
    overlay.classList.remove("show");
    overlay.removeEventListener("click", finish);
    clearTimeout(timer);
    transitioning = false;
    done();
  };
  const timer = setTimeout(finish, 950);
  overlay.addEventListener("click", finish);
}

// ── Navigation ────────────────────────────────────────────────────────────────

function next() {
  if (transitioning) return;
  const step = STEPS[state.i];
  if (state.i === STEPS.length - 1) { go(0); return; } // play again
  if (step.clearAfter) {
    playClear(step.clearLabel, () => go(state.i + 1));
  } else {
    go(state.i + 1);
  }
}

function back() {
  if (transitioning) return;
  go(state.i - 1);
}

nextBtn.addEventListener("click", () => { flashPad("red"); next(); });
backBtn.addEventListener("click", () => { flashPad("yellow"); back(); });

// Arrow keys mirror the game's pad bindings: up/right/down/left = green/red/blue/yellow.
const ARROW_PADS = { ArrowUp: "green", ArrowRight: "red", ArrowDown: "blue", ArrowLeft: "yellow" };

document.addEventListener("keydown", (e) => {
  if (e.metaKey || e.ctrlKey || e.altKey) return;
  const onControl = e.target.closest && e.target.closest("button, a, input, textarea");

  // Mid-run, the arrows belong to the game — no navigation.
  if (simonActive()) {
    if (ARROW_PADS[e.key]) {
      e.preventDefault();
      flashPad(ARROW_PADS[e.key]);
      simonInput(ARROW_PADS[e.key]);
    } else if ((e.key === "Enter" || e.key === " ") && !onControl) {
      e.preventDefault();
    }
    return;
  }

  if (ARROW_PADS[e.key]) flashPad(ARROW_PADS[e.key]);
  if (e.key === "ArrowRight") {
    next();
  } else if (e.key === "ArrowLeft") {
    back();
  } else if ((e.key === "Enter" || e.key === " ") && !onControl) {
    e.preventDefault();
    flashPad("red");
    next();
  }
});

// ── Copy buttons ──────────────────────────────────────────────────────────────

bodyEl.addEventListener("click", (e) => {
  const btn = e.target.closest(".copy");
  if (!btn) return;
  const code = btn.closest(".prompt").querySelector("code");
  navigator.clipboard.writeText(code.textContent).then(() => {
    btn.textContent = "copied!";
    btn.classList.add("copied");
    setTimeout(() => {
      btn.textContent = "copy";
      btn.classList.remove("copied");
    }, 1200);
  });
});

// ── Simon mini-game (last step) ───────────────────────────────────────────────
// The rules mirror the example app: Watch then Echo, sudden death, a 3.0 s
// per-key timeout, speed tiers every 4 rounds doubling as the score multiplier.
// High score persists as a localStorage kv pair.

const SIMON_KEY = "workflow-simon-high-score";
const PAD_ORDER = ["green", "red", "yellow", "blue"];
const PAD_KEYS = { green: "↑", red: "→", yellow: "←", blue: "↓" };
let simon = null;

function simonActive() {
  return !!simon && ["getready", "watch", "echo", "break", "freeze"].includes(simon.phase);
}

function stopSimon() {
  if (!simon) return;
  simon.timers.forEach(clearTimeout);
  simon = null;
}

function simonLater(fn, ms) {
  simon.timers.push(setTimeout(fn, ms));
}

function initSimon() {
  const root = document.getElementById("simon");
  if (!root) return;
  simon = {
    phase: "idle", seq: [], pos: 0, score: 0, round: 0, tier: 1,
    timers: [], keyTimer: null,
    high: parseInt(localStorage.getItem(SIMON_KEY), 10) || 0,
    els: {
      pads: {},
      hub: document.getElementById("simon-hub"),
      score: document.getElementById("simon-score"),
      high: document.getElementById("simon-high"),
      round: document.getElementById("simon-round"),
      tier: document.getElementById("simon-tier"),
      msg: document.getElementById("simon-msg"),
      start: document.getElementById("simon-start"),
    },
  };
  root.querySelectorAll(".spad").forEach((pad) => {
    simon.els.pads[pad.dataset.color] = pad;
    pad.addEventListener("click", () => simonInput(pad.dataset.color));
  });
  simon.els.high.textContent = simon.high;
  simon.els.start.addEventListener("click", simonStart);
}

function simonStats() {
  const s = simon;
  s.els.score.textContent = s.score;
  s.els.high.textContent = s.high;
  s.els.round.textContent = s.round || "–";
  s.els.tier.textContent = "×" + s.tier;
}

function simonFlash(color, ms) {
  const pad = simon.els.pads[color];
  pad.classList.add("lit");
  simonLater(() => pad.classList.remove("lit"), ms);
}

function simonTempo() {
  return [0, 600, 470, 350, 250][simon.tier];
}

function simonArmKeyTimeout() {
  clearTimeout(simon.keyTimer);
  simon.keyTimer = setTimeout(() => simonMistake(true), 3000);
  simon.timers.push(simon.keyTimer);
}

function simonStart() {
  const s = simon;
  s.timers.forEach(clearTimeout);
  s.timers = [];
  s.seq = []; s.pos = 0; s.score = 0; s.round = 0; s.tier = 1;
  s.phase = "getready";
  s.els.start.style.visibility = "hidden";
  s.els.hub.textContent = "★";
  s.els.msg.textContent = "get ready…";
  simonStats();
  simonLater(simonNextRound, 900);
}

function simonNextRound() {
  const s = simon;
  s.round++;
  const tier = Math.min(4, 1 + Math.floor((s.round - 1) / 4));
  const tierUp = tier > s.tier;
  s.tier = tier;
  s.seq.push(PAD_ORDER[Math.floor(Math.random() * PAD_ORDER.length)]);
  s.pos = 0;
  s.phase = "watch";
  s.els.hub.textContent = s.round;
  s.els.msg.textContent = tierUp ? "SPEED UP! — watch…" : "watch…";
  simonStats();
  const gap = simonTempo();
  s.seq.forEach((color, n) => simonLater(() => simonFlash(color, gap * 0.6), 500 + n * gap));
  simonLater(() => {
    s.phase = "echo";
    s.els.msg.textContent = "echo!";
    simonArmKeyTimeout();
  }, 500 + s.seq.length * gap);
}

function simonInput(color) {
  const s = simon;
  if (!s || s.phase !== "echo") return;
  simonFlash(color, 170);
  if (color !== s.seq[s.pos]) { simonMistake(false); return; }
  s.pos++;
  s.score += 10 * s.tier;
  simonStats();
  if (s.pos === s.seq.length) {
    s.phase = "break";
    clearTimeout(s.keyTimer);
    s.els.msg.textContent = "round " + s.round + " cleared";
    simonLater(simonNextRound, 900);
  } else {
    simonArmKeyTimeout();
  }
}

function simonMistake(timedOut) {
  const s = simon;
  const expected = s.seq[s.pos];
  s.phase = "freeze";
  clearTimeout(s.keyTimer);
  s.els.msg.textContent = (timedOut ? "too slow — " : "wrong pad — ") +
    "it wanted " + expected + " (" + PAD_KEYS[expected] + ")";
  simonFlash(expected, 900);
  simonLater(simonGameOver, 1000);
}

function simonGameOver() {
  const s = simon;
  s.phase = "over";
  s.els.hub.textContent = "✖";
  if (s.score > s.high) {
    s.high = s.score;
    localStorage.setItem(SIMON_KEY, String(s.score));
    s.els.msg.textContent = "game over — NEW HIGH SCORE: " + s.score;
  } else {
    s.els.msg.textContent = "game over — score " + s.score;
  }
  simonStats();
  s.els.start.textContent = "↺ new run";
  s.els.start.style.visibility = "visible";
}

// ── Boot ──────────────────────────────────────────────────────────────────────

loadState();
render();
flashSequence(["green", "red", "blue", "yellow"], 220); // attract beat, like the game's Watch
