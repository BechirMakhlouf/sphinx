-- Add migration script here

CREATE TYPE auth_provider
AS 
ENUM('email', 'google', 'discord', 'apple', 'github');

CREATE TABLE IF NOT EXISTS auth.identities (
    user_id UUID NOT NULL,

    provider AUTH_PROVIDER NOT NULL,
    provider_user_id TEXT NOT NULL,

    provider_data JSONB NOT NULL,
    
    email VARCHAR(320) NOT NULL,
    is_email_confirmed BOOL NULL,

    phone text NULL,
    is_phone_confirmed BOOL NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT identities_pkey PRIMARY KEY (provider, provider_user_id),
    CONSTRAINT identities_user_id_fkey FOREIGN KEY (user_id) REFERENCES auth.users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS identities_user_id_idx ON auth.identities using btree (user_id);

CREATE TRIGGER handle_updated_at BEFORE
UPDATE ON auth.identities FOR EACH ROW EXECUTE FUNCTION moddatetime ();
