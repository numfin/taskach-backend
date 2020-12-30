use std::collections::HashMap;

use super::Story;
use crate::datastore::prelude::*;
use crate::projects::service::get_project_path;
use juniper::ID;
use utils::PathToRef;

pub fn get_story_path<'a>(project_id: &'a ID, story_id: &'a ID) -> PathToRef<'a> {
    let mut project_path = get_project_path(project_id);
    project_path.push((KeyKind("Stories"), KeyId::Cuid(story_id.to_string())));
    project_path
}

pub async fn get_story(client: &Client, project_id: &ID, story_id: &ID) -> Response<Entity> {
    let entity = operations::run_query_id(client, "Stories", &get_story_path(project_id, story_id))
        .await
        .or(Err(ResponseError::NotFound(format!(
            "Story {} in Project {}",
            story_id, project_id
        ))))?;
    Ok(entity)
}

pub struct StoryFilters {
    pub name: Option<String>,
}

fn build_story_filters<'a>(
    mut query: String,
    filters: Option<StoryFilters>,
) -> (String, DbProperties) {
    if let Some(story_filters) = filters {
        if let Some(_) = story_filters.name {
            query = format!("{} name = @name", query);
        }
        let params = fields_to_db_values(&[AppValue::Str("name", story_filters.name)]);
        return (query, params);
    }
    return (query, HashMap::new());
}

pub async fn get_stories_of_project(
    client: &Client,
    project_id: &ID,
    filters: Option<StoryFilters>,
) -> Response<Vec<Story>> {
    let query = format!("SELECT * FROM Stories where __key__ HAS ANCESTOR @1");
    let (query, named_bindings) = build_story_filters(query, filters);

    let results_iter = operations::run_query(
        client,
        query,
        named_bindings,
        &[insert::to_db_key(&get_project_path(project_id))],
    )
    .await
    .or(Err(ResponseError::NotFound("Story".into())))?
    .entity_results
    .into_iter()
    .filter_map(|entity_result| entity_result.entity)
    .map(Story::from);

    Ok(results_iter.collect())
}

pub async fn create_story(
    client: &Client,
    project_id: &ID,
    new_story: super::NewStoryInput,
) -> Response<super::Story> {
    let existing_user = get_story_by_name(client, project_id, &new_story.name).await;
    if existing_user.is_ok() {
        return Err(ResponseError::AlreadyExists(
            "Story with this name already exists".to_string(),
        ));
    }
    let id = gen_cuid().map_err(ResponseError::UnexpectedError)?;
    let story_entity = operations::create_doc(
        client,
        &get_story_path(project_id, &id),
        Story::new(new_story),
    )
    .await?;

    Ok(Story::from(story_entity))
}

pub async fn update_story(
    client: &Client,
    project_id: &ID,
    story_id: &ID,
    upd_story: super::UpdateStoryInput,
) -> Response<super::Story> {
    let mut story = get_story(client, project_id, story_id).await?.properties;
    for (k, v) in Story::update(upd_story).into_iter() {
        story.insert(k, v);
    }

    let story_entity =
        operations::update_doc(client, &get_story_path(project_id, story_id), story).await?;

    Ok(Story::from(story_entity))
}

pub async fn get_story_by_name(client: &Client, project_id: &ID, name: &String) -> Response<Story> {
    let query_batch = operations::run_query(
        client,
        format!("SELECT * FROM Stories WHERE __key__ HAS ANCESTOR @1 AND name = @2  LIMIT 1"),
        Default::default(),
        &[
            insert::to_db_key(&get_project_path(project_id)),
            insert::to_db_string(name),
        ],
    )
    .await
    .or(Err(ResponseError::NotFound("User".into())))?;

    match operations::extract_first_entity(&query_batch.entity_results) {
        Some(v) => Ok(Story::from(v)),
        None => Err(ResponseError::NotFound(format!("User {}", name))),
    }
}
