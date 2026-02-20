// Tutorial prompt screen module
// Displays the initial prompt asking if the user wants to start the tutorial

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use crate::ui::app::App;

/// Render the tutorial prompt screen
///
/// Displays a simple menu with two options:
/// - "Start Tutorial"
/// - "Skip to Main Menu"
///
/// The selected option is highlighted with "> " symbol and bold text.
/// Uses consistent styling with the existing menu screen.
///
/// **Validates: Requirements 1.2, 11.1, 11.2, 11.5**
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)])
        .split(frame.size());

    let options = vec!["Start Tutorial", "Skip to Main Menu"];
    let items: Vec<ListItem> = options
        .iter()
        .map(|option| ListItem::new(*option))
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected));

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Welcome to Vocabulator"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, chunks[0], &mut state);
}

use crossterm::event::{KeyCode, KeyEvent};
use crate::core::tutorial::init_tutorial;
use crate::ui::app::Screen;
use crate::audio;

/// Handle keyboard events for the tutorial prompt screen
///
/// Handles navigation between two options:
/// - Index 0: "Start Tutorial"
/// - Index 1: "Skip to Main Menu"
///
/// Controls:
/// - Up arrow or 'k': Move selection up
/// - Down arrow or 'j': Move selection down
/// - Enter: Confirm selection
///
/// **Validates: Requirements 1.2, 1.3, 1.4**
pub fn handle_event(app: &mut App, key: KeyEvent) {
    match key.code {
        // Navigate down
        KeyCode::Down | KeyCode::Char('j') => {
            audio::play_menu_sound();
            app.selected = (app.selected + 1) % 2; // Wrap between 0 and 1
        }
        // Navigate up
        KeyCode::Up | KeyCode::Char('k') => {
            audio::play_menu_sound();
            app.selected = if app.selected == 0 { 1 } else { 0 }; // Wrap between 0 and 1
        }
        // Select option
        KeyCode::Enter => {
            match app.selected {
                0 => {
                    // Start Tutorial selected
                    app.tutorial_state = Some(init_tutorial());
                    app.current_screen = Screen::Tutorial;
                }
                1 => {
                    // Skip to Main Menu selected
                    app.current_screen = Screen::Menu;
                }
                _ => {} // Should never happen with only 2 options
            }
        }
        _ => {} // Ignore other keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::app::App;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_navigate_down_wraps() {
        let mut app = App::new_test();
        app.current_screen = Screen::TutorialPrompt;
        app.selected = 0;

        let key = KeyEvent::new(KeyCode::Down, KeyModifiers::empty());
        handle_event(&mut app, key);

        assert_eq!(app.selected, 1);
    }

    #[test]
    fn test_navigate_down_with_j() {
        let mut app = App::new_test();
        app.current_screen = Screen::TutorialPrompt;
        app.selected = 0;

        let key = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty());
        handle_event(&mut app, key);

        assert_eq!(app.selected, 1);
    }

    #[test]
    fn test_navigate_down_wraps_to_zero() {
        let mut app = App::new_test();
        app.current_screen = Screen::TutorialPrompt;
        app.selected = 1;

        let key = KeyEvent::new(KeyCode::Down, KeyModifiers::empty());
        handle_event(&mut app, key);

        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_navigate_up_wraps() {
        let mut app = App::new_test();
        app.current_screen = Screen::TutorialPrompt;
        app.selected = 1;

        let key = KeyEvent::new(KeyCode::Up, KeyModifiers::empty());
        handle_event(&mut app, key);

        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_navigate_up_with_k() {
        let mut app = App::new_test();
        app.current_screen = Screen::TutorialPrompt;
        app.selected = 1;

        let key = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::empty());
        handle_event(&mut app, key);

        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_navigate_up_wraps_to_one() {
        let mut app = App::new_test();
        app.current_screen = Screen::TutorialPrompt;
        app.selected = 0;

        let key = KeyEvent::new(KeyCode::Up, KeyModifiers::empty());
        handle_event(&mut app, key);

        assert_eq!(app.selected, 1);
    }

    #[test]
    fn test_select_start_tutorial() {
        let mut app = App::new_test();
        app.current_screen = Screen::TutorialPrompt;
        app.selected = 0;

        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        handle_event(&mut app, key);

        assert_eq!(app.current_screen, Screen::Tutorial);
        assert!(app.tutorial_state.is_some());
    }

    #[test]
    fn test_select_skip_to_menu() {
        let mut app = App::new_test();
        app.current_screen = Screen::TutorialPrompt;
        app.selected = 1;

        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        handle_event(&mut app, key);

        assert_eq!(app.current_screen, Screen::Menu);
        assert!(app.tutorial_state.is_none());
    }

    #[test]
    fn test_ignore_other_keys() {
        let mut app = App::new_test();
        app.current_screen = Screen::TutorialPrompt;
        app.selected = 0;

        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty());
        handle_event(&mut app, key);

        // Should remain unchanged
        assert_eq!(app.selected, 0);
        assert_eq!(app.current_screen, Screen::TutorialPrompt);
    }
}
