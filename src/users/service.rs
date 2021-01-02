use super::{
    pending::{prepare_user_activation, PendingUser},
    User,
};
use crate::{
    datastore::prelude::*,
    mail::{send_mail, templates::MailTemplate},
};
use futures::join;
use juniper::ID;
use utils::PathToRef;

pub fn get_user_path<'a>(id: &ID) -> PathToRef<'a> {
    vec![(KeyKind("Users"), KeyId::Cuid(id.clone()))]
}

pub async fn get_user(client: &Client, id: &ID) -> Response<Entity> {
    let entity = operations::run_query_id(client, "Users", &get_user_path(id))
        .await
        .or(Err(ResponseError::NotFound(format!("User {}", id))))?;
    Ok(entity)
}

pub async fn get_all_users(client: &Client) -> Response<Vec<User>> {
    let query_batch = operations::run_query(
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
            Some(entity) => Some(User::from(entity)),
            None => None,
        })
        .collect())
}

pub async fn create_user(client: &Client, new_user: super::NewUserInput) -> Response<User> {
    let existing_user = get_user_by_email(client, &new_user.email).await;
    if existing_user.is_ok() {
        return Err(ResponseError::AlreadyExists(
            "User with this e-mail already exists".to_string(),
        ));
    }
    let id = gen_cuid().map_err(ResponseError::UnexpectedError)?;
    let user = User::new(new_user.clone()).map_err(ResponseError::CreationError)?;

    let user_path = get_user_path(&id.clone());
    let create_user = operations::create_doc(client, &user_path, user);
    let set_activation = prepare_user_activation(
        client,
        PendingUser {
            email: new_user.email.clone(),
            user_id: id.clone(),
        },
    );
    if let (Ok(user_entity), Ok(activation_id)) = join!(create_user, set_activation) {
        send_mail(
            &new_user.email,
            MailTemplate::register_confirmation(&activation_id, &new_user.email),
        )
        .await
        .map_err(|err| {
            println!("{}", err);
            ResponseError::UnexpectedError("Cannot send email".into())
        })?;

        Ok(User::from(&user_entity))
    } else {
        Err(ResponseError::CreationError(
            "Cannot create user/activate it".into(),
        ))
    }
}

pub async fn update_user(
    client: &Client,
    id: &ID,
    upd_user: super::UpdateUserInput,
) -> Response<User> {
    let mut user = get_user(client, id).await?.properties;
    for (k, v) in User::update(upd_user).into_iter() {
        user.insert(k, v);
    }

    let user_entity = operations::update_doc(client, &get_user_path(id), user).await?;

    Ok(User::from(&user_entity))
}

pub async fn get_user_by_email(client: &Client, email: &String) -> Response<User> {
    let query_batch = operations::run_query(
        client,
        format!("SELECT * FROM Users WHERE email = @1 LIMIT 1"),
        Default::default(),
        &[insert::to_db_string(email)],
    )
    .await
    .or(Err(ResponseError::NotFound("User".into())))?;

    match operations::extract_first_entity(&query_batch.entity_results) {
        Some(v) => Ok(User::from(&v)),
        None => Err(ResponseError::NotFound(format!("User {}", email))),
    }
}
