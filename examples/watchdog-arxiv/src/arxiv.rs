use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;
use watchdog_core::{
    fetcher::{FetchResult, Fetcher},
    notifier::Notifier,
    SubscriptionCriteria,
};

#[derive(Builder, Clone, Debug)]
pub struct ArxivFetcher {
    #[builder(default = "String::from(\"machine learning\")")]
    query: String,
    #[builder(default = "5")]
    number: i32,
}

impl Default for ArxivFetcher {
    fn default() -> Self {
        Self {
            query: "machine learning".to_string(),
            number: 5,
        }
    }
}

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
        write!(f, "{} - {} by {}", self.title, self.summary, self.authors.join(", "))
    }
}

#[async_trait::async_trait]
impl Fetcher<ArxivPaper> for ArxivFetcher {
    async fn fetch(
        &self,
    ) -> Result<FetchResult<ArxivPaper>, Box<dyn std::error::Error + Send + Sync>> {
        info!("Fetching arXiv papers with query: {}", self.query);

        // Build the arXiv query
        let arxiv_query = self.build_arxiv_query();

        // Fetch papers from arXiv
        let arxivs = arxiv::fetch_arxivs(arxiv_query)
            .await
            .map_err(|e| format!("arxiv::fetch_arxivs error: {:?}", e))?;

        // Convert to our paper format
        let papers: Vec<ArxivPaper> = arxivs
            .into_iter()
            .map(|arxiv_entry| {
                // Extract author names - this might need adjustment based on the actual structure
                let authors: Vec<String> = arxiv_entry
                    .authors
                    .iter()
                    .map(|author| author.clone()) // Assuming authors are already strings
                    .collect();

                ArxivPaper {
                    id: arxiv_entry.id.clone(),
                    title: arxiv_entry.title.clone(),
                    summary: arxiv_entry.summary.clone(),
                    authors,
                    published: arxiv_entry.published.clone(),
                    updated: arxiv_entry.updated.clone(),
                    categories: vec![], // arxiv-rs might not have categories field
                    link: arxiv_entry.id,
                }
            })
            .collect();

        // Create metadata
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "arxiv".to_string());
        metadata.insert("query".to_string(), self.query.clone());
        metadata.insert("total_results".to_string(), papers.len().to_string());

        // For simplicity, we'll return the first paper as the content
        // In a real implementation, you might want to process all papers
        let content = if !papers.is_empty() {
            papers[0].clone()
        } else {
            ArxivPaper {
                id: "no_results".to_string(),
                title: "No papers found".to_string(),
                summary: "No papers matched the query.".to_string(),
                authors: vec![],
                published: chrono::Utc::now().to_rfc3339(),
                updated: chrono::Utc::now().to_rfc3339(),
                categories: vec![],
                link: "".to_string(),
            }
        };

        Ok(FetchResult {
            content,
            metadata,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        })
    }
}

impl ArxivFetcher {
    fn build_arxiv_query(&self) -> arxiv::ArxivQuery {
        arxiv::ArxivQueryBuilder::new()
            .search_query(&Self::query_adaptor(&self.query))
            .start(0)
            .max_results(self.number)
            .sort_by("submittedDate")
            .sort_order("descending")
            .build()
    }

    fn query_adaptor(query: &str) -> String {
        let words: Vec<String> = query.split(" ").map(|s| format!("all:{}", s)).collect();
        words.join("+AND+")
    }
}

// A simple notifier for arXiv papers (console output)
#[derive(Clone)]
pub struct ArxivNotifier;

#[async_trait::async_trait]
impl Notifier<ArxivPaper> for ArxivNotifier {
    async fn send(
        &self,
        notification: watchdog_core::notifier::Notification<ArxivPaper>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "=== New arXiv Paper Alert ===\n\
             To: {}\n\
             Title: {}\n\
             Authors: {}\n\
             Published: {}\n\
             Summary: {}\n\
             Link: {}\n\
             ==============================",
            notification.user_id,
            notification.content.title,
            notification.content.authors.join(", "),
            notification.content.published,
            notification.content.summary,
            notification.content.link
        );
        Ok(())
    }
}

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
                .categories.iter().map(|s| s.to_lowercase())
                .collect::<Vec<_>>()
                .contains(&keyword.to_lowercase())
        })
    }

    fn id(&self) -> &Self::Id {
        &self.id
    }
}