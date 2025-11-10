//! Transactions
//! =============
//! The Transaction route allows to create and manage payments on your integration.

use serde_json::json;

use super::PAYSTACK_BASE_URL;
use crate::{
    CreateSubscriptionRequest, FetchSubscriptionRequest, HttpClient, PaystackAPIError,
    PaystackResult, Response, Subscription, UpdateSubscriptionRequest,
};
use std::sync::Arc;

/// A struct to hold all the functions of the transaction API endpoint
#[derive(Debug, Clone)]
pub struct SubscriptionEndpoints<T: HttpClient + Default> {
    /// Paystack API Key
    key: String,
    /// Base URL for the transaction route
    base_url: String,
    /// Http client for the route
    http: Arc<T>,
}

impl<T: HttpClient + Default> SubscriptionEndpoints<T> {
    /// Creates a new SubscriptionEndpoints instance
    ///
    /// # Arguments
    /// * `key` - The Paystack API key
    /// * `http` - The HTTP client implementation to use for API requests
    ///
    /// # Returns
    /// A new SubscriptionEndpoints instance
    pub fn new(key: Arc<String>, http: Arc<T>) -> SubscriptionEndpoints<T> {
        let base_url = format!("{PAYSTACK_BASE_URL}/subscription");
        SubscriptionEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Create a subscription in your integration
    ///
    /// # Arguments
    /// * `create_subscription_request` - The request data to create the subscription.
    ///   Should be created with a `CreateSubscriptionRequestBuilder` struct
    ///
    /// # Returns
    /// A Result containing the transaction response data or an error
    pub async fn create_subscription(
        &self,
        create_subscription_request: CreateSubscriptionRequest,
    ) -> PaystackResult<Subscription> {
        let url = self.base_url.clone();
        let body = serde_json::to_value(create_subscription_request)
            .map_err(|e| PaystackAPIError::Subscription(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Subscription(e.to_string()))?;

        let parsed_response: Response<Subscription> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Subscription(e.to_string()))?;
        Ok(parsed_response)
    }

    /// List subscriptions
    ///
    /// # Arguments
    /// * `fetch_subscription_request` - The request data to create the subscription.
    ///   Should be created with a `FetchSubscriptionRequestBuilder` struct
    ///
    /// # Returns
    /// A Result containing the subscriptions data or an error
    pub async fn list_subscriptions(
        &self,
        fetch_subscription_request: FetchSubscriptionRequest,
    ) -> PaystackResult<Vec<Subscription>> {
        let (page, per_page, customer, plan) = (
            fetch_subscription_request.page.unwrap_or(1),
            fetch_subscription_request.per_page.unwrap_or(50),
            fetch_subscription_request.customer,
            fetch_subscription_request.plan,
        );

        let mut url = format!("{}?perPage={}&page={}", self.base_url, per_page, page);
        if let Some(customer) = customer {
            url.push_str(&format!("&customer={}", customer));
        }
        if let Some(plan) = plan {
            url.push_str(&format!("&plan={}", plan));
        }

        let response = self
            .http
            .get(&url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::Subscription(e.to_string()))?;

        let parsed_response: Response<Vec<Subscription>> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Subscription(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Gets details of a specific subscription
    ///
    /// # Arguments
    /// * `subscription_id` - The ID or code of the subscription to fetch
    ///
    /// # Returns
    /// A Result containing the subscription data or an error
    pub async fn fetch_subscription(
        &self,
        subscription_id: String,
    ) -> PaystackResult<Subscription> {
        let url = format!("{}/{}", self.base_url, subscription_id);

        let response = self
            .http
            .get(&url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::Subscription(e.to_string()))?;

        let parsed_response: Response<Subscription> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Subscription(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Enable a subscription
    ///
    /// # Arguments
    /// * `update_subscription_request` - the request body for updating a subscription
    ///
    pub async fn enable_subscription(
        &self,
        update_subscription_request: UpdateSubscriptionRequest,
    ) -> PaystackResult<()> {
        let url = format!("{}/enable", self.base_url);
        let body = json!({
            "code": update_subscription_request.code,
            "token": update_subscription_request.token
        });

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Subscription(e.to_string()))?;

        let parsed_response: Response<()> =
            serde_json::from_str(&response).map_err(|e| PaystackAPIError::Charge(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Disable a subscription
    ///
    /// # Arguments
    /// * `update_subscription_request` - the request body for updating a subscription
    ///
    pub async fn disable_subscription(
        &self,
        update_subscription_request: UpdateSubscriptionRequest,
    ) -> PaystackResult<()> {
        let url = format!("{}/disable", self.base_url);
        let body = json!({
            "code": update_subscription_request.code,
            "token": update_subscription_request.token
        });

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Subscription(e.to_string()))?;

        let parsed_response: Response<()> =
            serde_json::from_str(&response).map_err(|e| PaystackAPIError::Charge(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Generate Update Subscription Link
    ///
    /// # Arguments
    /// * `code` - the code of the subscription to enable
    ///
    pub async fn generate_update_subscription_link(&self, code: String) -> PaystackResult<String> {
        let url = format!("{}/{}/manage/link", self.base_url, code);

        let response = self
            .http
            .post(&url, &self.key, &serde_json::Value::Null)
            .await
            .map_err(|e| PaystackAPIError::Subscription(e.to_string()))?;

        let parsed_response: Response<String> =
            serde_json::from_str(&response).map_err(|e| PaystackAPIError::Charge(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Send Update Subscription Link
    ///
    /// # Arguments
    /// * `code` - the code of the subscription to enable
    ///
    pub async fn send_update_subscription_link(&self, code: String) -> PaystackResult<String> {
        let url = format!("{}/{}/manage/email", self.base_url, code);

        let response = self
            .http
            .post(&url, &self.key, &serde_json::Value::Null)
            .await
            .map_err(|e| PaystackAPIError::Subscription(e.to_string()))?;

        let parsed_response: Response<String> =
            serde_json::from_str(&response).map_err(|e| PaystackAPIError::Charge(e.to_string()))?;

        Ok(parsed_response)
    }
}
