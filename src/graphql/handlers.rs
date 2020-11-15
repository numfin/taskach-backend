use actix_web::{web, Error, HttpResponse};
use juniper_actix::{graphql_handler, playground_handler};
use std::sync::Mutex;

pub async fn graphiql() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}

pub async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    app_data: web::Data<Mutex<crate::AppData>>,
) -> Result<HttpResponse, Error> {
    let data = app_data.lock().map_err(|e| println!("{:#?}", e)).unwrap();
    let context = super::Context {
        client: data.firestore_client.clone(),
    };
    graphql_handler(&data.graphql_schema, &context, req, payload).await
}
