use http_types::{Body, Method, Request, Url};
use serde::{de::DeserializeOwned, Serialize};

use surf::http::auth::BasicAuth;

use crate::{
    client::{BaseClient, Response},
    config::err,
    params::AppInfo,
    Headers, PinError,
};

const USER_AGENT: &str = concat!("PinPayments/1 RustClient/", env!("CARGO_PKG_VERSION"));

pub const DEFAULT_API_BASE_URL: &str = "https://api.pinpayments.com/1/";
pub const DEFAULT_TEST_API_BASE_URL: &str = "https://test-api.pinpayments.com/1/";

#[derive(Clone)]
pub struct Client {
    client: crate::client::BaseClient,
    secret_key: String,
    headers: Headers,
    app_info: Option<AppInfo>,
    api_base: Url
}

impl Client {
    /// Create a new client using the presented secret key.
    pub fn new(secret_key: impl Into<String>) -> Self {
        Self::from_url(DEFAULT_API_BASE_URL, secret_key)
    }

    /// Create a new client making use of the specified URL. Typically used in sandbox and test
    /// scenarios.
    pub fn from_url<'a>(url: impl Into<&'a str>, secret_key: impl Into<String>) -> Self {
        Client {
            client: BaseClient::new(),
            secret_key: secret_key.into(),
            headers: Headers {
                user_agent: USER_AGENT.to_string()
            },
            app_info: None,
            api_base: Url::parse(url.into()).expect("invalid url"),
        }
    }

    /// Set the application info of the client.
    pub fn with_app_info(
        mut self,
        name: String,
        version: Option<String>,
        url: Option<String>,
    ) -> Self {
        let app_info = AppInfo { name, version, url };
        self.headers.user_agent = format!("{} {}", USER_AGENT, app_info.to_string());
        self.app_info = Some(app_info);
        self
    }

    /// Make a http `GET` request using presented path
    pub fn get<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        self.client.execute::<T>(self.create_request(Method::Get, url))
    }

    /// Make a http `GET` request appending presented query parameters
    pub fn get_query<T: DeserializeOwned + Send + 'static, P: Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = match self.url_with_params(path, params) {
            Err(e) => return err(e),
            Ok(ok) => ok,
        };
        self.client.execute::<T>(self.create_request(Method::Get, url))
    }

    /// Make a http `DELETE` request using presented path
    pub fn delete<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        self.client.execute::<T>(self.create_request(Method::Delete, url))
    }

    /// Make a http `DELETE` request using presented query parameters
    pub fn delete_query<T: DeserializeOwned + Send + 'static, P: Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = match self.url_with_params(path, params) {
            Err(e) => return err(e),
            Ok(ok) => ok,
        };
        self.client.execute::<T>(self.create_request(Method::Delete, url))
    }

    /// Make a http `PUT` request using presented path
    pub fn put<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        self.client.execute::<T>(self.create_request(Method::Put, url))
    }

    /// Make a http `POST` request using presented path
    pub fn post<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        self.client.execute::<T>(self.create_request(Method::Post, url))
    }

    /// Make a http `POST` request urlencoding the body
    pub fn post_form<T: DeserializeOwned + Send + 'static, F: Serialize>(
        &self,
        path: &str,
        form: F,
    ) -> Response<T> {
        let url = self.url(path);
        let mut req = self.create_request(Method::Post, url);

        let mut params_buffer = Vec::new();
        let qs_ser = &mut serde_qs::Serializer::new(&mut params_buffer);
        if let Err(qs_ser_err) = serde_path_to_error::serialize(&form, qs_ser) {
            return err(PinError::QueryStringSerialize(qs_ser_err));
        }

        let body = std::str::from_utf8(params_buffer.as_slice())
            .expect("Unable to extract string from params_buffer")
            .to_string();

        req.set_body(Body::from_string(body));

        req.insert_header("content-type", "application/x-www-form-urlencoded");
        self.client.execute::<T>(req)
    }

    fn url(&self, path: &str) -> Url {
        let base = self.api_base.clone();
        let url = base.join(path.trim_start_matches('/')).unwrap();
        url
    }

    fn url_with_params<P: Serialize>(&self, path: &str, params: P) -> Result<Url, PinError> {
        let mut url = self.url(path);

        let mut params_buffer = Vec::new();
        let qs_ser = &mut serde_qs::Serializer::new(&mut params_buffer);
        serde_path_to_error::serialize(&params, qs_ser).map_err(PinError::from)?;

        let params = std::str::from_utf8(params_buffer.as_slice())
            .expect("Unable to extract string from params_buffer")
            .to_string();

        url.set_query(Some(&params));
        Ok(url)
    }

    fn create_request(&self, method: Method, url: Url) -> Request {
        let mut req = Request::new(method, url);
        let auth = BasicAuth::new(&self.secret_key, "");
        req.insert_header(auth.name(), auth.value());

        for (key, value) in self.headers.to_array().iter().filter_map(|(k, v)| v.map(|v| (*k, v))) {
            req.insert_header(key, value);
        }

        req
    }
}


#[cfg(test)]
mod test {
    use super::Client;

    #[test]
    fn user_agent_base() {
        let client = Client::new("sk_test_12345");

        assert_eq!(
            client.headers.user_agent,
            format!("PinPayments/1 RustClient/{}", env!("CARGO_PKG_VERSION"))
        );
    }

    #[test]
    fn user_agent_minimal_app_info() {
        let client =
            Client::new("sk_test_12345").with_app_info("fusillade".to_string(), None, None);

        assert_eq!(
            client.headers.user_agent,
            format!("PinPayments/1 RustClient/{} fusillade", env!("CARGO_PKG_VERSION"))
        );
    }

    #[test]
    fn user_agent_all() {
        let client = Client::new("sk_test_12345").with_app_info(
            "fusillade".to_string(),
            Some("0.1.0".to_string()),
            Some("https://fusillade.app".to_string()),
        );

        assert_eq!(
            client.headers.user_agent,
            format!(
                "PinPayments/1 RustClient/{} fusillade/0.1.0 (https://fusillade.app)",
                env!("CARGO_PKG_VERSION")
            )
        );
    }
}
