use super::operations;
use crate::firestore::prelude::*;
use crate::firestore::{
    run_query_request::QueryType,
    structured_query::{CollectionSelector, Filter},
    RunQueryRequest, StructuredQuery,
};
use tonic::Code;

pub async fn find_doc(client: &Client, path: String, filter: Filter) -> Response<Document> {
    let mut client = client.clone();
    let (parent, collection_id) = operations::split_parent_and_collection_id(&path);
    let not_found_err = ResponseError::NotFound(path.to_string());

    let result = client
        .run_query(RunQueryRequest {
            parent,
            query_type: Some(QueryType::StructuredQuery(StructuredQuery {
                limit: Some(1),
                r#where: Some(filter),
                from: vec![CollectionSelector {
                    all_descendants: false,
                    collection_id,
                }],
                ..Default::default()
            })),
            ..Default::default()
        })
        .await
        .map_err(|err| match err.code() {
            Code::NotFound => not_found_err.clone(),
            e => ResponseError::UnexpectedError(e.to_string()),
        })?
        .get_mut()
        .message()
        .await
        .map_err(|e| ResponseError::NotFound(e.to_string()))?;
    if let Some(v) = result {
        if let Some(doc) = v.document {
            Ok(doc)
        } else {
            Err(not_found_err)
        }
    } else {
        Err(not_found_err)
    }
}
