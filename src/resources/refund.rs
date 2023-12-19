use time::{OffsetDateTime};
use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::error::PinError;
use crate::ids::{RefundId, ChargeId};
use crate::params::{Page, Paginator, unpack_contained, paginate};
use crate::resources::{
    Currency
};
use crate::build_map;

#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateRefund {
    pub amount: Option<i64>
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Refund { 
    pub token: RefundId,
    pub success: Option<bool>,
    pub amount: i64,
    pub currency: Currency,
    pub charge: ChargeId,
    #[serde(with = "time::serde::iso8601::option")]
    pub created_at: Option<OffsetDateTime>,
    pub error_message: Option<String>,
    pub status_message: String
}

impl Refund {
    pub fn create(client: &Client, token: &ChargeId, params: CreateRefund) -> Response<Refund> {
        unpack_contained(client.post_form(&format!("/charges/{}/refunds", token), &params))
    }

    pub fn list(client: &Client, page: Option<u32>, per_page: Option<u32>) -> Response<Page<Refund>> {
        let page = page.map(|s| s.to_string());
        let per_page = per_page.map(|s| s.to_string());
        let params = build_map([
            ("page", page.as_deref()),
            ("per_page", per_page.as_deref())
        ]);
        client.get_query("/refunds", &params)
    }

    pub fn list_with_paginator(client: &Client, per_page: Option<u32>) -> Paginator<Result<Refund, PinError>> {
        paginate(
            move |page, per_page| {
                Refund::list(client, Some(page), Some(per_page))
            },
            per_page.unwrap_or(25)
        )
    }

    pub fn list_for_charge(client: &Client, token: &ChargeId, page: Option<u32>, per_page: Option<u32>) -> Response<Page<Refund>> {
        let page = page.map(|s| s.to_string());
        let per_page = per_page.map(|s| s.to_string());
        let params = build_map([
            ("page", page.as_deref()),
            ("per_page", per_page.as_deref())
        ]);
        client.get_query(&format!("/charges/{}/refunds", token), &params)
    }

    pub fn list_for_charge_with_paginator<'a>(client: &'a Client, token: &'a ChargeId, per_page: Option<u32>) -> Paginator<'a, Result<Refund, PinError>> { 
        paginate( 
            move |page, per_page| {
                Refund::list_for_charge(client, token, Some(page), Some(per_page))
            },
            per_page.unwrap_or(25)
        )
    }

    pub fn retrieve(client: &Client, token: &RefundId) -> Response<Refund> {
        unpack_contained(client.get(&format!("/refunds/{}", token)))
    }
}
