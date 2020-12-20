use std::collections::HashMap;

use crate::firestore::{value::ValueType, Value};

pub fn into_firestore_string(value: String) -> Value {
    Value {
        value_type: Some(ValueType::StringValue(value)),
    }
}

pub fn into_firestore_bool(value: bool) -> Value {
    Value {
        value_type: Some(ValueType::BooleanValue(value)),
    }
}

pub fn into_firestore_bytes(value: String) -> Value {
    let bytes = value.as_bytes().iter().map(|v| *v).collect();
    Value {
        value_type: Some(ValueType::BytesValue(bytes)),
    }
}

pub fn into_firestore_ref(value: Option<juniper::ID>) -> Value {
    Value {
        value_type: match value {
            Some(v) => Some(ValueType::ReferenceValue(v.to_string())),
            None => None,
        },
    }
}

pub fn fields_to_firestore_value(fields: &[AppValue]) -> HashMap<String, Value> {
    fields
        .iter()
        .filter_map(|app_value| match app_value {
            AppValue::Str(key, Some(v)) => {
                Some((key.to_string(), into_firestore_string(v.clone())))
            }
            AppValue::Ref(key, id) => Some((key.to_string(), into_firestore_ref(id.clone()))),
            AppValue::Byte(key, Some(v)) => {
                Some((key.to_string(), into_firestore_bytes(v.clone())))
            }
            AppValue::Bool(key, Some(v)) => Some((key.to_string(), into_firestore_bool(*v))),
            _ => None,
        })
        .collect::<HashMap<String, Value>>()
}

pub enum AppValue<'a> {
    Str(&'a str, Option<String>),
    Ref(&'a str, Option<juniper::ID>),
    Byte(&'a str, Option<String>),
    Bool(&'a str, Option<bool>),
}
