use std::future::{self};
use futures::future::BoxFuture;

use http_types::{Request};
use serde::de::DeserializeOwned;

use crate::error::{ErrorResponse, PinError};

pub type Response<T> = BoxFuture<'static, Result<T, PinError>>;

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn ok<T: Send + 'static>(ok: T) -> Response<T> {
    Box::pin(future::ready(Ok(ok)))
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn err<T: Send + 'static>(err: PinError) -> Response<T> {
    Box::pin(future::ready(Err(err)))
}

#[derive(Clone, Debug)]
pub struct BaseClient {
    client: surf::Client,
}

impl BaseClient {
    pub fn new() -> Self {
        Self { client: surf::Client::new() }
    }

    pub fn execute<T: DeserializeOwned + Send + 'static>(
        &self,
        request: Request,
    ) -> Response<T> {
        // As the client could be used across threads it is cloned.
        // The client is send sync and cloned clients share the same pool.
        let client = self.client.clone();

        Box::pin(async move {
            let bytes = send_inner(&client, request).await?;
            let json_deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
            serde_path_to_error::deserialize(json_deserializer).map_err(PinError::from)
        })
    }
}

async fn send_inner(
    client: &surf::Client,
    mut request: Request,
) -> Result<Vec<u8>, PinError> {

    let body = request.body_bytes().await?;

    // clone the request before send so it can
    // be re-used if the need to retry arises.
    let mut request = request.clone();
    request.set_body(body.clone());

    let mut response = match client.send(request).await {
        Ok(response) => {
            response
        },
        Err(err) => { 
            return Err(PinError::from(err))
        }
    };

    let status = response.status();

    let bytes = response.body_bytes().await?;

    if !status.is_success() {
        let json_deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
        let error = serde_path_to_error::deserialize(json_deserializer)
            .map(|e: ErrorResponse| {
                PinError::from(e)
            })
            .unwrap_or_else(PinError::from);

            return Err(error)
    }

    Ok(bytes)
}


#[cfg(test)]
mod tests {
    use http_types::{Request, Url};
    use httpmock::prelude::*;

    use super::BaseClient;
    use crate::{PinError};

    #[async_std::test]
    async fn user_error() {
        let client = BaseClient::new();

        let server = MockServer::start_async().await;

        let mock = server.mock(|when, then| {
            when.method(GET).path("/1/missing");
            then.status(404).body("{
                \"error\": \"not_found\",
                \"error_description\": \"The requested resource was not found.\"
              }
              ");
        });

        let req = Request::get(Url::parse(&server.url("/1/missing")).unwrap());
        let res = client.execute::<()>(req).await;

        mock.assert_hits_async(1).await;

        match res {
            Err(PinError::PinPayments(x)) => println!("{:?}", x),
            _ => panic!("Expected PinPayments error {:?}", res),
        }
    }
}
