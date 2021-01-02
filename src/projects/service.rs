use super::Project;
use crate::datastore::prelude::*;
use juniper::ID;
use utils::PathToRef;

pub fn get_project_path<'a>(id: &'a ID) -> PathToRef<'a> {
    vec![(KeyKind("Projects"), KeyId::Cuid(id.clone()))]
}

pub async fn get_project(client: &Client, id: &ID) -> Response<Entity> {
    let entity = operations::run_query_id(client, "Projects", &get_project_path(id))
        .await
        .or(Err(ResponseError::NotFound(format!("Project {}", id))))?;

    Ok(entity)
}

pub async fn get_all_projects(client: &Client) -> Response<Vec<Project>> {
    let query_batch = operations::run_query(
        client,
        format!("SELECT * FROM Projects"),
        Default::default(),
        Default::default(),
    )
    .await
    .or(Err(ResponseError::NotFound("Projects".into())))?;

    Ok(query_batch
        .entity_results
        .iter()
        .filter_map(|entity_result| match &entity_result.entity {
            Some(entity) => Some(Project::from(entity)),
            None => None,
        })
        .collect())
}

pub async fn create_project(
    client: &Client,
    new_project: super::NewProjectInput,
) -> Response<super::Project> {
    let id = gen_cuid().map_err(ResponseError::UnexpectedError)?;
    let project_entity =
        operations::create_doc(client, &get_project_path(&id), Project::new(new_project)).await?;

    Ok(Project::from(&project_entity))
}

pub async fn update_project(
    client: &Client,
    id: &ID,
    upd_project: super::UpdateProjectInput,
) -> Response<super::Project> {
    let mut project = get_project(client, id).await?.properties;
    for (k, v) in Project::update(upd_project).into_iter() {
        project.insert(k, v);
    }

    let project_entity = operations::update_doc(client, &get_project_path(id), project).await?;

    Ok(Project::from(&project_entity))
}
