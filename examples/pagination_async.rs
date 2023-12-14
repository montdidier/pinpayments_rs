use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use pinpayments::{Client, Charge};
    
#[tokio::main]
async fn main() {
    let secret_key = std::env::var("PINPAYMENTS_SECRET_KEY").expect("Missing PINPAYMENTS_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let stream = Charge::list_with_paginator(&client, Some(5));

    pin_mut!(stream);
    println!("Items (blocking):");

    while let Some(item) = stream.try_next().await.unwrap() {
        println!("* {}", item.token);
    }
}
