CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table todos (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    title varchar not null,
    completed boolean not null,
    created_at timestamptz not null,
    updated_at timestamptz not null
);