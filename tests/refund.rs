use pinpayments::{Client, Currency, CreateRefund, Refund};
use httptest::{Expectation, matchers::*, responders::*};
use surf::http::auth::BasicAuth;
use time::macros::datetime;

pub mod common;

#[tokio::test]
async fn charge_refund_test() {
    let json = common::get_fixture("tests/fixtures/create-refund.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let charge_token = "ch_bZ3RhJnIUZ8HhfvH8CCvfA".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("POST", format!("/1/charges/{}/refunds", charge_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(
                status_code(201)
                .append_header("Content-Type", "application/json")
                .body(serde_json::to_string(&json).expect("failed to serialize body"))),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let refund = Refund::create(
        &client,
        &charge_token,
        CreateRefund {
            amount: None
        }
    )
    .await
    .unwrap();

    assert_eq!(refund.token, "rf_ERCQy--Ay6o-NKGiUVcKKA");
}

#[tokio::test]
async fn refund_list_test() {
    let json = common::get_fixture("tests/fixtures/get-refunds.json");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(request::method_path("GET", "/1/refunds")).
            respond_with(json_encoded(json)),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");
    let refunds = Refund::list(&client, None, None).await.unwrap();

    assert_eq!(refunds.items[0].token, "rf_ERCQy--Ay6o-NKGiUVcKKA");
    assert_eq!(refunds.items[0].success, None);
    assert_eq!(refunds.items[0].amount, 400);
    assert_eq!(refunds.items[0].currency, Currency::USD);
    assert_eq!(refunds.items[0].charge, "ch_bZ3RhJnIUZ8HhfvH8CCvfA");
    assert_eq!(refunds.items[0].created_at.unwrap(), datetime!(2012-10-27 13:00 UTC));
    assert_eq!(refunds.items[0].error_message, None);
    assert_eq!(refunds.items[0].status_message, "Pending");
}

#[tokio::test]
async fn charge_refunds_test() { 
    let json = common::get_fixture("tests/fixtures/get-refunds.json");
    
    let server = common::SERVER_POOL.get_server();

    let charge_token = "ch_bZ3RhJnIUZ8HhfvH8CCvfA".parse().unwrap();

    server.expect(
        Expectation::matching(request::method_path("GET", format!("/1/charges/{}/refunds", charge_token))).
            respond_with(json_encoded(json)),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");
    let refunds = Refund::list_for_charge(&client, &charge_token, None, None).await.unwrap();

    assert_eq!(refunds.items[0].token, "rf_ERCQy--Ay6o-NKGiUVcKKA");
    assert_eq!(refunds.items[0].success, None);
    assert_eq!(refunds.items[0].amount, 400);
    assert_eq!(refunds.items[0].currency, Currency::USD);
    assert_eq!(refunds.items[0].charge, "ch_bZ3RhJnIUZ8HhfvH8CCvfA");
    assert_eq!(refunds.items[0].created_at.unwrap(), datetime!(2012-10-27 13:00 UTC));
    assert_eq!(refunds.items[0].error_message, None);
    assert_eq!(refunds.items[0].status_message, "Pending");
}

#[tokio::test]
async fn get_refund_test() {
    let json = common::get_fixture("tests/fixtures/get-refund.json");
    
    let server = common::SERVER_POOL.get_server();

    let refund_token = "rf_ERCQy--Ay6o-NKGiUVcKKA".parse().unwrap();

    server.expect(
        Expectation::matching(request::method_path("GET", format!("/1/refunds/{}", refund_token))).
            respond_with(json_encoded(json)),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");
    let refund = Refund::retrieve(&client, &refund_token).await.unwrap();

    assert_eq!(refund.token, "rf_ERCQy--Ay6o-NKGiUVcKKA");
    assert_eq!(refund.success, None);
    assert_eq!(refund.amount, 400);
    assert_eq!(refund.currency, Currency::USD);
    assert_eq!(refund.charge, "ch_bZ3RhJnIUZ8HhfvH8CCvfA");
    assert_eq!(refund.created_at.unwrap(), datetime!(2012-10-27 13:00 UTC));
    assert_eq!(refund.error_message, None);
    assert_eq!(refund.status_message, "Pending");
}
