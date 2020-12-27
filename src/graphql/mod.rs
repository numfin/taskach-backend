pub mod handlers;
pub mod mutations;
pub mod queries;
pub mod utils;

use crate::auth::Claims;
use crate::datastore::client::{Client, ResponseError};
use juniper::{EmptySubscription, RootNode};

pub struct Context {
    pub client: Client,
    pub jwt_claims: Result<Claims, ResponseError>,
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
