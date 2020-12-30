use chrono::prelude::*;
use googapis::google::datastore::v1;
use prost_types::Timestamp;
use v1::Key;

use crate::datastore::{value::ValueType, Value};

use crate::datastore::operations::utils::{normalize_path, PathToRef};

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
