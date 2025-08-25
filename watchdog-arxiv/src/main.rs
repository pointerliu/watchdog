use actix::prelude::*;
use serde::{Deserialize, Serialize};
use watchdog::subscription::{Subscription, SubscriptionCriteria};
use watchdog_server::{
    AddSubscriptionMsg, RemoveSubscriptionMsg, ServerConfig, ShutdownMsg, SubscriptionServer,
};

// Import arxiv components from the local crate
mod arxiv;
use arxiv::{ArxivFetcher, ArxivFetcherBuilder, ArxivNotifier, ArxivPaper};

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
        })
    }

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("Starting arXiv subscription server example...");

    // Create components
    let config = ServerConfig::default();
    let fetcher = ArxivFetcherBuilder::default()
        .query("machine learning".to_string())
        .number(5)
        .build()
        .unwrap();
    let notifier = ArxivNotifier;

    // Create and start the server
    let server = SubscriptionServer::<ArxivFetcher, ArxivNotifier, ArxivCriteria>::new(
        config, fetcher, notifier,
    );
    let addr = server.start();

    // Add a subscription for ML papers
    let subscription = Subscription::new(
        "ml_researcher".to_string(),
        ArxivCriteria::new(
            "ml_papers".to_string(),
            vec!["neural".to_string(), "deep learning".to_string()],
        ),
    );
    addr.send(AddSubscriptionMsg { subscription }).await?;

    // Add another subscription for AI papers
    let subscription2 = Subscription::new(
        "ai_enthusiast".to_string(),
        ArxivCriteria::new(
            "ai_papers".to_string(),
            vec!["artificial intelligence".to_string(), "llm".to_string()],
        ),
    );
    addr.send(AddSubscriptionMsg {
        subscription: subscription2,
    })
    .await?;

    println!("Server running with 2 subscriptions. Waiting 5 seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Remove one subscription
    addr.send(RemoveSubscriptionMsg {
        id: "ml_papers".to_string(),
    })
    .await?;
    println!("Removed 'ml_papers' subscription");

    println!("Waiting 5 more seconds with 1 subscription...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Shutdown the server
    addr.send(ShutdownMsg).await?;

    println!("ArXiv subscription server example finished.");

    Ok(())
}
