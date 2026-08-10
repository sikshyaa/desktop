use surrealdb::types::{RecordId, SurrealValue};

#[derive(Debug, SurrealValue)]
pub struct Source {
    pub id: Option<RecordId>,
    pub path: String,
    pub pattern: String,
}
pub const SOURCE_TABLENAME: &str = "source";

impl Source {
    pub fn new(path: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self {
            id: None,
            path: path.into(),
            pattern: pattern.into(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

#[cfg(test)]
mod tests {
    use super::Source;

    #[test]
    fn new_stores_path_and_pattern() {
        let source = Source::new("videos/", "**/*.mp4");

        assert_eq!(source.path(), "videos/");
        assert_eq!(source.pattern(), "**/*.mp4");
    }

    #[test]
    fn new_accepts_owned_strings() {
        let source = Source::new(String::from("videos/"), String::from("*.mp4"));

        assert_eq!(source.path(), "videos/");
        assert_eq!(source.pattern(), "*.mp4");
    }
}
