pub mod mutations;
pub mod queries;
pub mod service;

use super::firestore::{prelude::*, Value};
use chrono::prelude::*;
use juniper::ID;
use std::collections::HashMap;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "Independent project")]
pub struct Project {
    id: ID,
    name: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
pub fn doc_to_project(doc: &Document) -> Project {
    Project {
        id: get_id(doc),
        name: get_field(doc, "name").into_string(),
        description: get_field(doc, "description").into_string(),
        created_at: get_datetime(&doc.create_time),
        updated_at: get_datetime(&doc.update_time),
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewProjectInput {
    name: String,
    description: String,
}
pub fn new_project_to_fields(project: NewProjectInput) -> Result<HashMap<String, Value>, String> {
    Ok([
        ("name", into_firestore_string(project.name)),
        ("description", into_firestore_string(project.description)),
    ]
    .iter()
    .map(|v| (v.0.into(), v.1.clone()))
    .collect::<HashMap<String, Value>>())
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdateProjectInput {
    name: Option<String>,
    description: Option<String>,
}
pub fn update_user_to_fields(project: UpdateProjectInput) -> HashMap<String, Value> {
    [
        ("name", project.name, into_firestore_string),
        ("description", project.description, into_firestore_string),
    ]
    .iter()
    .filter_map(|(field, value, convert_fn)| match value {
        Some(value) => Some((field.to_string(), convert_fn(value.to_string()))),
        _ => None,
    })
    .collect::<HashMap<String, Value>>()
}
