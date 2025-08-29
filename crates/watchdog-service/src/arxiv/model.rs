use serde::{Deserialize, Serialize};

/// Represents an arXiv paper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArxivPaper {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub authors: Vec<String>,
    pub published: String,
    pub updated: String,
    pub categories: Vec<String>,
    pub link: String,
}

impl std::fmt::Display for ArxivPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} by {}",
            self.title,
            self.summary,
            self.authors.join(", ")
        )
    }
}
