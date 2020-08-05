use crate::graphql::Schema;
use actix_web::{web, Error, HttpResponse};
use juniper::http::{playground::playground_source, GraphQLRequest};
use std::sync::Arc;

pub async fn graphiql() -> HttpResponse {
    let html = playground_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}
