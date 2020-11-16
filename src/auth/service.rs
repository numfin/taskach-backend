use crate::firestore::prelude::*;
use crate::firestore::structured_query::{
    field_filter, filter::FilterType, FieldFilter, FieldReference, Filter,
};

pub async fn authenticate(
    client: &Client,
    auth_data: super::AuthenticationData,
) -> Response<super::Session> {
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
    let user = crate::users::doc_to_user(&doc);
    let password_hash = &user.password_hash;
    if super::pwd::verify_pwd_hash(password_hash.to_string(), auth_data.password) {
        let jwt = super::jwt::token_from(vec![
            ("id", user.id.to_string()),
            ("email", user.email.to_string()),
            ("first_name", user.first_name.to_string()),
            ("last_name", user.last_name.to_string()),
            ("phone", user.phone.to_string()),
        ])
        .map_err(|err| ResponseError::AuthError(err))?;
        Ok(super::Session { jwt })
    } else {
        Err(ResponseError::AuthError(
            "E-mail or password is invalid".to_string(),
        ))
    }
}
