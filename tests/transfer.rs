use pinpayments::{Client, Currency, CreateTransfer, Transfer, TransferSearchParams};
use httptest::{Expectation, matchers::*, responders::*};
use surf::http::auth::BasicAuth;
use time::macros::datetime;

pub mod common;

#[tokio::test]
async fn create_transfer_test() {
    let json = common::get_fixture("tests/fixtures/create-transfer.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("POST", "/1/transfers"),
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

    let transfer = Transfer::create(
            &client,
            CreateTransfer {
                amount: 400,
                currency: Currency::AUD,
                description: "Earings for may",
                recipient: "rp_a98a4fafROQCOT5PdwLkQ".parse().unwrap()
            }
        )
        .await
        .unwrap();

    assert_eq!(transfer.token, "tfer_lfUYEBK14zotCTykezJkfg");
    assert_eq!(transfer.status, "succeeded");
    assert_eq!(transfer.currency, Currency::AUD);
    assert_eq!(transfer.description, "Earnings for may");
    assert_eq!(transfer.amount, 400);
    assert_eq!(transfer.total_debits, 200);
    assert_eq!(transfer.total_credits, 600);
    assert_eq!(transfer.created_at.unwrap(), datetime!(2012-06-20 3:10:49 UTC));
    assert_eq!(transfer.paid_at.unwrap(), datetime!(2012-06-20 3:10:49 UTC));
    assert_eq!(transfer.reference, "Test Business");
    assert_eq!(transfer.line_items_count, 2);
    assert_eq!(transfer.bank_account.token, "ba_nytGw7koRg23EEp9NTmz9w");
    assert_eq!(transfer.bank_account.name, "Mr Roland Robot");
    assert_eq!(transfer.bank_account.bsb, "123456");
    assert_eq!(transfer.bank_account.number, "XXXXXX321");
    assert_eq!(transfer.bank_account.bank_name.unwrap(), "");
    assert_eq!(transfer.bank_account.branch.unwrap(), "");
}


#[tokio::test]
async fn list_transfers_test() {
    let json = common::get_fixture("tests/fixtures/get-transfers.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", "/1/transfers"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(
                status_code(200)
                .append_header("Content-Type", "application/json")
                .body(serde_json::to_string(&json).expect("failed to serialize body"))),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let transfers = Transfer::list(&client, None, None).await.unwrap();

    assert_eq!(transfers.items[0].token, "tfer_lfUYEBK14zotCTykezJkfg");
    assert_eq!(transfers.items[0].status, "succeeded");
    assert_eq!(transfers.items[0].currency, Currency::AUD);
    assert_eq!(transfers.items[0].description, "Earnings for may");
    assert_eq!(transfers.items[0].amount, 400);
    assert_eq!(transfers.items[0].total_debits, 200);
    assert_eq!(transfers.items[0].total_credits, 600);
    assert_eq!(transfers.items[0].created_at.unwrap(), datetime!(2012-06-20 3:10:49 UTC));
    assert_eq!(transfers.items[0].paid_at.unwrap(), datetime!(2012-06-20 3:10:49 UTC));
    assert_eq!(transfers.items[0].reference, "Test Business");
    assert_eq!(transfers.items[0].line_items_count, 2);
    assert_eq!(transfers.items[0].bank_account.token, "ba_nytGw7koRg23EEp9NTmz9w");
    assert_eq!(transfers.items[0].bank_account.name, "Mr Roland Robot");
    assert_eq!(transfers.items[0].bank_account.bsb, "123456");
    assert_eq!(transfers.items[0].bank_account.number, "XXXXXX321");
    assert_eq!(transfers.items[0].bank_account.bank_name.as_ref().unwrap(), "");
    assert_eq!(transfers.items[0].bank_account.branch.as_ref().unwrap(), "");
}

#[tokio::test]
async fn search_transfer_test() {
    let json = common::get_fixture("tests/fixtures/get-transfers.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", "/1/transfers/search"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(
                status_code(200)
                .append_header("Content-Type", "application/json")
                .body(serde_json::to_string(&json).expect("failed to serialize body"))),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let transfers = Transfer::search(
            &client,
            &TransferSearchParams {
                query: Some("Earnings"),
                ..Default::default()
            }
        )
        .await
        .unwrap();

    assert_eq!(transfers.items[0].token, "tfer_lfUYEBK14zotCTykezJkfg");
    assert_eq!(transfers.items[0].status, "succeeded");
    assert_eq!(transfers.items[0].currency, Currency::AUD);
    assert_eq!(transfers.items[0].description, "Earnings for may");
    assert_eq!(transfers.items[0].amount, 400);
    assert_eq!(transfers.items[0].total_debits, 200);
    assert_eq!(transfers.items[0].total_credits, 600);
    assert_eq!(transfers.items[0].created_at.unwrap(), datetime!(2012-06-20 3:10:49 UTC));
    assert_eq!(transfers.items[0].paid_at.unwrap(), datetime!(2012-06-20 3:10:49 UTC));
    assert_eq!(transfers.items[0].reference, "Test Business");
    assert_eq!(transfers.items[0].line_items_count, 2);
    assert_eq!(transfers.items[0].bank_account.token, "ba_nytGw7koRg23EEp9NTmz9w");
    assert_eq!(transfers.items[0].bank_account.name, "Mr Roland Robot");
    assert_eq!(transfers.items[0].bank_account.bsb, "123456");
    assert_eq!(transfers.items[0].bank_account.number, "XXXXXX321");
    assert_eq!(transfers.items[0].bank_account.bank_name.as_ref().unwrap(), "");
    assert_eq!(transfers.items[0].bank_account.branch.as_ref().unwrap(), "");
}

#[tokio::test]
async fn retrieve_transfer_test() {
    let json = common::get_fixture("tests/fixtures/get-transfer.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let transfer_token = "tfer_lfUYEBK14zotCTykezJkfg".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", format!("/1/transfers/{}", transfer_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(
                status_code(200)
                .append_header("Content-Type", "application/json")
                .body(serde_json::to_string(&json).expect("failed to serialize body"))),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let transfer = Transfer::retrieve(&client, &transfer_token).await.unwrap();

    assert_eq!(transfer.token, "tfer_lfUYEBK14zotCTykezJkfg");
    assert_eq!(transfer.status, "succeeded");
    assert_eq!(transfer.currency, Currency::AUD);
    assert_eq!(transfer.description, "Earnings for may");
    assert_eq!(transfer.amount, 400);
    assert_eq!(transfer.total_debits, 200);
    assert_eq!(transfer.total_credits, 600);
    assert_eq!(transfer.created_at.unwrap(), datetime!(2012-06-20 3:10:49 UTC));
    assert_eq!(transfer.paid_at.unwrap(), datetime!(2012-06-20 3:10:49 UTC));
    assert_eq!(transfer.reference, "Test Business");
    assert_eq!(transfer.line_items_count, 2);
    assert_eq!(transfer.bank_account.token, "ba_nytGw7koRg23EEp9NTmz9w");
    assert_eq!(transfer.bank_account.name, "Mr Roland Robot");
    assert_eq!(transfer.bank_account.bsb, "123456");
    assert_eq!(transfer.bank_account.number, "XXXXXX321");
    assert_eq!(transfer.bank_account.bank_name.as_ref().unwrap(), "");
    assert_eq!(transfer.bank_account.branch.as_ref().unwrap(), "");
    assert_eq!(transfer.recipient, "rp_a98a4fafROQCOT5PdwLkQ");
}
