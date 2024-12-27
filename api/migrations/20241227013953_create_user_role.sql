CREATE TYPE user_role AS ENUM ('user', 'admin');

-- Add role column to user table
ALTER TABLE "user" 
ADD COLUMN role user_role NOT NULL DEFAULT 'user';

-- Index for role lookups if needed
CREATE INDEX idx_user_role ON "user"(role);