use googapis::google::datastore::v1::{key::path_element::IdType, value::ValueType};
use prost_types::Timestamp;

use crate::datastore::prelude::*;
use chrono::prelude::*;
use juniper::ID;

/// Extract Value from Entity
pub fn field(entity: &Entity, field: &str) -> Option<ValueType> {
    if let Some(value) = entity.properties.get(field) {
        value.value_type.to_owned()
    } else {
        None
    }
}

/// Extract ID from Entity
pub fn id(entity: &Entity) -> ID {
    if let Some(key) = &entity.key {
        id_from_key(key)
    } else {
        ID::new("")
    }
}
/// Extract ID from Key
pub fn id_from_key(key: &Key) -> ID {
    if let Some(path) = key.path.last() {
        return match &path.id_type {
            Some(IdType::Id(id)) => ID::new(id.to_string()),
            Some(IdType::Name(name)) => ID::new(name),
            _ => ID::new(""),
        };
    }
    ID::new("")
}
/// Convert datastore Timestamp to chrono::DateTime
pub fn timestamp_to_datetime(t: &Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(t.seconds, t.nanos as u32),
        Utc,
    )
}
