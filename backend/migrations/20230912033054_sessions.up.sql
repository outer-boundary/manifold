-- Add up migration script here
CREATE TABLE login_sessions (
  PRIMARY KEY (user_id, session_id),
  user_id binary(16) NOT NULL,
  session_id varchar(255) NOT NULL,
  
  created_at datetime NOT NULL DEFAULT current_timestamp
);