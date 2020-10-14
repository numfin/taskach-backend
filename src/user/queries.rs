use crate::graphql::Context;
use juniper::{FieldError, FieldResult};

pub struct QueryUsers;
#[juniper::graphql_object(
    Context = Context
)]
impl QueryUsers {
    /// Get a HOOMAN
    fn getById(user_id: uuid::Uuid) -> FieldResult<super::User> {
        super::service::get_user(&user_id).map_err(FieldError::from)
    }

    /// Get list of HOOMANs
    fn getList() -> FieldResult<Vec<super::User>> {
        super::service::get_all_users().map_err(FieldError::from)
    }
}
