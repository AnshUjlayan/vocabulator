use crate::core::session::{Session, Type};
use crate::core::tutorial::TutorialState;
use rusqlite::Connection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Menu,
    Practice,
    Test,
    TutorialPrompt,
    Tutorial,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MenuAction {
    Session(Type),
    RestartTutorial,
    Exit,
}

impl MenuAction {
    pub fn label(&self) -> &'static str {
        match self {
            MenuAction::Session(t) => t.label(),
            MenuAction::RestartTutorial => "Restart Tutorial",
            MenuAction::Exit => "Exit",
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub conn: Connection,
    pub current_screen: Screen,
    pub menu_items: Vec<MenuAction>,
    pub selected: usize,
    pub should_quit: bool,
    pub session: Option<Session>,
    pub error: Option<String>,
    pub tutorial_state: Option<TutorialState>,
}

impl App {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn,
            current_screen: Screen::Menu,
            menu_items: vec![
                MenuAction::Session(Type::Group),
                MenuAction::Session(Type::Marked),
                MenuAction::Session(Type::Weak),
                MenuAction::RestartTutorial,
                MenuAction::Exit,
            ],
            selected: 0,
            should_quit: false,
            session: None,
            error: None,
            tutorial_state: None,
        }
    }

    #[cfg(test)]
    pub fn new_test() -> Self {
        Self::new(Connection::open_in_memory().unwrap())
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.menu_items.len();
    }

    pub fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.menu_items.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn select(&mut self) {
        match self.menu_items[self.selected] {
            MenuAction::Exit => self.should_quit = true,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_wraps_forward() {
        let mut app = App::new(Connection::open_in_memory().unwrap());
        app.selected = app.menu_items.len() - 1;
        app.next();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_navigation_wraps_backward() {
        let mut app = App::new(Connection::open_in_memory().unwrap());
        app.selected = 0;
        app.previous();
        assert_eq!(app.selected, app.menu_items.len() - 1);
    }

    #[test]
    fn test_exit_sets_flag() {
        let mut app = App::new(Connection::open_in_memory().unwrap());
        app.selected = app
            .menu_items
            .iter()
            .position(|x| *x == MenuAction::Exit)
            .unwrap();
        app.select();
        assert!(app.should_quit);
    }

    #[test]
    fn test_restart_tutorial_option_exists() {
        let app = App::new(Connection::open_in_memory().unwrap());
        let has_restart = app
            .menu_items
            .iter()
            .any(|x| *x == MenuAction::RestartTutorial);
        assert!(has_restart, "Menu should contain RestartTutorial option");
    }

    #[test]
    fn test_restart_tutorial_label() {
        let action = MenuAction::RestartTutorial;
        assert_eq!(action.label(), "Restart Tutorial");
    }
}
