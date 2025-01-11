-- Your SQL goes here
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username VARCHAR(100) NOT NULL,
  email VARCHAR(100) NOT NULL,
  phone VARCHAR(50),
  password VARCHAR(100) NOT NULL,
  full_name VARCHAR(100) NOT NULL,
  biography TEXT,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),
  deleted_at TIMESTAMP
);

CREATE UNIQUE INDEX "users_username_unique"
ON users(username, deleted_at) WHERE deleted_at IS NULL;

CREATE UNIQUE INDEX "users_email_unique"
ON users(email, deleted_at) WHERE deleted_at IS NULL;

CREATE UNIQUE INDEX "users_phone_unique"
ON users(phone, deleted_at) WHERE deleted_at IS NULL;