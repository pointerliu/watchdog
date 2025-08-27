//! Composite notifier that can send notifications through multiple channels

use crate::{
    notifiers::{Notifier, Notification},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// A composite notifier that can send notifications through multiple channels
#[derive(Clone)]
pub struct CompositeNotifier<T: Clone> {
    notifiers: Arc<RwLock<HashMap<String, Box<dyn Notifier<T> + Send + Sync>>>>,
}

impl<T: Clone> CompositeNotifier<T> {
    pub fn new() -> Self {
        Self {
            notifiers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a notifier to the composite
    pub async fn add_notifier(&self, name: String, notifier: Box<dyn Notifier<T> + Send + Sync>) {
        let mut notifiers = self.notifiers.write().await;
        notifiers.insert(name, notifier);
    }

    /// Remove a notifier from the composite
    pub async fn remove_notifier(&self, name: &str) -> Option<Box<dyn Notifier<T> + Send + Sync>> {
        let mut notifiers = self.notifiers.write().await;
        notifiers.remove(name)
    }

    /// Get a list of notifier names
    pub async fn list_notifiers(&self) -> Vec<String> {
        let notifiers = self.notifiers.read().await;
        notifiers.keys().cloned().collect()
    }
}

#[async_trait::async_trait]
impl<T: Clone + Send + Sync + 'static> Notifier<T> for CompositeNotifier<T> {
    async fn send(&self, notification: Notification<T>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Get the count of notifiers first
        let notifier_count = {
            let notifiers = self.notifiers.read().await;
            notifiers.len()
        };
        
        if notifier_count == 0 {
            info!("No notifiers configured, skipping notification");
            return Ok(());
        }
        
        info!("Sending notification to {} notifiers", notifier_count);
        
        // Send to all notifiers sequentially (simpler approach to avoid borrowing issues)
        let notifier_names: Vec<String> = {
            let notifiers = self.notifiers.read().await;
            notifiers.keys().cloned().collect()
        };
        
        for name in notifier_names {
            // We'll get and send in separate read operations to avoid holding the lock
            let send_result = {
                let notifiers = self.notifiers.read().await;
                if let Some(notifier) = notifiers.get(&name) {
                    let notification = notification.clone();
                    // We can't clone the notifier, so we'll send directly within the read lock
                    match notifier.send(notification).await {
                        Ok(()) => {
                            info!("Successfully sent notification via {}", name);
                            Ok(())
                        },
                        Err(e) => {
                            tracing::error!("Failed to send notification via {}: {}", name, e);
                            Err(e)
                        }
                    }
                } else {
                    Ok(()) // Notifier not found, but that's OK
                }
            };
            
            // Handle any errors from the send operation
            if let Err(e) = send_result {
                tracing::error!("Error sending via {}: {}", name, e);
            }
        }
        
        Ok(())
    }
}