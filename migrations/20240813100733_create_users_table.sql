-- Add migration script here
CREATE SCHEMA IF NOT EXISTS auth;

CREATE TABLE auth.users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),

  encrypted_password VARCHAR(1000) NOT NULL,

  email VARCHAR(320) CHECK(email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z]{2,}$'),
	email_confirmed_at TIMESTAMPTZ NULL,

	last_sign_in_at TIMESTAMPTZ NULL,

  phone VARCHAR(255) NULL,
  phone_confirmed_at TIMESTAMPTZ NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS users_id_email_idx ON auth.users USING btree (id, lower(email));
CREATE INDEX IF NOT EXISTS users_id_idx ON auth.users USING btree (id);

CREATE OR REPLACE FUNCTION moddatetime()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER handle_updated_at
BEFORE UPDATE ON auth.users
FOR EACH ROW
EXECUTE FUNCTION moddatetime();
