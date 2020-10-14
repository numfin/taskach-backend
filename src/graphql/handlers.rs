use crate::graphql::Schema;
use actix_web::{web, Error, HttpResponse};
use juniper_actix::{graphql_handler, playground_handler};

pub async fn graphiql() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}

pub async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = &super::Context {};
    graphql_handler(&schema, context, req, payload).await
}
