use std::collections::HashMap;
pub mod insert;

use crate::datastore::prelude::*;
use crate::datastore::{operations::utils::PathToRef, Value};
use chrono::prelude::*;

pub fn fields_to_db_values<'a>(fields: &[AppValue<'a>]) -> DbProperties {
    let mut store = HashMap::new();
    for app_value in fields.into_iter() {
        if let Some((k, v)) = app_value.to_db_value() {
            store.insert(k, v);
        }
    }
    store.to_owned()
}

pub enum AppValue<'a> {
    Str(&'a str, Option<String>),
    Ref(&'a str, &'a PathToRef<'a>),
    Byte(&'a str, Option<String>),
    Bool(&'a str, Option<bool>),
    Date(&'a str, Option<DateTime<Utc>>),
}

impl<'a> AppValue<'a> {
    fn to_db_value(&self) -> Option<(String, Value)> {
        match self {
            AppValue::Str(key, Some(v)) => Some((key.to_string(), insert::to_db_string(v))),
            AppValue::Ref(key, path_to_ref) => {
                Some((key.to_string(), insert::to_db_key(path_to_ref)))
            }
            AppValue::Byte(key, Some(v)) => Some((key.to_string(), insert::to_db_bytes(v))),
            AppValue::Bool(key, Some(v)) => Some((key.to_string(), insert::to_db_bool(v))),
            AppValue::Date(key, Some(datetime)) => {
                Some((key.to_string(), insert::to_db_timestamp(datetime)))
            }
            _ => None,
        }
    }
}
