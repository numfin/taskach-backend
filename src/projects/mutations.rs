use crate::graphql::Context;
use juniper::{FieldError, FieldResult, ID};

pub struct MutationProjects;
#[juniper::graphql_object(Context = Context)]
impl MutationProjects {
    #[graphql(description = "Create project")]
    async fn create(
        new_project: super::NewProjectInput,
        context: &Context,
    ) -> FieldResult<super::Project> {
        super::service::create_project(&context.client, new_project)
            .await
            .map_err(FieldError::from)
    }

    #[graphql(description = "Update project")]
    async fn update(
        project_id: ID,
        updated_project: super::UpdateProjectInput,
        context: &Context,
    ) -> FieldResult<super::Project> {
        super::service::update_project(&context.client, project_id, updated_project)
            .await
            .map_err(FieldError::from)
    }
}
