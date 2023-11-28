-- Domains table
CREATE TABLE domains (
    id UUID PRIMARY KEY NOT NULL,
    display_name VARCHAR(48) NOT NULL UNIQUE,
    description_text VARCHAR(512) NOT NULL,
    icon_url VARCHAR(128) NOT NULL,
    banner_url VARCHAR(128) NOT NULL,
    public boolean NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp()
);

-- Domain memberships table
CREATE TABLE domain_memberships (
    domain_id UUID NOT NULL REFERENCES domains(id),
    user_id UUID NOT NULL REFERENCES users(id),
    role_name VARCHAR(32) NOT NULL,
    PRIMARY KEY (domain_id, user_id)
);