CREATE TABLE if not exists users
(
    id         integer     not null,
    login      varchar(50),
    node_id    varchar(50),
    avatar_url varchar(200),
    site_admin boolean,
    unique (id),
    unique (login)
);
