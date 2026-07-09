//! All rendering: the arrow-cross board with its outlined Pads, the Hub at
//! the center, the stats Sidebar, and the overlay states. Visual reference:
//! the picked variant of the retired `example/board-prototype/`.

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Clear, Paragraph};
use simon_says::{Game, Mistake, Pad, Phase};

const SIDEBAR_WIDTH: u16 = 22;

/// One cell of the board's plus formation.
#[derive(Clone, Copy)]
enum Cell {
    Pad(Pad),
    Hub,
    Empty,
}

/// The board's plus formation, mirroring the arrow keys, with the Hub at the
/// center.
const CROSS: [[Cell; 3]; 3] = [
    [Cell::Empty, Cell::Pad(Pad::Up), Cell::Empty],
    [Cell::Pad(Pad::Left), Cell::Hub, Cell::Pad(Pad::Right)],
    [Cell::Empty, Cell::Pad(Pad::Down), Cell::Empty],
];

struct PadStyle {
    glyph: &'static str,
    name: &'static str,
    color: Color,
    flood: Color,
}

fn pad_style(pad: Pad) -> PadStyle {
    match pad {
        Pad::Up => PadStyle {
            glyph: "▲",
            name: "UP",
            color: Color::Rgb(0, 110, 45),
            flood: Color::Rgb(70, 255, 120),
        },
        Pad::Down => PadStyle {
            glyph: "▼",
            name: "DOWN",
            color: Color::Rgb(140, 115, 0),
            flood: Color::Rgb(255, 235, 80),
        },
        Pad::Left => PadStyle {
            glyph: "◀",
            name: "LEFT",
            color: Color::Rgb(150, 25, 25),
            flood: Color::Rgb(255, 95, 85),
        },
        Pad::Right => PadStyle {
            glyph: "▶",
            name: "RIGHT",
            color: Color::Rgb(25, 50, 160),
            flood: Color::Rgb(105, 155, 255),
        },
    }
}

pub fn render(frame: &mut Frame, game: &Game) {
    let [board, sidebar] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Length(SIDEBAR_WIDTH)])
            .areas(frame.area());
    draw_board(frame, board, game);
    draw_sidebar(frame, sidebar, game);
    match game.phase() {
        Phase::Title => draw_title_overlay(frame, board),
        Phase::GameOver => draw_game_over_overlay(frame, board, game),
        Phase::GetReady | Phase::Watch | Phase::Echo | Phase::RoundBreak | Phase::DeathFreeze => {}
    }
}

/// The always-present board: four outlined Pads (colored border, dark
/// interior) around the Hub.
fn draw_board(frame: &mut Frame, area: Rect, game: &Game) {
    let rows: [Rect; 3] = Layout::vertical([Constraint::Fill(1); 3])
        .spacing(1)
        .areas(area);
    for (row_index, row) in rows.iter().enumerate() {
        let cells: [Rect; 3] = Layout::horizontal([Constraint::Fill(1); 3])
            .spacing(2)
            .areas(*row);
        for (cell_index, cell) in cells.iter().enumerate() {
            match CROSS[row_index][cell_index] {
                Cell::Pad(pad) => draw_pad(frame, *cell, pad, game.lit_pad() == Some(pad)),
                Cell::Hub => draw_hub(frame, *cell, game),
                Cell::Empty => {}
            }
        }
    }
}

/// A Pad is outlined (colored border, dark interior) at rest and floods
/// solid with its bright color on flash — unlit Pads are never dimmed.
fn draw_pad(frame: &mut Frame, area: Rect, pad: Pad, lit: bool) {
    let PadStyle {
        glyph,
        name,
        color,
        flood,
    } = pad_style(pad);
    let style = if lit {
        Style::default().bg(flood).fg(Color::Black)
    } else {
        Style::default().fg(color)
    };
    let block = Block::bordered()
        .title(format!(" {name} "))
        .border_style(style);
    let content = center_vertically(area.height.saturating_sub(2), vec![Line::from(glyph)]);
    frame.render_widget(
        Paragraph::new(content)
            .alignment(Alignment::Center)
            .style(style)
            .block(block),
        area,
    );
}

/// The Hub reads out Round, Phase, and (in later slices) callouts at the
/// center of the board.
fn draw_hub(frame: &mut Frame, area: Rect, game: &Game) {
    // At the Death Freeze the Hub names the Mistake in red; on a tier-up
    // Round Break it calls out the speed-up; otherwise it reads the Phase.
    let callout = if let Some(mistake) = game.mistake() {
        Line::styled(
            mistake_text(mistake),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )
    } else if let Some(multiplier) = game.speed_up() {
        Line::styled(
            format!("SPEED UP! ×{multiplier}"),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Line::from(phase_text(game))
    };
    let content = center_vertically(
        area.height,
        vec![
            Line::styled(
                format!("ROUND {}", round_text(game)),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            callout,
        ],
    );
    frame.render_widget(
        Paragraph::new(content)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray)),
        area,
    );
}

fn draw_sidebar(frame: &mut Frame, area: Rect, game: &Game) {
    let [score, high_score, speed_tier, round, phase, keys] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .areas(area);
    let bold = Style::default().add_modifier(Modifier::BOLD);
    stat(
        frame,
        score,
        " SCORE ",
        game.score().to_string(),
        bold.fg(Color::White),
    );
    stat(
        frame,
        high_score,
        " HIGH SCORE ",
        game.high_score().to_string(),
        Style::default(),
    );
    stat(
        frame,
        speed_tier,
        " SPEED TIER ",
        format!("×{}", game.speed_tier()),
        bold.fg(Color::Cyan),
    );
    stat(frame, round, " ROUND ", round_text(game), Style::default());
    stat(
        frame,
        phase,
        " PHASE ",
        phase_text(game).to_string(),
        bold.fg(Color::White),
    );
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("↑↓←→   pads"),
            Line::from("Enter  start"),
            Line::from("Q/Esc  quit"),
        ])
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::bordered().title(" KEYS ")),
        keys,
    );
}

fn stat(frame: &mut Frame, area: Rect, title: &'static str, value: String, style: Style) {
    frame.render_widget(
        Paragraph::new(Line::styled(value, style))
            .alignment(Alignment::Center)
            .block(Block::bordered().title(title)),
        area,
    );
}

/// The launch-only Title, sitting as an overlay on the always-present board.
fn draw_title_overlay(frame: &mut Frame, board: Rect) {
    let area = centered(board, 44, 9);
    frame.render_widget(Clear, area);
    let content = center_vertically(
        area.height.saturating_sub(2),
        vec![
            Line::styled(
                "S I M O N   S A Y S",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Line::raw(""),
            Line::from("Watch the pads. Echo the sequence."),
            Line::from("One mistake ends the run."),
            Line::raw(""),
            Line::styled(
                "ENTER  start   ·   Q / ESC  quit",
                Style::default().fg(Color::DarkGray),
            ),
        ],
    );
    frame.render_widget(
        Paragraph::new(content)
            .alignment(Alignment::Center)
            .style(Style::default().bg(Color::Black).fg(Color::Gray))
            .block(Block::bordered()),
        area,
    );
}

/// The end-of-Run overlay: the Run's result, and Enter for a zero-friction
/// fresh Run (never back via the Title).
fn draw_game_over_overlay(frame: &mut Frame, board: Rect, game: &Game) {
    let area = centered(board, 44, 9);
    frame.render_widget(Clear, area);
    let content = center_vertically(
        area.height.saturating_sub(2),
        vec![
            Line::styled(
                "G A M E   O V E R",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Line::raw(""),
            Line::from(format!("SCORE  {}", game.score())),
            Line::from(format!(
                "ROUND {} · TIER ×{}",
                game.round(),
                game.speed_tier()
            )),
            Line::raw(""),
            Line::styled(
                "ENTER  play again   ·   Q / ESC  quit",
                Style::default().fg(Color::DarkGray),
            ),
        ],
    );
    frame.render_widget(
        Paragraph::new(content)
            .alignment(Alignment::Center)
            .style(Style::default().bg(Color::Black).fg(Color::Gray))
            .block(Block::bordered()),
        area,
    );
}

fn round_text(game: &Game) -> String {
    // Round 0 means no Run is underway yet.
    if game.round() == 0 {
        "—".to_string()
    } else {
        game.round().to_string()
    }
}

fn phase_text(game: &Game) -> &'static str {
    match game.phase() {
        Phase::Title => "TITLE",
        Phase::GetReady => "GET READY",
        Phase::Watch => "WATCH",
        Phase::Echo => "ECHO",
        Phase::RoundBreak => "ROUND BREAK",
        Phase::DeathFreeze => "DEATH FREEZE",
        Phase::GameOver => "GAME OVER",
    }
}

fn mistake_text(mistake: Mistake) -> &'static str {
    match mistake {
        Mistake::WrongPad => "WRONG PAD",
        Mistake::TooSlow => "TOO SLOW",
    }
}

fn center_vertically(height: u16, content: Vec<Line<'static>>) -> Vec<Line<'static>> {
    let padding = (height as usize).saturating_sub(content.len()) / 2;
    let mut lines = vec![Line::raw(""); padding];
    lines.extend(content);
    lines
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}
