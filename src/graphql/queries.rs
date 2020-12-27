use super::Context;
// use crate::projects::queries::QueryProjects;
// use crate::stories::queries::QueryStories;
use crate::users::queries::QueryUsers;

pub struct QueryRoot;
#[juniper::graphql_object(
    Context = Context
)]
impl QueryRoot {
    fn users(&self) -> QueryUsers {
        QueryUsers
    }
    // fn projects(&self) -> QueryProjects {
    //     QueryProjects
    // }
    // fn stories(&self) -> QueryStories {
    //     QueryStories
    // }
}
