use super::prelude::*;
use super::Value;
use super::{
    run_query_request::QueryType,
    structured_query::{CollectionSelector, Filter},
    StructuredQuery,
};
use std::collections::HashMap;
use tonic::Code;

pub async fn update_doc(
    client: &Client,
    path: String,
    fields: HashMap<String, Value>,
) -> Response<Document> {
    let mut client = client.clone();
    let field_paths = fields
        .iter()
        .map(|(x, _)| x.clone())
        .collect::<Vec<String>>();
    Ok(client
        .update_document(UpdateDocumentRequest {
            document: Some(Document {
                name: format!("{}/{}", PARENT, path),
                fields,
                ..Default::default()
            }),
            update_mask: Some(super::DocumentMask { field_paths }),
            ..Default::default()
        })
        .await
        .map_err(|err| match err.code() {
            Code::NotFound => ResponseError::NotFound(path),
            _ => ResponseError::UnexpectedError("updating document".to_string()),
        })?
        .get_ref()
        .clone())
}

fn split_parent_and_collection_id(path: &String) -> (String, String) {
    let mut path = path.split('/').collect::<Vec<&str>>();
    let collection_id = path.pop().unwrap();
    let path = if path.len() > 0 {
        format!("/{}", path.join("/"))
    } else {
        "".to_string()
    };
    (format!("{}{}", PARENT, path), collection_id.to_string())
}

pub async fn create_doc(
    client: &Client,
    path: String,
    fields: Result<HashMap<String, Value>, String>,
) -> Response<Document> {
    let mut client = client.clone();
    let (parent, collection_id) = split_parent_and_collection_id(&path);
    let fields = fields.map_err(|e| ResponseError::CreationError(e))?;

    Ok(client
        .create_document(CreateDocumentRequest {
            parent,
            collection_id,
            document: Some(Document {
                fields: fields.clone(),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await
        .map_err(|err| match (err.code(), fields.get("id")) {
            (
                Code::AlreadyExists,
                Some(Value {
                    value_type: Some(super::value::ValueType::StringValue(id)),
                }),
            ) => ResponseError::AlreadyExists(path, id.into()),
            _ => ResponseError::UnexpectedError("creating document".to_string()),
        })?
        .get_ref()
        .clone())
}

pub async fn get_doc(client: &Client, path: String) -> Response<Document> {
    let mut client = client.clone();

    Ok(client
        .get_document(GetDocumentRequest {
            name: format!("{}/{}", PARENT, path),
            ..Default::default()
        })
        .await
        .map_err(|err| match err.code() {
            Code::NotFound => ResponseError::NotFound(path),
            _ => ResponseError::UnexpectedError("getting document".to_string()),
        })?
        .get_ref()
        .clone())
}

pub async fn find_doc(client: &Client, path: String, filter: Filter) -> Response<Document> {
    let mut client = client.clone();
    let (parent, collection_id) = split_parent_and_collection_id(&path);
    let not_found_err = ResponseError::NotFound(path.clone());

    let result = client
        .run_query(super::RunQueryRequest {
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
            _ => ResponseError::UnexpectedError("getting document".to_string()),
        })?
        .get_mut()
        .message()
        .await
        .map_err(|_| not_found_err.clone())?;
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

pub async fn get_doc_list(client: &Client, path: String) -> Response<Vec<Document>> {
    let mut client = client.clone();
    let (parent, collection_id) = split_parent_and_collection_id(&path);
    Ok(client
        .list_documents(ListDocumentsRequest {
            parent,
            collection_id,
            ..Default::default()
        })
        .await
        .map_err(|err| match err.code() {
            _ => ResponseError::UnexpectedError("getting list of documents".to_string()),
        })?
        .get_ref()
        .documents
        .clone())
}
