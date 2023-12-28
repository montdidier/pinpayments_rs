use time::{OffsetDateTime};
use serde::{Deserialize, Serialize};

use crate::client::{Client, Response, StatusOnlyResponse};
use crate::error::PinError;
use crate::ids::{PlanId};
use crate::params::{unpack_contained, Page, Paginator, paginate};
use crate::resources::{Currency};
use crate::build_map;

#[derive(PartialEq, Debug, Serialize, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntervalUnit {
    #[default]
    Day,
    Week,
    Month,
    Year
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerPermission {
    Cancel
}

#[derive(Debug, Default, Serialize)]
pub struct CreatePlan<'a> {
    pub name: &'a str,
    pub amount: i64,
    pub currency: Currency,
    pub interval: u32,
    pub interval_unit: IntervalUnit,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intervals: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_amount: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_amount: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_interval: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_interval_unit: Option<IntervalUnit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_permissions: Option<Vec<CustomerPermission>>
}

#[derive(Debug, Default, Deserialize)]
pub struct SubscriptionCounts {
    pub trial: u32,
    pub active: u32,
    pub cancelling: u32,
    pub cancelled: u32
} 

#[derive(Debug, Default, Deserialize)]
pub struct Plan {
    pub token: PlanId,
    pub name: String,
    pub amount: i64,
    pub currency: Currency,
    pub interval: u32,
    pub interval_unit: IntervalUnit,
    pub intervals: u32,
    pub setup_amount: u32,
    pub trial_amount: u32,
    pub trial_interval: u32,
    pub trial_interval_unit: IntervalUnit,
    pub customer_permissions: Vec<CustomerPermission>, 
    pub subscription_counts: SubscriptionCounts,

    #[serde(with = "time::serde::iso8601::option")]
    pub created_at: Option<OffsetDateTime>
}

impl Plan {
    pub fn create(client: &Client, params: CreatePlan<'_>) -> Response<Plan> {
        unpack_contained(client.post_form("/plans", &params))
    }

    pub fn list(client: &Client, page: Option<u32>, per_page: Option<u32>) -> Response<Page<Plan>> {
        let page = page.map(|s| s.to_string());
        let per_page = per_page.map(|s| s.to_string());
        let params = build_map([
            ("page", page.as_deref()),
            ("per_page", per_page.as_deref())
        ]);
        client.get_query("/plans", &params)
    }

    pub fn list_with_paginator(client: &Client, per_page: Option<u32>) -> Paginator<Result<Plan, PinError>> {
        paginate(
            move |page, per_page| {
                Plan::list(client, Some(page), Some(per_page))
            },
            per_page.unwrap_or(25)
        )
    }

    pub fn retrieve(client: &Client, token: &PlanId) -> Response<Plan> {
        unpack_contained(client.get(&format!("/plans/{}", token)))
    }

    pub fn delete(client: &Client, token: &PlanId) -> StatusOnlyResponse { 
        client.delete_status_only(&format!("/plans/{}", token))
    }
}
