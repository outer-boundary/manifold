-- Base user table for essential user data
CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL,
    username VARCHAR(32) NOT NULL UNIQUE,
    account_role VARCHAR(32) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp()
);

-- Table for ancillary user data
CREATE TABLE user_profile (
    user_id UUID PRIMARY KEY NOT NULL REFERENCES users(id),
    display_name VARCHAR(32) NOT NULL,
    first_name VARCHAR(32) NOT NULL,
    last_name VARCHAR(32) NOT NULL,
    date_of_birth DATE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp()
);

-- Login identity table for email authentication
CREATE TABLE login_identity__email (
    user_id UUID PRIMARY KEY NOT NULL REFERENCES users(id),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    salt VARCHAR(255) NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp()
);