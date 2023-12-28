use pinpayments::{Client, Currency, CreatePlan, Plan, IntervalUnit, CustomerPermission};
use httptest::{Expectation, matchers::*, responders::*};
use surf::http::auth::BasicAuth;
use time::macros::datetime;
use http::StatusCode;

pub mod common;

#[tokio::test]
async fn create_plan_test() {
    let json = common::get_fixture("tests/fixtures/create-plan.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("POST", "/1/plans"),
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

    let plan = Plan::create(
        &client,
        CreatePlan {
            name: "Coffee Plan",
            amount: 1000,
            currency: Currency::USD,
            interval: 30,
            interval_unit: IntervalUnit::Day,
            intervals: Some(6),
            setup_amount: Some(0),
            trial_amount: Some(0),
            trial_interval: Some(7),
            trial_interval_unit: Some(IntervalUnit::Day),
            customer_permissions: Some(vec![CustomerPermission::Cancel]),
            ..Default::default()
         }
    )
    .await
    .unwrap();

    assert_eq!(plan.token, "plan_ZyDee4HNeUHFHC4SpM2idg");
    assert_eq!(plan.name, "Coffee Plan");
    assert_eq!(plan.currency, Currency::USD);
    assert_eq!(plan.setup_amount, 0);
    assert_eq!(plan.trial_amount, 0);
    assert_eq!(plan.interval, 30);
    assert_eq!(plan.interval_unit, IntervalUnit::Day);
    assert_eq!(plan.intervals, 6);
    assert_eq!(plan.trial_interval, 7);
    assert_eq!(plan.trial_interval_unit, IntervalUnit::Day);
    assert_eq!(plan.created_at.unwrap(), datetime!(2023-12-28 5:13:07 UTC));
    assert_eq!(plan.customer_permissions, vec![CustomerPermission::Cancel]);
    assert_eq!(plan.subscription_counts.trial, 0);
    assert_eq!(plan.subscription_counts.active, 0);
    assert_eq!(plan.subscription_counts.cancelling, 0);
    assert_eq!(plan.subscription_counts.cancelled, 0);
}

#[tokio::test]
async fn list_plans_test() {
    let json = common::get_fixture("tests/fixtures/get-plans.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", "/1/plans"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let plans = Plan::list(&client, None, None)
        .await
        .unwrap();

    assert_eq!(plans.items[0].token, "plan_ZyDee4HNeUHFHC4SpM2idg");
    assert_eq!(plans.items[0].name, "Coffee Plan");
    assert_eq!(plans.items[0].amount, 1000);
    assert_eq!(plans.items[0].setup_amount, 0);
    assert_eq!(plans.items[0].trial_amount, 0);
    assert_eq!(plans.items[0].interval, 30);
    assert_eq!(plans.items[0].interval_unit, IntervalUnit::Day);
    assert_eq!(plans.items[0].trial_interval, 7);
    assert_eq!(plans.items[0].trial_interval_unit, IntervalUnit::Day);
    assert_eq!(plans.items[0].created_at.unwrap(), datetime!(2023-12-28 5:31:34 UTC));
}

#[tokio::test]
async fn retrieve_plan_test() {
    let json = common::get_fixture("tests/fixtures/get-plan.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let plan_token = "plan_ZyDee4HNeUHFHC4SpM2idg".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", format!("/1/plans/{}", plan_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let plan = Plan::retrieve(&client, &plan_token).await.unwrap();

    assert_eq!(plan.token, "plan_ZyDee4HNeUHFHC4SpM2idg");
    assert_eq!(plan.name, "Coffee Plan");
    assert_eq!(plan.amount, 1000);
    assert_eq!(plan.setup_amount, 0);
    assert_eq!(plan.trial_amount, 0);
    assert_eq!(plan.interval, 30);
    assert_eq!(plan.interval_unit, IntervalUnit::Day);
    assert_eq!(plan.trial_interval, 7);
    assert_eq!(plan.trial_interval_unit, IntervalUnit::Day);
    assert_eq!(plan.created_at.unwrap(), datetime!(2023-12-28 5:44:36 UTC));
}

#[tokio::test]
async fn delete_plan_test() {
    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let plan_token = "plan_lfUYEBK14zotCTykezJkfg".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("DELETE", format!("/1/plans/{}", plan_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(status_code(StatusCode::NO_CONTENT.into()))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let result = Plan::delete(&client, &plan_token)
        .await
        .unwrap();

    assert_eq!(result, StatusCode::NO_CONTENT);
}
