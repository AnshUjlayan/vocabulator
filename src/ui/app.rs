#[derive(Debug)]
pub struct App {
    pub menu_items: Vec<&'static str>,
    pub selected: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            menu_items: vec!["Start Practice", "Review Incorrect", "Exit"],
            selected: 0,
            should_quit: false,
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
        if self.menu_items[self.selected] == "Exit" {
            self.should_quit = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_wraps_forward() {
        let mut app = App::new();
        app.selected = app.menu_items.len() - 1;
        app.next();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_navigation_wraps_backward() {
        let mut app = App::new();
        app.selected = 0;
        app.previous();
        assert_eq!(app.selected, app.menu_items.len() - 1);
    }

    #[test]
    fn test_exit_sets_flag() {
        let mut app = App::new();
        app.selected = 2;
        app.select();
        assert!(app.should_quit);
    }
}
