use diesel::result::{DatabaseErrorKind::UniqueViolation, Error::DatabaseError};
use juniper::{FieldError, FieldResult};

pub struct MutationStories;
#[juniper::object]
impl MutationStories {
    fn create(new_story: super::NewStoryInput) -> FieldResult<super::Story> {
        super::service::create_story(&new_story).map_err(|err| match err {
            DatabaseError(UniqueViolation, ..) => {
                FieldError::from(format!("Story {} already exists", new_story.name))
            }
            err => FieldError::from(err),
        })
    }

    fn update(
        story_id: uuid::Uuid,
        updated_story: super::UpdateStoryInput,
    ) -> FieldResult<super::Story> {
        super::service::update_story(&story_id, &updated_story).map_err(FieldError::from)
    }
}
