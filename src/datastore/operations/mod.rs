mod create_doc;
mod delete_doc;
mod run_query;
mod update_doc;
pub mod utils;

pub mod operations {
    pub use super::create_doc::*;
    pub use super::delete_doc::*;
    pub use super::run_query::*;
    pub use super::update_doc::*;
}
