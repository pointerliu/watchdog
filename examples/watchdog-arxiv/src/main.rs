use crate::arxiv::*;
use actix::prelude::*;
use watchdog::subscription::Subscription;
use watchdog_server::{
    AddSubscriptionMsg, ServerConfig, ShutdownMsg, SubscriptionServer, server::AddUserWorkerMsg
};

// Import arxiv components from the local crate
mod arxiv;

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("Starting arXiv subscription server example...");

    // Create server config
    let config = ServerConfig::default();
    
    // Create and start the multi-user server
    let server = SubscriptionServer::<ArxivFetcher, ArxivNotifier, ArxivCriteria>::new(config);
    let server_addr = server.start();

    // Add a user worker for ml_researcher
    let ml_fetcher = ArxivFetcherBuilder::default()
        .query("machine learning".to_string())
        .number(5)
        .build()
        .unwrap();
    let ml_notifier = ArxivNotifier;
    
    let ml_worker_result = server_addr
        .send(AddUserWorkerMsg {
            user_id: "ml_researcher".to_string(),
            fetcher: ml_fetcher,
            notifier: ml_notifier,
            phantom: std::marker::PhantomData,
        })
        .await?;
    
    if let Ok(ml_worker_addr) = ml_worker_result {
        // Add a subscription for ML papers
        let ml_subscription = Subscription::new(
            "ml_researcher".to_string(),
            ArxivCriteria::new(
                "ml_papers".to_string(),
                vec!["neural".to_string(), "deep learning".to_string()],
            ),
        );
        ml_worker_addr.send(AddSubscriptionMsg { subscription: ml_subscription }).await?;
    }

    // Add a user worker for ai_enthusiast
    let ai_fetcher = ArxivFetcherBuilder::default()
        .query("artificial intelligence".to_string())
        .number(5)
        .build()
        .unwrap();
    let ai_notifier = ArxivNotifier;
    
    let ai_worker_result = server_addr
        .send(AddUserWorkerMsg {
            user_id: "ai_enthusiast".to_string(),
            fetcher: ai_fetcher,
            notifier: ai_notifier,
            phantom: std::marker::PhantomData,
        })
        .await?;
    
    if let Ok(ai_worker_addr) = ai_worker_result {
        // Add another subscription for AI papers
        let ai_subscription = Subscription::new(
            "ai_enthusiast".to_string(),
            ArxivCriteria::new(
                "ai_papers".to_string(),
                vec!["artificial intelligence".to_string(), "llm".to_string()],
            ),
        );
        ai_worker_addr.send(AddSubscriptionMsg { subscription: ai_subscription }).await?;
    }

    println!("Server running with 2 users and their subscriptions. Waiting 5 seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Shutdown the server
    server_addr.send(ShutdownMsg).await?;

    println!("ArXiv subscription server example finished.");

    Ok(())
}