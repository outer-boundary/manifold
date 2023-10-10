CREATE TABLE domains (
  id binary(16) PRIMARY KEY NOT NULL,

  display_name varchar(48) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  description_text varchar(512) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  
  icon_url varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  banner_url varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  
  created_at datetime NOT NULL DEFAULT current_timestamp
);

CREATE TABLE domain_memberships (
  PRIMARY KEY (domain_id, user_id),

  domain_id binary(16) NOT NULL,
  user_id binary(16) NOT NULL,

  role_name varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL
);