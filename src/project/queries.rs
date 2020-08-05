use juniper::{FieldError, FieldResult};

pub struct QueryProjects;
#[juniper::object]
impl QueryProjects {
    /// Get project by project ID
    fn getById(id: uuid::Uuid) -> FieldResult<super::Project> {
        super::service::get_project(&id).map_err(FieldError::from)
    }

    /// Get list of projects
    fn getList() -> FieldResult<Vec<super::Project>> {
        super::service::get_all_projects().map_err(FieldError::from)
    }
}
