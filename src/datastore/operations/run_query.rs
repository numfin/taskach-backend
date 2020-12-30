use crate::{app_env::get_env, datastore::prelude::*};

use googapis::google::datastore::v1;
use v1::{
    run_query_request::QueryType, Entity, EntityResult, GqlQuery, QueryResultBatch,
    RunQueryRequest, Value,
};

use super::utils::{value_to_gql_param, PathToRef};

pub async fn run_query_id<'a>(
    client: &Client,
    kind: &'a str,
    entity_path: &PathToRef<'a>,
) -> Response<Entity> {
    let query_batch = run_query(
        client,
        format!("SELECT * FROM {} WHERE __key__ = @1", kind),
        Default::default(),
        &[insert::to_db_key(entity_path)],
    )
    .await
    .or(Err(ResponseError::NotFound(kind.into())))?;
    let first_entity = extract_first_entity(&query_batch.entity_results);
    match first_entity {
        Some(v) => Ok(v),
        None => Err(ResponseError::NotFound(format!("{}", kind))),
    }
}

pub async fn run_query<'a>(
    client: &Client,
    query_string: String,
    named_bindings: DbProperties,
    positional_bindings: &[Value],
) -> Response<QueryResultBatch> {
    let mut client = client.clone();

    let query = RunQueryRequest {
        project_id: get_env::project_id(),
        query_type: Some(QueryType::GqlQuery(GqlQuery {
            named_bindings: named_bindings
                .iter()
                .map(|(k, v)| (k.to_string(), value_to_gql_param(v)))
                .collect(),
            positional_bindings: positional_bindings.iter().map(value_to_gql_param).collect(),
            query_string,
            allow_literals: true,
        })),
        ..Default::default()
    };

    let query_batch = client
        .run_query(query)
        .await
        .map_err(|err| {
            println!("{:#?}", err.to_string());
            ResponseError::UnexpectedError(err.to_string())
        })?
        .get_ref()
        .batch
        .clone();
    match query_batch {
        Some(batch) => Ok(batch),
        None => Err(ResponseError::UnexpectedError(
            "Batch is not present".into(),
        )),
    }
}

pub fn extract_first_entity(entities: &Vec<EntityResult>) -> Option<Entity> {
    match entities.last() {
        Some(entity) if entity.entity.is_some() => entity.entity.to_owned(),
        _ => None,
    }
}
