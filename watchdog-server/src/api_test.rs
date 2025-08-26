#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use watchdog::subscription::Subscription;
    use watchdog_server::{
        api::{CreateSubscriptionRequest, subscription_scope},
        service::StorageSubscriptionService,
    };

    #[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
    struct TestCriteria {
        id: String,
        keyword: String,
    }

    impl TestCriteria {
        fn new(id: String, keyword: String) -> Self {
            Self { id, keyword }
        }
    }

    impl SubscriptionCriteria for TestCriteria {
        type Id = String;
        type Content = String;

        fn matches(&self, content: &Self::Content) -> bool {
            content.contains(&self.keyword)
        }

        fn id(&self) -> &Self::Id {
            &self.id
        }
    }

    #[actix_web::test]
    async fn test_create_subscription() {
        let subscription_service = Arc::new(RwLock::new(StorageSubscriptionService::<TestCriteria>::new()));
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(subscription_service.clone()))
                .service(subscription_scope::<TestCriteria, StorageSubscriptionService<TestCriteria>>())
        ).await;

        let req = test::TestRequest::post()
            .uri("/subscriptions")
            .set_json(&CreateSubscriptionRequest {
                user_id: "test_user".to_string(),
                criteria: TestCriteria::new("test_id".to_string(), "test".to_string()),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}