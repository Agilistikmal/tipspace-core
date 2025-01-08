-- Add up migration script here
CREATE TABLE oauths (
  user_id UUID PRIMARY KEY,
  google_id VARCHAR(100),
  discord_id VARCHAR(100),
  x_id VARCHAR(100),
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),
  deleted_at TIMESTAMP,
  CONSTRAINT fk_user
  FOREIGN KEY(user_id)
  REFERENCES users(id)
);

CREATE UNIQUE INDEX "oauths_google_id_unique"
ON oauths(google_id, deleted_at) WHERE deleted_at IS NULL;

CREATE UNIQUE INDEX "oauths_discord_id_unique"
ON oauths(discord_id, deleted_at) WHERE deleted_at IS NULL;

CREATE UNIQUE INDEX "oauths_x_id_unique"
ON oauths(x_id, deleted_at) WHERE deleted_at IS NULL;
