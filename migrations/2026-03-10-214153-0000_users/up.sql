create table users (
    id integer not null,
    name text not null,
    email text not null unique,
    hashed_password text not null,

    primary key (id)
)
