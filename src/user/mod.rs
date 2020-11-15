pub mod mutations;
pub mod queries;
pub mod service;

use crate::firestore::{prelude::*, Value};
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A user in a taskach system")]
pub struct User {
    id: String,
    /// имя
    first_name: String,
    /// фамилия
    last_name: String,
    /// email
    email: String,
    /// phone
    phone: String,
    /// активность пользователя
    active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
pub fn doc_to_user(doc: &Document) -> User {
    User {
        id: get_id(doc).to_string(),
        first_name: get_field(doc, "first_name").into_string(),
        last_name: get_field(doc, "last_name").into_string(),
        email: get_field(doc, "email").into_string(),
        phone: get_field(doc, "phone").into_string(),
        active: get_field(doc, "active").into_bool(),
        created_at: get_datetime(&doc.create_time),
        updated_at: get_datetime(&doc.update_time),
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewUserInput {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
}

pub fn new_user_to_fields(user: NewUserInput) -> HashMap<String, Value> {
    [
        ("first_name", into_firestore_string(user.first_name)),
        ("last_name", into_firestore_string(user.last_name)),
        ("email", into_firestore_string(user.email)),
        ("phone", into_firestore_string(user.phone)),
        ("active", into_firestore_bool(false)),
    ]
    .iter()
    .map(|v| (v.0.into(), v.1.clone()))
    .collect::<HashMap<String, Value>>()
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdateUserInput {
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
}
pub fn update_user_to_fields(user: UpdateUserInput) -> HashMap<String, Value> {
    [
        ("first_name", user.first_name, into_firestore_string),
        ("last_name", user.last_name, into_firestore_string),
        ("phone", user.phone, into_firestore_string),
    ]
    .iter()
    .filter_map(|(field, value, convert_fn)| match value {
        Some(value) => Some((field.to_string(), convert_fn(value.to_string()))),
        _ => None,
    })
    .collect::<HashMap<String, Value>>()
}
