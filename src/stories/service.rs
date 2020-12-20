use firestore::prelude::*;
use juniper::ID;
use operations::FindFilter;

use crate::firestore;

pub async fn get_story(client: &Client, project_id: &ID, story_id: &ID) -> Response<super::Story> {
    let doc = operations::get_doc(
        client,
        format!("projects/{}/stories/{}", project_id, story_id),
    )
    .await?;
    Ok(super::doc_to_story(&doc))
}

pub async fn get_all_stories(client: &Client, project_id: &ID) -> Response<Vec<super::Story>> {
    let docs = operations::get_doc_list(client, format!("projects/{}/stories", project_id)).await?;
    Ok(docs
        .iter()
        .map(super::doc_to_story)
        .collect::<Vec<super::Story>>())
}

pub async fn create_story(
    client: &Client,
    project_id: &ID,
    new_story: super::NewStoryInput,
) -> Response<super::Story> {
    let existing_doc = operations::find_doc(
        client,
        format!("projects/{}/stories", project_id).as_str(),
        vec![FindFilter::Equal(
            "name",
            into_firestore_string(new_story.name.to_string()),
        )],
        Some(1),
        None,
    )
    .await;
    if existing_doc.is_ok() {
        return Err(ResponseError::AlreadyExists(
            "Story with this name already exists".to_string(),
        ));
    }
    let doc = operations::create_doc(
        client,
        format!("projects/{}/stories", project_id),
        super::new_story_to_fields(new_story),
    )
    .await?;
    Ok(super::doc_to_story(&doc))
}

pub async fn update_story(
    client: &Client,
    project_id: &ID,
    story_id: &ID,
    upd_story: super::UpdateStoryInput,
) -> Response<super::Story> {
    let doc = operations::update_doc(
        client,
        format!("projects/{}/stories/{}", project_id, story_id),
        super::update_story_to_fields(upd_story),
    )
    .await?;
    Ok(super::doc_to_story(&doc))
}
