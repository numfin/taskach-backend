pub mod mutations;
pub mod queries;
pub mod service;

use crate::db::schema::stories;
use chrono::{prelude::*, DateTime};

#[derive(juniper::GraphQLObject, Queryable)]
#[graphql(description = "A story in a taskach system")]
pub struct Story {
    id: uuid::Uuid,
    name: String,
    description: String,
    story_type_id: uuid::Uuid,
    story_status_id: uuid::Uuid,
    sprint_id: Option<uuid::Uuid>,
    creator_id: uuid::Uuid,
    project_id: uuid::Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "stories"]
pub struct NewStoryInput {
    name: String,
    description: String,
    story_type_id: uuid::Uuid,
    story_status_id: uuid::Uuid,
    sprint_id: Option<uuid::Uuid>,
    creator_id: uuid::Uuid,
    project_id: uuid::Uuid,
}

#[derive(juniper::GraphQLInputObject, AsChangeset)]
#[table_name = "stories"]
pub struct UpdateStoryInput {
    name: Option<String>,
    description: Option<String>,
    story_type_id: Option<uuid::Uuid>,
    story_status_id: Option<uuid::Uuid>,
    sprint_id: Option<uuid::Uuid>,
}
