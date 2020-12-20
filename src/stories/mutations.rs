use crate::graphql::Context;
use juniper::{FieldError, FieldResult, ID};

pub struct MutationStories;
#[juniper::graphql_object(Context = Context)]
impl MutationStories {
    async fn create(
        project_id: ID,
        new_story: super::NewStoryInput,
        context: &Context,
    ) -> FieldResult<super::Story> {
        super::service::create_story(&context.client, &project_id, new_story)
            .await
            .map_err(FieldError::from)
    }

    async fn update(
        project_id: ID,
        story_id: ID,
        updated_story: super::UpdateStoryInput,
        context: &Context,
    ) -> FieldResult<super::Story> {
        super::service::update_story(&context.client, &project_id, &story_id, updated_story)
            .await
            .map_err(FieldError::from)
    }
}
