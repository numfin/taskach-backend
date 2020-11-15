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
