use crate::helpers::get_paystack_client;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use paystack::{CreateCustomerRequestBuilder, CreateSubscriptionRequestBuilder};

#[tokio::test]
async fn create_subscription_valid() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let email: String = SafeEmail().fake();

    let response = client
        .customers
        .create_customer(
            CreateCustomerRequestBuilder::default()
                .email(email)
                .build()
                .expect("Error creating customer request"),
        )
        .await
        .expect("Error creating customer for subscription");
    assert!(response.status);

    let customer = response.data.unwrap();
    let res = client
        .subscription
        .create_subscription(
            CreateSubscriptionRequestBuilder::default()
                .customer(customer.customer_code.clone())
                .build()
                .expect("Error creating subscription request"),
        )
        .await
        .expect("unable to create transaction");

    // Assert
    assert!(res.status);
    assert_eq!("Subscription successfully created", res.message);
}
