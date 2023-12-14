use pinpayments::{Client, Charge};

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("PINPAYMENTS_SECRET_KEY").expect("Missing PINPAYMENTS_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let mut page_n = 1;
    let per_page = 5;
    loop {
        let page = Charge::list(&client, Some(page_n), Some(per_page))
                        .await
                        .unwrap();
        for charge in page.items {
            println!("* {}", charge.token);
        }

        if page.pagination.next.is_none() { 
            break;
        }
        
        page_n += 1;
        println!("* Next Page!");
    }
}
