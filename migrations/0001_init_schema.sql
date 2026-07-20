create table users (
                       id text primary key,
                       email text not null unique collate nocase,
                       name text,
                       status integer not null default 0,
                       created_at integer not null default (unixepoch()),
                       updated_at integer not null default (unixepoch())
);