use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::{
    global::PALACE_SIZE,
    palace::{Game, Palace},
    util,
};

pub fn create_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
}

pub fn crate_percentage_constraint(items: &[u16]) -> Vec<Constraint> {
    let mut constraints = Vec::with_capacity(items.len());
    for item in items {
        constraints.push(Constraint::Percentage(*item));
    }
    constraints
}

pub fn render_description(
    f: &mut Frame<impl Backend>,
    p: &str,
    title: &str,
    chunks: &[Rect],
    index: usize,
) {
    let paragraph = Paragraph::new(p)
        .alignment(Alignment::Left)
        .block(create_block(title).title(Spans::from(Span::styled(title, Style::default()))));
    f.render_widget(paragraph, chunks[index]);
}

pub fn render_paragraph(
    f: &mut Frame<impl Backend>,
    p: &str,
    title: &str,
    chunks: &[Rect],
    index: usize,
) {
    let paragraph = Paragraph::new(p)
        .alignment(Alignment::Center)
        .block(create_block(title).title(Spans::from(Span::styled(title, Style::default()))));
    f.render_widget(paragraph, chunks[index]);
}

pub fn render_palace(
    f: &mut Frame<impl Backend>,
    chunks: &[Rect],
    index: usize,
    palaces: &[Vec<u128>],
) {
    let palace_size = unsafe { PALACE_SIZE };

    let psf = (1.0 / palace_size as f64 * 100.0) as u16;

    let mut constraints = (0..palace_size).fold(Vec::with_capacity(palace_size), |mut cs, _| {
        cs.push(Constraint::Percentage(psf));
        cs
    });

    if util::odd(palace_size) {
        // 为奇数时，需要充满整个布局
        constraints.push(Constraint::Percentage(100 - psf));
    }

    let middle_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.clone())
        .split(chunks[index]);

    for (x, xp) in palaces.iter().enumerate() {
        let num_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints.clone())
            .split(middle_chunks[x]);
        for (y, &yp) in xp.iter().enumerate() {
            let palace = Palace::default().num(yp);
            f.render_widget(palace, num_chunks[y]);
        }
    }
}

pub fn render_model(f: &mut Frame<impl Backend>, chunks: &[Rect], index: usize, game: &mut Game) {
    let items = game
        .model
        .items
        .iter()
        .map(|(m, _)| ListItem::new(Spans::from(*m)))
        .collect::<Vec<_>>();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("模式"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, chunks[index], &mut game.model.state);
}

pub fn game_over_popup(f: &mut Frame<impl Backend>, total_score: u128) {
    let paragraph = Paragraph::new(format!("得分  {}", total_score))
        .alignment(Alignment::Center)
        .block(
            create_block("游戏结束").title(Spans::from(Span::styled("游戏结束", Style::default()))),
        );

    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
