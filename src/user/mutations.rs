use diesel::result::{DatabaseErrorKind::UniqueViolation, Error::DatabaseError};
use juniper::{FieldError, FieldResult};

pub struct MutationUsers;
#[juniper::graphql_object]
impl MutationUsers {
    fn register(new_user: super::NewUserInput) -> FieldResult<super::User> {
        super::service::create_user(&new_user).map_err(|err| match err {
            DatabaseError(UniqueViolation, ..) => {
                FieldError::from(format!("User {} already exists", new_user.email))
            }
            err => FieldError::from(err),
        })
    }

    fn update(
        user_id: uuid::Uuid,
        updated_user: super::UpdateUserInput,
    ) -> FieldResult<super::User> {
        super::service::update_user(&user_id, &updated_user).map_err(FieldError::from)
    }
}
