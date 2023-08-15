-- Base user table for essential user data.
CREATE TABLE users (
  id binary(16) PRIMARY KEY NOT NULL,

  username varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL UNIQUE,
  account_role varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  
  created_at datetime NOT NULL DEFAULT current_timestamp,
  updated_at datetime NOT NULL DEFAULT current_timestamp ON UPDATE current_timestamp
);

-- Table for ancillary user data.
CREATE TABLE user_profile (
  user_id binary(16) PRIMARY KEY NOT NULL,

  display_name varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,

  first_name varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  last_name varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  date_of_birth date NOT NULL,

  created_at datetime NOT NULL DEFAULT current_timestamp,
  updated_at datetime NOT NULL DEFAULT current_timestamp ON UPDATE current_timestamp
);

-- Login identity table for email authentication.
CREATE TABLE login_identity__email (
  user_id binary(16) PRIMARY KEY NOT NULL,

  email varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL UNIQUE,
  password_hash varchar(255) CHARACTER SET utf8mb4 NOT NULL,
  salt varchar(255) CHARACTER SET utf8mb4 NOT NULL,

  verified boolean DEFAULT false NOT NULL,

  created_at datetime NOT NULL DEFAULT current_timestamp,
  updated_at datetime NOT NULL DEFAULT current_timestamp ON UPDATE current_timestamp
);