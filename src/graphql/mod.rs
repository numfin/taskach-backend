pub mod handlers;
pub mod mutations;
pub mod queries;

use crate::firestore::client::ResponseError;
use juniper::{EmptySubscription, RootNode};

pub struct Context {
    pub client: crate::firestore::client::Client,
    pub jwt_claims: Result<crate::auth::Claims, ResponseError>,
}
impl juniper::Context for Context {}

pub type Schema =
    RootNode<'static, queries::QueryRoot, mutations::MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(
        queries::QueryRoot {},
        mutations::MutationRoot {},
        EmptySubscription::new(),
    )
}
