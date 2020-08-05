pub mod mutations;
pub mod queries;
pub mod service;

use crate::db::schema::users;
use chrono::{prelude::*, DateTime};

#[derive(juniper::GraphQLObject, Queryable)]
#[graphql(description = "A user in a taskach system")]
pub struct User {
    id: uuid::Uuid,
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "users"]
pub struct NewUserInput {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
}

#[derive(juniper::GraphQLInputObject, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUserInput {
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
}
