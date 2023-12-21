use serde::{Deserialize};

use crate::client::{Client, Response};
use crate::params::{unpack_contained};
use crate::resources::{Currency};

#[derive(Clone, Debug, Deserialize)]
pub struct AmountCurrency {
    pub amount: i64,
    pub currency: Currency
}

#[derive(Clone, Debug, Deserialize)]
pub struct Balance {
    pub available: Vec<AmountCurrency>,
    pub pending: Vec<AmountCurrency>
}

impl Balance { 
    pub fn retrieve(client: &Client) -> Response<Balance> {
        unpack_contained(client.get("/balance"))
    }
}
