use crate::project::queries::QueryProjects;
use crate::user::queries::QueryUsers;

pub struct QueryRoot;
#[juniper::object]
impl QueryRoot {
    fn users(&self) -> QueryUsers {
        QueryUsers
    }
    fn projects(&self) -> QueryProjects {
        QueryProjects
    }
}
