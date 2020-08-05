pub mod mutations;
pub mod queries;
pub mod service;

use crate::db::schema::projects;
use chrono::{prelude::*, DateTime};

#[derive(juniper::GraphQLObject, Queryable)]
#[graphql(description = "Independent project")]
pub struct Project {
    id: uuid::Uuid,
    name: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "projects"]
pub struct NewProjectInput {
    name: String,
    description: String,
}

#[derive(juniper::GraphQLInputObject, AsChangeset)]
#[table_name = "projects"]
pub struct UpdateProjectInput {
    name: Option<String>,
    description: Option<String>,
}
