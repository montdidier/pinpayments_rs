use pinpayments::{Client, Currency, Balance};
use httptest::{Expectation, matchers::*, responders::*};
use surf::http::auth::BasicAuth;

pub mod common;

#[tokio::test]
async fn get_balance_test() {
    let json = common::get_fixture("tests/fixtures/get-balance.json");

    println!("{json}");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", "/1/balance"),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let balance = Balance::retrieve(&client)
        .await
        .unwrap();

    assert_eq!(balance.available[0].amount, 400);
    assert_eq!(balance.available[0].currency, Currency::AUD);
    assert_eq!(balance.pending[0].amount, 1200);
    assert_eq!(balance.pending[0].currency, Currency::AUD);
}
