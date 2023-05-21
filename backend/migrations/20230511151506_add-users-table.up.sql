-- Define functions to simplify parsing ids between binary and string.
CREATE FUNCTION as_uuid(id BINARY(16))
RETURNS CHAR(36) CHARSET utf8mb4
DETERMINISTIC
BEGIN
  RETURN bin_to_uuid(id, true);
END;

CREATE FUNCTION as_bin(id CHAR(36) CHARSET utf8mb4)
RETURNS BINARY(16)
DETERMINISTIC
BEGIN
  RETURN uuid_to_bin(id, true);
END;

-- Create users table.
CREATE TABLE users (
  id binary(16) PRIMARY KEY NOT NULL DEFAULT (uuid_to_bin(uuid(), true)),

  username varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL UNIQUE,
  display_name varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  first_name varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  last_name varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,

  created_at datetime NOT NULL DEFAULT current_timestamp,
  updated_at datetime NOT NULL DEFAULT current_timestamp ON UPDATE current_timestamp
);

-- Login identity table for email and password authentication.
CREATE TABLE login_identity__email_password (
  user_id binary(16) PRIMARY KEY NOT NULL,

  email varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL UNIQUE,
  password_hash varchar(255) CHARACTER SET utf8mb4 NOT NULL,
  salt varchar(255) CHARACTER SET utf8mb4 NOT NULL,

  created_at datetime NOT NULL DEFAULT current_timestamp,
  updated_at datetime NOT NULL DEFAULT current_timestamp ON UPDATE current_timestamp
);