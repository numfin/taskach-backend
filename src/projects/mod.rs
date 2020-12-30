pub mod mutations;
pub mod queries;
pub mod service;

use super::datastore::prelude::*;
use chrono::prelude::*;
use juniper::ID;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "Independent project")]
pub struct Project {
    id: ID,
    /// project name
    name: String,
    /// project description
    description: String,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<&Entity> for Project {
    fn from(entity: &Entity) -> Self {
        Self {
            id: DbValue::Id(entity).into(),
            name: DbValue::Str("name", entity).into(),
            description: DbValue::Str("description", entity).into(),
            created_at: DbValue::Timestamp("created_at", entity).into(),
            updated_at: DbValue::Timestamp("updated_at", entity).into(),
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewProjectInput {
    name: String,
    description: String,
}
#[derive(juniper::GraphQLInputObject)]
pub struct UpdateProjectInput {
    name: Option<String>,
    description: Option<String>,
}

impl Project {
    fn new(project: NewProjectInput) -> DbProperties {
        fields_to_db_values(&[
            AppValue::Str("name", Some(project.name)),
            AppValue::Str("description", Some(project.description)),
        ])
    }

    fn update(project: UpdateProjectInput) -> DbProperties {
        fields_to_db_values(&[
            AppValue::Str("name", project.name),
            AppValue::Str("description", project.description),
        ])
    }
}
