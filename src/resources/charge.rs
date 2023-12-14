use std::fmt;
use time::{OffsetDateTime};
use http_types::Url;
use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::error::PinError;
use crate::ids::{ChargeId, SessionId};
use crate::params::{Metadata, Page, Paginator, unpack_contained, paginate};
use crate::resources::{
    CardParams,
    Card,
    Currency
};
use crate::build_map;

#[derive(Clone, Debug, Serialize)]
pub struct ThreeDSecure {
    pub enabled: bool,
    pub fallback_ok: bool,
    pub callback_url: Url
}

#[derive(Clone, Debug, Serialize)]
pub struct PlatformAdjustment {
    pub amount: i64,
    pub currency: Currency,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateCharge<'a> {
    pub email: String,
    pub description: &'a str, 
    pub amount: i64,
    pub ip_address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardParams<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<ThreeDSecure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_adjustment: Option<PlatformAdjustment>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Charge {
    pub token: ChargeId,
    pub success: bool,
    pub amount: i64,
    pub currency: Currency,
    pub description: String,
    pub email: String,
    pub ip_address: String,
    #[serde(with = "time::serde::iso8601::option")]
    pub created_at: Option<OffsetDateTime>,
    pub status_message: String,
    pub error_message: Option<String>,
    pub card: Card,
    pub amount_refunded: i64,
    pub total_fees: Option<i64>,
    pub merchant_entitlement: Option<i64>,
    pub refund_pending: bool,
    pub authorisation_token: Option<String>,
    pub authorisation_expired: bool,
    pub authorisation_voided: bool,
    pub captured: bool,
    #[serde(with = "time::serde::iso8601::option")]
    pub captured_at: Option<OffsetDateTime>,
    pub settlement_currency: Currency,
    pub active_chargebacks: bool,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize)]
pub struct VerifyCharge<'a> {
    pub session_token: &'a SessionId
}

impl Charge {
    pub fn create(client: &Client, params: CreateCharge<'_>) -> Response<Charge> {
        unpack_contained(client.post_form(&format!("/charges"), &params))
    }

    pub fn retrieve(client: &Client, token: &ChargeId) -> Response<Charge> {
        unpack_contained(client.get(&format!("/charges/{}", token)))
    }

    pub fn void(client: &Client, token: &ChargeId) -> Response<Charge> {
        unpack_contained(client.put(&format!("/charges/{}/void", token)))
    }

    pub fn capture(client: &Client, token: &ChargeId) -> Response<Charge> {
        unpack_contained(client.put(&format!("/charges/{}/capture", token)))
    }

    pub fn list(client: &Client, page: Option<u32>, per_page: Option<u32>) -> Response<Page<Charge>> {
        let page = page.map(|s| s.to_string());
        let per_page = per_page.map(|s| s.to_string());
        let params = build_map([
            ("page", page.as_deref()),
            ("per_page", per_page.as_deref())
        ]);
        client.get_query("/charges", &params)
    }

    pub fn list_with_paginator(client: &Client, per_page: Option<u32>) -> Paginator<Result<Charge, PinError>> {
        paginate(
            move |page, per_page| {
                Charge::list(client, Some(page), Some(per_page))
            },
            per_page.unwrap_or(25)
        )
    }

    pub fn verify(client: &Client, session_token: &SessionId) -> Response<Charge> {
        unpack_contained(client.get_query("/charges/verify", VerifyCharge { session_token: session_token }))
    }
}

impl fmt::Display for Charge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.token, self.amount)
    }
}
