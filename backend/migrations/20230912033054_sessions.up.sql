-- Login sessions table
CREATE TABLE login_sessions (
    user_id UUID NOT NULL REFERENCES users(id),
    session_id VARCHAR(255) NOT NULL,
    PRIMARY KEY (user_id, session_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp()
);