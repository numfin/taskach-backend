use crate::{datastore::prelude::*, users::service::get_user_by_email};
use actix_web::HttpRequest;

pub async fn authenticate(
    client: &Client,
    auth_data: super::AuthenticationData,
) -> Response<super::Session> {
    // Getting user from db
    let user = get_user_by_email(client, &auth_data.email).await?;

    if !user.active {
        return Err(ResponseError::AuthError("User is not verified".to_string()));
    }
    let password_hash = &user.password_hash;
    // If password and password_hash is valid - create jwt from user
    if super::pwd::verify_pwd_hash(&password_hash, &auth_data.password) {
        let jwt = super::jwt::token_from(user).map_err(|err| ResponseError::AuthError(err))?;
        Ok(super::Session { jwt })
    } else {
        Err(ResponseError::AuthError("Invalid password".to_string()))
    }
}

pub fn verify_session(req: &HttpRequest) -> Response<super::Claims> {
    let claims = req
        .headers()
        .get("Authorization")
        .map_or(Ok(""), |x| x.to_str())
        .map_err(|err| err.to_string())
        .and_then(|token_value| super::jwt::verify_token(token_value))
        .map_err(|err| ResponseError::AuthError(err))?;

    Ok(claims)
}
