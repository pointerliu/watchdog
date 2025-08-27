use crate::FetchResult;
use async_trait::async_trait;

#[async_trait]
pub trait FetchStorage<T: Clone> {
    async fn store(&self, result: FetchResult<T>);

    async fn get_all(&self) -> Vec<FetchResult<T>>;

    async fn clear(&self);
}
