mod app_env;
mod auth;
mod config;
mod datastore;
mod graphql;
mod mail;
mod projects;
mod scalars;
mod stories;
mod users;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use app_env::get_env;
use std::{env, io, sync::Mutex};

pub struct AppData {
    graphql_schema: graphql::Schema,
    datastore_client: datastore::client::Client,
}
#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Starting...");
    let app_start_time = chrono::prelude::Utc::now().timestamp_millis();
    app_env::check_env();
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let datastore_client = crate::datastore::client::create_service()
        .await
        .map_err(|err| println!("{:?}", err))
        .unwrap();
    let port = get_env::port();
    let addr = format!("0.0.0.0:{}", port);
    let data = web::Data::new(Mutex::new(AppData {
        graphql_schema: graphql::create_schema(),
        datastore_client,
    }));
    let app_connection_time = chrono::prelude::Utc::now().timestamp_millis();

    println!("App is ready in {}ms", app_connection_time - app_start_time);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .configure(config::config)
    })
    .bind(addr)?
    .run()
    .await
}
