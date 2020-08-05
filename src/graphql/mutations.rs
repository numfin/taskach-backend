use crate::project::mutations::MutationProjects;
use crate::user::mutations::MutationUsers;

pub struct MutationRoot;
#[juniper::object]
impl MutationRoot {
    fn users(&self) -> MutationUsers {
        MutationUsers
    }
    fn projects(&self) -> MutationProjects {
        MutationProjects
    }
}
