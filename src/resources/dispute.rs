use time::{OffsetDateTime};
use serde::{Deserialize, Serialize};

use crate::client::{Client, Response, StatusOnlyResponse};
use crate::error::PinError;
use crate::ids::{DisputeId};
use crate::params::{unpack_contained, SortDirection, Page, Paginator, paginate};
use crate::resources::{Currency, Charge};
use crate::build_map;


#[derive(Debug, Default, Deserialize)]
pub struct Dispute {
    pub token: DisputeId,
    pub category: String,
    pub status: String,
    pub amount: i64,
    pub currency: Currency,
    pub charge: Charge,

    #[serde(with = "time::serde::iso8601::option")]
    pub evidence_required_by: Option<OffsetDateTime>,
    pub relevant_evidence: Vec<String>,

    pub received_at: Option<OffsetDateTime>
}

#[derive(Debug, Default, Serialize)]
pub struct DisputeSearchParams<'a> {
    pub query: Option<&'a str>,
    pub status: Option<&'a str>,
    pub sort: Option<SortByField>,
    pub direction: Option<SortDirection>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortByField {
    ReceivedAt,
    EvidenceRequiredBy,
    Amount
}

impl Dispute {
    pub fn list(client: &Client, page: Option<u32>, per_page: Option<u32>) -> Response<Page<Dispute>> {
        let page = page.map(|s| s.to_string());
        let per_page = per_page.map(|s| s.to_string());
        let params = build_map([
            ("page", page.as_deref()),
            ("per_page", per_page.as_deref())
        ]);
        client.get_query("/disputes", &params)
    }

    pub fn list_with_paginator(client: &Client, per_page: Option<u32>) -> Paginator<Result<Dispute, PinError>> {
        paginate(
            move |page, per_page| {
                Dispute::list(client, Some(page), Some(per_page))
            },
            per_page.unwrap_or(25)
        )
    }

    pub fn search(client: &Client, search_params: DisputeSearchParams<'_>) -> Response<Page<Dispute>> {
        client.get_query("/disputes/search", &search_params)
    }

    pub fn retrieve(client: &Client, token: &DisputeId) -> Response<Dispute> {
        unpack_contained(client.get(&format!("/disputes/{}", token)))
    }

    pub fn submit_evidence(client: &Client, token: &DisputeId) -> StatusOnlyResponse {
        client.post_status_only(&format!("/disputes/{}/evidence", token))
    }

    pub fn accept(client: &Client, token: &DisputeId) -> StatusOnlyResponse {
        client.post_status_only(&format!("/disputes/{}/accept", token))
    }
}
