use crate::ui::app::App;
use ratatui::{
    Frame,
    widgets::{Block, Borders},
};

use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_event(app: &mut App, key: KeyEvent) {
    todo!();
}

pub fn render(f: &mut Frame, _app: &App) {
    let block = Block::default().title("Practice").borders(Borders::ALL);

    f.render_widget(block, f.size());
}
