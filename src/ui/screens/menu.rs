use crate::audio;
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
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
            audio::play_menu_sound();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
            audio::play_menu_sound();
        }
        KeyCode::Enter => {
            app.select();

            match app.menu_items[app.selected] {
                MenuAction::Session(session_type) => {
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
                MenuAction::RestartTutorial => {
                    // Reset tutorial completion flag
                    use crate::core::tutorial::{reset_tutorial, init_tutorial};
                    match reset_tutorial(&app.conn) {
                        Ok(_) => {
                            // Initialize new tutorial state starting at step 0
                            app.tutorial_state = Some(init_tutorial());
                            // Transition to Tutorial screen
                            app.current_screen = crate::ui::app::Screen::Tutorial;
                        }
                        Err(e) => app.error = Some(format!("Failed to restart tutorial: {}", e)),
                    }
                }
                MenuAction::Exit => {
                    // Exit is already handled by app.select()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tutorial::{mark_tutorial_completed, is_tutorial_completed};
    use crate::db::schema::INIT_SCHEMA;
    use crate::ui::app::{App, Screen};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use rusqlite::Connection;

    #[test]
    fn test_restart_tutorial_resets_completion_flag() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(INIT_SCHEMA).unwrap();

        // Mark tutorial as completed first
        mark_tutorial_completed(&conn).unwrap();
        assert!(is_tutorial_completed(&conn).unwrap());

        let mut app = App::new(conn);
        
        // Select the RestartTutorial option
        app.selected = app
            .menu_items
            .iter()
            .position(|x| *x == MenuAction::RestartTutorial)
            .unwrap();

        // Press Enter to restart tutorial
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        handle_event(&mut app, key);

        // Verify tutorial completion flag is reset
        assert!(!is_tutorial_completed(&app.conn).unwrap());
    }

    #[test]
    fn test_restart_tutorial_transitions_to_tutorial_screen() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(INIT_SCHEMA).unwrap();

        let mut app = App::new(conn);
        app.current_screen = Screen::Menu;

        // Select the RestartTutorial option
        app.selected = app
            .menu_items
            .iter()
            .position(|x| *x == MenuAction::RestartTutorial)
            .unwrap();

        // Press Enter to restart tutorial
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        handle_event(&mut app, key);

        // Verify screen transitioned to Tutorial
        assert_eq!(app.current_screen, Screen::Tutorial);
    }

    #[test]
    fn test_restart_tutorial_initializes_tutorial_state() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(INIT_SCHEMA).unwrap();

        let mut app = App::new(conn);
        app.tutorial_state = None;

        // Select the RestartTutorial option
        app.selected = app
            .menu_items
            .iter()
            .position(|x| *x == MenuAction::RestartTutorial)
            .unwrap();

        // Press Enter to restart tutorial
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        handle_event(&mut app, key);

        // Verify tutorial state is initialized
        assert!(app.tutorial_state.is_some());
        
        // Verify tutorial starts at step 0
        let state = app.tutorial_state.unwrap();
        assert_eq!(state.current_step, 0);
    }

    #[test]
    fn test_restart_tutorial_creates_sample_session() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(INIT_SCHEMA).unwrap();

        let mut app = App::new(conn);

        // Select the RestartTutorial option
        app.selected = app
            .menu_items
            .iter()
            .position(|x| *x == MenuAction::RestartTutorial)
            .unwrap();

        // Press Enter to restart tutorial
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        handle_event(&mut app, key);

        // Verify tutorial state has sample session
        assert!(app.tutorial_state.is_some());
        let state = app.tutorial_state.unwrap();
        assert!(state.sample_session.is_some());
    }
}
