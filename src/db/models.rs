#[derive(Debug, Clone)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub definition: String,
    pub group_id: i32,
    pub marked: bool,
    pub last_seen: Option<i32>,
    pub times_seen: u8,
    pub success_count: u8,
}
