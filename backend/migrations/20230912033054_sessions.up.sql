-- Add up migration script here
CREATE TABLE user_sessions (
  PRIMARY KEY (domain_id, user_id),
  user_id binary(16) NOT NULL,
  session_id binary(16) NOT NULL,
  
  created_at datetime NOT NULL DEFAULT current_timestamp,
  updated_at datetime NOT NULL DEFAULT current_timestamp ON UPDATE current_timestamp
);