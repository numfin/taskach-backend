use juniper::{FieldError, FieldResult};

pub struct QueryUsers;
#[juniper::object]
impl QueryUsers {
    /// Get a HOOMAN
    fn getById(id: uuid::Uuid) -> FieldResult<super::User> {
        super::service::get_user(&id).map_err(FieldError::from)
    }

    /// Get list of HOOMANs
    fn getList() -> FieldResult<Vec<super::User>> {
        super::service::get_all_users().map_err(FieldError::from)
    }
}
