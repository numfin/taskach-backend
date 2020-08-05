create extension if not exists "uuid-ossp";

-- users
create table positions (
    id uuid default uuid_generate_v4() primary key,
    name varchar(50) not null unique,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
SELECT diesel_manage_updated_at('positions');

create table users (
    id uuid default uuid_generate_v4() primary key,
    first_name varchar(50) not null,
    last_name varchar(50) not null,
    email varchar(255) not null unique,
    phone varchar(20) not null,
    active boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
SELECT diesel_manage_updated_at('users');

create table position_users (
    id uuid default uuid_generate_v4() primary key,
    position_id uuid not null references positions(id),
    users_id uuid not null references users(id)
);

-- projects
create table projects (
    id uuid default uuid_generate_v4() primary key,
    name varchar(50) not null unique,
    description text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
SELECT diesel_manage_updated_at('projects');


-- sprints
create table sprints (
    id uuid default uuid_generate_v4() primary key,
    name varchar(255) not null unique,
    starts_at timestamptz not null,
    ends_at timestamptz not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
SELECT diesel_manage_updated_at('sprints');


-- stories --
create table story_types (
    id uuid default uuid_generate_v4() primary key,
    name varchar(50) not null unique,
    position int2 not null check(position >= 0),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
SELECT diesel_manage_updated_at('story_types');

create table story_statuses (
    id uuid default uuid_generate_v4() primary key,
    name varchar(50) not null unique,
    position int2 not null check(position >= 0),
    story_type_id uuid not null references story_types(id),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
SELECT diesel_manage_updated_at('story_statuses');

create table stories (
    id uuid default uuid_generate_v4() primary key,
    name varchar(255) not null unique,
    description text not null,

    story_type_id uuid not null references story_types,
    story_status_id uuid not null references story_statuses,
    sprint_id uuid references sprints,

    creator_id uuid not null references users,
    project_id uuid not null references projects,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
SELECT diesel_manage_updated_at('stories');


-- tasks
create table task_types (
    id uuid default uuid_generate_v4() primary key,
    name varchar(30) not null unique,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
SELECT diesel_manage_updated_at('task_types');

create table processes (
    id uuid default uuid_generate_v4() primary key,
    name varchar(30) not null unique,
    name_active varchar(30) not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
SELECT diesel_manage_updated_at('processes');

-- here we store processes order for each task_type
create table process_stages (
    id uuid default uuid_generate_v4() primary key,
    task_type_id uuid not null references task_types,
    process_id uuid references processes unique,

    next_process_id uuid references processes,
    cancel_process_id uuid references processes,

    position int2 not null check(position >= 0),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
SELECT diesel_manage_updated_at('process_stages');

create table tasks (
    id uuid default uuid_generate_v4() primary key,
    name varchar(255) not null,
    description text not null,
    project_id uuid not null references projects(id),
    -- parent story
    story_id uuid not null references stories,
    -- user who created story
    creator_id uuid not null references users,
    -- user who is assigned to this task at this moment
    responsible_id uuid not null references users,
    -- type of the task dev, design, etc.
    task_type_id uuid not null references task_types,
    
    -- status of the task
    process_id uuid not null references processes,
    -- state of the status process (active or not)
    process_active boolean default false,
    
    -- task estimation for each process
    estimation_dev interval default interval '0',
    estimation_check interval default interval '0',
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
SELECT diesel_manage_updated_at('tasks');

create table task_wastes (
    id uuid default uuid_generate_v4() primary key,

    task_id uuid not null references tasks,
    user_id uuid not null references users,

    wasted_from timestamptz not null,
    wasted_to timestamptz not null,
    wasted interval not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);
SELECT diesel_manage_updated_at('task_wastes');
