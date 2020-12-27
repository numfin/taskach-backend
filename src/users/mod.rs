pub mod mutations;
pub mod queries;
pub mod service;

use crate::auth::pwd::create_pwd_hash;
use crate::datastore::{prelude::*, Value};
use chrono::prelude::*;
use juniper::ID;
use std::collections::HashMap;

#[derive(juniper::GraphQLObject, Debug, Clone)]
#[graphql(description = "A user in a taskach system")]
pub struct User {
    pub id: ID,
    /// имя
    pub first_name: String,
    /// фамилия
    pub last_name: String,
    /// email
    pub email: String,
    /// phone
    pub phone: String,
    /// активность пользователя
    pub active: bool,
    #[graphql(skip)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
pub fn doc_to_user(doc: &Entity) -> User {
    User {
        id: get_id(doc),
        first_name: get_field(doc, "first_name").into_string(),
        last_name: get_field(doc, "last_name").into_string(),
        email: get_field(doc, "email").into_string(),
        phone: get_field(doc, "phone").into_string(),
        active: get_field(doc, "active").into_bool(),
        password_hash: get_field(doc, "password_hash").into_byte_string(),
        created_at: get_field(doc, "created_at").into_date_time(),
        updated_at: get_field(doc, "updated_at").into_date_time(),
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewUserInput {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    password: String,
}

pub fn new_user_to_fields(user: NewUserInput) -> Result<HashMap<String, Value>, String> {
    let password = create_pwd_hash(user.password)?;
    Ok(fields_to_db_values(&[
        AppValue::Str("first_name", Some(user.first_name)),
        AppValue::Str("last_name", Some(user.last_name)),
        AppValue::Str("email", Some(user.email)),
        AppValue::Str("phone", Some(user.phone)),
        AppValue::Byte("password_hash", Some(password)),
        AppValue::Bool("active", Some(true)),
    ]))
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdateUserInput {
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
}
pub fn update_user_to_fields(user: UpdateUserInput) -> HashMap<String, Value> {
    fields_to_db_values(&[
        AppValue::Str("first_name", user.first_name),
        AppValue::Str("last_name", user.last_name),
        AppValue::Str("phone", user.phone),
    ])
}
