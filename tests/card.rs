use pinpayments::{Client, CardParams, Card};
use httptest::{Expectation, matchers::*, responders::*};
use surf::http::auth::BasicAuth;

pub mod common;

#[tokio::test]
async fn card_create_test() {
    let json = common::get_fixture("tests/fixtures/create-card.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("POST", "/1/cards"),
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

    let card = Card::create(
        &client, 
        CardParams {
            number: "5520000000000000",
            expiry_month: "05",
            expiry_year: "2024",
            cvc: "123",
            name: "Roland Roboat",
            address_line1: "42 Sevenoaks St",
            address_line2: None,
            address_city: "Lathlain",
            address_postcode: "6454",
            address_state: "WA",
            address_country: "Australia",
        }
    )
    .await
    .unwrap();

    assert_eq!(card.token, "card_pIQJKMs93GsCc9vLSLevbw");
    assert_eq!(card.scheme, "master");
    assert_eq!(card.display_number, "XXXX-XXXX-XXXX-0000");
    assert_eq!(card.issuing_country, "US");
    assert_eq!(card.expiry_month, 5);
    assert_eq!(card.expiry_year, 2024);
    assert_eq!(card.name, "Roland Robot");
    assert_eq!(card.address_line1, "42 Sevenoaks St");
    assert_eq!(card.address_line2.unwrap(), "");
    assert_eq!(card.address_city, "Lathlain");
    assert_eq!(card.address_postcode.unwrap(), "6454");
    assert_eq!(card.address_state.unwrap(), "WA");
    assert_eq!(card.address_country, "Australia");
    assert_eq!(card.network_type, None);
    assert_eq!(card.network_format, None);
    assert_eq!(card.customer_token, None);
    assert_eq!(card.primary, None);
}
