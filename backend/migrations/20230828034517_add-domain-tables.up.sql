-- Table for domains
CREATE TABLE domains (
  id binary(16) PRIMARY KEY NOT NULL,

  display_name varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  description_text varchar(1024) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  icon_url varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  banner_url varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,

  created_at datetime NOT NULL DEFAULT current_timestamp,
  updated_at datetime NOT NULL DEFAULT current_timestamp ON UPDATE current_timestamp
);

-- Table for the members in a domain
CREATE TABLE domain_memberships (
  PRIMARY KEY (domain_id, user_id),

  domain_id binary(16) NOT NULL,
  user_id binary(16) NOT NULL,
  role_name varchar(16) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,

  created_at datetime NOT NULL DEFAULT current_timestamp,
  updated_at datetime NOT NULL DEFAULT current_timestamp ON UPDATE current_timestamp
);