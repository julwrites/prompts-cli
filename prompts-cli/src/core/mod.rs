#[derive(Debug, PartialEq)]
pub struct Prompt {
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
}

impl Prompt {
    pub fn new(content: &str, tags: Option<Vec<String>>, categories: Option<Vec<String>>) -> Self {
        Self {
            content: content.to_string(),
            tags,
            categories,
        }
    }
}