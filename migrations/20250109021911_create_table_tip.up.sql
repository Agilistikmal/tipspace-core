-- Active: 1736389112589@@127.0.0.1@5432@tipspace
-- Add up migration script here
CREATE TABLE tips (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  for_user_id UUID NOT NULL,
  from_user_id UUID,
  from_guest BOOLEAN NOT NULL,
  from_full_name VARCHAR(100),
  message TEXT,
  transaction_id UUID NOT NULL,
  status VARCHAR(50) NOT NULL,
  available_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),
  deleted_at TIMESTAMP,
  CONSTRAINT fk_for_user
  FOREIGN KEY(for_user_id)
  REFERENCES users(id),
  CONSTRAINT fk_from_user
  FOREIGN KEY(from_user_id)
  REFERENCES users(id),
  CONSTRAINT fk_transaction
  FOREIGN KEY(transaction_id)
  REFERENCES transactions(id)
)