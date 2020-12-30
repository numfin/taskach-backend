use super::utils::{normalize_path, PathToRef};
use crate::datastore::Value;
use crate::{app_env::get_env, datastore::prelude::*};
use chrono::prelude::*;
use googapis::google::datastore::v1;
use std::collections::HashMap;
use v1::{commit_request::Mode, mutation::Operation, CommitRequest, Entity, Key, Mutation};

pub async fn create_doc<'a>(
    client: &Client,
    path: &PathToRef<'a>,
    properties: DbProperties,
) -> Response<Entity> {
    let mut client = client.clone();
    let created_at = insert::to_db_timestamp(&Utc::now());

    let mut properties = HashMap::from(properties);
    properties.insert("created_at".into(), created_at.clone());
    properties.insert("updated_at".into(), created_at);

    let mut key = Some(Key {
        path: normalize_path(path),
        ..Default::default()
    });

    let request = CommitRequest {
        project_id: get_env::project_id(),
        mode: Mode::NonTransactional.into(),
        mutations: vec![Mutation {
            operation: Some(Operation::Insert(Entity {
                key: key.clone(),
                properties: properties.clone(),
            })),
            ..Default::default()
        }],
        ..Default::default()
    };
    let mutation_result = client
        .commit(request)
        .await
        .map_err(|err| {
            println!("{:?}", err.to_string());
            ResponseError::UnexpectedError("creating document".to_string())
        })?
        .get_mut()
        .mutation_results
        .pop();

    if let Some(result) = mutation_result {
        key = result.key;
    }

    Ok(Entity { key, properties })
}
