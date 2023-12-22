use time::{OffsetDateTime};
use serde::{Deserialize, Serialize};

use crate::client::{Client, Response, StatusOnlyResponse};
use crate::error::PinError;
use crate::ids::{CardId, CustomerId};
use crate::params::{unpack_contained, Page, Paginator, paginate};
use crate::resources::{CardParams, Card, Charge};
use crate::build_map;

#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateCustomer<'a> {
    pub email: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardParams<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_token: Option<CardId>
}


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Customer {
    pub token: CustomerId,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<String>,
    pub notes: Option<String>,
    #[serde(with = "time::serde::iso8601::option")]
    pub created_at: Option<OffsetDateTime>,
    pub card: Card
}

impl Customer {
    pub fn create(client: &Client, params: CreateCustomer<'_>) -> Response<Customer> {
        unpack_contained(client.post_form("/customers", &params))
    }

    pub fn list(client: &Client, page: Option<u32>, per_page: Option<u32>) -> Response<Page<Customer>> {
        let page = page.map(|s| s.to_string());
        let per_page = per_page.map(|s| s.to_string());
        let params = build_map([
            ("page", page.as_deref()),
            ("per_page", per_page.as_deref())
        ]);
        client.get_query("/customers", &params)
    }

    pub fn list_with_paginator(client: &Client, per_page: Option<u32>) -> Paginator<Result<Customer, PinError>> {
        paginate(
            move |page, per_page| {
                Customer::list(client, Some(page), Some(per_page))
            },
            per_page.unwrap_or(25)
        )
    }

    pub fn retrieve(client: &Client, token: &CustomerId) -> Response<Customer> {
        unpack_contained(client.get(&format!("/customers/{}", token)))
    }

    pub fn delete(client: &Client, token: &CustomerId) -> StatusOnlyResponse { 
        client.delete_status_only(&format!("/customers/{}", token))
    }

    pub fn list_charges(client: &Client, token: &CustomerId, page: Option<u32>, per_page: Option<u32>) -> Response<Page<Charge>> {
        let page = page.map(|s| s.to_string());
        let per_page = per_page.map(|s| s.to_string());
        let params = build_map([
            ("page", page.as_deref()),
            ("per_page", per_page.as_deref())
        ]);
        client.get_query(&format!("/customers/{}/charges", token), &params)
    }

    pub fn list_charges_with_paginator<'a>(client: &'a Client, token: &'a CustomerId, per_page: Option<u32>) -> Paginator<'a, Result<Charge, PinError>> {
        paginate(
            move |page, per_page| {
                Customer::list_charges(client, token, Some(page), Some(per_page))
            },
            per_page.unwrap_or(25)
        )
    }

    pub fn list_cards(client: &Client, token: &CustomerId, page: Option<u32>, per_page: Option<u32>) -> Response<Page<Card>> {
        let page = page.map(|s| s.to_string());
        let per_page = per_page.map(|s| s.to_string());
        let params = build_map([
            ("page", page.as_deref()),
            ("per_page", per_page.as_deref())
        ]);
        client.get_query(&format!("/customers/{}/cards", token), &params)
    }

    pub fn list_cards_with_paginator<'a>(client: &'a Client, token: &'a CustomerId, per_page: Option<u32>) -> Paginator<'a, Result<Card, PinError>> {
        paginate(
            move |page, per_page| {
                Customer::list_cards(client, token, Some(page), Some(per_page))
            },
            per_page.unwrap_or(25)
        )
    }

    pub fn create_card(client: &Client, token: &CustomerId, params: CardParams<'_>) -> Response<Card> {
        unpack_contained(client.post_form(&format!("/customers/{}/cards", token), &params))
    }

    pub fn delete_card(client: &Client, token: &CustomerId, card_token: &CardId) -> StatusOnlyResponse { 
        client.delete_status_only(&format!("/customers/{}/cards/{}", token, card_token))
    }
}
