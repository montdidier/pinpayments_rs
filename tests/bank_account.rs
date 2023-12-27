use pinpayments::{Client, CreateBankAccount, BankAccount};
use httptest::{Expectation, matchers::*, responders::*};
use surf::http::auth::BasicAuth;

pub mod common;

#[tokio::test]
async fn create_bank_account() {
    let json = common::get_fixture("tests/fixtures/create-bank-account.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("POST", "/1/bank_accounts"),
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

    let bank_account = BankAccount::create(
        &client,
        CreateBankAccount {
            name: "Mr Roland Robot",
            bsb: "123456",
            number: "987654321"
        }
    )
    .await
    .unwrap();

    assert_eq!(bank_account.token, "ba_nytGw7koRg23EEp9NTmz9w");
    assert_eq!(bank_account.name, "Mr Roland Robot");
    assert_eq!(bank_account.bsb, "123456");
    assert_eq!(bank_account.number, "XXXXXX321");
    assert_eq!(bank_account.bank_name.unwrap(), ""); 
    assert_eq!(bank_account.branch.unwrap(), ""); 
}
