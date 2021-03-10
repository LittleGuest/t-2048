#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slice_as_array;

use std::io;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Direction, Layout};
use tui::Terminal;

use global::GAME_DESCRIPTION;

use crate::palace::MoveDirection;

mod global;
mod palace;
mod store;
mod ui;
mod util;

fn setup(terminal: &mut Terminal<impl Backend>) -> anyhow::Result<()> {
    let mut game = palace::Game::new();

    loop {
        terminal.draw(|f| {
            let game = &mut game;

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(ui::crate_percentage_constraint(&[20, 60, 20]))
                .split(f.size());

            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints(ui::crate_percentage_constraint(&[50; 2]))
                .split(chunks[0]);

            ui::render_description(f, GAME_DESCRIPTION, "游戏说明", &left, 0);

            ui::render_model(f, &left, 1, game);

            ui::render_palace(f, &chunks, 1, &game.palaces);

            let score_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(ui::crate_percentage_constraint(&[30; 3]))
                .split(chunks[2]);
            ui::render_paragraph(f, &game.total_score.to_string(), "得分", &score_chunks, 0);

            ui::render_paragraph(f, &game.top_score.to_string(), "最高分", &score_chunks, 1);

            if game.game_over() {
                ui::game_over_popup(f, game.total_score);
            }
        })?;

        if let Event::Key(KeyEvent { code, .. }) = crossterm::event::read()? {
            match code {
                KeyCode::Esc | KeyCode::Char('q') => break,
                KeyCode::Char('r') => {
                    game.change_model();
                }
                KeyCode::Char('z') => {
                    game.back()?;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    game.insert_history()?;
                    game.move_palaces(MoveDirection::Up);
                    game.insert_top_score()?;
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    game.insert_history()?;
                    game.move_palaces(MoveDirection::Down);
                    game.insert_top_score()?;
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    game.insert_history()?;
                    game.move_palaces(MoveDirection::Left);
                    game.insert_top_score()?;
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    game.insert_history()?;
                    game.move_palaces(MoveDirection::Right);
                    game.insert_top_score()?;
                }
                KeyCode::Char('m') => {
                    game.model.next();
                    game.change_model();
                }
                KeyCode::Char('M') => {
                    game.model.previous();
                    game.change_model();
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    setup(&mut terminal)?;

    execute!(stdout, LeaveAlternateScreen, Show, DisableMouseCapture)?;
    disable_raw_mode()?;

    Ok(())
}
