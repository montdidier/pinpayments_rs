use pinpayments::{Client, Charge};

#[tokio::main]
async fn main() { 
    let secret_key = std::env::var("PINPAYMENTS_SECRET_KEY").expect("Missing PINPAYMENTS_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let charges = match Charge::list(&client, None, None).await {
        Ok(c) => Some(c),
        Err(e) => {
            println!("{e:?}");
            None
        },
    };

    println!("Results are {charges:?}");

    let token = "ch_VjTEbYBpJF2UkQRLu9IHng".parse().unwrap();
    let charge = match Charge::retrieve(&client, &token).await {
        Ok(c) => Some(c),
        Err(e) => {
            println!("{e:?}");
            None
        },
    };

    println!("Result is {charge:?}");
//    let charge = Charge::create(
//        &client, 
//        CreateCharge {
//            email: String::from("test@test.com"),
//            description: "A fake description",
//            ..Default::default()
//        },
//    )
//    .await
//    .unwrap();

    println!("succesfully called main");
}
