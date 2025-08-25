use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Item not found: {0}")]
    NotFound(String),
}

/// A trait for storing and retrieving data
#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    type Key: Hash + Eq + Clone + Send + Sync + Debug;
    type Value: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync;

    /// Save a value with the given key
    async fn save(&self, key: Self::Key, value: Self::Value) -> Result<(), StorageError>;

    /// Load a value by key
    async fn load(&self, key: &Self::Key) -> Result<Self::Value, StorageError>;

    /// Delete a value by key
    async fn delete(&self, key: &Self::Key) -> Result<(), StorageError>;

    /// List all keys
    async fn list_keys(&self) -> Result<Vec<Self::Key>, StorageError>;
}

/// An in-memory storage implementation for testing
#[derive(Debug)]
pub struct InMemoryStorage<K, V> {
    data: tokio::sync::RwLock<HashMap<K, V>>,
}

impl<K, V> InMemoryStorage<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + Debug,
    V: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            data: tokio::sync::RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl<K, V> Storage for InMemoryStorage<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + Debug,
    V: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync,
{
    type Key = K;
    type Value = V;

    async fn save(&self, key: Self::Key, value: Self::Value) -> Result<(), StorageError> {
        let mut data = self.data.write().await;
        data.insert(key, value);
        Ok(())
    }

    async fn load(&self, key: &Self::Key) -> Result<Self::Value, StorageError> {
        let data = self.data.read().await;
        data.get(key)
            .cloned()
            .ok_or_else(|| StorageError::NotFound(format!("{:?}", key)))
    }

    async fn delete(&self, key: &Self::Key) -> Result<(), StorageError> {
        let mut data = self.data.write().await;
        data.remove(key);
        Ok(())
    }

    async fn list_keys(&self) -> Result<Vec<Self::Key>, StorageError> {
        let data = self.data.read().await;
        Ok(data.keys().cloned().collect())
    }
}

impl<K, V> Default for InMemoryStorage<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + Debug,
    V: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}