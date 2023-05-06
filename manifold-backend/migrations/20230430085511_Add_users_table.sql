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

  display_name varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  first_name varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  last_name varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,

  created_at datetime NOT NULL DEFAULT current_timestamp,
  updated_at datetime NOT NULL DEFAULT current_timestamp ON UPDATE current_timestamp
);

-- Generic table used for mapping a user to their login identities. This allows the user to use multiple different
-- identities to authenticate with a single account.
CREATE TABLE login_identity (
  id binary(16) PRIMARY KEY NOT NULL DEFAULT (uuid_to_bin(uuid(), true)),
  user_id binary(16) NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),

  identity_type enum("email_password", "phone_number")
);

-- Login identity table for email and password authentication.
CREATE TABLE li_email_password (
  id binary(16) NOT NULL,
  FOREIGN KEY (id) REFERENCES login_identity(id),

  email varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  password varchar(255) CHARACTER SET utf8mb4 NOT NULL,
  salt varchar(255) CHARACTER SET utf8mb4 NOT NULL,

  created_at datetime NOT NULL DEFAULT current_timestamp,
  updated_at datetime NOT NULL DEFAULT current_timestamp ON UPDATE current_timestamp
);

-- Login identity table for phone number authentication.
CREATE TABLE li_phone_number(
  id binary(16) NOT NULL,
  FOREIGN KEY (id) REFERENCES login_identity(id),

  phone_number varchar(15) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,

  created_at datetime NOT NULL DEFAULT current_timestamp,
  updated_at datetime NOT NULL DEFAULT current_timestamp ON UPDATE current_timestamp
);