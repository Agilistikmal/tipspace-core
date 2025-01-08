-- Add up migration script here
CREATE TABLE balances (
  user_id UUID PRIMARY KEY,
  available FLOAT DEFAULT 0,
  pending FLOAT DEFAULT 0,
  withdrawn FLOAT DEFAULT 0,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),
  deleted_at TIMESTAMP,
  CONSTRAINT fk_user
  FOREIGN KEY(user_id)
  REFERENCES users(id)
);