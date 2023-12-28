use std::collections::HashMap;
use futures::{stream::Stream};
use futures_util::FutureExt;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::PinError;
use crate::{
    client::{Response},
};

#[derive(Clone, Debug, Default)]
pub struct AppInfo {
    pub name: String,
    pub url: Option<String>,
    pub version: Option<String>,
}

impl ToString for AppInfo {
    fn to_string(&self) -> String {
        match (&self.version, &self.url) {
            (Some(a), Some(b)) => format!("{}/{} ({})", &self.name, a, b),
            (Some(a), None) => format!("{}/{}", &self.name, a),
            (None, Some(b)) => format!("{} ({})", &self.name, b),
            _ => self.name.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Headers {
    pub user_agent: String,
}

impl Headers {
    pub fn to_array(&self) -> [(&str, Option<&str>); 1] {
        [
            ("User-Agent", Some(&self.user_agent)),
        ]
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Single<T> {
    pub response: T
}

pub type Paginator<'a, T> = Pin<Box<dyn Stream<Item = T> + 'a + Send>>;

pub fn paginate<'a, T, Request>(req: Request, per_page: u32) -> Paginator<'a, Result<T, PinError>>
where
    T: 'a + Unpin + Send,
    Request: 'a + Fn(u32, u32) -> Response<Page<T>> + Send,
{
    use async_stream::stream;
    let mut page_n = 0;
    Box::pin(stream! {
        loop {
            let request = req(page_n, per_page);
            let page = request.await?;
            for item in page.items {
                yield Ok(item);
            }
            if page.pagination.next.is_none() {
                break;
            }
            page_n += 1
        }
    })
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginationDetails {
    pub current: u64,
    pub previous: Option<u64>,
    pub next: Option<u64>,
    pub per_page: u32,
    pub pages: Option<u32>,
    pub count: u64
}

/// A single page of a paginated list of objects.
#[derive(Debug, Deserialize, Serialize)]
pub struct Page<T> {
    #[serde(rename = "response")]
    pub items: Vec<T>,
    pub pagination: PaginationDetails
}

pub type Metadata = HashMap<String, String>;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct RangeBounds<T> {
    pub gt: Option<T>,
    pub gte: Option<T>,
    pub lt: Option<T>,
    pub lte: Option<T>,
}

#[derive(Debug, Serialize)]
pub enum SortDirection {
    Asc = 1,
    Desc = -1
}

impl<T> Default for RangeBounds<T> {
    fn default() -> Self {
        RangeBounds { gt: None, gte: None, lt: None, lte: None }
    }
}

pub fn to_snakecase(camel: &str) -> String {
    let mut i = 0;
    let mut snake = String::new();
    let mut chars = camel.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            if i > 0 && !chars.peek().unwrap_or(&'A').is_uppercase() {
                snake.push('_');
            }
            snake.push(ch.to_lowercase().next().unwrap_or(ch));
        } else {
            snake.push(ch);
        }
        i += 1;
    }

    snake
}

pub fn unpack_contained<T: 'static>(container_response: Response<Single<T>>) -> Response<T> {
    Box::pin(container_response.map(|pb| pb.map(|single| single.response)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_snakecase() {
        use super::to_snakecase;

        assert_eq!(to_snakecase("snake_case").as_str(), "snake_case");
        assert_eq!(to_snakecase("CamelCase").as_str(), "camel_case");
        assert_eq!(to_snakecase("XMLHttpRequest").as_str(), "xml_http_request");
        assert_eq!(to_snakecase("UPPER").as_str(), "upper");
        assert_eq!(to_snakecase("lower").as_str(), "lower");
    }
}
