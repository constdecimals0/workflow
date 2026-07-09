//! The Simon Says game core: the state machine, scoring, and (in later
//! slices) High Score persistence. Std-only — no terminal or UI types ever
//! appear here (ADR 0002); the bin shell owns the tick loop and rendering.
//!
//! Two seams keep the core deterministic under test: time never originates
//! inside this crate — every time-sensitive API takes `now: Instant` — and
//! randomness comes only from the seed passed to [`Game::new`].

mod game;
mod rng;

pub use game::{Game, Mistake, Pad, Phase};
