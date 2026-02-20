use crate::{
    db::{models::Word, queries},
    ui::app::Screen,
};
use anyhow::Result;
use rusqlite::Connection;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn save_progress(conn: &Connection, progress: (Screen, i32, usize)) -> Result<()> {
    queries::save_progress(conn, progress)
}

pub fn update_word_stats(conn: &Connection, word: &mut Word) -> Result<()> {
    word.last_seen = Some(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i32);
    queries::update_word_stats(conn, &word)
}
