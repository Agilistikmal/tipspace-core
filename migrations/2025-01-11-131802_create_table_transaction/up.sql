-- Your SQL goes here
CREATE TABLE transactions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  payment_method_code VARCHAR(50) NOT NULL,
  currency VARCHAR(10) NOT NULL,
  amount FLOAT NOT NULL,
  status VARCHAR(50) NOT NULL,
  payment_code VARCHAR(100),
  qr_string TEXT,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),
  deleted_at TIMESTAMP
)