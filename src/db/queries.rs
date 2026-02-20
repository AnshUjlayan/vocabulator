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

pub fn update_word_stats(conn: &Connection, word: &Word) -> Result<()> {
    conn.execute(
        "UPDATE words
         SET marked=?1,
             last_seen=?2,
             times_seen=?3,
             success_count=?4
         WHERE id=?5",
        params![
            word.marked,
            word.last_seen,
            word.times_seen,
            word.success_count,
            word.id
        ],
    )?;

    Ok(())
}

fn upsert_state(conn: &Connection, key: &str, value: i32) -> Result<()> {
    conn.execute(
        "INSERT INTO app_state(key,value)
         VALUES(?1,?2)
         ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        params![key, value],
    )?;
    Ok(())
}

pub fn save_progress(conn: &Connection, progress: (Screen, i32, usize)) -> Result<()> {
    let (screen, group_id, index) = progress;

    upsert_state(conn, "mode", screen_to_int(screen))?;
    upsert_state(conn, "group_id", group_id)?;
    upsert_state(conn, "index", index as i32)?;

    Ok(())
}

pub fn get_tutorial_completed(conn: &Connection) -> Result<bool> {
    let value: i32 = conn
        .query_row(
            "SELECT value FROM app_state WHERE key=?1",
            params!["tutorial_completed"],
            |row| row.get(0),
        )
        .unwrap_or(0);

    Ok(value == 1)
}

pub fn set_tutorial_completed(conn: &Connection, completed: bool) -> Result<()> {
    let value = if completed { 1 } else { 0 };
    upsert_state(conn, "tutorial_completed", value)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::INIT_SCHEMA;
    use rusqlite::Connection;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute_batch(INIT_SCHEMA).unwrap();

        conn
    }

    #[test]
    fn test_save_and_fetch_progress() {
        let conn = setup();

        save_progress(&conn, (Screen::Test, 3, 7)).unwrap();
        let (screen, group, idx) = fetch_progress(&conn).unwrap();

        assert!(matches!(screen, Screen::Test));
        assert_eq!(group, 3);
        assert_eq!(idx, 7);
    }

    #[test]
    fn test_update_word_stats() {
        let conn = setup();

        conn.execute("INSERT INTO words VALUES(1,'a','b',1,0,0,0,0)", [])
            .unwrap();

        let w = Word {
            id: 1,
            word: "a".into(),
            definition: "b".into(),
            group_id: 1,
            marked: true,
            last_seen: Some(10),
            times_seen: 5,
            success_count: 4,
        };

        update_word_stats(&conn, &w).unwrap();

        let v: i32 = conn
            .query_row("SELECT times_seen FROM words WHERE id=1", [], |r| r.get(0))
            .unwrap();

        assert_eq!(v, 5);
    }

    #[test]
    fn test_get_tutorial_completed_default() {
        let conn = setup();

        // When key doesn't exist, should return false
        let completed = get_tutorial_completed(&conn).unwrap();
        assert_eq!(completed, false);
    }

    #[test]
    fn test_set_and_get_tutorial_completed() {
        let conn = setup();

        // Set to true
        set_tutorial_completed(&conn, true).unwrap();
        let completed = get_tutorial_completed(&conn).unwrap();
        assert_eq!(completed, true);

        // Set to false
        set_tutorial_completed(&conn, false).unwrap();
        let completed = get_tutorial_completed(&conn).unwrap();
        assert_eq!(completed, false);

        // Set to true again
        set_tutorial_completed(&conn, true).unwrap();
        let completed = get_tutorial_completed(&conn).unwrap();
        assert_eq!(completed, true);
    }
}
