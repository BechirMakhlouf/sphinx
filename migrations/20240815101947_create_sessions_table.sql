-- Add migration script here
CREATE TABLE IF NOT EXISTS auth.sessions (
    id uuid NOT NULL,
    user_id uuid NOT NULL,
    user_agent text NOT NULL,
    ip inet NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    constraint sessions_pkey primary key (id),
    constraint sessions_user_id_fkey foreign key (user_id) references auth.users(id) ON DELETE cascade
);

CREATE INDEX IF NOT EXISTS sessions_user_id_idx ON auth.sessions using btree (user_id);
CREATE INDEX IF NOT EXISTS sessions_id_idx ON auth.sessions using btree (id);

CREATE TRIGGER handle_updated_at BEFORE
UPDATE ON auth.sessions FOR EACH ROW EXECUTE FUNCTION moddatetime ();
