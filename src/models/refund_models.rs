//! Refund Models
//! ==============

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Request body for creating a refund.
/// Build with `CreateRefundRequestBuilder`.
#[derive(Clone, Default, Debug, Serialize, Builder)]
pub struct CreateRefundRequest {
    /// Transaction reference or id
    pub transaction: String,
    /// Amount to refund in the subunit of the supported currency.
    /// Defaults to the original transaction amount; cannot exceed it.
    #[builder(setter(strip_option), default)]
    pub amount: Option<u64>,
    /// Currency of the refund
    #[builder(setter(strip_option), default)]
    pub currency: Option<String>,
    /// Customer-facing reason for the refund
    #[builder(setter(strip_option), default)]
    pub customer_note: Option<String>,
    /// Merchant-facing reason for the refund
    #[builder(setter(strip_option), default)]
    pub merchant_note: Option<String>,
}

/// Customer bank account details used when retrying a refund.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct RefundAccountDetails {
    /// The currency of the customer's bank account (must match the payment currency)
    pub currency: String,
    /// The customer's account number
    pub account_number: String,
    /// The bank ID (obtain from the List Banks miscellaneous endpoint)
    pub bank_id: String,
}

/// Request body for retrying a failed refund.
/// Build with `RetryRefundRequestBuilder`.
#[derive(Clone, Default, Debug, Serialize, Builder)]
pub struct RetryRefundRequest {
    /// The customer's bank account details for the retry
    pub refund_account_details: RefundAccountDetails,
}

/// Refund data returned by create, fetch, list, and retry endpoints.
///
/// Note: The `transaction` field is returned as a full transaction object on
/// the create endpoint, but as a plain integer ID on list/fetch. It is
/// represented as `serde_json::Value` to handle both cases.
#[derive(Clone, Debug, Deserialize, Default)]
pub struct RefundData {
    /// Refund ID
    pub id: u64,
    /// Integration ID
    pub integration: Option<u64>,
    /// Domain (`live` or `test`)
    pub domain: Option<String>,
    /// The transaction this refund belongs to.
    /// On create this is a transaction object; on list/fetch it is a numeric ID.
    pub transaction: Option<serde_json::Value>,
    /// Refund amount in the lowest denomination of the currency
    pub amount: u64,
    /// Amount deducted from the integration's balance
    pub deducted_amount: Option<u64>,
    /// Currency of the refund
    pub currency: String,
    /// Refund channel
    pub channel: Option<String>,
    /// Whether the full amount was deducted
    pub fully_deducted: Option<bool>,
    /// Email of the person who initiated the refund
    pub refunded_by: Option<String>,
    /// When the refund was processed
    pub refunded_at: Option<String>,
    /// When the refund is expected to complete
    pub expected_at: Option<String>,
    /// Customer-facing note
    pub customer_note: Option<String>,
    /// Merchant-facing note
    pub merchant_note: Option<String>,
    /// Refund status e.g. `pending`, `processing`, `processed`, `failed`
    pub status: String,
    /// Timestamp when the refund was created.
    /// Accepts both `created_at` (list) and `createdAt` (create) from the API.
    #[serde(alias = "createdAt")]
    pub created_at: Option<String>,
    /// Timestamp when the refund was last updated.
    /// Accepts both `updated_at` (list) and `updatedAt` (create) from the API.
    #[serde(alias = "updatedAt")]
    pub updated_at: Option<String>,
}
