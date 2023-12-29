use pinpayments::{Client, Currency, Dispute, DisputeSearchParams};
use httptest::{Expectation, matchers::*, responders::*};
use surf::http::auth::BasicAuth;
use time::macros::datetime;
use http::StatusCode;

pub mod common;

#[tokio::test]
async fn list_dispute_test() {
    let json = common::get_fixture("tests/fixtures/get-disputes.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", "/1/disputes"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let disputes = Dispute::list(&client, None, None).await.unwrap();

    assert_eq!(disputes.items[0].token, "dis_JRs6Xgk4jMyF33yGijQ7Nw");
    assert_eq!(disputes.items[0].category, "general");
    assert_eq!(disputes.items[0].status, "evidence_required");
    assert_eq!(disputes.items[0].amount, 100);
    assert_eq!(disputes.items[0].currency, Currency::AUD);
    assert_eq!(disputes.items[0].evidence_required_by.unwrap(), datetime!(2023-10-15 00:00:00 UTC));
    assert_eq!(disputes.items[0].relevant_evidence, vec![ 
        "proof_of_delivery_or_service",
        "invoice_or_receipt",
        "invoice_showing_distinct_transactions",
        "customer_communication",
        "refund_or_cancellation_policy",
        "recurring_transaction_agreement",
        "additional_evidence" 
    ]);
    assert_eq!(disputes.items[0].received_at.unwrap(), datetime!(2023-09-25 9:23:58 UTC));

    assert_eq!(disputes.items[0].charge.token, "ch_yJM0U_NaAsyY2A7Se3IFYQ");
}

#[tokio::test]
async fn search_dispute_test() {
    let json = common::get_fixture("tests/fixtures/get-disputes.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", "/1/disputes/search"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let disputes = Dispute::search(
        &client, 
        DisputeSearchParams {
            query: Some("evidence_required"),
            ..Default::default()
        }
    )
    .await
    .unwrap();

    assert_eq!(disputes.items[0].token, "dis_JRs6Xgk4jMyF33yGijQ7Nw");
    assert_eq!(disputes.items[0].category, "general");
    assert_eq!(disputes.items[0].status, "evidence_required");
    assert_eq!(disputes.items[0].amount, 100);
    assert_eq!(disputes.items[0].currency, Currency::AUD);
    assert_eq!(disputes.items[0].evidence_required_by.unwrap(), datetime!(2023-10-15 00:00:00 UTC));
    assert_eq!(disputes.items[0].relevant_evidence, vec![ 
        "proof_of_delivery_or_service",
        "invoice_or_receipt",
        "invoice_showing_distinct_transactions",
        "customer_communication",
        "refund_or_cancellation_policy",
        "recurring_transaction_agreement",
        "additional_evidence" 
    ]);
    assert_eq!(disputes.items[0].received_at.unwrap(), datetime!(2023-09-25 9:23:58 UTC));

    assert_eq!(disputes.items[0].charge.token, "ch_yJM0U_NaAsyY2A7Se3IFYQ");
}

#[tokio::test]
async fn retrieve_dispute_test() {
    let json = common::get_fixture("tests/fixtures/get-dispute.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let dispute_token = "dis_JRs6Xgk4jMyF33yGijQ7Nw".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", format!("/1/disputes/{}", dispute_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let dispute = Dispute::retrieve(&client, &dispute_token).await.unwrap();

    assert_eq!(dispute.token, "dis_JRs6Xgk4jMyF33yGijQ7Nw");
    assert_eq!(dispute.category, "general");
    assert_eq!(dispute.status, "evidence_required");
    assert_eq!(dispute.amount, 100);
    assert_eq!(dispute.currency, Currency::AUD);
    assert_eq!(dispute.evidence_required_by.unwrap(), datetime!(2023-10-15 00:00:00 UTC));
    assert_eq!(dispute.relevant_evidence, vec![ 
        "proof_of_delivery_or_service",
        "invoice_or_receipt",
        "invoice_showing_distinct_transactions",
        "customer_communication",
        "refund_or_cancellation_policy",
        "recurring_transaction_agreement",
        "additional_evidence" 
    ]);
    assert_eq!(dispute.received_at.unwrap(), datetime!(2023-09-25 9:23:58 UTC));

    assert_eq!(dispute.charge.token, "ch_yJM0U_NaAsyY2A7Se3IFYQ");
}

#[tokio::test]
async fn submit_evidence_dispute_test() {
    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let dispute_token = "dis_JRs6Xgk4jMyF33yGijQ7Nw".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("POST", format!("/1/disputes/{}/evidence", dispute_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(status_code(StatusCode::OK.into()))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let result = Dispute::submit_evidence(&client, &dispute_token).await.unwrap();

    assert_eq!(result, StatusCode::OK);
}

#[tokio::test]
async fn accept_dispute_test() {
    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let dispute_token = "dis_JRs6Xgk4jMyF33yGijQ7Nw".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("POST", format!("/1/disputes/{}/accept", dispute_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(status_code(StatusCode::OK.into()))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let result = Dispute::accept(&client, &dispute_token).await.unwrap();

    assert_eq!(result, StatusCode::OK);
}
