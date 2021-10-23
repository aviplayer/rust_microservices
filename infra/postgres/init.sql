CREATE USER "local" WITH PASSWORD 'local' CREATEDB;

CREATE DATABASE "user_db"
    WITH
    OWNER = "local"
    ENCODING = 'UTF8'
    LC_COLLATE = 'en_US.utf8'
    LC_CTYPE = 'en_US.utf8'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1;
