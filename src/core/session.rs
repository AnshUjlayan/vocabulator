use crate::db::models::Word;
use crate::db::queries;
use crate::ui::app::Screen;
use rusqlite::Connection;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Group,
    Marked,
    Weak,
    Custom,
}

impl Type {
    pub fn label(&self) -> &'static str {
        use Type::*;
        match self {
            Group => "Continue Learning",
            Marked => "Review Marks",
            Weak => "Revise Weak",
            Custom => "Custom Query",
        }
    }
}

#[derive(Debug)]
pub struct Session {
    pub words: Vec<Word>,
    pub index: usize,

    // UI state
    pub sesison_type: Type,
    pub show_definition: bool,
    pub graded: Option<bool>,
    pub input_buffer: String,
    pub insert_mode: bool,
}

impl Session {
    pub fn current(&self) -> &Word {
        &self.words[self.index]
    }

    pub fn current_mut(&mut self) -> &mut Word {
        &mut self.words[self.index]
    }

    pub fn reset_ui_state(&mut self) {
        self.show_definition = false;
        self.graded = None;
        self.input_buffer.clear();
        self.insert_mode = false;
    }
}

pub fn start_session(conn: &Connection, session_type: Type) -> anyhow::Result<(Session, Screen)> {
    let (screen, group_id, index) = queries::fetch_progress(conn)?;

    let words = queries::fetch_words_by_group(conn, group_id)?;

    Ok((
        Session {
            words,
            index,
            sesison_type: Type::Group,
            show_definition: false,
            graded: None,
            input_buffer: String::new(),
            insert_mode: false,
        },
        screen,
    ))
}
