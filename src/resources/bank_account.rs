use serde::{Deserialize, Serialize};

use crate::ids::{BankAccountId};
use crate::client::{Client, Response};
use crate::params::{unpack_contained};

#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateBankAccount<'a> {
    pub name: &'a str,
    pub bsb: &'a str,
    pub number: &'a str
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct BankAccount {
    pub token: BankAccountId,
    pub name: String,
    pub bsb: String,
    pub number: String,
    pub bank_name: Option<String>,
    pub branch: Option<String>
}

impl BankAccount {
    pub fn create(client: &Client, params: CreateBankAccount<'_>) -> Response<BankAccount> {
        unpack_contained(client.post_form(&format!("/bank_accounts"), &params))
    }
}
