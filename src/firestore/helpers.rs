use crate::firestore::{value::ValueType, Document};
use chrono::prelude::*;
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
        if let Some(ValueType::ReferenceValue(value)) = self {
            juniper::ID::from(value)
        } else {
            juniper::ID::new("")
        }
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
        if let Some(ValueType::BytesValue(value)) = self {
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

pub fn get_field(doc: &Document, field: &str) -> Option<ValueType> {
    if let Some(value) = doc.fields.get(field) {
        value.value_type.to_owned()
    } else {
        None
    }
}

pub fn get_id(doc: &Document) -> ID {
    ID::from(
        match doc.name.split("/").last() {
            Some(id) => id,
            None => "",
        }
        .to_string(),
    )
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
