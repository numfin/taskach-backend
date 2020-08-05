use juniper::{FieldError, FieldResult};

pub struct MutationProjects;
#[juniper::object]
impl MutationProjects {
    #[graphql(description = "Create project")]
    fn create(new_project: super::NewProjectInput) -> FieldResult<super::Project> {
        super::service::create_project(&new_project).map_err(|err| match err {
            diesel::NotFound => FieldError::from("Project with current name already exists"),
            err => FieldError::from(err),
        })
    }

    #[graphql(description = "Update project")]
    fn update(
        project_id: uuid::Uuid,
        updated_project: super::UpdateProjectInput,
    ) -> FieldResult<super::Project> {
        super::service::update_project(&project_id, &updated_project).map_err(FieldError::from)
    }
}
