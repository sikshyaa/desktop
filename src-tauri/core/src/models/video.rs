use surrealdb::types::{RecordId, SurrealValue};

#[derive(Debug, SurrealValue)]
pub struct Video {
    pub id: Option<RecordId>,
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
            id: None,
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

    pub fn with_id(mut self, id: RecordId) -> Self {
        self.id = Some(id);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Video;
    use surrealdb::types::RecordId;

    #[test]
    fn new_sets_required_fields_and_empty_optional_fields() {
        let video = Video::new("10", "Science", "Light", "Reflection");

        assert!(video.id.is_none());
        assert_eq!(video.grade, "10");
        assert_eq!(video.subject, "Science");
        assert_eq!(video.topic, "Light");
        assert_eq!(video.sub_topic, "Reflection");
        assert!(video.teacher_name.is_none());
        assert!(video.source.is_none());
    }

    #[test]
    fn optional_fields_can_be_added_fluently() {
        let video = Video::new("10", "Science", "Light", "Reflection")
            .with_teacher_name("Ada")
            .with_source("https://example.com/light");

        assert_eq!(video.teacher_name.as_deref(), Some("Ada"));
        assert_eq!(video.source.as_deref(), Some("https://example.com/light"));
    }

    #[test]
    fn with_id_sets_record_id() {
        let id = RecordId::new("video", "light");
        let video = Video::new("10", "Science", "Light", "Reflection").with_id(id);

        assert!(video.id.is_some());
    }

    #[test]
    fn new_accepts_owned_strings() {
        let video = Video::new(
            String::from("10"),
            String::from("Science"),
            String::from("Light"),
            String::from("Reflection"),
        );

        assert_eq!(video.grade, "10");
        assert_eq!(video.subject, "Science");
    }
}
