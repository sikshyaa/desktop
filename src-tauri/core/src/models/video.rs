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

impl Video {
    pub fn new(
        grade: impl Into<String>,
        subject: impl Into<String>,
        topic: impl Into<String>,
        sub_topic: impl Into<String>,
    ) -> Self {
        Self {
            grade: grade.into(),
            subject: subject.into(),
            topic: topic.into(),
            sub_topic: sub_topic.into(),
            teacher_name: None,
            source: None,
        }
    }

    pub fn with_teacher_name(mut self, teacher_name: impl Into<String>) -> Self {
        self.teacher_name = Some(teacher_name.into());
        self
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
}
