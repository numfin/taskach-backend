mod jwt;
pub mod mutations;
pub mod pwd;
pub mod service;

use serde::{Deserialize, Serialize};

#[derive(juniper::GraphQLObject)]
pub struct Session {
    jwt: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct AuthenticationData {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}
