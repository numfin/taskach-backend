use crate::datastore::prelude::*;
use crate::datastore::Value;
use chrono::prelude::*;
use googapis::google::datastore::v1;
use std::collections::HashMap;
use utils::{normalize_path, PathToRef};
use v1::{mutation::Operation, CommitRequest, Entity, Key, Mutation};

pub async fn update_doc<'a>(
    client: &Client,
    path: &PathToRef<'a>,
    properties: HashMap<String, Value>,
) -> Response<Key> {
    let mut client = client.clone();
    let updated_at = to_db_timestamp(&Utc::now());

    let mut properties = HashMap::from(properties);
    properties.insert("updated_at".into(), updated_at);

    let mutation_result = client
        .commit(CommitRequest {
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
        })
        .await
        .map_err(|err| {
            println!("{:?}", err.to_string());
            ResponseError::UnexpectedError("creating document".to_string())
        })?
        .get_mut()
        .mutation_results
        .pop();

    match mutation_result {
        Some(result) if result.key.is_some() => Ok(result.key.unwrap()),
        _ => Err(ResponseError::MutationError("Entity is not updated".into())),
    }
}
