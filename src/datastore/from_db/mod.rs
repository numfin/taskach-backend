pub mod extract;

use std::collections::HashMap;

use crate::datastore::prelude::*;
use crate::datastore::value::ValueType;
use chrono::prelude::*;
use googapis::google::datastore::v1::Value;
use juniper::ID;
use prost_types::Timestamp;

pub enum DbValue<'a> {
    /// ID of the entity
    Id(&'a Entity),
    /// field-Entity -> ID
    Key(&'a str, &'a Entity),
    /// field-Entity -> String
    Str(&'a str, &'a Entity),
    /// field-Entity -> String
    Blob(&'a str, &'a Entity),
    /// field-Entity -> DateTime<Utc>
    Timestamp(&'a str, &'a Entity),
    /// field-Entity -> bool
    Bool(&'a str, &'a Entity),
}
impl<'a> Into<ID> for DbValue<'a> {
    fn into(self) -> ID {
        match self {
            DbValue::Id(e) => extract::id(e),
            DbValue::Key(key, entity) => match &extract::field(entity, key) {
                Some(ValueType::KeyValue(key)) => extract::id_from_key(key),
                _ => ID::new(""),
            },
            _ => ID::new(""),
        }
    }
}
impl<'a> Into<String> for DbValue<'a> {
    fn into(self) -> String {
        match self {
            DbValue::Str(key, entity) => match extract::field(entity, key) {
                Some(ValueType::StringValue(v)) => v,
                Some(ValueType::BlobValue(v)) => match std::str::from_utf8(&v) {
                    Ok(v) => v.to_string(),
                    _ => "".to_string(),
                },
                _ => "".to_string(),
            },
            _ => "".to_string(),
        }
    }
}
impl<'a> Into<bool> for DbValue<'a> {
    fn into(self) -> bool {
        match self {
            DbValue::Bool(key, entity) => match extract::field(entity, key) {
                Some(ValueType::BooleanValue(v)) => v,
                _ => false,
            },
            _ => false,
        }
    }
}
impl<'a> Into<DateTime<Utc>> for DbValue<'a> {
    fn into(self) -> DateTime<Utc> {
        let default_datetime = extract::timestamp_to_datetime(&Timestamp {
            nanos: 0,
            seconds: 0,
        });
        match self {
            DbValue::Timestamp(key, entity) => match &extract::field(entity, key) {
                Some(ValueType::TimestampValue(v)) => extract::timestamp_to_datetime(v),
                _ => default_datetime,
            },
            _ => default_datetime,
        }
    }
}

pub type DbProperties = HashMap<String, Value>;
