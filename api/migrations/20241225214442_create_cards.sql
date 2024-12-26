-- Add migration script here
CREATE TABLE "card_type" (
    type_id UUID PRIMARY KEY DEFAULT UUID_GENERATE_V1MC(),
    name TEXT UNIQUE NOT NULL,
    schema JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);

-- Add updated_at trigger
SELECT trigger_updated_at('"card_type"');

-- Create the card table
CREATE TABLE "card" (
    card_id UUID PRIMARY KEY DEFAULT UUID_GENERATE_V1MC(),
    type_id UUID NOT NULL REFERENCES card_type(type_id),
    content JSONB NOT NULL,
    is_public BOOLEAN NOT NULL DEFAULT false,
    created_by_user_id UUID NOT NULL REFERENCES "user"(user_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);

-- Add updated_at trigger
SELECT trigger_updated_at('"card"');