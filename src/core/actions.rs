use crate::core::{progress, session};
use crate::ui::app::{App, Screen};
use anyhow::{Result, anyhow};

pub fn handle_enter(app: &mut App) -> Result<()> {
    let session = app.session.as_mut().ok_or_else(|| anyhow!("No session"))?;

    if !(session.show_definition && session.graded.is_some()) {
        return Ok(());
    }

    let word = session.current_mut();
    progress::update_word_stats(&app.conn, word)?;

    let finished = session.advance();

    if session.session_type == session::Type::Group {
        progress::save_progress(
            &app.conn,
            (
                app.current_screen,
                session.current().group_id,
                session.index,
            ),
        )?;
    }

    if finished {
        if app.current_screen == Screen::Test {
            app.current_screen = Screen::Menu;
        } else {
            app.current_screen = Screen::Test;
        }
    }

    Ok(())
}
