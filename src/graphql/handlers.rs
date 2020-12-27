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
    let jwt_claims = crate::auth::service::verify_session(&req);
    let app_data = app_data.lock().map_err(|e| println!("{:#?}", e)).unwrap();
    let context = super::Context {
        client: app_data.datastore_client.clone(),
        jwt_claims,
    };
    graphql_handler(&app_data.graphql_schema, &context, req, payload).await
}
