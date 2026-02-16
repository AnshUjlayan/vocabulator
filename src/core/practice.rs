use crate::core::session::Session;
use crate::db::models::Word;

pub fn start_session() -> Session {
    let words = vec![
        Word {
            id: 1,
            word: "ephemeral".into(),
            definition: "lasting for a very short time".into(),
            group_id: 1,
            marked: false,
            last_seen: 4,
            times_seen: 7,
            success_count: 5,
        },
        Word {
            id: 2,
            word: "lucid".into(),
            definition: "expressed clearly; easy to understand".into(),
            group_id: 1,
            marked: true,
            last_seen: 2,
            times_seen: 3,
            success_count: 2,
        },
    ];

    Session {
        words,
        index: 0,
        show_definition: false,
        graded: None,
        input_buffer: String::new(),
        insert_mode: false,
    }
}
