//! The Simon Says state machine: Runs, Rounds, the Watch/Echo cycle, and
//! Sudden Death. All animation is deadline state checked against the
//! injected `now` each tick — no sleeps, no threads, no clock reads.

use std::time::{Duration, Instant};

use crate::rng::XorShift64;

/// The beat between starting (or restarting) a Run and its first Watch.
const GET_READY: Duration = Duration::from_millis(1000);
/// How long a Pad floods solid during Watch playback.
const WATCH_FLASH: Duration = Duration::from_millis(450);
/// The dark beat between two Watch flashes (also what separates a repeated
/// Pad from one long flash).
const WATCH_GAP: Duration = Duration::from_millis(120);
/// How long an Echo keypress flashes its Pad as confirmation.
const ECHO_FLASH: Duration = Duration::from_millis(250);
/// The pause between a completed Echo and the next Round's Watch.
const ROUND_BREAK: Duration = Duration::from_millis(800);
/// How long the player has to give each key during Echo — fixed across all
/// tiers, so hesitation is part of the challenge.
const ECHO_TIMEOUT: Duration = Duration::from_millis(3000);
/// The beat after a Mistake in which the board reveals the expected Pad.
const DEATH_FREEZE: Duration = Duration::from_millis(1000);

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
    /// The brief beat between starting or restarting a Run and its first
    /// Watch.
    GetReady,
    /// The Sequence plays back; player input is locked.
    Watch,
    /// The player replays the Sequence; each keypress is judged immediately.
    Echo,
    /// The fixed pause between a completed Echo and the next Round's Watch.
    RoundBreak,
    /// The beat after a Mistake: the board reveals the expected Pad and the
    /// Hub names what killed the Run.
    DeathFreeze,
    /// The end-of-Run overlay reporting the Run's result; Enter starts a
    /// fresh Run, Q/Esc quits.
    GameOver,
}

/// What ended the Run: a wrong Pad, or an expired per-key timeout, during
/// Echo. Sudden Death — the first Mistake is the last.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mistake {
    WrongPad,
    TooSlow,
}

/// The internal state machine: [`Phase`] plus each phase's deadline state.
/// Deadlines chain off the previous deadline, not off the observing tick's
/// `now`, so playback tempo never drifts with tick granularity.
enum State {
    Title,
    GetReady {
        until: Instant,
    },
    /// Playing back `sequence[step]`: flooded while `lit`, dark in the gap
    /// after; `until` is when the current stage ends.
    Watch {
        step: usize,
        lit: bool,
        until: Instant,
    },
    /// Waiting on `sequence[expected]`, which must arrive by `deadline`;
    /// `flash` is the confirmation flash of the most recent keypress, if
    /// still showing.
    Echo {
        expected: usize,
        flash: Option<Flash>,
        deadline: Instant,
    },
    /// Between Rounds; `flash` lets the Echo's final confirmation flash
    /// play out into the break.
    RoundBreak {
        flash: Option<Flash>,
        until: Instant,
    },
    /// The Run has ended on `mistake`; the board reveals the `reveal` Pad
    /// the player owed until the freeze lifts into Game Over.
    DeathFreeze {
        reveal: Pad,
        mistake: Mistake,
        until: Instant,
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
            State::GetReady { .. }
            | State::Watch { .. }
            | State::Echo { .. }
            | State::RoundBreak { .. }
            | State::DeathFreeze { .. } => return,
        }
        self.score = 0;
        self.round = 1;
        self.sequence.clear();
        let step = self.random_step();
        self.sequence.push(step);
        self.state = State::GetReady {
            until: now + GET_READY,
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
            // Sudden Death: the first Mistake ends the Run on the spot,
            // through the freeze that shows what was owed.
            self.state = State::DeathFreeze {
                reveal: self.sequence[expected],
                mistake: Mistake::WrongPad,
                until: now + DEATH_FREEZE,
            };
            return;
        }
        let flash = Some(Flash {
            pad,
            until: now + ECHO_FLASH,
        });
        if expected + 1 == self.sequence.len() {
            // The Echo is complete: rest through the Round Break, the final
            // confirmation flash playing out into it.
            self.state = State::RoundBreak {
                flash,
                until: now + ROUND_BREAK,
            };
        } else {
            self.state = State::Echo {
                expected: expected + 1,
                flash,
                // Each key gets its own fixed window.
                deadline: now + ECHO_TIMEOUT,
            };
        }
    }

    /// Advance time-driven state. `now` is the time seam: the shell calls
    /// this every tick with `Instant::now()`; tests script their own
    /// timeline. Loops so one late tick can cross several deadlines.
    pub fn tick(&mut self, now: Instant) {
        loop {
            match self.state {
                State::GetReady { until } if now >= until => {
                    self.state = State::Watch {
                        step: 0,
                        lit: true,
                        until: until + WATCH_FLASH,
                    };
                }
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
                            deadline: until + ECHO_TIMEOUT,
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
                    expected, deadline, ..
                } if now >= deadline => {
                    // The pending key never came: TOO SLOW, Sudden Death.
                    self.state = State::DeathFreeze {
                        reveal: self.sequence[expected],
                        mistake: Mistake::TooSlow,
                        until: deadline + DEATH_FREEZE,
                    };
                }
                State::Echo {
                    expected,
                    flash: Some(flash),
                    deadline,
                } if now >= flash.until => {
                    self.state = State::Echo {
                        expected,
                        flash: None,
                        deadline,
                    };
                }
                State::RoundBreak {
                    flash: Some(flash),
                    until,
                } if now >= flash.until => {
                    self.state = State::RoundBreak { flash: None, until };
                }
                State::RoundBreak { until, .. } if now >= until => {
                    self.begin_next_round(until);
                }
                State::DeathFreeze { until, .. } if now >= until => {
                    self.state = State::GameOver;
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
            }
            | State::RoundBreak {
                flash: Some(flash), ..
            } => Some(flash.pad),
            State::DeathFreeze { reveal, .. } => Some(reveal),
            _ => None,
        }
    }

    /// What killed the Run, readable during the Death Freeze.
    pub fn mistake(&self) -> Option<Mistake> {
        match self.state {
            State::DeathFreeze { mistake, .. } => Some(mistake),
            _ => None,
        }
    }

    pub fn phase(&self) -> Phase {
        match self.state {
            State::Title => Phase::Title,
            State::GetReady { .. } => Phase::GetReady,
            State::Watch { .. } => Phase::Watch,
            State::Echo { .. } => Phase::Echo,
            State::RoundBreak { .. } => Phase::RoundBreak,
            State::DeathFreeze { .. } => Phase::DeathFreeze,
            State::GameOver => Phase::GameOver,
        }
    }

    /// The Round Break has rested out: the Sequence grows by exactly one
    /// uniformly-random Step (repeats allowed) and the next Round's Watch
    /// begins.
    fn begin_next_round(&mut self, now: Instant) {
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

    /// Tick the game forward `ms` milliseconds in shell-sized 10 ms steps.
    fn advance_ms(game: &mut Game, now: &mut Instant, ms: u64) {
        let target = *now + Duration::from_millis(ms);
        while *now < target {
            *now += Duration::from_millis(10);
            game.tick(*now);
        }
    }

    /// Start a Run and cross the Get Ready beat into its first Watch.
    fn start_run(game: &mut Game, now: &mut Instant) {
        game.start(*now);
        advance_ms(game, now, 1100);
        assert_eq!(game.phase(), Phase::Watch);
    }

    #[test]
    fn get_ready_beat_precedes_the_first_watch() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        game.start(now);
        // Starting a Run lands in a ~1 s Get Ready beat, nothing flashing...
        assert_eq!(game.phase(), Phase::GetReady);
        assert_eq!(game.lit_pad(), None);
        advance_ms(&mut game, &mut now, 900);
        assert_eq!(game.phase(), Phase::GetReady);
        // ...and only then does the first Watch begin.
        advance_ms(&mut game, &mut now, 150);
        assert_eq!(game.phase(), Phase::Watch);
        assert_eq!(game.round(), 1);
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
        start_run(&mut game, &mut now);
        let started = now;
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
        start_run(&mut game, &mut now);
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
        start_run(&mut game, &mut now);
        let round_one = observe_watch(&mut game, &mut now);
        game.press(round_one[0], now);
        advance_ms(&mut game, &mut now, 900);
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
        start_run(&mut game, &mut now);
        let sequence = observe_watch(&mut game, &mut now);
        game.press(sequence[0], now);
        advance_ms(&mut game, &mut now, 900);
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

    #[test]
    fn round_break_separates_a_completed_echo_from_the_next_watch() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        start_run(&mut game, &mut now);
        let sequence = observe_watch(&mut game, &mut now);
        game.press(sequence[0], now);
        // The completed Echo lands in a fixed ~800 ms Round Break, still
        // showing the final press's confirmation flash...
        assert_eq!(game.phase(), Phase::RoundBreak);
        assert_eq!(game.lit_pad(), Some(sequence[0]));
        advance_ms(&mut game, &mut now, 300);
        assert_eq!(game.lit_pad(), None, "confirmation flash still expires");
        advance_ms(&mut game, &mut now, 400);
        assert_eq!(game.phase(), Phase::RoundBreak);
        // ...and then the next Round's Watch begins.
        advance_ms(&mut game, &mut now, 200);
        assert_eq!(game.phase(), Phase::Watch);
        assert_eq!(game.round(), 2);
    }

    #[test]
    fn hesitating_past_the_echo_timeout_is_too_slow_sudden_death() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        start_run(&mut game, &mut now);
        let sequence = observe_watch(&mut game, &mut now);
        // 2.9 s of hesitation is still alive...
        advance_ms(&mut game, &mut now, 2900);
        assert_eq!(game.phase(), Phase::Echo);
        // ...but at 3.0 s the pending key expires: the Run ends.
        advance_ms(&mut game, &mut now, 150);
        assert_eq!(game.phase(), Phase::DeathFreeze);
        assert_eq!(game.mistake(), Some(Mistake::TooSlow));
        // The Death Freeze reveals the Step the player never gave.
        assert_eq!(game.lit_pad(), Some(sequence[0]));
    }

    #[test]
    fn each_correct_press_restarts_the_echo_timeout() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        start_run(&mut game, &mut now);
        let sequence = observe_watch(&mut game, &mut now);
        game.press(sequence[0], now);
        advance_ms(&mut game, &mut now, 900);
        let sequence = observe_watch(&mut game, &mut now);
        // The timeout is per key, not per Echo: two waits of 2.5 s with a
        // correct press between them never trip it...
        advance_ms(&mut game, &mut now, 2500);
        assert_eq!(game.phase(), Phase::Echo);
        game.press(sequence[0], now);
        advance_ms(&mut game, &mut now, 2500);
        assert_eq!(game.phase(), Phase::Echo);
        // ...but the freshly-restarted window still expires on its own.
        advance_ms(&mut game, &mut now, 600);
        assert_eq!(game.phase(), Phase::DeathFreeze);
        assert_eq!(game.mistake(), Some(Mistake::TooSlow));
    }

    /// Any Pad that isn't the expected Step.
    fn wrong_pad(expected: Pad) -> Pad {
        [Pad::Up, Pad::Down, Pad::Left, Pad::Right]
            .into_iter()
            .find(|pad| *pad != expected)
            .unwrap()
    }

    #[test]
    fn wrong_pad_is_sudden_death_through_the_death_freeze() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        start_run(&mut game, &mut now);
        let sequence = observe_watch(&mut game, &mut now);
        game.press(wrong_pad(sequence[0]), now);
        // The Mistake lands in a ~1 s Death Freeze that reveals the Pad the
        // player owed and names what killed the Run...
        assert_eq!(game.phase(), Phase::DeathFreeze);
        assert_eq!(game.mistake(), Some(Mistake::WrongPad));
        assert_eq!(game.lit_pad(), Some(sequence[0]));
        // ...Enter cannot skip the freeze...
        game.start(now);
        advance_ms(&mut game, &mut now, 900);
        assert_eq!(game.phase(), Phase::DeathFreeze);
        // ...and only once it lifts does Game Over arrive.
        advance_ms(&mut game, &mut now, 200);
        assert_eq!(game.phase(), Phase::GameOver);
        assert_eq!(game.mistake(), None);
    }

    #[test]
    fn enter_on_game_over_starts_a_fresh_run() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        start_run(&mut game, &mut now);
        let sequence = observe_watch(&mut game, &mut now);
        game.press(wrong_pad(sequence[0]), now);
        advance_ms(&mut game, &mut now, 1100);
        assert_eq!(game.phase(), Phase::GameOver);
        // Enter goes straight into a fresh Run — never back via the Title —
        // and restarting gets the same Get Ready beat as starting.
        game.start(now);
        assert_eq!(game.phase(), Phase::GetReady);
        assert_eq!(game.round(), 1);
        advance_ms(&mut game, &mut now, 1100);
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
        // Enter during Get Ready must not restart the beat...
        advance_ms(&mut game, &mut now, 900);
        game.start(now);
        advance_ms(&mut game, &mut now, 200);
        assert_eq!(game.phase(), Phase::Watch);
        // ...Enter during Watch must not abort or restart the Run...
        game.start(now);
        let sequence = observe_watch(&mut game, &mut now);
        assert_eq!(sequence.len(), 1);
        // ...and neither must Enter during Echo or the Round Break.
        game.start(now);
        assert_eq!(game.phase(), Phase::Echo);
        game.press(sequence[0], now);
        assert_eq!(game.phase(), Phase::RoundBreak);
        game.start(now);
        assert_eq!(game.phase(), Phase::RoundBreak);
        advance_ms(&mut game, &mut now, 900);
        assert_eq!(game.round(), 2);
    }

    #[test]
    fn runs_are_endless_and_steps_are_drawn_uniformly_from_all_pads() {
        let mut game = Game::new(42);
        let mut now = Instant::now();
        start_run(&mut game, &mut now);
        let mut sequence = Vec::new();
        for round in 1..=30 {
            assert_eq!(game.round(), round);
            sequence = observe_watch(&mut game, &mut now);
            assert_eq!(sequence.len(), round as usize);
            for &pad in &sequence {
                game.press(pad, now);
            }
            advance_ms(&mut game, &mut now, 900);
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
    fn new_game_opens_at_title_with_zeroed_stats() {
        let game = Game::new(42);
        assert_eq!(game.phase(), Phase::Title);
        assert_eq!(game.score(), 0);
        assert_eq!(game.high_score(), 0);
        assert_eq!(game.round(), 0);
        assert_eq!(game.speed_tier(), 1);
    }
}
