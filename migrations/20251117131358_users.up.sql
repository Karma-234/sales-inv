-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_role') THEN
        CREATE TYPE user_role AS ENUM ('admin', 'user', 'guest');
    END IF;
END
$$;


CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT (uuid_generate_v4()), -- or use uuid_generate_v4()
    username text NOT NULL DEFAULT 'admin',
    first_name text NOT NULL DEFAULT 'Admin',
    last_name text NOT NULL DEFAULT 'User',
    email text NOT NULL DEFAULT 'admin@user.com',
    role user_role NOT NULL DEFAULT 'admin',
    hashed_password text NOT NULL DEFAULT 'password',
    is_verified boolean NOT NULL DEFAULT false,
    verification_token text,
    token_expiry TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP 
        WITH 
            TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);


CREATE UNIQUE INDEX users_username_idx ON users (lower(username));
CREATE UNIQUE INDEX users_email_idx ON users (lower(email));