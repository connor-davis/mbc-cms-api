-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    users (
        id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4 (),
        email VARCHAR(255) NOT NULL,
        password VARCHAR(255) NOT NULL,
        role VARCHAR(255) NOT NULL DEFAULT 'Guest',
        active BOOLEAN NOT NULL DEFAULT TRUE,
        mfa_enabled BOOLEAN NOT NULL DEFAULT FALSE,
        mfa_verified BOOLEAN NOT NULL DEFAULT FALSE,
        mfa_secret VARCHAR(255),
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );