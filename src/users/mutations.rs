use crate::graphql::Context;
use juniper::{FieldError, FieldResult, ID};

pub struct MutationUsers;
#[juniper::graphql_object(Context = Context)]
impl MutationUsers {
    async fn register<'a>(
        new_user: super::NewUserInput,
        context: &Context,
    ) -> FieldResult<super::User> {
        super::service::create_user(&context.client, new_user)
            .await
            .map_err(FieldError::from)
    }

    async fn update(
        user_id: ID,
        updated_user: super::UpdateUserInput,
        context: &Context,
    ) -> FieldResult<super::User> {
        super::service::update_user(&context.client, user_id, updated_user)
            .await
            .map_err(FieldError::from)
    }
}
