use crate::graphql::Context;
use juniper::{FieldError, FieldResult, ID};

pub struct QueryStories;
#[juniper::graphql_object(
    Context = Context
)]
impl QueryStories {
    async fn getById(project_id: ID, story_id: ID, context: &Context) -> FieldResult<super::Story> {
        super::service::get_story(&context.client, &project_id, &story_id)
            .await
            .map_err(FieldError::from)
    }

    async fn getList(project_id: ID, context: &Context) -> FieldResult<Vec<super::Story>> {
        super::service::get_all_stories(&context.client, &project_id)
            .await
            .map_err(FieldError::from)
    }
}
