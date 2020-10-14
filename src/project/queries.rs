use juniper::{FieldError, FieldResult};

pub struct QueryProjects;
#[juniper::graphql_object]
impl QueryProjects {
    /// Get project by project ID
    fn getById(project_id: uuid::Uuid) -> FieldResult<super::Project> {
        super::service::get_project(&project_id).map_err(FieldError::from)
    }

    /// Get list of projects
    fn getList() -> FieldResult<Vec<super::Project>> {
        super::service::get_all_projects().map_err(FieldError::from)
    }
}
