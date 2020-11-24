use crate::firestore::prelude::*;
use crate::firestore::structured_query::{
    field_filter, filter::FilterType, FieldFilter, FieldReference, Filter,
};
use actix_web::HttpRequest;

pub async fn authenticate(
    client: &Client,
    auth_data: super::AuthenticationData,
) -> Response<super::Session> {
    // Getting user from db
    let doc = find_doc(
        client,
        "users".to_string(),
        Filter {
            filter_type: Some(FilterType::FieldFilter(FieldFilter {
                field: Some(FieldReference {
                    field_path: "email".to_string(),
                }),
                op: field_filter::Operator::Equal.into(),
                value: Some(into_firestore_string(auth_data.email)),
            })),
        },
    )
    .await?;
    // Converting user from dbref to struct
    let user = crate::users::doc_to_user(&doc);
    if !user.active {
        return Err(ResponseError::AuthError("User is not verified".to_string()));
    }
    let password_hash = &user.password_hash;
    // If password and password_hash is valid - create jwt from user
    if super::pwd::verify_pwd_hash(password_hash.to_string(), auth_data.password) {
        let jwt = super::jwt::token_from(user).map_err(|err| ResponseError::AuthError(err))?;
        Ok(super::Session { jwt })
    } else {
        Err(ResponseError::AuthError(
            "E-mail or password is invalid".to_string(),
        ))
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
