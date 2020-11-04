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

    let addr = "127.0.0.1:8081";
    println!("Graphiql working on http://{}/graphiql", addr);

    HttpServer::new(move || {
        App::new()
            .data(graphql::create_schema())
            .wrap(middleware::Logger::default())
            .configure(config::config)
    })
    .bind(addr)?
    .run()
    .await
}
