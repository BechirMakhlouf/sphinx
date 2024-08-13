-- Add migration script here

CREATE TABLE IF NOT EXISTS auth.identities (
    id text NOT NULL,
    user_id uuid NOT NULL,
    identity_data JSONB NOT NULL,
    provider text NOT NULL,
    last_sign_in_at timestamptz NULL,

    email VARCHAR(320) UNIQUE CHECK(email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z]{2,}$'),

    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP

    CONSTRAINT identities_pkey PRIMARY KEY (provider, id),
    CONSTRAINT identities_user_id_fkey FOREIGN KEY (user_id) REFERENCES auth.users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS identities_user_id_idx ON auth.identities using btree (user_id);

CREATE TRIGGER handle_updated_at BEFORE
UPDATE ON users_weight FOR EACH ROW EXECUTE FUNCTION moddatetime ();
