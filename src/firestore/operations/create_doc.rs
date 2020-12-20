use super::operations;
use crate::firestore::prelude::*;
use crate::firestore::value::ValueType;
use crate::firestore::Value;
use std::collections::HashMap;
use tonic::Code;

pub async fn create_doc(
    client: &Client,
    path: String,
    fields: HashMap<String, Value>,
) -> Response<Document> {
    let mut client = client.clone();
    let (parent, collection_id) = operations::split_parent_and_collection_id(&path);

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
                    value_type: Some(ValueType::StringValue(id)),
                }),
            ) => ResponseError::AlreadyExists(format!("Document {} already exists", id)),
            err => {
                println!("{:?}", err);
                ResponseError::UnexpectedError("creating document".to_string())
            }
        })?
        .get_ref()
        .clone())
}
