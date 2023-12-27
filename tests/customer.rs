use pinpayments::{Client, Currency, CreateCustomer, Customer, CardParams};
use httptest::{Expectation, matchers::*, responders::*};
use surf::http::auth::BasicAuth;
use time::macros::datetime;
use http::StatusCode;

pub mod common;

#[tokio::test]
async fn customer_create_test() {
    let json = common::get_fixture("tests/fixtures/create-customer.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("POST", "/1/customers"),
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

    let customer = Customer::create(
        &client,
        CreateCustomer {
            email: "roland@pinpayments.com",
            first_name: Some("Roland"),
            last_name: Some("Robot"),
            phone_number: Some("1300 364 800"),
            company: Some("Pin Payments"),
            notes: Some("Account managers at Pin Payments"),
            card: Some(
                CardParams {
                    number: "5520000000000000",
                    expiry_month: "05",
                    expiry_year: "2024",
                    cvc: "123",
                    name: "Roland Robot",
                    address_line1: "42 Severnoaks St",
                    address_city: "Lathlain",
                    address_postcode: "6454",
                    address_state: "WA",
                    address_country: "Australia",
                    ..Default::default()
            }),
            ..Default::default()
        }
    )
    .await
    .unwrap();

    assert_eq!(customer.token, "cus_XZg1ULpWaROQCOT5PdwLkQ");
    assert_eq!(customer.email, "roland@pinpayments.com");
    assert_eq!(customer.first_name.unwrap(), "Roland");
    assert_eq!(customer.last_name.unwrap(), "Robot");
    assert_eq!(customer.phone_number.unwrap(), "1300 364 800");
    assert_eq!(customer.company.unwrap(), "Pin Payments");
    assert_eq!(customer.notes.unwrap(), "Account manager at Pin Payments");
    assert_eq!(customer.created_at.unwrap(), datetime!(2012-06-22 6:27:33 UTC));
    assert_eq!(customer.card.token, "card_nytGw7koRg23EEp9NTmz9w");
    assert_eq!(customer.card.display_number, "XXXX-XXXX-XXXX-0000");
}

#[tokio::test]
async fn customer_retrieve_test() {
    let json = common::get_fixture("tests/fixtures/get-customer.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let customer_token = "cus_XZg1ULpWaROQCOT5PdwLkQ".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", format!("/1/customers/{}", customer_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let customer = Customer::retrieve(&client, &customer_token)
        .await
        .unwrap();

    assert_eq!(customer.token, "cus_XZg1ULpWaROQCOT5PdwLkQ");
    assert_eq!(customer.email, "roland@pinpayments.com");
    assert_eq!(customer.first_name.unwrap(), "Roland");
    assert_eq!(customer.last_name.unwrap(), "Robot");
    assert_eq!(customer.phone_number.unwrap(), "1300 364 800");
    assert_eq!(customer.company.unwrap(), "Pin Payments");
    assert_eq!(customer.notes.unwrap(), "Account manager at Pin Payments");
    assert_eq!(customer.created_at.unwrap(), datetime!(2012-06-22 6:27:33 UTC));
    assert_eq!(customer.card.token, "card_nytGw7koRg23EEp9NTmz9w");
    assert_eq!(customer.card.display_number, "XXXX-XXXX-XXXX-0000");
}

#[tokio::test]
async fn customer_list_test() {
    let json = common::get_fixture("tests/fixtures/get-customers.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", "/1/customers"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let customers = Customer::list(&client, None, None)
        .await
        .unwrap();

    assert_eq!(customers.items[0].token, "cus_XZg1ULpWaROQCOT5PdwLkQ");
    assert_eq!(customers.items[0].email, "roland@pinpayments.com");
    assert_eq!(customers.items[0].first_name.as_ref().unwrap(), "Roland");
    assert_eq!(customers.items[0].last_name.as_ref().unwrap(), "Robot");
    assert_eq!(customers.items[0].phone_number.as_ref().unwrap(), "1300 364 800");
    assert_eq!(customers.items[0].company.as_ref().unwrap(), "Pin Payments");
    assert_eq!(customers.items[0].notes.as_ref().unwrap(), "Account manager at Pin Payments");
    assert_eq!(customers.items[0].created_at.unwrap(), datetime!(2012-06-22 6:27:33 UTC));
}

#[tokio::test]
async fn customer_delete_test() {
    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let customer_token = "cus_XZg1ULpWaROQCOT5PdwLkQ".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("DELETE", format!("/1/customers/{}", customer_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(status_code(StatusCode::NO_CONTENT.into()))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let result = Customer::delete(&client, &customer_token)
        .await
        .unwrap();

    assert_eq!(result, StatusCode::NO_CONTENT);
}


#[tokio::test]
async fn customer_list_charges_test() {
    let json = common::get_fixture("tests/fixtures/get-customer-charges.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let customer_token = "cus_XZg1ULpWaROQCOT5PdwLkQ".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", format!("/1/customers/{}/charges", customer_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let charges = Customer::list_charges(&client, &customer_token, None, None)
        .await
        .unwrap();

    assert_eq!(charges.items[0].token, "ch_lfUYEBK14zotCTykezJkfg");
    assert_eq!(charges.items[0].success, true);
    assert_eq!(charges.items[0].amount, 400);
    assert_eq!(charges.items[0].currency, Currency::AUD);
    assert_eq!(charges.items[0].description, "test charge");
    assert_eq!(charges.items[0].email, "roland@pinpayments.com");
    assert_eq!(charges.items[0].ip_address, "203.192.1.172");
}

#[tokio::test]
async fn customer_list_cards_test() {
    let json = common::get_fixture("tests/fixtures/get-customer-cards.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let customer_token = "cus_XZg1ULpWaROQCOT5PdwLkQ".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", format!("/1/customers/{}/cards", customer_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let cards = Customer::list_cards(&client, &customer_token, None, None)
        .await
        .unwrap();

    assert_eq!(cards.items[0].token, "card_nytGw7koRg23EEp9NTmz9w");
    assert_eq!(cards.items[0].scheme, "master");
    assert_eq!(cards.items[1].token, "card_ZFThCjFi7wCNkopytxQVKA");
    assert_eq!(cards.items[1].scheme, "master");
    assert_eq!(cards.pagination.count, 2);
}

#[tokio::test]
async fn customer_create_card_test() {
    let json = common::get_fixture("tests/fixtures/create-customer-card.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let customer_token = "cus_XZg1ULpWaROQCOT5PdwLkQ".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("POST", format!("/1/customers/{}/cards", customer_token)),
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

    let card = Customer::create_card(
        &client,
        &customer_token,
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
    )
    .await
    .unwrap();

    assert_eq!(card.token, "card_ZFThCjFi7wCNkopytxQVKA");
    assert_eq!(card.scheme, "master");
}
