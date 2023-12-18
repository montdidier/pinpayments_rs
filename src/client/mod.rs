mod pinpayments;

pub mod async_std;

pub(crate) mod config {
    pub(crate) use super::async_std::{err};
    pub use super::async_std::{BaseClient, Response};
}

pub use config::BaseClient;
pub use config::Response;

pub use self::pinpayments::DEFAULT_TEST_API_BASE_URL;
pub use self::pinpayments::Client;
