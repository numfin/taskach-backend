use crate::graphql::Context;
use juniper::{FieldError, FieldResult, ID};

pub struct QueryUsers;
#[juniper::graphql_object(
    Context = Context
)]
impl QueryUsers {
    /// Get a HOOMAN
    async fn getById(user_id: ID, context: &Context) -> FieldResult<super::User> {
        println!("{:#?}", context.jwt_claims);
        super::service::get_user(&context.client, user_id)
            .await
            .map_err(FieldError::from)
    }

    /// Get list of HOOMANs
    async fn getList(context: &Context) -> FieldResult<Vec<super::User>> {
        super::service::get_all_users(&context.client)
            .await
            .map_err(FieldError::from)
    }
}
