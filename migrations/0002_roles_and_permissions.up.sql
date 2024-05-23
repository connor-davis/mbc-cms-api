-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS roles (
        id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4 (),
        name VARCHAR(255) UNIQUE NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    IF NOT EXISTS roles_permissions (
        id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4 (),
        role_id UUID NOT NULL,
        permission_name VARCHAR(255) NOT NULL,
        permission_level BIGINT NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );