mod jwt;
pub mod mutations;
pub mod pwd;
pub mod service;

#[derive(juniper::GraphQLObject)]
pub struct Session {
    jwt: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct AuthenticationData {
    email: String,
    password: String,
}
