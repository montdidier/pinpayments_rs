use time::{OffsetDateTime};
use serde::{Deserialize, Serialize};
use crate::ids::{RecipientId, TransferId};
use crate::params::{Page, unpack_contained, SortDirection};
use crate::resources::{Currency, BankAccount};
use crate::{Client, Response};
use crate::build_map;

#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateTransfer<'a> {
    pub amount: i64,
    pub currency: Currency,
    pub description: &'a str,
    pub recipient: RecipientId
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Transfer {
    pub token: TransferId,
    pub status: String,
    pub currency: Currency,
    pub description: String,
    pub amount: i64,
    pub total_debits: i64,
    pub total_credits: i64,

    #[serde(with = "time::serde::iso8601::option")]
    pub created_at: Option<OffsetDateTime>,

    #[serde(with = "time::serde::iso8601::option")]
    pub paid_at: Option<OffsetDateTime>,

    pub reference: String,
    pub line_items_count: i32,

    pub bank_account: BankAccount,
    pub recipient: RecipientId
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortByField {
    PaidAt
}

#[derive(Debug, Serialize, Default)]
pub struct TransferSearchParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<&'a str>,
    #[serde(with = "time::serde::iso8601::option", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601::option", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortByField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<SortDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>
}

impl Transfer {
    pub fn create(client: &Client, params: CreateTransfer<'_>) -> Response<Transfer> {
        unpack_contained(client.post_form(&format!("/transfers"), &params))
    }

    pub fn list(client: &Client, page: Option<u32>, per_page: Option<u32>) -> Response<Page<Transfer>> {
        let page = page.map(|s| s.to_string());
        let per_page = per_page.map(|s| s.to_string());
        let params = build_map([
            ("page", page.as_deref()),
            ("per_page", per_page.as_deref())
        ]);
        client.get_query("/transfers", &params)
    }

    pub fn search(client: &Client, search_params: &TransferSearchParams) -> Response<Page<Transfer>> {
        client.get_query("/transfers/search", &search_params)
    }

    pub fn retrieve(client: &Client, token: &TransferId) -> Response<Transfer> {
        unpack_contained(client.get(&format!("/transfers/{}", token)))
    }
}
