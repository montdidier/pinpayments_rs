use pinpayments::{Client, CreateCustomer, Customer, CardParams};
use std::fs::File;
use httptest::{ServerPool, Expectation, matchers::*, responders::*};
use surf::http::auth::BasicAuth;
use time::macros::datetime;

static SERVER_POOL: ServerPool = ServerPool::new(2);

fn get_fixture(path: &str) -> serde_json::Value {
    let file = File::open(path)
        .expect("file should open read only");
    serde_json::from_reader(file).expect("file should be JSON")
}

#[tokio::test]
async fn customer_create_test() {
    let json = get_fixture("tests/fixtures/create-customer.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("POST", "/1/customers"),
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
}
