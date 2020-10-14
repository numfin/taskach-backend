pub mod handlers;
pub mod mutations;
pub mod queries;

use juniper::{EmptySubscription, RootNode};

pub struct Context {}
impl juniper::Context for Context {}

type Schema =
    RootNode<'static, queries::QueryRoot, mutations::MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(
        queries::QueryRoot {},
        mutations::MutationRoot {},
        EmptySubscription::<Context>::new(),
    )
}
