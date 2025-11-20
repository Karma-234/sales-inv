-- Add down migration script here
DROP TABLE IF EXISTS cart_items;
DROP TABLE IF EXISTS carts;
DROP TYPE IF EXISTS cart_status;
DROP FUNCTION IF EXISTS recalc_cart_total;
DROP FUNCTION IF EXISTS touch_updated_at;

DROP EXTENSION IF EXISTS "uuid-ossp";