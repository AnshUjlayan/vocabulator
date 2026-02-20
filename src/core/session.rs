use crate::db::models::Word;
use crate::db::queries;
use crate::ui::app::Screen;
use anyhow::Result;
use rusqlite::Connection;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Type {
    #[default]
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

#[derive(Debug, Default)]
pub struct Session {
    pub words: Vec<Word>,
    pub index: usize,

    // UI state
    pub session_type: Type,
    pub show_definition: bool,
    pub graded: Option<bool>,
    pub input_buffer: String,
    pub insert_mode: bool,
}

impl Session {
    pub fn new(words: Vec<Word>, index: usize, session_type: Type) -> Self {
        Self {
            words,
            index,
            session_type,
            ..Default::default()
        }
    }

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

    pub fn advance(&mut self) -> bool {
        if self.index + 1 >= self.words.len() {
            self.index = 0;
            true
        } else {
            self.index += 1;
            self.reset_ui_state();
            false
        }
    }
}

pub fn start_session(conn: &Connection, session_type: Type) -> Result<(Session, Screen)> {
    match session_type {
        Type::Group => group_session(&conn),
        Type::Marked => marks_session(&conn),
        Type::Weak => weak_session(&conn),
        Type::Custom => anyhow::bail!("Custom session requires query input"),
    }
}

pub fn group_session(conn: &Connection) -> Result<(Session, Screen)> {
    let (screen, group_id, index) = queries::fetch_progress(conn)?;

    let words = queries::fetch_words_by_group(&conn, group_id)?;

    Ok((Session::new(words, index, Type::Group), screen))
}

pub fn marks_session(conn: &Connection) -> Result<(Session, Screen)> {
    let words = queries::fetch_marked_words(&conn)?;

    Ok((Session::new(words, 0, Type::Marked), Screen::Practice))
}

pub fn weak_session(conn: &Connection) -> Result<(Session, Screen)> {
    let words = queries::fetch_weak_words(&conn)?;

    Ok((Session::new(words, 0, Type::Weak), Screen::Practice))
}
