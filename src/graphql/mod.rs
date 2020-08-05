pub mod handlers;
pub mod mutations;
pub mod queries;

use juniper::RootNode;

type Schema = RootNode<'static, queries::QueryRoot, mutations::MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(queries::QueryRoot {}, mutations::MutationRoot {})
}
