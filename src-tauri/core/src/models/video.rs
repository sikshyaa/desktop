use surrealdb::types::SurrealValue;

#[derive(Debug, SurrealValue)]
pub struct Video {
    pub grade: String,
    pub subject: String,
    pub topic: String,
    pub sub_topic: String,
    pub teacher_name: Option<String>,
    pub source: Option<String>,
}

pub fn new_video() -> Video {
    Video {
        grade: String::new(),
        subject: String::new(),
        topic: String::new(),
        sub_topic: String::new(),
        teacher_name: None,
        source: None,
    }
}
