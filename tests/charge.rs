use pinpayments::{Client, Currency, CreateCharge, Charge, CardParams};
use httptest::{ServerPool, Expectation, matchers::*, responders::*};
use surf::http::auth::BasicAuth;
use time::macros::datetime;

mod common;

static SERVER_POOL: ServerPool = ServerPool::new(2);

#[tokio::test]
async fn charge_create_test() {
    let json = common::get_fixture("tests/fixtures/create-charge.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("POST", "/1/charges"),
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

    let charge = Charge::create(
        &client, 
        CreateCharge {
            amount: 400,
            currency: Some(Currency::AUD),
            description: "test charge",
            email: String::from("roland@pinpayments.com"),
            ip_address: String::from("203.192.1.172"),
            card: Some(
                CardParams {
                    number: "5520000000000000",
                    expiry_month: "05",
                    expiry_year: "2024",
                    cvc: "123",
                    name: "Roland Roboat",
                    address_line1: "42 Sevenoaks St",
                    address_city: "Lathlain",
                    address_postcode: "6454",
                    address_state: "WA",
                    address_country: "Australia",
                    ..Default::default()
                }
            ),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    assert_eq!(charge.token, "ch_lfUYEBK14zotCTykezJkfg");
    assert_eq!(charge.success, true);
    assert_eq!(charge.amount, 400);
    assert_eq!(charge.currency, Currency::AUD);
    assert_eq!(charge.description, "test charge");
    assert_eq!(charge.email, "roland@pinpayments.com");
    assert_eq!(charge.ip_address, "203.192.1.172");
    assert_eq!(charge.created_at.unwrap(), datetime!(2012-06-20 3:10:49 UTC));
    assert_eq!(charge.status_message, "Success");
    assert!(charge.error_message.is_none());
    assert_eq!(charge.amount_refunded, 0);
    assert_eq!(charge.total_fees.unwrap(), 42);
    assert_eq!(charge.merchant_entitlement.unwrap(), 358);
    assert_eq!(charge.refund_pending, false);


    assert_eq!(charge.card.token, "card_pIQJKMs93GsCc9vLSLevbw");
    assert_eq!(charge.card.scheme, "master");
    assert_eq!(charge.card.display_number, "XXXX-XXXX-XXXX-0000");
    assert_eq!(charge.card.issuing_country, "US");
    assert_eq!(charge.card.expiry_month, 5);
    assert_eq!(charge.card.expiry_year, 2024);
    assert_eq!(charge.card.name, "Roland Robot");
}

#[tokio::test]
async fn get_charge_test() {
    let json = common::get_fixture("tests/fixtures/get-charge.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", "/1/charges/ch_lfUYEBK14zotCTykezJkfg"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json)),
    );

    let token = "ch_lfUYEBK14zotCTykezJkfg".parse().unwrap();

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");
    let charge = Charge::retrieve(&client, &token).await.unwrap();

    assert_eq!(charge.token, "ch_lfUYEBK14zotCTykezJkfg");
}

#[tokio::test]
async fn charge_void_test() {
    let json = common::get_fixture("tests/fixtures/charge-void.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("PUT", "/1/charges/ch_lfUYEBK14zotCTykezJkfg/void"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json)),
    );

    let token = "ch_lfUYEBK14zotCTykezJkfg".parse().unwrap();

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");
    let charge = Charge::void(&client, &token).await.unwrap();

    assert_eq!(charge.token, "ch_lfUYEBK14zotCTykezJkfg");
    assert_eq!(charge.status_message, "Authorisation Voided");
    assert_eq!(charge.authorisation_voided, true);
}

#[tokio::test]
async fn charge_capture_test() {
    let json = common::get_fixture("tests/fixtures/charge-capture.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("PUT", "/1/charges/ch_lfUYEBK14zotCTykezJkfg/capture"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json)),
    );

    let token = "ch_lfUYEBK14zotCTykezJkfg".parse().unwrap();

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");
    let charge = Charge::capture(&client, &token).await.unwrap();

    assert_eq!(charge.token, "ch_lfUYEBK14zotCTykezJkfg");
    assert_eq!(charge.status_message, "Success");
    assert_eq!(charge.captured, true);
    if let Some(cap) = charge.captured_at {
        assert_eq!(cap, datetime!(2012-06-20 3:10:49 UTC));
    }
}

#[tokio::test]
async fn charge_list_test() {
    let json = common::get_fixture("tests/fixtures/get-charges.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", "/1/charges"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");
    let charges = Charge::list(&client, None, None).await.unwrap();

    assert_eq!(charges.items[0].token, "ch_lfUYEBK14zotCTykezJkfg");
    assert_eq!(charges.items[0].status_message, "Success");
}

#[tokio::test]
async fn charge_verify_test() {
    let json = common::get_fixture("tests/fixtures/charge-verify.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", "/1/charges/verify"),
                request::query(url_decoded(contains(("session_token", "se_sGt9PuNYfVzJqTSLP2CV8g")))),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json)),
    );

    let session_token = "se_sGt9PuNYfVzJqTSLP2CV8g".parse().unwrap();

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");
    let charge = Charge::verify(&client, &session_token).await.unwrap();

    assert_eq!(charge.success, true);
    assert_eq!(charge.token, "ch_lfUYEBK14zotCTykezJkfg");
}
