use crate::ui::screens::{menu, practice, test};
use anyhow::Result;
use crossterm::event::{self, Event};

use super::{
    app::{App, Screen},
    terminal::{init_terminal, restore_terminal},
};

pub fn run() -> Result<()> {
    let mut terminal = init_terminal()?;
    let mut app = App::new();

    loop {
        terminal.draw(|f| match app.current_screen {
            Screen::Menu => menu::render(f, &app),
            Screen::Practice => practice::render(f, &app),
            Screen::Test => test::render(f, &app),
        })?;

        if let Event::Key(key) = event::read()? {
            match app.current_screen {
                Screen::Menu => menu::handle_event(&mut app, key),
                Screen::Practice => practice::handle_event(&mut app, key),
                Screen::Test => test::handle_event(&mut app, key),
            }
        }

        if app.should_quit {
            break;
        }
    }

    restore_terminal(terminal)?;
    Ok(())
}
