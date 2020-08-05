use crate::db;
use diesel::{prelude::*, QueryResult};

pub fn get_user(id: &uuid::Uuid) -> QueryResult<super::User> {
    use crate::db::schema::users::dsl::users;
    users.find(id).first(&db::connect())
}

pub fn get_all_users() -> QueryResult<Vec<super::User>> {
    use crate::db::schema::users::dsl::users;

    users.limit(10).load(&db::connect())
}

pub fn create_user(new_user: &super::NewUserInput) -> QueryResult<super::User> {
    use crate::db::schema::users::dsl::users;

    diesel::insert_into(users)
        .values(new_user)
        .get_result(&db::connect())
}

pub fn update_user(id: &uuid::Uuid, upd_user: &super::UpdateUserInput) -> QueryResult<super::User> {
    use crate::db::schema::users::dsl::users;
    diesel::update(users.find(id))
        .set(upd_user)
        .get_result(&db::connect())
}
