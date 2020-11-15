use crate::firestore::prelude::*;

pub async fn get_user(client: &Client, id: &String) -> Response<super::User> {
    let doc = get_doc(client, format!("users/{}", id)).await?;
    Ok(super::doc_to_user(&doc))
}

pub async fn get_all_users(client: &Client) -> Response<Vec<super::User>> {
    let docs = get_doc_list(client, "users".to_string()).await?;
    Ok(docs
        .iter()
        .map(super::doc_to_user)
        .collect::<Vec<super::User>>())
}

pub async fn create_user(client: &Client, new_user: super::NewUserInput) -> Response<super::User> {
    let doc = create_doc(
        client,
        "users".to_string(),
        super::new_user_to_fields(new_user),
    )
    .await?;
    Ok(super::doc_to_user(&doc))
}

pub async fn update_user(
    client: &Client,
    id: &str,
    upd_user: super::UpdateUserInput,
) -> Response<super::User> {
    let doc = update_doc(
        client,
        format!("users/{}", id),
        super::update_user_to_fields(upd_user),
    )
    .await?;

    Ok(super::doc_to_user(&doc))
}
