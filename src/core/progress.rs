use crate::{
    db::{models::Word, queries},
    ui::app::Screen,
};
use anyhow::Result;
use rusqlite::Connection;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn save_progress(conn: &Connection, progress: (Screen, i32, usize)) -> Result<()> {
    let (screen, mut group_id, index) = progress;

    let final_group = queries::fetch_final_group(conn)?.unwrap_or(1);

    if group_id > final_group {
        group_id = 1;
    }

    queries::save_progress(conn, (screen, group_id, index))
}

pub fn update_word_stats(conn: &Connection, word: &mut Word, correct: bool) -> Result<()> {
    word.last_seen = Some(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i32);
    word.times_seen += 1;
    word.success_count += correct as u8;
    queries::update_word_stats(conn, &word)
}
