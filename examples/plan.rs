use pinpayments::{Client, Plan};

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("PINPAYMENTS_SECRET_KEY").expect("Missing PINPAYMENTS_SECRET_KEY in env");
    let client = Client::from_url(pinpayments::DEFAULT_TEST_API_BASE_URL, secret_key);

    let plans = match Plan::list(&client, None, None).await {
        Ok(c) => Some(c),
        Err(e) => {
            println!("{e:?}");
            None
        },
    };

    println!("Results are {plans:?}");
}
