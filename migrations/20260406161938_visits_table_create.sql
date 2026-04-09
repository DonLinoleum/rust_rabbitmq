-- Add migration script here
create table if not exists visits(
    id serial primary key,
    ip varchar(255) null,
    date timestamp with time zone default now(),
    score int not null,
    level int not null,
    name varchar(255) null
);