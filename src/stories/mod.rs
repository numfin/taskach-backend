pub mod mutations;
pub mod queries;
pub mod service;

use std::collections::HashMap;

use crate::firestore::{prelude::*, Value};
use chrono::prelude::*;
use juniper::ID;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "Story contains tasks that needs to be done for one feature")]
pub struct Story {
    id: juniper::ID,
    name: String,
    description: String,
    /// Story type (Bug, Feature, etc)
    story_type_id: ID,
    /// Story status (In Progress, Deploy, etc)
    story_status_id: ID,
    sprint_id: Option<ID>,
    /// Story author
    creator_id: ID,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub fn doc_to_story(doc: &Document) -> Story {
    Story {
        id: get_id(doc),
        name: get_field(doc, "name").into_string(),
        description: get_field(doc, "description").into_string(),
        story_type_id: get_field(doc, "story_type_id").into_id(),
        story_status_id: get_field(doc, "story_status_id").into_id(),
        sprint_id: Some(get_field(doc, "sprint_id").into_id()),
        creator_id: get_field(doc, "creator_id").into_id(),
        created_at: get_datetime(&doc.create_time),
        updated_at: get_datetime(&doc.update_time),
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewStoryInput {
    name: String,
    description: String,
    story_type_id: ID,
    story_status_id: ID,
    sprint_id: Option<ID>,
    creator_id: ID,
    project_id: ID,
}

pub fn new_story_to_fields(story: NewStoryInput) -> DbProperties {
    fields_to_firestore_value(&[
        AppValue::Str("name", Some(story.name)),
        AppValue::Str("description", Some(story.description)),
        AppValue::Ref("story_type_id", Some(story.story_type_id)),
        AppValue::Ref("story_status_id", Some(story.story_status_id)),
        AppValue::Ref("sprint_id", story.sprint_id),
        AppValue::Ref("creator_id", Some(story.creator_id)),
    ])
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdateStoryInput {
    name: Option<String>,
    description: Option<String>,
    story_type_id: Option<ID>,
    story_status_id: Option<ID>,
    sprint_id: Option<ID>,
}
pub fn update_story_to_fields(story: UpdateStoryInput) -> DbProperties {
    fields_to_firestore_value(&[
        AppValue::Str("name", story.name),
        AppValue::Str("description", story.description),
        AppValue::Ref("story_type_id", story.story_type_id),
        AppValue::Ref("story_status_id", story.story_status_id),
        AppValue::Ref("sprint_id", story.sprint_id),
    ])
}
