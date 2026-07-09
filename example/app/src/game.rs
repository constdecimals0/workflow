//! The Simon Says state machine: Runs, Rounds, the Watch/Echo cycle, and
//! Sudden Death. All animation is deadline state checked against the
//! injected `now` each tick — no sleeps, no threads, no clock reads.

use std::time::{Duration, Instant};

use crate::rng::XorShift64;

/// How long a Pad floods solid during Watch playback.
const WATCH_FLASH: Duration = Duration::from_millis(450);
/// The dark beat between two Watch flashes (also what separates a repeated
/// Pad from one long flash).
const WATCH_GAP: Duration = Duration::from_millis(120);
/// How long an Echo keypress flashes its Pad as confirmation.
const ECHO_FLASH: Duration = Duration::from_millis(250);

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
    /// The Sequence plays back; player input is locked.
    Watch,
    /// The player replays the Sequence; each keypress is judged immediately.
    Echo,
    /// The end-of-Run overlay reporting the Run's result; Enter starts a
    /// fresh Run, Q/Esc quits.
    GameOver,
}

/// The internal state machine: [`Phase`] plus each phase's deadline state.
/// Deadlines chain off the previous deadline, not off the observing tick's
/// `now`, so playback tempo never drifts with tick granularity.
enum State {
    Title,
    /// Playing back `sequence[step]`: flooded while `lit`, dark in the gap
    /// after; `until` is when the current stage ends.
    Watch {
        step: usize,
        lit: bool,
        until: Instant,
    },
    /// Waiting on `sequence[expected]`; `flash` is the confirmation flash
    /// of the most recent keypress, if still showing.
    Echo {
        expected: usize,
        flash: Option<Flash>,
    },
    GameOver,
}

/// A Pad flooding solid as confirmation of an Echo keypress.
#[derive(Clone, Copy)]
struct Flash {
    pad: Pad,
    until: Instant,
}

/// One complete game's worth of state — the single object the shell drives.
///
/// At the Title there is no Run yet, so every stat reads as its resting
/// placeholder: zero Score, Round 0, Speed Tier ×1.
pub struct Game {
    rng: XorShift64,
    state: State,
    sequence: Vec<Pad>,
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
            state: State::Title,
            sequence: Vec::new(),
            score: 0,
            high_score: 0,
            round: 0,
        }
    }

    /// Start a Run: Enter on the Title or Game Over overlay. Mid-Run,
    /// Enter is ignored so it can never abort a live Run. The Title is
    /// never returned to afterward.
    pub fn start(&mut self, now: Instant) {
        match self.state {
            State::Title | State::GameOver => {}
            State::Watch { .. } | State::Echo { .. } => return,
        }
        self.score = 0;
        self.round = 1;
        self.sequence.clear();
        let step = self.random_step();
        self.sequence.push(step);
        self.state = State::Watch {
            step: 0,
            lit: true,
            until: now + WATCH_FLASH,
        };
    }

    /// Echo input from an arrow key, judged immediately against the
    /// expected Step. Outside Echo this is a no-op: input is locked during
    /// Watch, so a stray keypress can never cost the Run.
    pub fn press(&mut self, pad: Pad, now: Instant) {
        let State::Echo { expected, .. } = self.state else {
            return;
        };
        if self.sequence[expected] != pad {
            // Sudden Death: the first Mistake ends the Run on the spot.
            self.state = State::GameOver;
            return;
        }
        if expected + 1 == self.sequence.len() {
            self.advance_round(now);
        } else {
            self.state = State::Echo {
                expected: expected + 1,
                flash: Some(Flash {
                    pad,
                    until: now + ECHO_FLASH,
                }),
            };
        }
    }

    /// Advance time-driven state. `now` is the time seam: the shell calls
    /// this every tick with `Instant::now()`; tests script their own
    /// timeline. Loops so one late tick can cross several deadlines.
    pub fn tick(&mut self, now: Instant) {
        loop {
            match self.state {
                State::Watch {
                    step,
                    lit: true,
                    until,
                } if now >= until => {
                    if step + 1 == self.sequence.len() {
                        // The Watch→Echo flip is instant: no trailing gap.
                        self.state = State::Echo {
                            expected: 0,
                            flash: None,
                        };
                    } else {
                        self.state = State::Watch {
                            step,
                            lit: false,
                            until: until + WATCH_GAP,
                        };
                    }
                }
                State::Watch {
                    step,
                    lit: false,
                    until,
                } if now >= until => {
                    self.state = State::Watch {
                        step: step + 1,
                        lit: true,
                        until: until + WATCH_FLASH,
                    };
                }
                State::Echo {
                    expected,
                    flash: Some(flash),
                } if now >= flash.until => {
                    self.state = State::Echo {
                        expected,
                        flash: None,
                    };
                }
                _ => break,
            }
        }
    }

    /// The Pad currently flooding solid with its bright color, if any. Pads
    /// flash only for game reasons: Watch playback here, Echo presses in a
    /// later slice.
    pub fn lit_pad(&self) -> Option<Pad> {
        match self.state {
            State::Watch {
                step, lit: true, ..
            } => Some(self.sequence[step]),
            State::Echo {
                flash: Some(flash), ..
            } => Some(flash.pad),
            _ => None,
        }
    }

    pub fn phase(&self) -> Phase {
        match self.state {
            State::Title => Phase::Title,
            State::Watch { .. } => Phase::Watch,
            State::Echo { .. } => Phase::Echo,
            State::GameOver => Phase::GameOver,
        }
    }

    /// A completed Echo rolls the Run into the next Round: the Sequence
    /// grows by exactly one uniformly-random Step (repeats allowed) and its
    /// Watch begins.
    fn advance_round(&mut self, now: Instant) {
        self.round += 1;
        let step = self.random_step();
        self.sequence.push(step);
        self.state = State::Watch {
            step: 0,
            lit: true,
            until: now + WATCH_FLASH,
        };
    }

    /// One uniformly-random Step. `next_u64() & 3` is exactly uniform over
    /// the four Pads because 2^64 divides evenly by 4.
    fn random_step(&mut self) -> Pad {
        match self.rng.next_u64() & 3 {
            0 => Pad::Up,
            1 => Pad::Down,
            2 => Pad::Left,
            _ => Pad::Right,
        }
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

    /// Drive the game through Watch the way the shell would — ticking at a
    /// fine granularity and watching `lit_pad()` — collecting each flash.
    /// Returns once the phase has left Watch. This is how a player learns
    /// the Sequence, so tests never reach into internals for it.
    fn observe_watch(game: &mut Game, now: &mut Instant) -> Vec<Pad> {
        assert_eq!(game.phase(), Phase::Watch);
        let mut flashes = Vec::new();
        let mut was_lit = false;
        // Cap the walk so a stuck Watch fails the test instead of hanging it.
        for _ in 0..100_000 {
            if game.phase() != Phase::Watch {
                return flashes;
            }
            match (was_lit, game.lit_pad()) {
                (false, Some(pad)) => {
                    flashes.push(pad);
                    was_lit = true;
                }
                (true, None) => was_lit = false,
                _ => {}
            }
            *now += Duration::from_millis(10);
            game.tick(*now);
        }
        panic!("Watch never ended");
    }

    #[test]
    fn watch_flashes_the_sequence_then_flips_instantly_to_echo() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        let started = now;
        game.start(now);
        let flashes = observe_watch(&mut game, &mut now);
        // Round 1 plays the length-1 Sequence...
        assert_eq!(flashes.len(), 1);
        // ...and the Watch→Echo flip is instant: one 450 ms flash plus at
        // most one 10 ms observation tick, with no trailing gap or dwell.
        assert_eq!(game.phase(), Phase::Echo);
        assert!(now - started <= Duration::from_millis(470));
    }

    #[test]
    fn input_is_locked_during_watch() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        game.start(now);
        // A stray press of every Pad during Watch must never cost the Run.
        for pad in [Pad::Up, Pad::Down, Pad::Left, Pad::Right] {
            game.press(pad, now);
        }
        assert_eq!(game.phase(), Phase::Watch);
        let flashes = observe_watch(&mut game, &mut now);
        assert_eq!(flashes.len(), 1);
        assert_eq!(game.phase(), Phase::Echo);
    }

    #[test]
    fn completing_echo_grows_the_sequence_by_one_into_the_next_watch() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        game.start(now);
        let round_one = observe_watch(&mut game, &mut now);
        game.press(round_one[0], now);
        assert_eq!(game.phase(), Phase::Watch);
        assert_eq!(game.round(), 2);
        let round_two = observe_watch(&mut game, &mut now);
        assert_eq!(round_two.len(), 2);
        assert_eq!(
            round_two[0], round_one[0],
            "append-one growth preserves the existing prefix"
        );
    }

    #[test]
    fn echo_keypress_flashes_its_pad_as_confirmation() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        game.start(now);
        let sequence = observe_watch(&mut game, &mut now);
        game.press(sequence[0], now);
        // Round 2's Echo has a second Step pending, so the confirmation
        // flash of the first press is observable.
        let sequence = observe_watch(&mut game, &mut now);
        game.press(sequence[0], now);
        assert_eq!(game.phase(), Phase::Echo);
        assert_eq!(game.lit_pad(), Some(sequence[0]));
        // The confirmation flash is 250 ms; after it the Pad goes dark and
        // the Echo is still waiting on the next Step.
        now += Duration::from_millis(260);
        game.tick(now);
        assert_eq!(game.lit_pad(), None);
        assert_eq!(game.phase(), Phase::Echo);
    }

    /// Any Pad that isn't the expected Step.
    fn wrong_pad(expected: Pad) -> Pad {
        [Pad::Up, Pad::Down, Pad::Left, Pad::Right]
            .into_iter()
            .find(|pad| *pad != expected)
            .unwrap()
    }

    #[test]
    fn wrong_pad_is_sudden_death_into_game_over() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        game.start(now);
        let sequence = observe_watch(&mut game, &mut now);
        game.press(wrong_pad(sequence[0]), now);
        assert_eq!(game.phase(), Phase::GameOver);
    }

    #[test]
    fn enter_on_game_over_starts_a_fresh_run() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        game.start(now);
        let sequence = observe_watch(&mut game, &mut now);
        game.press(wrong_pad(sequence[0]), now);
        assert_eq!(game.phase(), Phase::GameOver);
        // Enter goes straight into a fresh Run — never back via the Title.
        game.start(now);
        assert_eq!(game.phase(), Phase::Watch);
        assert_eq!(game.round(), 1);
        let fresh = observe_watch(&mut game, &mut now);
        assert_eq!(
            fresh.len(),
            1,
            "the fresh Run begins at a length-1 Sequence"
        );
    }

    #[test]
    fn enter_mid_run_is_ignored() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        game.start(now);
        // Enter during Watch must not abort or restart the Run...
        game.start(now);
        let sequence = observe_watch(&mut game, &mut now);
        assert_eq!(sequence.len(), 1);
        // ...and neither must Enter during Echo.
        game.start(now);
        assert_eq!(game.phase(), Phase::Echo);
        game.press(sequence[0], now);
        assert_eq!(game.round(), 2);
    }

    #[test]
    fn runs_are_endless_and_steps_are_drawn_uniformly_from_all_pads() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        game.start(now);
        let mut sequence = Vec::new();
        for round in 1..=30 {
            assert_eq!(game.round(), round);
            sequence = observe_watch(&mut game, &mut now);
            assert_eq!(sequence.len(), round as usize);
            for &pad in &sequence {
                game.press(pad, now);
            }
        }
        // Thirty Rounds in and the Run is still going: no win cap.
        assert_eq!(game.phase(), Phase::Watch);
        assert_eq!(game.round(), 31);
        // A uniform draw over thirty Steps reaches every Pad...
        for pad in [Pad::Up, Pad::Down, Pad::Left, Pad::Right] {
            assert!(sequence.contains(&pad), "{pad:?} never drawn");
        }
        // ...and repeats are allowed: this seed produces back-to-back
        // duplicates, which a no-repeat generator would never emit.
        assert!(sequence.windows(2).any(|pair| pair[0] == pair[1]));
    }

    #[test]
    fn enter_starts_a_run_into_watch_at_round_one() {
        let mut game = Game::new(42);
        game.start(Instant::now());
        assert_eq!(game.phase(), Phase::Watch);
        assert_eq!(game.round(), 1);
        assert_eq!(game.score(), 0);
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
