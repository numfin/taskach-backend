use crate::db;
use diesel::{prelude::*, QueryResult};

pub fn get_project(id: &uuid::Uuid) -> QueryResult<super::Project> {
    use crate::db::schema::projects::dsl::projects;
    projects.find(id).first(&db::connect())
}

pub fn get_all_projects() -> QueryResult<Vec<super::Project>> {
    use crate::db::schema::projects::dsl::projects;

    projects.limit(10).load(&db::connect())
}

pub fn create_project(new_project: &super::NewProjectInput) -> QueryResult<super::Project> {
    use crate::db::schema::projects::dsl::projects;

    diesel::insert_into(projects)
        .values(new_project)
        .get_result(&db::connect())
}

pub fn update_project(
    id: &uuid::Uuid,
    upd_project: &super::UpdateProjectInput,
) -> QueryResult<super::Project> {
    use crate::db::schema::projects::dsl::projects;
    diesel::update(projects.find(id))
        .set(upd_project)
        .get_result(&db::connect())
}
