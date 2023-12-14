use crate::ids::{CardId};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct CardParams<'a> {
    pub number: &'a str,
    pub expiry_month: &'a str,
    pub expiry_year: &'a str,
    pub cvc: &'a str,
    pub name: &'a str,
    pub address_line1: &'a str,
    pub address_city: &'a str,
    pub address_state: &'a str,
    pub address_postcode: &'a str,
    pub address_country: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Card {
    pub token: CardId,
    pub scheme: String,
    pub display_number: String,
    pub issuing_country: String,
    pub expiry_month: i64,
    pub expiry_year: i64,
    pub name: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub address_city: String,
    pub address_postcode: String,
    pub address_country: String,
    pub network_type: Option<String>,
    pub network_format: Option<String>,
    pub customer_token: Option<String>,
    pub primary: Option<bool>,
}
