use super::{
    service::{get_user_path, update_user},
    UpdateUserInput,
};
use crate::datastore::prelude::*;
use juniper::ID;
use utils::PathToRef;

pub struct PendingUser {
    pub user_id: ID,
    pub email: String,
}
impl PendingUser {
    fn new(user: Self) -> DbProperties {
        fields_to_db_values(&[
            AppValue::Ref("user_id", Some(&get_user_path(&user.user_id))),
            AppValue::Str("email", Some(user.email)),
        ])
    }
}
impl From<&Entity> for PendingUser {
    fn from(entity: &Entity) -> Self {
        Self {
            user_id: DbValue::Key("user_id", entity).into(),
            email: DbValue::Str("email", entity).into(),
        }
    }
}

fn get_pending_user_path<'a>(id: &ID) -> PathToRef<'a> {
    vec![(KeyKind("PendingUsers"), KeyId::Cuid(id.clone()))]
}

pub async fn activate_user(client: &Client, pending_id: &ID, email: &String) -> Response<()> {
    let query_batch = operations::run_query(
        client,
        format!("SELECT * FROM PendingUsers WHERE __key__ = @1 AND email = @2 LIMIT 1"),
        Default::default(),
        &[
            insert::to_db_key(&get_pending_user_path(pending_id)),
            insert::to_db_string(email),
        ],
    )
    .await
    .or(Err(ResponseError::NotFound(
        "User is not waiting activation".into(),
    )))?;
    if let Some(entity) = &operations::extract_first_entity(&query_batch.entity_results) {
        let user = PendingUser::from(entity);
        update_user(
            client,
            &user.user_id,
            UpdateUserInput {
                active: Some(true),
                ..Default::default()
            },
        )
        .await?;
        Ok(())
    } else {
        Err(ResponseError::NotFound(
            "User is not waiting activation".into(),
        ))
    }
}

pub async fn prepare_user_activation(client: &Client, user: PendingUser) -> Response<ID> {
    let id = gen_cuid().map_err(ResponseError::UnexpectedError)?;
    operations::create_doc(client, &get_pending_user_path(&id), PendingUser::new(user)).await?;

    Ok(id)
}
