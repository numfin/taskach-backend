use crate::firestore::prelude::*;
use juniper::ID;
use operations::FindFilter;

pub async fn get_user(client: &Client, id: ID) -> Response<super::User> {
    let doc = operations::get_doc(client, format!("users/{}", id)).await?;
    Ok(super::doc_to_user(&doc))
}

pub async fn get_all_users(client: &Client) -> Response<Vec<super::User>> {
    let docs = operations::get_doc_list(client, "users".to_string()).await?;
    Ok(docs
        .iter()
        .map(super::doc_to_user)
        .collect::<Vec<super::User>>())
}

pub async fn create_user(client: &Client, new_user: super::NewUserInput) -> Response<super::User> {
    let existing_doc = operations::find_doc(
        client,
        "users",
        vec![FindFilter::Equal(
            "email",
            into_firestore_string(new_user.email.to_string()),
        )],
        Some(1),
        None,
    )
    .await;
    if existing_doc.is_ok() {
        return Err(ResponseError::AlreadyExists(
            "User with this e-mail already exists".to_string(),
        ));
    }
    let doc = operations::create_doc(
        client,
        "users".to_string(),
        super::new_user_to_fields(new_user).map_err(ResponseError::CreationError)?,
    )
    .await?;
    Ok(super::doc_to_user(&doc))
}

pub async fn update_user(
    client: &Client,
    id: ID,
    upd_user: super::UpdateUserInput,
) -> Response<super::User> {
    let doc = operations::update_doc(
        client,
        format!("users/{}", id),
        super::update_user_to_fields(upd_user),
    )
    .await?;

    Ok(super::doc_to_user(&doc))
}
