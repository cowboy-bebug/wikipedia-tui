use crate::app::{App, Mode};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, Paragraph, Text},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Length(1),
                Constraint::Percentage(70),
            ]
            .as_ref(),
        )
        .split(size);

    draw_left_panels(f, app, chunks[0]);
    draw_right_panels(f, app, chunks[2]);
}

fn draw_left_panels<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(area);

    draw_search_and_mode(f, app, chunks[0]);
    draw_page_list(f, app, chunks[1]);
    draw_content(f, app, chunks[2]);
}

fn draw_right_panels<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(area);

    draw_help(f, app, chunks[0]);
    draw_page(f, app, chunks[1]);
    draw_url(f, app, chunks[2]);
}

fn draw_search_and_mode<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(10)].as_ref())
        .split(area);

    draw_search(f, app, chunks[0]);
    draw_mode(f, app, chunks[1]);
}

fn draw_search<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = [Text::styled(
        &app.search_input,
        Style::default().fg(Color::Green).modifier(Modifier::BOLD),
    )];
    let modifier = match app.mode {
        Mode::Search => Modifier::empty(),
        Mode::Browse => Modifier::DIM,
        Mode::Read => Modifier::DIM,
    };
    let input = Paragraph::new(text.iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().modifier(modifier))
                .title("Search")
                .style(Style::default().modifier(modifier)),
        )
        .wrap(true);
    app.search_cursor_x_max = area.right() - 4;
    app.search_cursor_x = area.left() + app.search_input.len() as u16 + 1;
    app.search_cursor_y = area.top() + 1;
    f.render_widget(input, area);
}

fn draw_page_list<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let pages = app.pages.items.iter().map(|item| Text::raw(item));
    let pages = List::new(pages)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().modifier(match app.mode {
                    Mode::Search => Modifier::DIM,
                    Mode::Browse => Modifier::empty(),
                    Mode::Read => Modifier::empty(),
                }))
                .title("Page"),
        )
        .style(Style::default().modifier(match app.mode {
            Mode::Search => Modifier::DIM,
            Mode::Browse => Modifier::empty(),
            Mode::Read => Modifier::DIM,
        }))
        .highlight_style(Style::default().fg(Color::Green).modifier(match app.mode {
            Mode::Search => Modifier::DIM,
            Mode::Browse => Modifier::BOLD,
            Mode::Read => Modifier::DIM,
        }))
        .highlight_symbol("> ");
    f.render_stateful_widget(pages, area, &mut app.pages.state);
}

fn draw_mode<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = match app.mode {
        Mode::Search => [Text::raw("Search")],
        Mode::Browse => [Text::raw("Browse")],
        Mode::Read => [Text::raw("Read")],
    };
    let paragraph = Paragraph::new(text.iter())
        .block(
            Block::default()
                .title("Mode")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn draw_help<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = match app.mode {
        Mode::Search => [
            Text::raw(" Press any key to type. "),
            Text::raw("Enter to search."),
        ],
        Mode::Browse => [
            Text::raw(" Up & Down to navigate. Left & Right to jump. "),
            Text::raw("Esc to go back to search mode."),
        ],
        Mode::Read => [
            Text::raw(" Up & Down to navigate. Left & Right to jump. "),
            Text::raw("Esc to go back to browse mode."),
        ],
    };
    let paragraph = Paragraph::new(text.iter())
        .block(
            Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Left);
    f.render_widget(paragraph, area);
}

fn draw_content<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = app.toc.iter().map(|item| Text::raw(item));
    let modifier = match app.mode {
        Mode::Search => Modifier::DIM,
        Mode::Browse => Modifier::empty(),
        Mode::Read => Modifier::empty(),
    };
    let paragraph = List::new(text).block(
        Block::default()
            .title("Table of Content")
            .borders(Borders::ALL)
            .border_style(Style::default().modifier(modifier))
            .border_type(BorderType::Rounded)
            .style(Style::default().modifier(modifier)),
    );
    f.render_widget(paragraph, area);
}

fn draw_page<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let page = app.page.iter().map(|x| Text::raw(x)).collect::<Vec<_>>();
    let paragraph = Paragraph::new(page.iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Left)
        .wrap(true)
        .scroll(app.page_scroll);
    f.render_widget(paragraph, area);
}

fn draw_url<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = [Text::raw(app.url.to_owned())];
    let paragraph = Paragraph::new(text.iter())
        .block(
            Block::default()
                .title("URL")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Left)
        .wrap(false);
    f.render_widget(paragraph, area);
}
