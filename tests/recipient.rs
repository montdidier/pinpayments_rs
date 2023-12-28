use pinpayments::{Client, CreateRecipient, Recipient, CreateBankAccount};
use httptest::{Expectation, matchers::*, responders::*};
use surf::http::auth::BasicAuth;
use time::macros::datetime;

pub mod common;

#[tokio::test]
async fn create_recipient() {
    let json = common::get_fixture("tests/fixtures/create-recipient.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("POST", "/1/recipients"),
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

    let recipient = Recipient::create(
        &client,
        CreateRecipient {
            email: "roland@pinpayments.com",
            name: Some("Mr Roland Robot"),
            bank_account: Some(
                CreateBankAccount {
                    name: "Mr Roland Robot",
                    bsb: "123456",
                    number: "987654321"
                }),
            ..Default::default()
        }
    )
    .await
    .unwrap();

    assert_eq!(recipient.token, "rp_a98a4fafROQCOT5PdwLkQ");
    assert_eq!(recipient.name.unwrap(), "Mr Roland Robot");
    assert_eq!(recipient.email, "roland@pinpayments.com");
    assert_eq!(recipient.created_at.unwrap(), datetime!(2012-06-22 6:27:33 UTC));
    assert_eq!(recipient.bank_account.token, "ba_nytGw7koRg23EEp9NTmz9w");
    assert_eq!(recipient.bank_account.name, "Mr Roland Robot");
}

#[tokio::test]
async fn list_recipients_test() {
    let json = common::get_fixture("tests/fixtures/get-recipients.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", "/1/recipients"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json)),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");
    let recipients = Recipient::list(&client, None, None).await.unwrap();

    assert_eq!(recipients.items[0].token, "rp_a98a4fafROQCOT5PdwLkQ");
    assert_eq!(recipients.items[0].name.as_ref().unwrap(), "Mr Roland Robot");
    assert_eq!(recipients.items[0].email, "roland@pinpayments.com");
    assert_eq!(recipients.items[0].created_at.unwrap(), datetime!(2012-06-22 6:27:33 UTC));
}

#[tokio::test]
async fn get_recipient_test() {
    let json = common::get_fixture("tests/fixtures/get-recipient.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let recipient_token = "rp_a98a4fafROQCOT5PdwLkQ".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", format!("/1/recipients/{}", recipient_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json)),
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");
    let recipient = Recipient::retrieve(&client, &recipient_token).await.unwrap();

    assert_eq!(recipient.token, "rp_a98a4fafROQCOT5PdwLkQ");
    assert_eq!(recipient.name.unwrap(), "Mr Roland Robot");
    assert_eq!(recipient.email, "roland@pinpayments.com");
    assert_eq!(recipient.created_at.unwrap(), datetime!(2012-06-22 6:27:33 UTC));
    assert_eq!(recipient.bank_account.token, "ba_nytGw7koRg23EEp9NTmz9w");
    assert_eq!(recipient.bank_account.name, "Mr Roland Robot");
    assert_eq!(recipient.bank_account.bsb, "123456");
    assert_eq!(recipient.bank_account.number, "XXXXXX321");
    assert_eq!(recipient.bank_account.bank_name.unwrap(), "");
    assert_eq!(recipient.bank_account.branch.unwrap(), "");
}
