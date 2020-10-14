use super::Context;
use crate::project::queries::QueryProjects;
use crate::user::queries::QueryUsers;

pub struct QueryRoot;
#[juniper::graphql_object(
    Context = Context
)]
impl QueryRoot {
    fn users(&self) -> QueryUsers {
        QueryUsers
    }
    fn projects(&self) -> QueryProjects {
        QueryProjects
    }
}
