use crate::graphql::Context;
use juniper::{FieldError, FieldResult};

pub struct MutationAuth;
#[juniper::graphql_object(Context = Context)]
impl MutationAuth {
    async fn login(
        auth: super::AuthenticationData,
        context: &Context,
    ) -> FieldResult<super::Session> {
        super::service::authenticate(&context.client, auth)
            .await
            .map_err(FieldError::from)
    }
}
