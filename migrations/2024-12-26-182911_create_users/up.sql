
create table public.users
(
    id        text         not null
        primary key,
    email     varchar(100) not null
        constraint unique_email
            unique,
    birthdate text         not null,
    firstname varchar(100) not null,
    lastname  varchar(100) not null,
    username  varchar(100) not null
        constraint unique_username
            unique,
    password  text         not null
);

alter table public.users
    owner to postgres;

create unique index users_email_username_uindex
    on public.users (email, username);

