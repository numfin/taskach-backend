use crate::firestore::prelude::*;
use tonic::Code;

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
