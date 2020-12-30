pub mod mutations;
pub mod queries;
pub mod service;

use std::collections::HashMap;

use crate::{
    datastore::{prelude::*, Value},
    users::service::get_user_path,
};
use chrono::prelude::*;
use juniper::ID;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "Story contains tasks that needs to be done for one feature")]
pub struct Story {
    id: ID,
    /// story title
    name: String,
    /// story text
    description: String,
    /// story type (Bug, Feature, etc)
    story_type_id: ID,
    /// story status (In Progress, Deploy, etc)
    story_status_id: ID,
    /// story author
    creator_id: ID,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<Entity> for Story {
    fn from(entity: Entity) -> Self {
        Self {
            id: DbValue::Id(&entity).into(),
            name: DbValue::Str("name", &entity).into(),
            description: DbValue::Str("description", &entity).into(),
            story_type_id: DbValue::Key("story_type_id", &entity).into(),
            story_status_id: DbValue::Key("story_status_id", &entity).into(),
            creator_id: DbValue::Key("creator_id", &entity).into(),
            created_at: DbValue::Timestamp("created_at", &entity).into(),
            updated_at: DbValue::Timestamp("updated_at", &entity).into(),
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewStoryInput {
    name: String,
    description: String,
    story_type_id: ID,
    story_status_id: ID,
    creator_id: ID,
    project_id: ID,
}
#[derive(juniper::GraphQLInputObject)]
pub struct UpdateStoryInput {
    name: Option<String>,
    description: Option<String>,
    story_type_id: Option<ID>,
    story_status_id: Option<ID>,
}

impl Story {
    fn new(story: NewStoryInput) -> DbProperties {
        fields_to_db_values(&[
            AppValue::Str("name", Some(story.name)),
            AppValue::Str("description", Some(story.description)),
            // AppValue::Ref("story_type_id", Some(&story.story_type_id)),
            // AppValue::Ref("story_status_id", Some(&story.story_status_id)),
            AppValue::Ref("creator_id", Some(&get_user_path(&story.creator_id))),
        ])
    }

    fn update(story: UpdateStoryInput) -> DbProperties {
        fields_to_db_values(&[
            AppValue::Str("name", story.name),
            AppValue::Str("description", story.description),
            // AppValue::Ref("story_type_id", story.story_type_id),
            // AppValue::Ref("story_status_id", story.story_status_id),
        ])
    }
}
