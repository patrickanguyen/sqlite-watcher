use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
        .split(frame.size());

    assert!(chunks.len() == 2);

    draw_header(frame, app, chunks[0]);
    draw_body(frame, app, chunks[1]);
}

fn draw_header<B: Backend>(frame: &mut Frame<B>, app: &mut App, chunk: Rect) {
    let block = Block::default().title("Tables");
    let titles = app
        .tabs
        .iter()
        .map(|table| Spans::from(table.clone()))
        .collect();
    let tabs = Tabs::new(titles)
        .block(block)
        .select(app.tab_idx)
        .divider(symbols::DOT)
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White));

    frame.render_widget(tabs, chunk);
}

fn draw_body<B: Backend>(frame: &mut Frame<B>, app: &mut App, chunk: Rect) {
    let lines: Vec<Spans> = app
        .rows
        .iter()
        .map(|row| Spans::from(Span::raw(row)))
        .collect();
    let block = Block::default().title("Rows").borders(Borders::ALL);
    let paragraph = Paragraph::new(lines)
        .block(block)
        .scroll((app.scroll.try_into().unwrap(), 0));

    frame.render_widget(paragraph, chunk);
}
