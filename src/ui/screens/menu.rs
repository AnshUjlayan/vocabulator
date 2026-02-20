use crate::core::session;
use crate::ui::app::{App, MenuAction};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub fn handle_event(app: &mut App, key: KeyEvent) {
    app.error = None;
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Down | KeyCode::Char('j') => app.next(),
        KeyCode::Up | KeyCode::Char('k') => app.previous(),
        KeyCode::Enter => {
            app.select();

            if let MenuAction::Session(session_type) = app.menu_items[app.selected] {
                match session::start_session(&app.conn, session_type) {
                    Ok((session, screen)) => {
                        if session.index < session.words.len() {
                            app.session = Some(session);
                            app.current_screen = screen;
                        } else {
                            let err: String;
                            if session.words.is_empty() {
                                err = "Word list is empty".to_string();
                            } else {
                                err = format!(
                                    "Index {} out of bounds for vector of length {}. Db corrupted",
                                    session.index,
                                    session.words.len()
                                )
                                .to_string();
                            }
                            app.error = Some(err);
                        }
                    }
                    Err(e) => app.error = Some(e.to_string()),
                }
            }
        }
        _ => {}
    }
}

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.size());

    let items: Vec<ListItem> = app
        .menu_items
        .iter()
        .map(|item| ListItem::new(item.label()))
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected));

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Main Menu"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ")
        .repeat_highlight_symbol(true);

    f.render_stateful_widget(list, chunks[0], &mut state);

    if let Some(err) = &app.error {
        let error_block = Block::default().borders(Borders::ALL).title("Error");

        let paragraph = ratatui::widgets::Paragraph::new(err.clone())
            .block(error_block)
            .style(Style::default().fg(ratatui::style::Color::Red));

        f.render_widget(paragraph, chunks[1]);
    }
}
