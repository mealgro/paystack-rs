//! Refund
//! =======
//! The Refund route allows you to create and manage transaction refunds on your integration.

use super::PAYSTACK_BASE_URL;
use crate::{
    CreateRefundRequest, HttpClient, PaystackAPIError, PaystackResult, RefundData,
    RetryRefundRequest, Response,
};
use std::sync::Arc;

/// A struct to hold all the functions of the refund API endpoint
#[derive(Debug, Clone)]
pub struct RefundEndpoints<T: HttpClient + Default> {
    /// Paystack API Key
    key: String,
    /// Base URL for the refund route
    base_url: String,
    /// Http client for the route
    http: Arc<T>,
}

impl<T: HttpClient + Default> RefundEndpoints<T> {
    /// Creates a new RefundEndpoints instance
    ///
    /// # Arguments
    /// * `key` - The Paystack API key
    /// * `http` - The HTTP client implementation to use for API requests
    ///
    /// # Returns
    /// A new RefundEndpoints instance
    pub fn new(key: Arc<String>, http: Arc<T>) -> RefundEndpoints<T> {
        let base_url = format!("{PAYSTACK_BASE_URL}/refund");
        RefundEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Initiate a refund on your integration
    ///
    /// # Arguments
    /// * `request` - The refund request body. Build with `CreateRefundRequestBuilder`.
    ///
    /// # Returns
    /// A Result containing the refund data or an error
    pub async fn create_refund(
        &self,
        request: CreateRefundRequest,
    ) -> PaystackResult<RefundData> {
        let url = &self.base_url;
        let body = serde_json::to_value(request)
            .map_err(|e| PaystackAPIError::Refund(e.to_string()))?;

        let response = self
            .http
            .post(url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Refund(e.to_string()))?;

        let parsed_response: Response<RefundData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Refund(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Retry a previously failed refund using the customer's bank account details
    ///
    /// # Arguments
    /// * `id` - The refund ID to retry
    /// * `request` - The retry request body. Build with `RetryRefundRequestBuilder`.
    ///
    /// # Returns
    /// A Result containing the updated refund data or an error
    pub async fn retry_refund(
        &self,
        id: u64,
        request: RetryRefundRequest,
    ) -> PaystackResult<RefundData> {
        let url = format!("{}/retry_with_customer_details/{}", self.base_url, id);
        let body = serde_json::to_value(request)
            .map_err(|e| PaystackAPIError::Refund(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Refund(e.to_string()))?;

        let parsed_response: Response<RefundData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Refund(e.to_string()))?;

        Ok(parsed_response)
    }

    /// List refunds available on your integration
    ///
    /// # Arguments
    /// * `transaction` - Optional transaction ID or reference to filter by
    /// * `currency` - Optional currency to filter by
    /// * `from` - Optional start date (ISO 8601)
    /// * `to` - Optional end date (ISO 8601)
    /// * `per_page` - Optional number of records per page
    /// * `page` - Optional page number
    ///
    /// # Returns
    /// A Result containing a list of refund data or an error
    pub async fn list_refunds(
        &self,
        transaction: Option<&str>,
        currency: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
        per_page: Option<u32>,
        page: Option<u32>,
    ) -> PaystackResult<Vec<RefundData>> {
        let url = &self.base_url;

        let per_page_str;
        let page_str;
        let mut query: Vec<(&str, &str)> = Vec::new();

        if let Some(t) = transaction {
            query.push(("transaction", t));
        }
        if let Some(c) = currency {
            query.push(("currency", c));
        }
        if let Some(f) = from {
            query.push(("from", f));
        }
        if let Some(t) = to {
            query.push(("to", t));
        }
        if let Some(p) = per_page {
            per_page_str = p.to_string();
            query.push(("perPage", per_page_str.as_str()));
        }
        if let Some(p) = page {
            page_str = p.to_string();
            query.push(("page", page_str.as_str()));
        }

        let response = self
            .http
            .get(url, &self.key, if query.is_empty() { None } else { Some(&query) })
            .await
            .map_err(|e| PaystackAPIError::Refund(e.to_string()))?;

        let parsed_response: Response<Vec<RefundData>> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Refund(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Get details of a refund on your integration
    ///
    /// # Arguments
    /// * `id` - The refund ID to fetch
    ///
    /// # Returns
    /// A Result containing the refund data or an error
    pub async fn fetch_refund(&self, id: u64) -> PaystackResult<RefundData> {
        let url = format!("{}/{}", self.base_url, id);

        let response = self
            .http
            .get(&url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::Refund(e.to_string()))?;

        let parsed_response: Response<RefundData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Refund(e.to_string()))?;

        Ok(parsed_response)
    }
}
