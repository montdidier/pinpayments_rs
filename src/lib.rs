#![recursion_limit = "128"]

#![allow(clippy::map_clone, clippy::large_enum_variant)]
#![warn(clippy::unwrap_used, clippy::missing_errors_doc, clippy::missing_panics_doc)]
#![forbid(unsafe_code)]

mod client;
mod params;
mod ids;
mod error;
mod resources;

pub use crate::client::*;
pub use crate::error::{ErrorCode, PinError};
pub use crate::ids::*;
pub use crate::params::{
    Headers,
};
pub use crate::resources::*;


use std::collections::HashMap;

pub fn build_map<'key, 'value, const N: usize>(
    array: [(&'key str, Option<&'value str>); N],
) -> HashMap<&'key str, &'value str> {
    let mut map = HashMap::with_capacity(N);
    for (key, value) in array {
        if let Some(value) = value {
            map.insert(key, value);
        }
    }
    map
}
