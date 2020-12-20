use crate::firestore::prelude::*;

use super::operations;

pub async fn get_doc_list(client: &Client, path: String) -> Response<Vec<Document>> {
    let mut client = client.clone();
    let (parent, collection_id) = operations::split_parent_and_collection_id(&path);
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
