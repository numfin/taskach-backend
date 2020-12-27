use crate::datastore::{key::path_element::IdType, value::ValueType, Entity};
use chrono::prelude::*;
use googapis::google::datastore::v1::Key;
use juniper::ID;
use prost_types::Timestamp;

pub trait Helpers {
    fn into_id(self) -> juniper::ID;
    fn into_string(self) -> String;
    fn into_date_time(self) -> DateTime<Utc>;
    fn into_bool(self) -> bool;
    fn into_byte_string(self) -> String;
}
impl Helpers for Option<ValueType> {
    fn into_id(self) -> juniper::ID {
        if let Some(ValueType::KeyValue(key)) = self {
            let last_path = key.path.last();
            if let Some(path) = last_path {
                if let Some(IdType::Id(id)) = path.id_type {
                    return ID::new(id.to_string());
                }
            }
        }

        juniper::ID::new("")
    }
    fn into_string(self) -> String {
        if let Some(ValueType::StringValue(value)) = self {
            value
        } else {
            "".to_string()
        }
    }
    fn into_date_time(self) -> DateTime<Utc> {
        if let Some(ValueType::TimestampValue(t)) = self {
            get_datetime(&Some(t))
        } else {
            get_datetime(&None)
        }
    }
    fn into_bool(self) -> bool {
        if let Some(ValueType::BooleanValue(b)) = self {
            b
        } else {
            false
        }
    }
    fn into_byte_string(self) -> String {
        if let Some(ValueType::BlobValue(value)) = self {
            if let Ok(value) = std::str::from_utf8(&value) {
                value.to_string()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    }
}

pub fn get_field(doc: &Entity, field: &str) -> Option<ValueType> {
    if let Some(value) = doc.properties.get(field) {
        value.value_type.to_owned()
    } else {
        None
    }
}

pub fn get_id(doc: &Entity) -> ID {
    if let Some(key) = &doc.key {
        get_id_from_key(key)
    } else {
        ID::new("")
    }
}
pub fn get_id_from_key(key: &Key) -> ID {
    if let Some(path) = key.path.last() {
        if let Some(IdType::Id(id)) = path.id_type {
            return ID::new(id.to_string());
        }
    }
    ID::new("")
}

pub fn get_datetime(timestamp: &Option<Timestamp>) -> DateTime<Utc> {
    if let Some(t) = timestamp {
        DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(t.seconds, t.nanos as u32),
            Utc,
        )
    } else {
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)
    }
}
