//! The thin terminal shell around the game core: terminal lifecycle, the
//! classic single-threaded draw/poll/tick loop (~30 Hz, mirroring the
//! official ratatui `demo` example), and key handling. All game rules live
//! in the `simon_says` lib; all rendering lives in `ui`.

mod ui;

use std::io;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use simon_says::{Game, Pad};

const TICK: Duration = Duration::from_millis(33);

struct App {
    game: Game,
    exit: bool,
}

impl App {
    fn new(game: Game) -> Self {
        Self { game, exit: false }
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut last_tick = Instant::now();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            let timeout = TICK.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)?
                && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                self.on_key(key.code);
            }
            if last_tick.elapsed() >= TICK {
                self.game.tick(Instant::now());
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn on_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => self.exit = true,
            KeyCode::Enter => self.game.start(Instant::now()),
            KeyCode::Up => self.game.press(Pad::Up, Instant::now()),
            KeyCode::Down => self.game.press(Pad::Down, Instant::now()),
            KeyCode::Left => self.game.press(Pad::Left, Instant::now()),
            KeyCode::Right => self.game.press(Pad::Right, Instant::now()),
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        ui::render(frame, &self.game);
    }
}

fn main() -> io::Result<()> {
    // The core never reads the clock or entropy itself: seed its PRNG here,
    // at the edge, from wall time.
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|since_epoch| since_epoch.as_nanos() as u64)
        .unwrap_or(1);
    // `ratatui::run` restores the terminal on both exit and panic, so the
    // shell is never left in raw mode.
    ratatui::run(|terminal| App::new(Game::new(seed)).run(terminal))
}
