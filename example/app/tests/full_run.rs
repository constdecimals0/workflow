//! The scripted full-Run integration test: one complete Run driven through
//! the lib's public API exactly as the shell would drive it — fixed seed,
//! scripted `now` timeline, key presses fed in, the Sequence learned by
//! watching `lit_pad()` — from the length-1 Sequence to Game Over, crossing
//! a Speed Tier boundary, and asserting the final Score and the High Score
//! behavior. No UI tests here or anywhere (ADR 0002): the shell is
//! verified by playing.

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use simon_says::{Game, Mistake, Pad, Phase, highscore};

/// A fresh temp data dir so the test can never touch the real record.
fn temp_data_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("simon-says-full-run-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Tick forward `ms` milliseconds in shell-sized steps.
fn advance_ms(game: &mut Game, now: &mut Instant, ms: u64) {
    let target = *now + Duration::from_millis(ms);
    while *now < target {
        *now += Duration::from_millis(10);
        game.tick(*now);
    }
}

/// Watch the playback the way a player would, collecting each flash from
/// `lit_pad()`, until the phase leaves Watch.
fn observe_watch(game: &mut Game, now: &mut Instant) -> Vec<Pad> {
    assert_eq!(game.phase(), Phase::Watch);
    let mut flashes = Vec::new();
    let mut was_lit = false;
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

/// Tick through whatever break is in progress until the next Watch begins.
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

#[test]
fn a_full_run_from_first_flash_to_game_over() {
    let dir = temp_data_dir();
    let mut game = Game::new(42, Some(dir.clone()));
    let mut now = Instant::now();

    // Launch: Title over the board, stats at rest, no record yet.
    assert_eq!(game.phase(), Phase::Title);
    assert_eq!(game.high_score(), 0);

    // Enter → the Get Ready beat → Round 1's Watch.
    game.start(now);
    assert_eq!(game.phase(), Phase::GetReady);
    advance_to_watch(&mut game, &mut now);

    // Play Rounds 1–5 correctly: watch, echo, rest. Round 5 crosses the
    // first Speed Tier boundary.
    let mut previous: Vec<Pad> = Vec::new();
    for round in 1..=5u32 {
        assert_eq!(game.round(), round);
        let expected_tier = if round < 5 { 1 } else { 2 };
        assert_eq!(game.speed_tier(), expected_tier);

        let sequence = observe_watch(&mut game, &mut now);
        assert_eq!(sequence.len(), round as usize);
        assert_eq!(
            &sequence[..previous.len()],
            &previous[..],
            "the Sequence only ever grows by one"
        );

        assert_eq!(game.phase(), Phase::Echo);
        for &pad in &sequence {
            game.press(pad, now);
        }
        previous = sequence;

        if round == 4 {
            // The Round Break into tier ×2 carries the callout.
            assert_eq!(game.phase(), Phase::RoundBreak);
            assert_eq!(game.speed_up(), Some(2));
        }
        advance_to_watch(&mut game, &mut now);
    }

    // Rounds 1–4 at ×1 scored 10+20+30+40; Round 5 at ×2 scored 5·20.
    assert_eq!(game.score(), 200);

    // Round 6: one wrong Pad is Sudden Death — through the Death Freeze
    // naming the Mistake and revealing what was owed, into Game Over.
    assert_eq!(game.round(), 6);
    let sequence = observe_watch(&mut game, &mut now);
    let wrong = [Pad::Up, Pad::Down, Pad::Left, Pad::Right]
        .into_iter()
        .find(|pad| *pad != sequence[0])
        .unwrap();
    game.press(wrong, now);
    assert_eq!(game.phase(), Phase::DeathFreeze);
    assert_eq!(game.mistake(), Some(Mistake::WrongPad));
    assert_eq!(game.lit_pad(), Some(sequence[0]));
    advance_ms(&mut game, &mut now, 1100);

    // Game Over reports the Run: Score 200, ROUND 6 · TIER ×2, and the
    // beaten record — written once, as literal plain text.
    assert_eq!(game.phase(), Phase::GameOver);
    assert_eq!(game.score(), 200);
    assert_eq!(game.round(), 6);
    assert_eq!(game.speed_tier(), 2);
    assert!(game.new_high_score());
    assert_eq!(game.high_score(), 200);
    assert_eq!(
        fs::read_to_string(dir.join("simon-says/highscore")).unwrap(),
        "200\n"
    );

    // One more go, straight from Game Over: dying at 0 leaves the record
    // untouched and uncelebrated.
    game.start(now);
    assert_eq!(game.phase(), Phase::GetReady);
    advance_to_watch(&mut game, &mut now);
    let sequence = observe_watch(&mut game, &mut now);
    let wrong = [Pad::Up, Pad::Down, Pad::Left, Pad::Right]
        .into_iter()
        .find(|pad| *pad != sequence[0])
        .unwrap();
    game.press(wrong, now);
    advance_ms(&mut game, &mut now, 1100);
    assert_eq!(game.phase(), Phase::GameOver);
    assert!(!game.new_high_score());
    assert_eq!(game.high_score(), 200);
    assert_eq!(highscore::load(&dir), 200);

    let _ = fs::remove_dir_all(&dir);
}
