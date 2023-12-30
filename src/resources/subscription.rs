use time::{OffsetDateTime};
use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::error::PinError;
use crate::ids::{PlanId, CustomerId, SubscriptionId, CardId};
use crate::params::{unpack_contained, Page, Paginator, paginate};
use crate::resources::{Currency};
use crate::build_map;

#[derive(Debug, Default, Serialize)]
pub struct CreateSubscription {
    pub plan_token: PlanId,
    pub customer_token: CustomerId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_setup_fee: Option<bool>
}


#[derive(Debug, Default, Deserialize)]
pub struct Subscription {
    pub token: SubscriptionId,
    pub plan_token: PlanId,
    pub customer_token: CustomerId,
    pub card_token: CardId,
    pub state: String,

    #[serde(with = "time::serde::iso8601::option")]
    pub next_billing_date: Option<OffsetDateTime>,

    #[serde(with = "time::serde::iso8601::option")]
    pub active_interval_started_at: Option<OffsetDateTime>,

    #[serde(with = "time::serde::iso8601::option")]
    pub active_interval_finishes_at: Option<OffsetDateTime>,

    #[serde(with = "time::serde::iso8601::option")]
    pub cancelled_at: Option<OffsetDateTime>,

    #[serde(with = "time::serde::iso8601::option")]
    pub created_at: Option<OffsetDateTime>
}

#[derive(Debug, Default, Deserialize)]
pub struct LedgerEntry {
    pub r#type: String,
    pub amount: i64,
    pub currency: Currency,
    pub annotation: String,

    #[serde(with = "time::serde::iso8601::option")]
    pub created_at: Option<OffsetDateTime>
}

impl Subscription {
    pub fn create(client: &Client, params: CreateSubscription) -> Response<Subscription> {
        unpack_contained(client.post_form("/subscriptions", &params))
    }

    pub fn list(client: &Client, page: Option<u32>, per_page: Option<u32>) -> Response<Page<Subscription>> {
        let page = page.map(|s| s.to_string());
        let per_page = per_page.map(|s| s.to_string());
        let params = build_map([
            ("page", page.as_deref()),
            ("per_page", per_page.as_deref())
        ]);
        client.get_query("/subscriptions", &params)
    }

    pub fn list_with_paginator(client: &Client, per_page: Option<u32>) -> Paginator<Result<Subscription, PinError>> {
        paginate(
            move |page, per_page| {
                Subscription::list(client, Some(page), Some(per_page))
            },
            per_page.unwrap_or(25)
        )
    }

    pub fn retrieve(client: &Client, token: &SubscriptionId) -> Response<Subscription> {
        unpack_contained(client.get(&format!("/subscriptions/{}", token)))
    }

    pub fn delete(client: &Client, token: &SubscriptionId) -> Response<Subscription> {
        unpack_contained(client.delete(&format!("/subscriptions/{}", token)))
    }

    pub fn reactivate(client: &Client, token: &SubscriptionId) -> Response<Subscription> {
        unpack_contained(client.put(&format!("/subscriptions/{}/reactivate", token)))
    }

    pub fn list_ledger_entries(client: &Client, token: &SubscriptionId, page: Option<u32>, per_page: Option<u32>) -> Response<Page<LedgerEntry>> {
        let page = page.map(|s| s.to_string());
        let per_page = per_page.map(|s| s.to_string());
        let params = build_map([
            ("page", page.as_deref()),
            ("per_page", per_page.as_deref())
        ]);
        client.get_query(&format!("/subscriptions/{}/ledger", token), &params)
    }
}
