mod config;
mod firestore;
mod graphql;
mod project;
mod scalars;
mod user;
// mod story;

use actix_web::{middleware, web, App, HttpServer};
use std::{env, io, sync::Mutex};

pub struct AppData {
    graphql_schema: graphql::Schema,
    firestore_client: firestore::client::Client,
}
#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let firestore_client = crate::firestore::client::create_service()
        .await
        .map_err(|err| println!("{:?}", err))
        .unwrap();
    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(_) => "8081".to_string(),
    };
    let addr = format!("0.0.0.0:{}", port);
    let data = web::Data::new(Mutex::new(AppData {
        graphql_schema: graphql::create_schema(),
        firestore_client,
    }));
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .configure(config::config)
    })
    .bind(addr)?
    .run()
    .await
}
