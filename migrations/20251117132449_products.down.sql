-- Add down migration script here
DROP TABLE IF EXISTS products;

DROP EXTENSION IF EXISTS "uuid-ossp";