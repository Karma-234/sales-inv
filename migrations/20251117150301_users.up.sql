-- Add up migration script here

ALTER TABLE users ADD COLUMN IF NOT EXISTS search_tsv tsvector;

UPDATE users SET search_tsv = to_tsvector('english', coalesce(username,'') || ' ' || coalesce(first_name,'') || ' ' || coalesce(last_name,'') || ' ' || coalesce(email,''));

CREATE INDEX IF NOT EXISTS users_search_tsv_idx ON users USING gin(search_tsv);

