//! High Score persistence (ADR 0001): a single integer stored as literal
//! plain text (e.g. `12\n`) at `$XDG_DATA_HOME/simon-says/highscore`,
//! falling back to `~/.local/share/simon-says/highscore`, with the XDG
//! lookup hand-rolled in std — no `serde`, no `dirs`. Best-effort and
//! silent: a missing or corrupt file loads as 0 and save errors are
//! swallowed, so the scoreboard can never crash or block the game.

use std::fs;
use std::path::{Path, PathBuf};

/// The stored file, relative to the data dir.
const RELATIVE_PATH: &str = "simon-says/highscore";

/// The XDG data dir for the running user: `$XDG_DATA_HOME` if set and
/// non-empty, else `~/.local/share`. `None` when the environment offers
/// neither — the game simply runs without a scoreboard.
pub fn data_dir() -> Option<PathBuf> {
    let xdg_data_home = std::env::var("XDG_DATA_HOME").ok();
    let home = std::env::var("HOME").ok();
    resolve_data_dir(xdg_data_home.as_deref(), home.as_deref())
}

/// The pure XDG lookup behind [`data_dir`], split out so the resolution
/// order is testable without touching the process environment.
pub fn resolve_data_dir(xdg_data_home: Option<&str>, home: Option<&str>) -> Option<PathBuf> {
    match xdg_data_home {
        Some(dir) if !dir.is_empty() => Some(PathBuf::from(dir)),
        _ => match home {
            Some(dir) if !dir.is_empty() => Some(Path::new(dir).join(".local/share")),
            _ => None,
        },
    }
}

/// The stored High Score; a missing or unreadable file is silently 0.
pub fn load(data_dir: &Path) -> u32 {
    fs::read_to_string(data_dir.join(RELATIVE_PATH))
        .ok()
        .and_then(|text| text.trim().parse().ok())
        .unwrap_or(0)
}

/// Store `score` as a literal plain-text integer. Best-effort: any error
/// is swallowed — persistence can never crash the game.
pub fn save(data_dir: &Path, score: u32) {
    let path = data_dir.join(RELATIVE_PATH);
    if let Some(parent) = path.parent()
        && fs::create_dir_all(parent).is_ok()
    {
        let _ = fs::write(path, format!("{score}\n"));
    }
}

/// A fresh per-test data dir under the system temp dir — never the real
/// XDG location.
#[cfg(test)]
pub(crate) fn temp_data_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("simon-says-test-{}-{tag}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn high_score_round_trips_as_literal_plain_text() {
        let dir = temp_data_dir("round-trip");
        save(&dir, 12);
        // The stored form is the ADR 0001 literal: ASCII digits, newline.
        let stored = fs::read_to_string(dir.join("simon-says/highscore")).unwrap();
        assert_eq!(stored, "12\n");
        assert_eq!(load(&dir), 12);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn missing_or_corrupt_file_loads_as_zero() {
        let dir = temp_data_dir("corrupt");
        // Missing file → 0, silently.
        assert_eq!(load(&dir), 0);
        // Corrupt contents → 0, silently.
        fs::create_dir_all(dir.join("simon-says")).unwrap();
        fs::write(dir.join("simon-says/highscore"), "not a number").unwrap();
        assert_eq!(load(&dir), 0);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn xdg_data_home_wins_and_home_local_share_is_the_fallback() {
        // Set and non-empty: $XDG_DATA_HOME wins.
        assert_eq!(
            resolve_data_dir(Some("/xdg/data"), Some("/home/player")),
            Some(PathBuf::from("/xdg/data"))
        );
        // Unset or empty: fall back to ~/.local/share.
        assert_eq!(
            resolve_data_dir(None, Some("/home/player")),
            Some(PathBuf::from("/home/player/.local/share"))
        );
        assert_eq!(
            resolve_data_dir(Some(""), Some("/home/player")),
            Some(PathBuf::from("/home/player/.local/share"))
        );
        // No usable environment at all: no data dir, and that's fine.
        assert_eq!(resolve_data_dir(None, None), None);
        assert_eq!(resolve_data_dir(None, Some("")), None);
    }

    #[test]
    fn save_errors_are_swallowed() {
        let dir = temp_data_dir("swallow");
        // Make `<dir>/simon-says` a *file*, so creating the subdir — and
        // therefore the save — must fail. The game never notices.
        fs::write(dir.join("simon-says"), "in the way").unwrap();
        save(&dir, 99);
        assert_eq!(load(&dir), 0);
        let _ = fs::remove_dir_all(&dir);
    }
}
