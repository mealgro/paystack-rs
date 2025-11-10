use std::fmt;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{Authorization, Domain};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Subscription {
    pub customer: u32,
    pub plan: u32,
    pub integration: u32,
    pub domain: Domain,
    pub start: u32,
    pub status: SubscriptionStatus,
    pub quantity: u32,
    pub amount: u32,
    pub subscription_code: String,
    pub email_token: String,
    pub authorization: Authorization,
    pub easy_cron_id: Option<String>,
    pub cron_expression: String,
    pub next_payment_date: String,
    pub open_invoice: Option<String>,
    pub id: u32,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum SubscriptionStatus {
    #[default]
    Complete,
}

impl fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            SubscriptionStatus::Complete => "complete",
        };
        write!(f, "{status}")
    }
}

/// This struct is used to create a subscription body for creating a subscription using the Paystack API.
/// This struct is built using the `CreateSubscriptionRequestBuilder` struct.
#[derive(Clone, Default, Debug, Serialize, Builder)]
pub struct CreateSubscriptionRequest {
    /// Plan code for the subscription
    pub plan: String,
    /// Customer's email address or code
    pub customer: String,
    // optional parameters from here on
    pub authorization: Option<String>,
    /// start date of the subscription.
    pub start_date: Option<String>,
}

/// This struct is used to create a subscription body for creating a subscription using the Paystack API.
/// This struct is built using the `FetchSubscriptionRequestBuilder` struct.
#[derive(Clone, Default, Debug, Serialize, Builder)]
pub struct FetchSubscriptionRequest {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub customer: Option<u32>,
    pub plan: Option<String>,
}

/// This struct is used to create a subscription body for creating a subscription using the Paystack API.
/// This struct is built using the `UpdateSubscriptionRequestBuilder` struct.
#[derive(Clone, Default, Debug, Serialize, Builder)]
pub struct UpdateSubscriptionRequest {
    pub token: String,
    pub code: String,
}
