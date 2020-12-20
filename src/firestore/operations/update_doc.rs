use crate::firestore::prelude::*;
use crate::firestore::Value;
use crate::firestore::{Document, DocumentMask, UpdateDocumentRequest};
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
            update_mask: Some(DocumentMask { field_paths }),
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
