CREATE EXTENSION IF NOT EXISTS citext;

CREATE TABLE IF NOT EXISTS clients (
    id serial NOT NULL PRIMARY KEY,
    email citext UNIQUE NOT NULL,
    password_hash bytea NOT NULL
    );

