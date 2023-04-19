-- create 'messages' table
CREATE TABLE messages (
  id binary(16) PRIMARY KEY NOT NULL DEFAULT (uuid_to_bin(uuid(), true)),
  content TEXT NOT NULL
);