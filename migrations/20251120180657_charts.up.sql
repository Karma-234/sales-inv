-- Add up migration script here
-- Use DOUBLE PRECISION for monetary fields (amount, unit_amount, line_total, total_amount)
-- WARNING: Floating point is imprecise for money; user requested float.
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE cart_status AS ENUM ('open','paid','refund','foc');

CREATE TABLE carts (
    id           UUID PRIMARY KEY DEFAULT (uuid_generate_v4()),
    user_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status       cart_status NOT NULL DEFAULT 'open',
    total_amount DOUBLE PRECISION NOT NULL DEFAULT 0,  -- recomputed via triggers
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX ux_cart_user_open ON carts(user_id) WHERE status = 'open';

CREATE TABLE cart_items (
    id          BIGSERIAL PRIMARY KEY,
    cart_id     UUID NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
    product_id  UUID NOT NULL REFERENCES products(id),
    quantity    INTEGER NOT NULL CHECK (quantity > 0),
    unit_amount DOUBLE PRECISION NOT NULL CHECK (unit_amount >= 0),
    line_total  DOUBLE PRECISION GENERATED ALWAYS AS (unit_amount * quantity) STORED,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (cart_id, product_id)
);

CREATE OR REPLACE FUNCTION touch_updated_at() RETURNS trigger AS $$
BEGIN
  NEW.updated_at := now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_carts_touch
BEFORE UPDATE ON carts
FOR EACH ROW EXECUTE FUNCTION touch_updated_at();

CREATE TRIGGER trg_cart_items_touch
BEFORE UPDATE ON cart_items
FOR EACH ROW EXECUTE FUNCTION touch_updated_at();

CREATE OR REPLACE FUNCTION recalc_cart_total() RETURNS trigger AS $$
DECLARE
  v_cart UUID;
BEGIN
  v_cart := COALESCE(NEW.cart_id, OLD.cart_id);
  UPDATE carts c
    SET total_amount = (
        SELECT COALESCE(SUM(line_total), 0)
        FROM cart_items ci
        WHERE ci.cart_id = v_cart
    ),
    updated_at = now()
  WHERE c.id = v_cart;
  RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_cart_items_after_ins
AFTER INSERT ON cart_items
FOR EACH ROW EXECUTE FUNCTION recalc_cart_total();

CREATE TRIGGER trg_cart_items_after_upd
AFTER UPDATE ON cart_items
FOR EACH ROW EXECUTE FUNCTION recalc_cart_total();

CREATE TRIGGER trg_cart_items_after_del
AFTER DELETE ON cart_items
FOR EACH ROW EXECUTE FUNCTION recalc_cart_total();