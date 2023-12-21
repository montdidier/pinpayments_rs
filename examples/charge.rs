use pinpayments::{Client, Charge, ChargeSearchParams, SortByField, SortDirection};
//use time::macros::datetime;

#[tokio::main]
async fn main() { 
    let secret_key = std::env::var("PINPAYMENTS_SECRET_KEY").expect("Missing PINPAYMENTS_SECRET_KEY in env");
    let client = Client::from_url(pinpayments::DEFAULT_TEST_API_BASE_URL, secret_key);

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

//    let search_params = ChargeSearchParams {
//        query: Some("Kruger"),
//        start_date: Some(datetime!(2017-02-24 3:10:49 UTC)),
//        end_date: None,
//        sort_by: Some(SortByField::CreatedAt),
//        direction: Some(SortDirection::Asc),
//        page: None,
//        per_page: None
//    };
//
    let search_params = ChargeSearchParams { ..Default::default() };

    let searched_charges = Charge::search(&client, search_params).await.unwrap();

    println!("Search for charges {searched_charges:?}");

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
