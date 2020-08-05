use crate::db;
use diesel::{prelude::*, QueryResult};

pub fn get_story(id: &uuid::Uuid) -> QueryResult<super::Story> {
    use crate::db::schema::stories::dsl::stories;
    stories.find(id).first(&db::connect())
}

pub fn get_all_stories() -> QueryResult<Vec<super::Story>> {
    use crate::db::schema::stories::dsl::stories;

    stories.limit(10).load(&db::connect())
}

pub fn create_story(new_story: &super::NewStoryInput) -> QueryResult<super::Story> {
    use crate::db::schema::stories::dsl::stories;

    diesel::insert_into(stories)
        .values(new_story)
        .get_result(&db::connect())
}

pub fn update_story(id: &uuid::Uuid, upd_story: &super::UpdateStoryInput) -> QueryResult<super::Story> {
    use crate::db::schema::stories::dsl::stories;
    diesel::update(stories.find(id))
        .set(upd_story)
        .get_result(&db::connect())
}
