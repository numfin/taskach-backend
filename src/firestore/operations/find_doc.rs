use googapis::google::firestore::v1::structured_query::FieldReference;
use tonic::Code;

use super::operations;
use crate::firestore::prelude::*;
use crate::firestore::{
    run_query_request::QueryType,
    structured_query::{
        composite_filter, field_filter, filter::FilterType, CollectionSelector, CompositeFilter,
        FieldFilter, Filter,
    },
    RunQueryRequest, StructuredQuery, Value,
};

pub enum FindFilter<'a> {
    Equal(&'a str, Value),
}

fn prepare_filters(filters: Vec<FindFilter<'_>>) -> Vec<Filter> {
    filters
        .iter()
        .filter_map(|f| match f {
            FindFilter::Equal(key, v) => Some(Filter {
                filter_type: Some(FilterType::FieldFilter(FieldFilter {
                    field: Some(FieldReference {
                        field_path: key.to_string(),
                    }),
                    op: field_filter::Operator::Equal.into(),
                    value: Some(v.clone()),
                })),
            }),
        })
        .collect::<Vec<Filter>>()
}

pub async fn find_doc<'a>(
    client: &Client,
    collection: &'a str,
    filters: Vec<FindFilter<'_>>,
    limit: Option<i32>,
    skip: Option<i32>,
) -> Response<Document> {
    let mut client = client.clone();
    let (parent, collection_id) = operations::split_parent_and_collection_id(collection);

    let query = client
        .run_query(RunQueryRequest {
            parent,
            query_type: Some(QueryType::StructuredQuery(StructuredQuery {
                limit: if let Some(l) = limit {
                    Some(l)
                } else {
                    Some(10)
                },
                offset: if let Some(s) = skip { s } else { 0 },
                from: vec![CollectionSelector {
                    collection_id,
                    all_descendants: false,
                }],
                r#where: if filters.len() > 0 {
                    Some(Filter {
                        filter_type: Some(FilterType::CompositeFilter(CompositeFilter {
                            op: composite_filter::Operator::And.into(),
                            filters: prepare_filters(filters),
                        })),
                    })
                } else {
                    None
                },
                ..Default::default()
            })),
            ..Default::default()
        })
        .await
        .map_err(|err| match err.code() {
            Code::NotFound => ResponseError::NotFound("Document not found".to_string()),
            e => ResponseError::UnexpectedError(e.to_string()),
        })?
        .get_mut()
        .message()
        .await
        .map_err(|e| ResponseError::NotFound(e.to_string()))?;
    if let Some(v) = query {
        if let Some(doc) = v.document {
            Ok(doc)
        } else {
            Err(ResponseError::NotFound("Document not found".to_string()))
        }
    } else {
        Err(ResponseError::NotFound("Document not found".to_string()))
    }
}
