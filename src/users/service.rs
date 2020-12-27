use crate::datastore::prelude::*;

use juniper::ID;

pub async fn get_user(client: &Client, id: &ID) -> Response<Entity> {
    let entity = run_query_id(
        client,
        "Users",
        &[(KeyKind("Users"), KeyId::Cuid(&id.to_string()))],
    )
    .await
    .or(Err(ResponseError::NotFound(format!("User {}", id))))?;
    Ok(entity)
}

pub async fn get_all_users(client: &Client) -> Response<Vec<super::User>> {
    let query_batch = run_query(
        client,
        format!("SELECT * FROM Users"),
        Default::default(),
        Default::default(),
    )
    .await
    .or(Err(ResponseError::NotFound("User".into())))?;

    Ok(query_batch
        .entity_results
        .iter()
        .filter_map(|entity_result| match &entity_result.entity {
            Some(entity) => Some(super::doc_to_user(entity)),
            None => None,
        })
        .collect())
}

pub async fn create_user(client: &Client, new_user: super::NewUserInput) -> Response<super::User> {
    let existing_user = get_user_by_email(client, &new_user.email).await;
    if existing_user.is_ok() {
        return Err(ResponseError::AlreadyExists(
            "User with this e-mail already exists".to_string(),
        ));
    }
    let id = gen_cuid().map_err(ResponseError::UnexpectedError)?;
    let user_entity = create_doc(
        client,
        &[(KeyKind("Users"), KeyId::Cuid(&id))],
        super::new_user_to_fields(new_user).map_err(ResponseError::CreationError)?,
    )
    .await?;

    Ok(super::doc_to_user(&user_entity))
}

pub async fn update_user(
    client: &Client,
    id: &ID,
    upd_user: super::UpdateUserInput,
) -> Response<super::User> {
    let user = get_user(client, id).await?.properties;

    let user_entity = update_doc(
        client,
        &[(KeyKind("Users"), KeyId::Cuid(&id.to_string()))],
        super::update_user_to_fields(user, upd_user),
    )
    .await?;

    Ok(super::doc_to_user(&user_entity))
}

pub async fn get_user_by_email(client: &Client, email: &String) -> Response<super::User> {
    let query_batch = run_query(
        client,
        format!("SELECT * FROM Users WHERE email = @1 LIMIT 1"),
        Default::default(),
        &[to_db_string(email)],
    )
    .await
    .or(Err(ResponseError::NotFound("User".into())))?;

    let first_entity = extract_first_entity(&query_batch.entity_results);

    match first_entity {
        Some(v) => Ok(super::doc_to_user(&v)),
        None => Err(ResponseError::NotFound(format!("User {}", email))),
    }
}
