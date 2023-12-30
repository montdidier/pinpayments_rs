use time::{OffsetDateTime};
use serde::{Deserialize};

use crate::client::{Client, Response, StatusOnlyResponse};
use crate::ids::{FileId};
use crate::params::{unpack_contained};

#[derive(Debug, Default, Deserialize)]
pub struct File {
    pub token: FileId,
    pub original_filename: String,
    pub presigned_url: String,

    #[serde(with = "time::serde::iso8601::option")]
    pub presigned_url_expires_at: Option<OffsetDateTime>,

    pub purpose: String, 
    pub size: u32,
    pub mime_type: String,

    #[serde(with = "time::serde::iso8601::option")]
    pub uploaded_at: Option<OffsetDateTime>,
}


impl File {
    pub fn retrieve(client: &Client, token: &FileId) -> Response<File> {
        unpack_contained(client.get(&format!("/files/{}", token)))
    }

    pub fn delete(client: &Client, token: &FileId) -> StatusOnlyResponse {
        client.delete_status_only(&format!("/files/{}", token))
    }
}
