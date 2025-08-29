use crate::arxiv::model::ArxivPaper;
use serde::{Deserialize, Serialize};
use watchdog_core::SubscriptionCriteria;

/// Subscription criteria for arXiv papers
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct ArxivCriteria {
    id: String,
    keywords: Vec<String>,
}

impl ArxivCriteria {
    pub fn new(id: String, keywords: Vec<String>) -> Self {
        Self { id, keywords }
    }
}

impl SubscriptionCriteria for ArxivCriteria {
    type Id = String;
    type Content = ArxivPaper;

    fn matches(&self, content: &ArxivPaper) -> bool {
        self.keywords.iter().any(|keyword| {
            content
                .title
                .to_lowercase()
                .contains(&keyword.to_lowercase())
                || content
                    .summary
                    .to_lowercase()
                    .contains(&keyword.to_lowercase())
                || content
                    .categories
                    .iter()
                    .any(|keyword| keyword.to_lowercase().contains(&keyword.to_lowercase()))
        })
    }

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
