use std::io;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use global::GAME_DESCRIPTION;

use tui::backend::CrosstermBackend;
use tui::layout::{Direction, Layout};

use crate::palace::MoveDirection;

use tui::Terminal;

#[macro_use]
extern crate lazy_static;

mod global;
mod palace;
mod ui;
mod util;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut game = palace::Game::new();

    loop {
        terminal.draw(|f| {
            let game = &mut game;

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(ui::crate_percentage_constraint(&[20, 60, 20]))
                .split(f.size());

            ui::render_paragraph(f, GAME_DESCRIPTION, "游戏说明", &chunks, 0);

            f.render_widget(ui::crate_block(""), chunks[1]);

            ui::render_palace(f, &chunks, 1, &game.palaces);

            let score_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(ui::crate_percentage_constraint(&[30; 3]))
                .split(chunks[2]);
            ui::render_paragraph(f, &game.total_score.to_string(), "得分", &score_chunks, 0);

            // ui::render_paragraph(f, &game.top_score.to_string(), "最高分", &score_chunks, 1);

            // ui::render_paragraph(
            //     f,
            //     &game.move_steps.to_string(),
            //     "移动步数",
            //     &score_chunks,
            //     2,
            // );

            if game.game_over() {
                ui::game_over_popup(f, game.total_score);
            }
        })?;

        if let Event::Key(KeyEvent { code, .. }) = crossterm::event::read()? {
            match code {
                KeyCode::Esc | KeyCode::Char('q') => break,
                KeyCode::Char('r') => {
                    game = palace::Game::new();
                }
                KeyCode::Char('z') => {}
                KeyCode::Up | KeyCode::Char('k') => {
                    game.move_palaces(MoveDirection::Up);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    game.move_palaces(MoveDirection::Down);
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    game.move_palaces(MoveDirection::Left);
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    game.move_palaces(MoveDirection::Right);
                }
                _ => {}
            }
        }
    }

    execute!(stdout, LeaveAlternateScreen, Show, DisableMouseCapture)?;
    disable_raw_mode()?;

    Ok(())
}
