//! Example demonstrating how to use the Watchdog system with arXiv fetcher
//!
//! This example shows how to set up and run a complete watchdog system
//! with arXiv fetchers, notifiers, and subscriptions.

use async_trait::async_trait;
use chrono;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::info;
use watchdog_core::fetchers::Fetcher;
use watchdog_core::notifiers::{Notification, Notifier};
use watchdog_core::storage::FetchStorage;
use watchdog_core::subscription::{Subscription, SubscriptionCriteria};
use watchdog_core::{FetchResult, FrameworkError, Watchdog};

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

/// Fetcher for arXiv papers
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

#[async_trait]
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
            .map_err(|e| format!("arxiv::fetch_arxivs error: {e:?}"))?;

        // Convert to our paper format
        let papers: Vec<ArxivPaper> = arxivs
            .into_iter()
            .map(|arxiv_entry| {
                // Extract author names
                let authors: Vec<String> = arxiv_entry.authors.iter().cloned().collect();

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
        let words: Vec<String> = query.split(" ").map(|s| format!("all:{s}")).collect();
        words.join("+AND+")
    }
}

/// A notifier for arXiv papers (console output)
#[derive(Clone)]
pub struct ArxivNotifier;

#[async_trait]
impl Notifier<ArxivPaper> for ArxivNotifier {
    async fn send(
        &self,
        notification: Notification<ArxivPaper>,
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

    fn name(&self) -> &str {
        "arxiv_notifier"
    }

    fn set_name(&mut self, _name: String) {
        // Not implemented for this example
    }
}

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

/// Storage for fetched arXiv data
#[derive(Debug, Clone)]
pub struct FetchedDataStorage<T: Clone> {
    data: Arc<RwLock<Vec<FetchResult<T>>>>,
}

impl<T: Clone> FetchedDataStorage<T> {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait]
impl<T: Clone + Send + Sync> FetchStorage<T> for FetchedDataStorage<T> {
    async fn store(&self, result: FetchResult<T>) {
        let mut data = self.data.write().await;
        data.push(result);
    }

    async fn get_all(&self) -> Vec<FetchResult<T>> {
        let data = self.data.read().await;
        data.clone()
    }

    async fn clear(&self) {
        let mut data = self.data.write().await;
        data.clear();
    }
}

#[actix::main]
async fn main() -> Result<(), FrameworkError> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create storage for fetched data
    let storage = FetchedDataStorage::<ArxivPaper>::new();

    // Create the watchdog system with default configuration
    let watchdog: Watchdog<ArxivPaper, ArxivCriteria, FetchedDataStorage<ArxivPaper>> =
        Watchdog::with_defaults(storage);

    // Start the watchdog system
    println!("Starting watchdog system...");
    watchdog.start()?;

    // Create arXiv fetchers
    let ml_fetcher = ArxivFetcherBuilder::default()
        .query("machine learning".to_string())
        .number(3)
        .build()
        .unwrap();

    let rust_fetcher = ArxivFetcherBuilder::default()
        .query("rust programming".to_string())
        .number(3)
        .build()
        .unwrap();

    // Add fetchers to the watchdog
    watchdog
        .add_fetcher("arxiv_ml".to_string(), Box::new(ml_fetcher))
        .await?;
    watchdog
        .add_fetcher("arxiv_rust".to_string(), Box::new(rust_fetcher))
        .await?;

    // Create notifiers for users
    let notifier1 = Arc::new(ArxivNotifier);
    let notifier2 = Arc::new(ArxivNotifier);

    // Add notifiers to the watchdog
    watchdog
        .add_notifier("researcher1".to_string(), notifier1)
        .await?;
    watchdog
        .add_notifier("researcher2".to_string(), notifier2)
        .await?;

    // Create subscriptions
    let subscription1 = Subscription::new(
        "researcher1".to_string(),
        ArxivCriteria::new("sub1".to_string(), vec!["learning".to_string()]),
    );

    let subscription2 = Subscription::new(
        "researcher2".to_string(),
        ArxivCriteria::new(
            "sub2".to_string(),
            vec!["rust".to_string(), "rust".to_string()],
        ),
    );

    // Add subscriptions to the watchdog
    watchdog.add_subscription(subscription1).await?;
    watchdog.add_subscription(subscription2).await?;

    // Let it run for a while
    println!("Watchdog system running for 60 seconds...");
    tokio::time::sleep(Duration::from_secs(60)).await;

    // Stop the watchdog system
    println!("Stopping watchdog system...");
    watchdog.stop()?;

    println!("Watchdog system finished");
    Ok(())
}
