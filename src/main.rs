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

    let mut palaces = palace::init_palace();

    let mut total_score = 0_u128;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(ui::crate_percentage_constraint(&[20, 60, 20]))
                .split(f.size());

            ui::render_paragraph(f, GAME_DESCRIPTION, "游戏说明", &chunks, 0);

            f.render_widget(ui::crate_block(""), chunks[1]);

            ui::render_palace(f, &chunks, 1, &palaces);

            let score_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(ui::crate_percentage_constraint(&[30; 3]))
                .split(chunks[2]);
            ui::render_paragraph(f, &total_score.to_string(), "得分", &score_chunks, 0);

            // let top_score = top_score.to_string();
            // render_paragraph(f, &top_score, "最高分", &score_chunks, 1);

            // let move_steps = move_steps.to_string();
            // ui::render_paragraph(f, &move_steps, "移动步数", &score_chunks, 2);

            if palace::game_over(&palaces) {
                ui::game_over_popup(f, total_score);
            }
        })?;

        if let Event::Key(KeyEvent { code, .. }) = crossterm::event::read()? {
            match code {
                KeyCode::Esc | KeyCode::Char('q') => break,
                KeyCode::Char('r') => {
                    total_score = 0_u128;
                    palaces = palace::init_palace();
                }
                KeyCode::Char('z') => {}
                KeyCode::Up | KeyCode::Char('k') => {
                    palace::move_palaces(&mut palaces, &mut total_score, MoveDirection::Up);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    palace::move_palaces(&mut palaces, &mut total_score, MoveDirection::Down);
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    palace::move_palaces(&mut palaces, &mut total_score, MoveDirection::Left);
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    palace::move_palaces(&mut palaces, &mut total_score, MoveDirection::Right);
                }
                _ => {}
            }
        }
    }

    execute!(stdout, LeaveAlternateScreen, Show, DisableMouseCapture)?;
    disable_raw_mode()?;

    Ok(())
}
