use anyhow::{Result, anyhow};
use rusqlite::{Connection, params};
use std::fs;

pub fn seed_from_file(conn: &Connection, path: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let mut group_id: i32 = 0;

    let mut current_word: Option<String> = None;
    let mut current_definition = String::new();

    for raw_line in content.lines() {
        let line = raw_line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("Group") {
            flush_current(conn, &mut current_word, &mut current_definition, group_id)?;

            let id = line
                .split_whitespace()
                .last()
                .ok_or_else(|| anyhow!("Invalid group line: {line}"))?;

            group_id = id.parse::<i32>()?;
            continue;
        }

        // Continuation definition line (starts with digit.)
        if line
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
            && line.contains('.')
        {
            let cleaned = line
                .split_once('.')
                .map(|(_, rest)| rest.trim())
                .unwrap_or(line);

            if !current_definition.is_empty() {
                current_definition.push('\n');
            }
            current_definition.push_str(cleaned);
            continue;
        }

        // Continuation if line starts with '('
        if line.starts_with('(') {
            if !current_definition.is_empty() {
                current_definition.push('\n');
            }
            current_definition.push_str(line.trim());
            continue;
        }

        // New word — flush previous
        flush_current(conn, &mut current_word, &mut current_definition, group_id)?;

        let mut parts = line.splitn(2, ' ');
        let word = parts.next().unwrap().to_string();
        let definition_part = parts.next().unwrap_or("").trim();

        current_word = Some(word);
        current_definition = normalize_inline_definitions(definition_part);
    }

    // flush last entry
    flush_current(conn, &mut current_word, &mut current_definition, group_id)?;

    Ok(())
}

fn flush_current(
    conn: &Connection,
    current_word: &mut Option<String>,
    current_definition: &mut String,
    group_id: i32,
) -> Result<()> {
    if let Some(word) = current_word.take() {
        conn.execute(
            "INSERT OR IGNORE INTO words (word, group_id, definition)
             VALUES (?1, ?2, ?3)",
            params![word, group_id, current_definition.trim()],
        )?;
    }

    current_definition.clear();
    Ok(())
}

fn normalize_inline_definitions(input: &str) -> String {
    let mut result = String::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_ascii_digit() && chars.peek() == Some(&'.') {
            chars.next();
            if !current.trim().is_empty() {
                result.push_str(current.trim());
                result.push('\n');
            }
            current.clear();
            continue;
        }
        current.push(c);
    }

    if !current.trim().is_empty() {
        result.push_str(current.trim());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_basic_insert() {
        let conn = init_db(":memory:").unwrap();

        let data = r#"
Group 1
abound be present in large quantities
"#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", data).unwrap();

        seed_from_file(&conn, file.path().to_str().unwrap()).unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM words", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 1);
    }

    #[test]
    fn test_leading_trailing_spaces() {
        let conn = init_db(":memory:").unwrap();

        let data = r#"
Group 1

   contrite    feeling regretful or guilty   

"#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", data).unwrap();

        seed_from_file(&conn, file.path().to_str().unwrap()).unwrap();

        let word: String = conn
            .query_row("SELECT word FROM words", [], |row| row.get(0))
            .unwrap();

        assert_eq!(word, "contrite");
    }

    #[test]
    fn test_multiple_definitions_numbered() {
        let conn = init_db(":memory:").unwrap();

        let data = r#"
Group 1
austere 1. strict and stern
2. lacking luxury
"#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", data).unwrap();

        seed_from_file(&conn, file.path().to_str().unwrap()).unwrap();

        let definition: String = conn
            .query_row("SELECT definition FROM words", [], |row| row.get(0))
            .unwrap();

        assert_eq!(definition, "strict and stern\nlacking luxury");
    }

    #[test]
    fn test_multiple_definitions_numbered_same_line() {
        let conn = init_db(":memory:").unwrap();

        let data = r#"
Group 1
austere 1. strict and stern 2. lacking luxury
"#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", data).unwrap();

        seed_from_file(&conn, file.path().to_str().unwrap()).unwrap();

        let definition: String = conn
            .query_row("SELECT definition FROM words", [], |row| row.get(0))
            .unwrap();

        assert_eq!(definition, "strict and stern\nlacking luxury");
    }

    #[test]
    fn test_multiple_definitions_braced() {
        let conn = init_db(":memory:").unwrap();

        let data = r#"
Group 1
amenable (of a person) receptive to change; open
(of a thing) responsive to
"#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", data).unwrap();

        seed_from_file(&conn, file.path().to_str().unwrap()).unwrap();

        let definition: String = conn
            .query_row("SELECT definition FROM words", [], |row| row.get(0))
            .unwrap();

        assert_eq!(
            definition,
            "(of a person) receptive to change; open\n(of a thing) responsive to"
        );
    }

    #[test]
    fn test_group_parsing() {
        let conn = init_db(":memory:").unwrap();

        let data = r#"
Group 42
adulterate damage the quality of; corrupt
"#;

        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", data).unwrap();

        seed_from_file(&conn, file.path().to_str().unwrap()).unwrap();

        let group_id: i32 = conn
            .query_row("SELECT group_id FROM words", [], |row| row.get(0))
            .unwrap();

        assert_eq!(group_id, 42);
    }
}
