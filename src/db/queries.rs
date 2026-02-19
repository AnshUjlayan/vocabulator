use super::models::Word;
use crate::ui::app::Screen;
use anyhow::Result;
use rusqlite::{Connection, params};

fn screen_to_int(screen: Screen) -> i32 {
    match screen {
        Screen::Practice => 0,
        Screen::Test => 1,
        _ => 0,
    }
}

fn int_to_screen(v: i32) -> Screen {
    match v {
        1 => Screen::Test,
        _ => Screen::Practice,
    }
}

fn map_word(row: &rusqlite::Row) -> rusqlite::Result<Word> {
    Ok(Word {
        id: row.get(0)?,
        word: row.get(1)?,
        definition: row.get(2)?,
        group_id: row.get(3)?,
        marked: row.get(4)?,
        last_seen: row.get(5)?,
        times_seen: row.get(6)?,
        success_count: row.get(7)?,
    })
}

pub fn fetch_progress(conn: &Connection) -> Result<(Screen, i32, usize)> {
    let mode: i32 = conn
        .query_row(
            "SELECT value FROM app_state WHERE key=?1",
            params!["mode"],
            |row| row.get(0),
        )
        .unwrap_or(0);

    let group_id: i32 = conn
        .query_row(
            "SELECT value FROM app_state WHERE key=?1",
            params!["group_id"],
            |row| row.get(0),
        )
        .unwrap_or(1);

    let index: i32 = conn
        .query_row(
            "SELECT value FROM app_state WHERE key=?1",
            params!["index"],
            |row| row.get(0),
        )
        .unwrap_or(0);

    Ok((int_to_screen(mode), group_id, index as usize))
}

pub fn fetch_words_by_group(conn: &Connection, group_id: i32) -> Result<Vec<Word>> {
    let mut stmt = conn.prepare(
        "SELECT id, word, definition, group_id,
                marked, last_seen, times_seen, success_count
         FROM words WHERE group_id=?1",
    )?;

    Ok(stmt
        .query_map(params![group_id], map_word)?
        .collect::<Result<Vec<_>, _>>()?)
}

pub fn fetch_marked_words(conn: &Connection) -> Result<Vec<Word>> {
    let mut stmt = conn.prepare(
        "SELECT id, word, definition, group_id,
                marked, last_seen, times_seen, success_count
         FROM words
         WHERE marked=1
         ORDER BY last_seen DESC
         LIMIT 20",
    )?;

    Ok(stmt
        .query_map([], map_word)?
        .collect::<Result<Vec<_>, _>>()?)
}

pub fn fetch_weak_words(conn: &Connection) -> Result<Vec<Word>> {
    let mut stmt = conn.prepare(
        "SELECT id, word, definition, group_id,
                marked, last_seen, times_seen, success_count
         FROM words
         WHERE times_seen>0
         ORDER BY 1.0*success_count/times_seen ASC
         LIMIT 20",
    )?;

    Ok(stmt
        .query_map([], map_word)?
        .collect::<Result<Vec<_>, _>>()?)
}
