//! The Simon Says state machine. This walking-skeleton slice covers only the
//! Title idle state; Runs, Rounds, Watch/Echo, and scoring land in later
//! slices.

use std::time::Instant;

use crate::rng::XorShift64;

/// One of the four colored quadrants on the board, each bound to an arrow
/// key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pad {
    Up,
    Down,
    Left,
    Right,
}

/// The screen-flow state the board currently reads out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// The launch-only idle state, shown as an overlay on the board until
    /// the player starts a Run; never returned to afterward.
    Title,
}

/// One complete game's worth of state — the single object the shell drives.
///
/// At the Title there is no Run yet, so every stat reads as its resting
/// placeholder: zero Score, Round 0, Speed Tier ×1.
pub struct Game {
    #[expect(
        dead_code,
        reason = "walking skeleton: first drawn on by the sequence-growth slice"
    )]
    rng: XorShift64,
    phase: Phase,
    score: u32,
    high_score: u32,
    round: u32,
}

impl Game {
    /// `seed` is the randomness seam: the only entropy the core ever sees.
    /// The shell seeds from `SystemTime`; tests pass a fixed value.
    pub fn new(seed: u64) -> Self {
        Self {
            rng: XorShift64::new(seed),
            phase: Phase::Title,
            score: 0,
            high_score: 0,
            round: 0,
        }
    }

    /// Advance time-driven state. `now` is the time seam: the shell calls
    /// this every tick with `Instant::now()`; tests script their own
    /// timeline. The Title has no time-driven behavior — deadlines arrive
    /// with the Watch/Echo slices.
    pub fn tick(&mut self, _now: Instant) {}

    pub fn phase(&self) -> Phase {
        self.phase
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn high_score(&self) -> u32 {
        self.high_score
    }

    /// The current Round, or 0 while no Run is underway.
    pub fn round(&self) -> u32 {
        self.round
    }

    /// The Speed Tier's score multiplier (×1–×4).
    pub fn speed_tier(&self) -> u32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;

    #[test]
    fn title_never_advances_on_its_own() {
        // No attract mode: the Title idles until the player acts, however
        // long the shell keeps ticking.
        let mut game = Game::new(42);
        let start = Instant::now();
        for tick in 0..120 {
            game.tick(start + Duration::from_millis(33 * tick));
        }
        assert_eq!(game.phase(), Phase::Title);
        assert_eq!(game.score(), 0);
        assert_eq!(game.round(), 0);
    }

    #[test]
    fn new_game_opens_at_title_with_zeroed_stats() {
        let game = Game::new(42);
        assert_eq!(game.phase(), Phase::Title);
        assert_eq!(game.score(), 0);
        assert_eq!(game.high_score(), 0);
        assert_eq!(game.round(), 0);
        assert_eq!(game.speed_tier(), 1);
    }
}
