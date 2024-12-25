CREATE TABLE "user" (
    user_id UUID PRIMARY KEY DEFAULT UUID_GENERATE_V1MC(),
    username TEXT COLLATE "case_insensitive" UNIQUE NOT NULL,
    email TEXT COLLATE "case_insensitive" UNIQUE NOT NULL,
    password_hash TEXT COLLATE "case_insensitive"  NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);

-- Add updated_at trigger
SELECT trigger_updated_at('"user"');
