use serde::{Deserialize, Serialize};
use subscription_framework::{
    fetcher::{Fetcher, FetchResult},
    notifier::{Notifier, Notification, ConsoleNotifier},
    subscription::{Subscription, SubscriptionCriteria, SubscriptionManager},
    FrameworkError,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct PaperCriteria {
    id: String,
    keywords: Vec<String>,
}

impl PaperCriteria {
    pub fn new(id: String, keywords: Vec<String>) -> Self {
        Self { id, keywords }
    }
}

impl SubscriptionCriteria for PaperCriteria {
    type Id = String;
    type Content = String;

    fn matches(&self, content: &String) -> bool {
        self.keywords
            .iter()
            .any(|keyword| content.to_lowercase().contains(&keyword.to_lowercase()))
    }

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

pub struct ArxivFetcher {
    keywords: Vec<String>,
}

impl ArxivFetcher {
    pub fn new(keywords: Vec<String>) -> Self {
        Self { keywords }
    }
}

#[async_trait::async_trait]
impl Fetcher<String> for ArxivFetcher {
    async fn fetch(&self) -> Result<FetchResult<String>, Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, this would fetch from the arXiv API
        // For this example, we'll simulate some paper data
        
        let _search_query = self.keywords.join("+OR+");
        let _url = format!(
            "http://export.arxiv.org/api/query?search_query=all:{}&max_results=5",
            _search_query
        );
        
        // Simulate fetching data
        let content = format!("Simulated paper data for keywords: {}", self.keywords.join(", "));
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "arxiv".to_string());
        metadata.insert("query".to_string(), _search_query);
        
        Ok(FetchResult {
            content,
            metadata,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        })
    }
}

async fn run_subscription_system() -> Result<(), FrameworkError> {
    // Initialize components
    let mut subscription_manager = SubscriptionManager::<PaperCriteria>::new();
    let notifier = ConsoleNotifier;
    
    // Add some sample subscriptions
    let ai_subscription = Subscription::new(
        "user1".to_string(),
        PaperCriteria::new("ai_papers".to_string(), vec!["neural".to_string(), "machine learning".to_string()])
    );
    
    let crypto_subscription = Subscription::new(
        "user2".to_string(),
        PaperCriteria::new("crypto_papers".to_string(), vec!["cryptography".to_string(), "blockchain".to_string()])
    );
    
    subscription_manager.add_subscription(ai_subscription);
    subscription_manager.add_subscription(crypto_subscription);
    
    // Process subscriptions
    for subscription in subscription_manager.get_subscriptions().values() {
        println!("Processing subscription for user: {}", subscription.user_id);
        
        let fetcher = ArxivFetcher::new(subscription.criteria.keywords.clone());
        match fetcher.fetch().await {
            Ok(result) => {
                if subscription.criteria.matches(&result.content) {
                    let notification = Notification {
                        user_id: subscription.user_id.clone(),
                        title: "New papers available!".to_string(),
                        content: result.content,
                        timestamp: result.timestamp,
                    };
                    
                    if let Err(e) = notifier.send(notification).await {
                        eprintln!("Failed to send notification: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch data: {}", e);
            }
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), FrameworkError> {
    tracing_subscriber::fmt::init();
    
    println!("Starting paper subscriber system...");
    run_subscription_system().await?;
    println!("Paper subscriber system finished.");
    
    Ok(())
}