use crate::graphql::Context;
use juniper::{FieldError, FieldResult};

pub struct QueryProjects;
#[juniper::graphql_object(Context = Context)]
impl QueryProjects {
    /// Get project by project ID
    async fn getById(project_id: String, context: &Context) -> FieldResult<super::Project> {
        super::service::get_project(&context.client, &project_id)
            .await
            .map_err(FieldError::from)
    }

    /// Get list of projects
    async fn getList(context: &Context) -> FieldResult<Vec<super::Project>> {
        super::service::get_all_projects(&context.client)
            .await
            .map_err(FieldError::from)
    }
}
