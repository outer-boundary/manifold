-- Create users table
CREATE TABLE users (
  id binary(16) PRIMARY KEY NOT NULL DEFAULT (uuid_to_bin(uuid(), true)),
  username varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL
);