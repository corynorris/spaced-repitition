CREATE TYPE role AS ENUM ('user', 'admin');

CREATE TABLE "user" (
    user_id UUID PRIMARY KEY DEFAULT UUID_GENERATE_V1MC(),
    username TEXT COLLATE "case_insensitive" UNIQUE NOT NULL,
    email TEXT COLLATE "case_insensitive" UNIQUE NOT NULL,
    password_hash TEXT COLLATE "case_insensitive"  NOT NULL,
    role role NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);

-- Add updated_at trigger
SELECT trigger_updated_at('"user"');

-- Index for role lookups if needed
CREATE INDEX idx_role ON "user"(role);