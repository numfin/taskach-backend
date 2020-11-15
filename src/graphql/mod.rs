pub mod handlers;
pub mod mutations;
pub mod queries;

use juniper::{EmptySubscription, RootNode};

pub struct Context {
    pub client: crate::firestore::client::Client,
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
