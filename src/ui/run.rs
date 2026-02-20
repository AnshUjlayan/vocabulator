use crate::core::tutorial::{is_tutorial_completed, should_auto_advance};
use crate::ui::screens::{menu, practice, test, tutorial, tutorial_prompt};
use anyhow::Result;
use crossterm::event::{self, Event};
use rusqlite::Connection;
use std::time::Duration;

use super::{
    app::{App, Screen},
    terminal::{init_terminal, restore_terminal},
};

pub fn run() -> Result<()> {
    let mut terminal = init_terminal()?;
    let conn = Connection::open("vocab.db")?;
    
    // Check tutorial completion status and set initial screen
    let initial_screen = if is_tutorial_completed(&conn)? {
        Screen::Menu
    } else {
        Screen::TutorialPrompt
    };
    
    let mut app = App::new(conn);
    app.current_screen = initial_screen;

    loop {
        terminal.draw(|f| match app.current_screen {
            Screen::Menu => menu::render(f, &app),
            Screen::Practice => practice::render(f, &app),
            Screen::Test => test::render(f, &app),
            Screen::TutorialPrompt => tutorial_prompt::render(f, &app),
            Screen::Tutorial => tutorial::render(f, &app),
        })?;

        // Check for auto-advance in tutorial step 4
        if app.current_screen == Screen::Tutorial {
            if let Some(ref mut tutorial_state) = app.tutorial_state {
                if should_auto_advance(tutorial_state) {
                    // Auto-advance from step 4 to step 5
                    tutorial_state.current_step = 5;
                    tutorial_state.step_entered_at = Some(std::time::Instant::now());
                    app.error = None;
                    continue; // Skip event polling and redraw immediately
                }
            }
        }

        // Poll for events with a timeout to allow auto-advance checking
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.current_screen {
                    Screen::Menu => menu::handle_event(&mut app, key),
                    Screen::Practice => practice::handle_event(&mut app, key),
                    Screen::Test => test::handle_event(&mut app, key),
                    Screen::TutorialPrompt => tutorial_prompt::handle_event(&mut app, key),
                    Screen::Tutorial => tutorial::handle_event(&mut app, key),
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    restore_terminal(terminal)?;
    Ok(())
}
