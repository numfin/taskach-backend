use crate::graphql;
use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(|| HttpResponse::Ok())))
        .service(web::resource("/graphql").route(web::post().to(graphql::handlers::graphql)))
        .service(web::resource("/graphiql").route(web::get().to(graphql::handlers::graphiql)));
}
