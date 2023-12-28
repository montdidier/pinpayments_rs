use pinpayments::{Client, Currency, CreateSubscription, Subscription};
use httptest::{Expectation, matchers::*, responders::*};
use surf::http::auth::BasicAuth;
use time::macros::datetime;
use http::StatusCode;

pub mod common;

#[tokio::test]
async fn create_subscription_test() {
    let json = common::get_fixture("tests/fixtures/create-subscription.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("POST", "/1/subscriptions"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(
                status_code(StatusCode::CREATED.into())
                .append_header("Content-Type", "application/json")
                .body(serde_json::to_string(&json).expect("failed to serialize body"))),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let subscription = Subscription::create(
        &client,
        CreateSubscription {
            plan_token: "plan_ZyDee4HNeUHFHC4SpM2idg".parse().unwrap(),
            customer_token: "cus_XZg1ULpWaROQCOT5PdwLkQ".parse().unwrap(), 
            ..Default::default()
         }
    )
    .await
    .unwrap();

    assert_eq!(subscription.token, "sub_bZWXhTzHooKpk9FZjQfzqQ");
    assert_eq!(subscription.state, "active");
    assert_eq!(subscription.next_billing_date.unwrap(), datetime!(2023-12-28 22:05:8 UTC));
    assert_eq!(subscription.active_interval_started_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.active_interval_finishes_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.cancelled_at, None);
    assert_eq!(subscription.created_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.plan_token, "plan_ZyDee4HNeUHFHC4SpM2idg");
    assert_eq!(subscription.customer_token, "cus_XZg1ULpWaROQCOT5PdwLkQ");
    assert_eq!(subscription.card_token, "card_nytGw7koRg23EEp9NTmz9w");
}

#[tokio::test]
async fn list_subscription_test() {
    let json = common::get_fixture("tests/fixtures/get-subscriptions.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", "/1/subscriptions"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(
                status_code(StatusCode::OK.into())
                .append_header("Content-Type", "application/json")
                .body(serde_json::to_string(&json).expect("failed to serialize body"))),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let subscriptions = Subscription::list(&client, None, None).await.unwrap();

    assert_eq!(subscriptions.items[0].token, "sub_bZWXhTzHooKpk9FZjQfzqQ");
    assert_eq!(subscriptions.items[0].state, "active");
    assert_eq!(subscriptions.items[0].next_billing_date.unwrap(), datetime!(2023-12-28 22:05:8 UTC));
    assert_eq!(subscriptions.items[0].active_interval_started_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscriptions.items[0].active_interval_finishes_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscriptions.items[0].cancelled_at, None);
    assert_eq!(subscriptions.items[0].created_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscriptions.items[0].plan_token, "plan_ZyDee4HNeUHFHC4SpM2idg");
    assert_eq!(subscriptions.items[0].customer_token, "cus_XZg1ULpWaROQCOT5PdwLkQ");
    assert_eq!(subscriptions.items[0].card_token, "card_nytGw7koRg23EEp9NTmz9w");
}

#[tokio::test]
async fn retrieve_subscription_test() {
    let json = common::get_fixture("tests/fixtures/get-subscription.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let subscription_token = "sub_bZWXhTzHooKpk9FZjQfzqQ".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", format!("/1/subscriptions/{}", subscription_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(
                status_code(StatusCode::OK.into())
                .append_header("Content-Type", "application/json")
                .body(serde_json::to_string(&json).expect("failed to serialize body"))),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let subscription = Subscription::retrieve(&client, &subscription_token).await.unwrap();

    assert_eq!(subscription.token, "sub_bZWXhTzHooKpk9FZjQfzqQ");
    assert_eq!(subscription.state, "active");
    assert_eq!(subscription.next_billing_date.unwrap(), datetime!(2023-12-28 22:05:8 UTC));
    assert_eq!(subscription.active_interval_started_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.active_interval_finishes_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.cancelled_at, None);
    assert_eq!(subscription.created_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.plan_token, "plan_ZyDee4HNeUHFHC4SpM2idg");
    assert_eq!(subscription.customer_token, "cus_XZg1ULpWaROQCOT5PdwLkQ");
    assert_eq!(subscription.card_token, "card_nytGw7koRg23EEp9NTmz9w");
}

#[tokio::test]
async fn delete_subscription_test() {
    let json = common::get_fixture("tests/fixtures/delete-subscription.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let subscription_token = "sub_bZWXhTzHooKpk9FZjQfzqQ".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("DELETE", format!("/1/subscriptions/{}", subscription_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(
                status_code(StatusCode::OK.into())
                .append_header("Content-Type", "application/json")
                .body(serde_json::to_string(&json).expect("failed to serialize body"))),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let subscription = Subscription::delete(&client, &subscription_token).await.unwrap();

    assert_eq!(subscription.token, "sub_bZWXhTzHooKpk9FZjQfzqQ");
    assert_eq!(subscription.state, "cancelled");
    assert_eq!(subscription.next_billing_date, None);
    assert_eq!(subscription.active_interval_started_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.active_interval_finishes_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.cancelled_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.created_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.plan_token, "plan_ZyDee4HNeUHFHC4SpM2idg");
    assert_eq!(subscription.customer_token, "cus_XZg1ULpWaROQCOT5PdwLkQ");
    assert_eq!(subscription.card_token, "card_nytGw7koRg23EEp9NTmz9w");
}

#[tokio::test]
async fn reactivate_subscription_test() {
    let json = common::get_fixture("tests/fixtures/reactivate-subscription.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let subscription_token = "sub_bZWXhTzHooKpk9FZjQfzqQ".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("PUT", format!("/1/subscriptions/{}/reactivate", subscription_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(
                status_code(StatusCode::OK.into())
                .append_header("Content-Type", "application/json")
                .body(serde_json::to_string(&json).expect("failed to serialize body"))),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let subscription = Subscription::reactivate(&client, &subscription_token).await.unwrap();

    assert_eq!(subscription.token, "sub_bZWXhTzHooKpk9FZjQfzqQ");
    assert_eq!(subscription.state, "active");
    assert_eq!(subscription.next_billing_date.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.active_interval_started_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.active_interval_finishes_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.cancelled_at, None);
    assert_eq!(subscription.created_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(subscription.plan_token, "plan_ZyDee4HNeUHFHC4SpM2idg");
    assert_eq!(subscription.customer_token, "cus_XZg1ULpWaROQCOT5PdwLkQ");
    assert_eq!(subscription.card_token, "card_nytGw7koRg23EEp9NTmz9w");

}

#[tokio::test]
async fn list_subscription_ledger_entries_test() {
    let json = common::get_fixture("tests/fixtures/list-subscription-ledger-entries.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let subscription_token = "sub_bZWXhTzHooKpk9FZjQfzqQ".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", format!("/1/subscriptions/{}/ledger", subscription_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(
                status_code(StatusCode::OK.into())
                .append_header("Content-Type", "application/json")
                .body(serde_json::to_string(&json).expect("failed to serialize body"))),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let ledger_entries  = Subscription::list_ledger_entries(&client, &subscription_token, None, None).await.unwrap();


    assert_eq!(ledger_entries.items[0].created_at.unwrap(), datetime!(2023-12-28 22:05:08 UTC));
    assert_eq!(ledger_entries.items[0].r#type, "credit");
    assert_eq!(ledger_entries.items[0].amount, 1000);
    assert_eq!(ledger_entries.items[0].currency, Currency::AUD);
    assert_eq!(ledger_entries.items[0].annotation, "charge_credit");

    assert_eq!(ledger_entries.pagination.count, 1);
    assert_eq!(ledger_entries.pagination.per_page, 25);
    assert_eq!(ledger_entries.pagination.current, 1);
}
