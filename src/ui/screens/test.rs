use crate::ui::app::{App, Screen::Menu};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    widgets::{Block, Borders},
};

pub fn handle_event(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.current_screen = Menu,
        _ => {}
    }
}

pub fn render(f: &mut Frame, _app: &App) {
    let block = Block::default().title("Test").borders(Borders::ALL);

    f.render_widget(block, f.size());
}
