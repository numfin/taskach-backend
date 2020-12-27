use super::Context;
// use crate::projects::mutations::MutationProjects;
// use crate::stories::mutations::MutationStories;
use crate::users::mutations::MutationUsers;

pub struct MutationRoot;
#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    fn users(&self) -> MutationUsers {
        MutationUsers
    }
    // fn projects(&self) -> MutationProjects {
    //     MutationProjects
    // }
    // fn stories(&self) -> MutationStories {
    //     MutationStories
    // }
}
