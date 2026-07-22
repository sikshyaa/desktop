use surrealdb::types::SurrealValue;

#[derive(Debug, SurrealValue)]
pub struct Source {
    path: String,
    pattern: String,
}
