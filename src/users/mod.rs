pub mod mutations;
pub mod queries;
pub mod service;

use crate::auth::pwd::create_pwd_hash;
use crate::datastore::prelude::*;
use chrono::prelude::*;
use juniper::ID;

#[derive(juniper::GraphQLObject, Debug, Clone)]
#[graphql(description = "A user in a taskach system")]
pub struct User {
    pub id: ID,
    /// firstname
    pub first_name: String,
    /// lastname
    pub last_name: String,
    /// email
    pub email: String,
    /// phone
    pub phone: String,
    /// user is enabled
    pub active: bool,
    #[graphql(skip)]
    pub password_hash: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&Entity> for User {
    fn from(entity: &Entity) -> Self {
        Self {
            id: DbValue::Id(entity).into(),
            first_name: DbValue::Str("first_name", entity).into(),
            last_name: DbValue::Str("last_name", entity).into(),
            email: DbValue::Str("email", entity).into(),
            phone: DbValue::Str("phone", entity).into(),
            active: DbValue::Bool("active", entity).into(),
            password_hash: DbValue::Blob("password_hash", entity).into(),
            created_at: DbValue::Timestamp("created_at", entity).into(),
            updated_at: DbValue::Timestamp("updated_at", entity).into(),
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewUserInput {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    password: String,
}
#[derive(juniper::GraphQLInputObject)]
pub struct UpdateUserInput {
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
}

impl User {
    fn new(user: NewUserInput) -> Result<DbProperties, String> {
        let password = create_pwd_hash(user.password)?;

        let db_values = fields_to_db_values(&[
            AppValue::Str("first_name", Some(user.first_name)),
            AppValue::Str("last_name", Some(user.last_name)),
            AppValue::Str("email", Some(user.email)),
            AppValue::Str("phone", Some(user.phone)),
            AppValue::Byte("password_hash", Some(password)),
            AppValue::Bool("active", Some(true)),
        ]);
        Ok(db_values)
    }

    fn update(fields_to_update: UpdateUserInput) -> DbProperties {
        fields_to_db_values(&[
            AppValue::Str("first_name", fields_to_update.first_name),
            AppValue::Str("last_name", fields_to_update.last_name),
            AppValue::Str("phone", fields_to_update.phone),
        ])
    }
}
