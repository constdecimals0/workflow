//! The Simon Says state machine: Runs, Rounds, the Watch/Echo cycle, and
//! Sudden Death. All animation is deadline state checked against the
//! injected `now` each tick — no sleeps, no threads, no clock reads.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use crate::highscore;
use crate::rng::XorShift64;

/// The beat between starting (or restarting) a Run and its first Watch.
const GET_READY: Duration = Duration::from_millis(1000);
/// Watch tempo (flash ms, gap ms) per Speed Tier, indexed by multiplier − 1.
/// The flash is how long a Pad floods solid during playback; the gap is the
/// dark beat between flashes (what separates a repeated Pad from one long
/// flash).
const TIER_TEMPO_MS: [(u64, u64); 4] = [(450, 120), (330, 100), (240, 80), (180, 60)];
/// How long an Echo keypress flashes its Pad as confirmation.
const ECHO_FLASH: Duration = Duration::from_millis(250);
/// The pause between a completed Echo and the next Round's Watch.
const ROUND_BREAK: Duration = Duration::from_millis(800);
/// The stretched Round Break on tier-up Rounds, carrying the SPEED UP!
/// callout.
const TIER_UP_BREAK: Duration = Duration::from_millis(1500);
/// Base points per correct Step, multiplied by the Speed Tier.
const STEP_POINTS: u32 = 10;

/// The Speed Tier (also the score multiplier, ×1–×4) in effect at `round`.
/// Tiers enter at Rounds 1 / 5 / 9 / 13; the top tier plateaus.
fn tier_for_round(round: u32) -> u32 {
    match round {
        0..=4 => 1,
        5..=8 => 2,
        9..=12 => 3,
        _ => 4,
    }
}
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
    new_high_score: bool,
    round: u32,
    data_dir: Option<PathBuf>,
}

impl Game {
    /// `seed` is the randomness seam: the only entropy the core ever sees.
    /// The shell seeds from `SystemTime`; tests pass a fixed value.
    ///
    /// `data_dir` is the persistence seam — the XDG data home the High
    /// Score lives under. The shell passes [`highscore::data_dir`]; tests
    /// pass a temp dir, or `None` to run without a scoreboard. The stored
    /// High Score loads here, at launch, and a missing or unreadable file
    /// silently loads as 0.
    pub fn new(seed: u64, data_dir: Option<PathBuf>) -> Self {
        let high_score = data_dir.as_deref().map(highscore::load).unwrap_or(0);
        Self {
            rng: XorShift64::new(seed),
            state: State::Title,
            sequence: Vec::new(),
            score: 0,
            high_score,
            new_high_score: false,
            round: 0,
            data_dir,
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
        self.new_high_score = false;
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
        // Judged correct: the Step scores immediately, at the current
        // tier's multiplier.
        self.score += STEP_POINTS * tier_for_round(self.round);
        let flash = Some(Flash {
            pad,
            until: now + ECHO_FLASH,
        });
        if expected + 1 == self.sequence.len() {
            // The Echo is complete: rest through the Round Break, the final
            // confirmation flash playing out into it. Tier-up breaks
            // stretch so the SPEED UP! callout can land.
            let stretch = tier_for_round(self.round + 1) > tier_for_round(self.round);
            self.state = State::RoundBreak {
                flash,
                until: now + if stretch { TIER_UP_BREAK } else { ROUND_BREAK },
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
                        until: until + self.watch_flash(),
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
                            until: until + self.watch_gap(),
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
                        until: until + self.watch_flash(),
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
                    self.finish_run();
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

    /// True at Game Over when this Run's final Score beat the stored High
    /// Score — the ★ NEW HIGH SCORE! ★ moment.
    pub fn new_high_score(&self) -> bool {
        self.new_high_score
    }

    /// The "SPEED UP! ×n" callout: during a tier-up Round Break, the
    /// multiplier of the incoming tier.
    pub fn speed_up(&self) -> Option<u32> {
        match self.state {
            State::RoundBreak { .. } => {
                let next = tier_for_round(self.round + 1);
                (next > tier_for_round(self.round)).then_some(next)
            }
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
            until: now + self.watch_flash(),
        };
    }

    /// The Death Freeze has lifted: land on Game Over, and — only when this
    /// Run's final Score beats the stored High Score — write the record,
    /// once. A Run abandoned mid-flight (the shell quitting) never gets
    /// here, so it can never pollute the record.
    fn finish_run(&mut self) {
        self.new_high_score = self.score > self.high_score;
        if self.new_high_score {
            self.high_score = self.score;
            if let Some(dir) = &self.data_dir {
                highscore::save(dir, self.high_score);
            }
        }
        self.state = State::GameOver;
    }

    fn watch_flash(&self) -> Duration {
        let (flash_ms, _) = TIER_TEMPO_MS[(tier_for_round(self.round) - 1) as usize];
        Duration::from_millis(flash_ms)
    }

    fn watch_gap(&self) -> Duration {
        let (_, gap_ms) = TIER_TEMPO_MS[(tier_for_round(self.round) - 1) as usize];
        Duration::from_millis(gap_ms)
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

    /// The Speed Tier's score multiplier (×1–×4), following the current
    /// Round; ×1 while no Run is underway.
    pub fn speed_tier(&self) -> u32 {
        tier_for_round(self.round)
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
        let mut game = Game::new(42, None);
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
        let mut game = Game::new(42, None);
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
        let mut game = Game::new(42, None);
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
        let mut game = Game::new(42, None);
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
        let mut game = Game::new(42, None);
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
        let mut game = Game::new(42, None);
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
        let mut game = Game::new(42, None);
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
        let mut game = Game::new(42, None);
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
        let mut game = Game::new(42, None);
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

    /// Tick until the next Watch begins (through any Round Break, however
    /// long), landing at most one 10 ms tick into its first flash.
    fn advance_to_watch(game: &mut Game, now: &mut Instant) {
        for _ in 0..10_000 {
            if game.phase() == Phase::Watch {
                return;
            }
            *now += Duration::from_millis(10);
            game.tick(*now);
        }
        panic!("Watch never arrived");
    }

    /// Play the current Round correctly: watch, echo, and rest into the
    /// next Round's Watch.
    fn play_round(game: &mut Game, now: &mut Instant) {
        let sequence = observe_watch(game, now);
        for &pad in &sequence {
            game.press(pad, *now);
        }
        advance_to_watch(game, now);
    }

    #[test]
    fn speed_tiers_enter_at_rounds_1_5_9_13_and_the_top_tier_plateaus() {
        let mut game = Game::new(42, None);
        let mut now = Instant::now();
        start_run(&mut game, &mut now);
        for round in 1..=17 {
            let expected_tier = match round {
                1..=4 => 1,
                5..=8 => 2,
                9..=12 => 3,
                // No fifth tier: 13 and beyond stay at ×4.
                _ => 4,
            };
            assert_eq!(game.round(), round);
            assert_eq!(game.speed_tier(), expected_tier, "round {round}");
            play_round(&mut game, &mut now);
        }
    }

    /// Measure the first Step's flash and gap at 1 ms ticks, returning
    /// (flashed pad, flash ms, gap ms). Needs a Sequence of length ≥ 2 so a
    /// gap follows the flash.
    fn measure_watch_tempo(game: &mut Game, now: &mut Instant) -> (Pad, u64, u64) {
        assert_eq!(game.phase(), Phase::Watch);
        let pad = game.lit_pad().expect("Watch opens with its first flash");
        let mut flash_ms = 0;
        while game.lit_pad().is_some() {
            *now += Duration::from_millis(1);
            game.tick(*now);
            flash_ms += 1;
        }
        let mut gap_ms = 0;
        while game.lit_pad().is_none() {
            assert_eq!(game.phase(), Phase::Watch, "gap must end in another flash");
            *now += Duration::from_millis(1);
            game.tick(*now);
            gap_ms += 1;
        }
        (pad, flash_ms, gap_ms)
    }

    #[test]
    fn watch_tempo_follows_the_tier_and_echo_timeout_does_not() {
        let mut game = Game::new(42, None);
        let mut now = Instant::now();
        start_run(&mut game, &mut now);
        play_round(&mut game, &mut now);
        // (round to measure at, expected flash ms, expected gap ms)
        let expectations = [(2, 450, 120), (5, 330, 100), (9, 240, 80), (13, 180, 60)];
        for (at_round, flash_ms, gap_ms) in expectations {
            while game.round() < at_round {
                play_round(&mut game, &mut now);
            }
            let (first, measured_flash, measured_gap) = measure_watch_tempo(&mut game, &mut now);
            // Allow the up-to-10 ms of observation slop advance_to_watch
            // leaves before the measurement starts.
            assert!(
                measured_flash.abs_diff(flash_ms) <= 15,
                "round {at_round}: flash {measured_flash} ms, want ~{flash_ms}"
            );
            assert!(
                measured_gap.abs_diff(gap_ms) <= 15,
                "round {at_round}: gap {measured_gap} ms, want ~{gap_ms}"
            );
            // Finish the Round: the measured flash plus the rest.
            let mut sequence = vec![first];
            sequence.extend(observe_watch(&mut game, &mut now));
            // The Echo timeout stays 3.0 s at every tier: hesitate 2.9 s on
            // the first key and live.
            advance_ms(&mut game, &mut now, 2900);
            assert_eq!(game.phase(), Phase::Echo, "round {at_round}");
            for &pad in &sequence {
                game.press(pad, now);
            }
            advance_to_watch(&mut game, &mut now);
        }
    }

    #[test]
    fn tier_up_round_breaks_stretch_and_carry_the_speed_up_callout() {
        let mut game = Game::new(42, None);
        let mut now = Instant::now();
        start_run(&mut game, &mut now);
        // Rounds 1–3 end in plain 800 ms breaks with no callout.
        let sequence = observe_watch(&mut game, &mut now);
        game.press(sequence[0], now);
        assert_eq!(game.phase(), Phase::RoundBreak);
        assert_eq!(game.speed_up(), None);
        advance_to_watch(&mut game, &mut now);
        while game.round() < 4 {
            play_round(&mut game, &mut now);
        }
        // Round 4's break leads into tier ×2: stretched to ~1.5 s, calling
        // out the speed-up.
        let sequence = observe_watch(&mut game, &mut now);
        for &pad in &sequence {
            game.press(pad, now);
        }
        assert_eq!(game.phase(), Phase::RoundBreak);
        assert_eq!(game.speed_up(), Some(2));
        advance_ms(&mut game, &mut now, 1300);
        assert_eq!(game.phase(), Phase::RoundBreak, "stretched past 800 ms");
        advance_ms(&mut game, &mut now, 300);
        assert_eq!(game.phase(), Phase::Watch);
        assert_eq!(game.round(), 5);
        assert_eq!(game.speed_up(), None);
    }

    #[test]
    fn each_correct_step_scores_ten_times_the_tier_multiplier() {
        let mut game = Game::new(42, None);
        let mut now = Instant::now();
        start_run(&mut game, &mut now);
        assert_eq!(game.score(), 0);
        // Rounds 1–4 at ×1: 10·1 + 10·2 + 10·3 + 10·4 = 100.
        for _ in 1..=4 {
            play_round(&mut game, &mut now);
        }
        assert_eq!(game.score(), 100);
        // Round 5 at ×2 scores live, per correct Step.
        let sequence = observe_watch(&mut game, &mut now);
        game.press(sequence[0], now);
        assert_eq!(game.score(), 120, "each Step lands the instant it's judged");
        for &pad in &sequence[1..] {
            game.press(pad, now);
        }
        assert_eq!(game.score(), 200);
        // A Mistake ends the Run with the Score intact for the overlay.
        advance_to_watch(&mut game, &mut now);
        let sequence = observe_watch(&mut game, &mut now);
        game.press(wrong_pad(sequence[0]), now);
        advance_ms(&mut game, &mut now, 1100);
        assert_eq!(game.phase(), Phase::GameOver);
        assert_eq!(game.score(), 200);
        assert_eq!(game.round(), 6);
        assert_eq!(game.speed_tier(), 2);
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
        let mut game = Game::new(42, None);
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
        let mut game = Game::new(42, None);
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
        let mut game = Game::new(42, None);
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
        let mut game = Game::new(42, None);
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
            advance_to_watch(&mut game, &mut now);
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
    fn high_score_loads_at_launch() {
        let dir = crate::highscore::temp_data_dir("loads-at-launch");
        crate::highscore::save(&dir, 50);
        let game = Game::new(42, Some(dir.clone()));
        assert_eq!(game.high_score(), 50);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Die on the current Round's first Step and ride the freeze into
    /// Game Over.
    fn die_now(game: &mut Game, now: &mut Instant) {
        let sequence = observe_watch(game, now);
        game.press(wrong_pad(sequence[0]), *now);
        advance_ms(game, now, 1100);
        assert_eq!(game.phase(), Phase::GameOver);
    }

    #[test]
    fn game_over_writes_the_high_score_once_and_only_on_a_beat() {
        let dir = crate::highscore::temp_data_dir("write-on-beat");
        crate::highscore::save(&dir, 50);
        let mut game = Game::new(42, Some(dir.clone()));
        let mut now = Instant::now();
        // A Run that dies at 0 doesn't touch the record or celebrate.
        start_run(&mut game, &mut now);
        die_now(&mut game, &mut now);
        assert_eq!(game.score(), 0);
        assert!(!game.new_high_score());
        assert_eq!(game.high_score(), 50);
        assert_eq!(crate::highscore::load(&dir), 50);
        // A Run that beats the record writes it and celebrates: Rounds 1–3
        // score 10 + 20 + 30 = 60 > 50.
        game.start(now);
        advance_ms(&mut game, &mut now, 1100);
        for _ in 1..=3 {
            play_round(&mut game, &mut now);
        }
        die_now(&mut game, &mut now);
        assert_eq!(game.score(), 60);
        assert!(game.new_high_score());
        assert_eq!(game.high_score(), 60);
        assert_eq!(crate::highscore::load(&dir), 60);
        // Matching (not beating) the record is no event.
        game.start(now);
        advance_ms(&mut game, &mut now, 1100);
        for _ in 1..=3 {
            play_round(&mut game, &mut now);
        }
        die_now(&mut game, &mut now);
        assert_eq!(game.score(), 60);
        assert!(!game.new_high_score());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_run_in_flight_never_touches_the_record() {
        let dir = crate::highscore::temp_data_dir("mid-run");
        let mut game = Game::new(42, Some(dir.clone()));
        let mut now = Instant::now();
        start_run(&mut game, &mut now);
        for _ in 1..=3 {
            play_round(&mut game, &mut now);
        }
        // 60 points in and mid-Run: nothing on disk. A quit here (the
        // shell just drops the Game) therefore discards the Run entirely.
        assert_eq!(game.score(), 60);
        assert_eq!(crate::highscore::load(&dir), 0);
        drop(game);
        assert_eq!(crate::highscore::load(&dir), 0);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn new_game_opens_at_title_with_zeroed_stats() {
        let game = Game::new(42, None);
        assert_eq!(game.phase(), Phase::Title);
        assert_eq!(game.score(), 0);
        assert_eq!(game.high_score(), 0);
        assert_eq!(game.round(), 0);
        assert_eq!(game.speed_tier(), 1);
    }
}
