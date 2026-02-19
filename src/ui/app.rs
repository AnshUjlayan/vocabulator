use crate::core::session::Session;
use rusqlite::Connection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Menu,
    Practice,
    Test,
}

#[derive(Debug)]
pub struct App {
    pub conn: Connection,
    pub current_screen: Screen,
    pub menu_items: Vec<&'static str>,
    pub selected: usize,
    pub should_quit: bool,
    pub session: Option<Session>,
}

impl App {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn,
            current_screen: Screen::Menu,
            menu_items: vec![
                "Continue Learning",
                "Revise Weak",
                "Review Marks",
                "Custom Query",
                "Test",
                "Exit",
            ],
            selected: 0,
            should_quit: false,
            session: None,
        }
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
        match self.selected {
            0..=3 => self.current_screen = Screen::Practice,
            4 => self.current_screen = Screen::Test,
            _ => self.should_quit = true,
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
        app.selected = 5;
        app.select();
        assert!(app.should_quit);
    }
}
