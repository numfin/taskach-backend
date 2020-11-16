use super::Context;
use crate::auth::mutations::MutationAuth;
use crate::projects::mutations::MutationProjects;
use crate::users::mutations::MutationUsers;

pub struct MutationRoot;
#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    fn auth(&self) -> MutationAuth {
        MutationAuth
    }
    fn users(&self) -> MutationUsers {
        MutationUsers
    }
    fn projects(&self) -> MutationProjects {
        MutationProjects
    }
}
