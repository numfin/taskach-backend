table! {
    position_users (id) {
        id -> Uuid,
        position_id -> Uuid,
        users_id -> Uuid,
    }
}

table! {
    positions (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    process_stages (id) {
        id -> Uuid,
        task_type_id -> Uuid,
        process_id -> Nullable<Uuid>,
        next_process_id -> Nullable<Uuid>,
        cancel_process_id -> Nullable<Uuid>,
        position -> Int2,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    processes (id) {
        id -> Uuid,
        name -> Varchar,
        name_active -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    projects (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    sprints (id) {
        id -> Uuid,
        name -> Varchar,
        starts_at -> Timestamptz,
        ends_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    stories (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Text,
        story_type_id -> Uuid,
        story_status_id -> Uuid,
        sprint_id -> Nullable<Uuid>,
        creator_id -> Uuid,
        project_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    story_statuses (id) {
        id -> Uuid,
        name -> Varchar,
        position -> Int2,
        story_type_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    story_types (id) {
        id -> Uuid,
        name -> Varchar,
        position -> Int2,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    task_types (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    task_wastes (id) {
        id -> Uuid,
        task_id -> Uuid,
        user_id -> Uuid,
        wasted_from -> Timestamptz,
        wasted_to -> Timestamptz,
        wasted -> Interval,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    tasks (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Text,
        project_id -> Uuid,
        story_id -> Uuid,
        creator_id -> Uuid,
        responsible_id -> Uuid,
        task_type_id -> Uuid,
        process_id -> Uuid,
        process_active -> Nullable<Bool>,
        estimation_dev -> Nullable<Interval>,
        estimation_check -> Nullable<Interval>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(position_users -> positions (position_id));
joinable!(position_users -> users (users_id));
joinable!(process_stages -> task_types (task_type_id));
joinable!(stories -> projects (project_id));
joinable!(stories -> sprints (sprint_id));
joinable!(stories -> story_statuses (story_status_id));
joinable!(stories -> story_types (story_type_id));
joinable!(stories -> users (creator_id));
joinable!(story_statuses -> story_types (story_type_id));
joinable!(task_wastes -> tasks (task_id));
joinable!(task_wastes -> users (user_id));
joinable!(tasks -> processes (process_id));
joinable!(tasks -> projects (project_id));
joinable!(tasks -> stories (story_id));
joinable!(tasks -> task_types (task_type_id));

allow_tables_to_appear_in_same_query!(
    position_users,
    positions,
    process_stages,
    processes,
    projects,
    sprints,
    stories,
    story_statuses,
    story_types,
    task_types,
    task_wastes,
    tasks,
    users,
);
