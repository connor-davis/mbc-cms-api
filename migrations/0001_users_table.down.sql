-- Add down migration script here
DROP EXTENSION IF EXISTS "uuid-ossp";

DROP TABLE IF EXISTS users;