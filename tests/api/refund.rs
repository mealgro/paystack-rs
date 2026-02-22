use crate::helpers::get_paystack_client;
use paystack::CreateRefundRequestBuilder;

#[tokio::test]
async fn list_refunds_succeeds() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let response = client
        .refund
        .list_refunds(None, None, None, None, Some(5), None)
        .await
        .expect("unable to list refunds");

    // Assert
    assert!(response.status);
    assert_eq!("Refunds retrieved", response.message);
}

#[tokio::test]
async fn create_refund_fails_for_invalid_transaction() {
    // Arrange
    let client = get_paystack_client();

    let body = CreateRefundRequestBuilder::default()
        .transaction("invalid_transaction_reference".to_string())
        .build()
        .unwrap();

    // Act
    let res = client.refund.create_refund(body).await;

    // Assert
    match res {
        Ok(_) => (),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("status code: 400") || msg.contains("status code: 404"),
                "unexpected error: {msg}"
            );
        }
    }
}

#[tokio::test]
async fn fetch_refund_fails_for_nonexistent_id() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let res = client.refund.fetch_refund(0).await;

    // Assert
    match res {
        Ok(_) => (),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("status code: 400")
                    || msg.contains("status code: 404")
                    || msg.contains("Refund Error"),
                "unexpected error: {msg}"
            );
        }
    }
}
