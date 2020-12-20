use crate::firestore::prelude::*;
use juniper::ID;

pub async fn get_project(client: &Client, id: ID) -> Response<super::Project> {
    let doc = operations::get_doc(client, format!("projects/{}", id))
        .await
        .map_err(|err| match err {
            ResponseError::NotFound(_) => ResponseError::NotFound(format!("Project {}", id)),
            e => e,
        })?;
    Ok(super::doc_to_project(&doc))
}

pub async fn get_all_projects(client: &Client) -> Response<Vec<super::Project>> {
    let docs = operations::get_doc_list(client, "projects".to_string()).await?;
    Ok(docs
        .iter()
        .map(super::doc_to_project)
        .collect::<Vec<super::Project>>())
}

pub async fn create_project(
    client: &Client,
    new_project: super::NewProjectInput,
) -> Response<super::Project> {
    let doc = operations::create_doc(
        client,
        "projects".to_string(),
        super::new_project_to_fields(new_project),
    )
    .await?;
    Ok(super::doc_to_project(&doc))
}

pub async fn update_project(
    client: &Client,
    id: ID,
    upd_project: super::UpdateProjectInput,
) -> Response<super::Project> {
    let doc = operations::update_doc(
        client,
        format!("projects/{}", id),
        super::update_user_to_fields(upd_project),
    )
    .await?;

    Ok(super::doc_to_project(&doc))
}
