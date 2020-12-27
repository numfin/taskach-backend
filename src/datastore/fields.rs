use std::collections::HashMap;

use chrono::prelude::*;
use googapis::google::datastore::v1;
use prost_types::Timestamp;
use v1::Key;

use crate::datastore::{value::ValueType, Value};

use super::operations::utils::{normalize_path, PathToRef};

pub fn to_db_string(value: &String) -> Value {
    Value {
        value_type: Some(ValueType::StringValue(value.clone())),
        ..Default::default()
    }
}

pub fn to_db_bool(value: &bool) -> Value {
    Value {
        value_type: Some(ValueType::BooleanValue(*value)),
        ..Default::default()
    }
}

pub fn to_db_bytes(value: &String) -> Value {
    let bytes = value.as_bytes().iter().map(|v| *v).collect();
    Value {
        value_type: Some(ValueType::BlobValue(bytes)),
        ..Default::default()
    }
}

pub fn to_db_key<'a>(entity_path: &PathToRef<'a>) -> Value {
    let path = normalize_path(entity_path);

    Value {
        value_type: Some(ValueType::KeyValue(Key {
            path,
            ..Default::default()
        })),
        ..Default::default()
    }
}

pub fn to_db_timestamp(datetime: &DateTime<Utc>) -> Value {
    Value {
        value_type: Some(ValueType::TimestampValue(Timestamp {
            nanos: datetime.timestamp_subsec_nanos() as i32,
            seconds: datetime.timestamp() as i64,
        })),
        ..Default::default()
    }
}

pub fn fields_to_db_values<'a>(
    original: &mut HashMap<String, Value>,
    fields: &[AppValue<'a>],
) -> HashMap<String, Value> {
    fields.iter().for_each(|app_value| {
        if let Some((k, v)) = app_value.to_db_value() {
            original.insert(k, v);
        }
    });
    original.to_owned()
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
            AppValue::Str(key, Some(v)) => Some((key.to_string(), to_db_string(v))),
            AppValue::Ref(key, path_to_ref) => Some((key.to_string(), to_db_key(path_to_ref))),
            AppValue::Byte(key, Some(v)) => Some((key.to_string(), to_db_bytes(v))),
            AppValue::Bool(key, Some(v)) => Some((key.to_string(), to_db_bool(v))),
            AppValue::Date(key, Some(datetime)) => {
                Some((key.to_string(), to_db_timestamp(datetime)))
            }
            _ => None,
        }
    }
}
