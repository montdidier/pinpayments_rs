use time::{OffsetDateTime};
use serde::{Deserialize, Serialize};
use crate::ids::{RecipientId, BankAccountId};
use crate::resources::{CreateBankAccount, BankAccount};
use crate::params::{Page, unpack_contained};
use crate::{Client, Response};
use crate::build_map;

#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateRecipient<'a> {
    pub email: &'a str,
    pub name: Option<&'a str>,

    pub bank_account: Option<CreateBankAccount<'a>>,
    pub bank_account_token: Option<BankAccountId>
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Recipient {
    pub token: RecipientId,
    pub email: String,
    pub name: Option<String>,

    #[serde(with = "time::serde::iso8601::option")]
    pub created_at: Option<OffsetDateTime>,

    pub bank_account: BankAccount
}

impl Recipient {
    pub fn create(client: &Client, params: CreateRecipient<'_>) -> Response<Recipient> {
        unpack_contained(client.post_form(&format!("/recipients"), &params))
    }

    pub fn list(client: &Client, page: Option<u32>, per_page: Option<u32>) -> Response<Page<Recipient>> {
        let page = page.map(|s| s.to_string());
        let per_page = per_page.map(|s| s.to_string());
        let params = build_map([
            ("page", page.as_deref()),
            ("per_page", per_page.as_deref())
        ]);
        client.get_query("/recipients", &params)
    }

    pub fn retrieve(client: &Client, token: &RecipientId) -> Response<Recipient> {
        unpack_contained(client.get(&format!("/recipients/{}", token)))
    }
}
