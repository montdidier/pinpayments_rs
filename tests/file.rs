use pinpayments::{Client, File};
use httptest::{Expectation, matchers::*, responders::*};
use surf::http::auth::BasicAuth;
use time::macros::datetime;
use http::StatusCode;

pub mod common;

#[tokio::test]
async fn retrieve_file_test() {
    let json = common::get_fixture("tests/fixtures/get-file.json");

    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let file_token = "file_lfUYEBK14zotCTykezJkfg".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("GET", format!("/1/files/{}", file_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(json_encoded(json))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let file = File::retrieve(&client, &file_token).await.unwrap();

    assert_eq!(file.token, "file_lfUYEBK14zotCTykezJkfg");
    assert_eq!(file.original_filename, "cat.jpeg");
    assert_eq!(file.presigned_url, "https://pin-gateway-api-files-production-sandbox.s3.ap-southeast-2.amazonaws.com/sample.jpg");
    assert_eq!(file.presigned_url_expires_at.unwrap(), datetime!(2023-09-19 7:49:26 UTC));
    assert_eq!(file.purpose, "dispute_evidence");
    assert_eq!(file.size, 8060);
    assert_eq!(file.mime_type, "image/jpeg");
    assert_eq!(file.uploaded_at.unwrap(), datetime!(2023-09-19 6:47:54 UTC));
}

#[tokio::test]
async fn delete_file_test() {
    let auth = BasicAuth::new("sk_test_12345", "");

    let server = common::SERVER_POOL.get_server();

    let file_token = "file_lfUYEBK14zotCTykezJkfg".parse().unwrap();

    server.expect(
        Expectation::matching(
            all_of![
                request::method_path("DELETE", format!("/1/files/{}", file_token)),
                request::headers(
                    contains((String::from(auth.name().as_str()), String::from(auth.value().as_str())))
                ),
            ]).
            respond_with(status_code(StatusCode::OK.into()))
    );

    let client = Client::from_url(server.url_str("/1/").as_str(), "sk_test_12345");

    let result = File::delete(&client, &file_token).await.unwrap();

    assert_eq!(result, StatusCode::OK);
}
