#[macro_use]
extern crate diesel;

mod config;
mod db;
mod graphql;
mod project;
mod scalars;
// mod story;
mod user;

use actix_web::{middleware, App, HttpServer};
use std::{env, io};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let schema = std::sync::Arc::new(graphql::create_schema());

    println!("Graphiql working on http://localhost:8080/graphiql");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .configure(config::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
