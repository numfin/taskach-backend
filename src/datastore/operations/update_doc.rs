use super::utils::{normalize_path, PathToRef};
use crate::datastore::Value;
use crate::{app_env::get_env, datastore::prelude::*};
use chrono::prelude::*;
use googapis::google::datastore::v1;
use std::collections::HashMap;
use v1::{commit_request::Mode, mutation::Operation, CommitRequest, Entity, Key, Mutation};

pub async fn update_doc<'a>(
    client: &Client,
    path: &PathToRef<'a>,
    properties: HashMap<String, Value>,
) -> Response<()> {
    let mut client = client.clone();
    let updated_at = to_db_timestamp(&Utc::now());

    let mut properties = HashMap::from(properties);
    properties.insert("updated_at".into(), updated_at);

    let request = CommitRequest {
        project_id: get_env::gcp_project(),
        mode: Mode::NonTransactional.into(),
        mutations: vec![Mutation {
            operation: Some(Operation::Upsert(Entity {
                key: Some(Key {
                    path: normalize_path(path),
                    ..Default::default()
                }),
                properties,
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
    Ok(())
}
