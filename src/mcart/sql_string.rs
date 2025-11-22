pub struct CartSQLString;
impl CartSQLString {
    pub const GET_OPEN_CART_BY_USER_ID: &'static str = r#"
        SELECT
          c.id,
          c.user_id,
          c.status,
          c.total_amount,
          c.created_at,
          c.updated_at,
          (
            SELECT COALESCE(
              json_agg(json_build_object(
                'item_id', ci.id,
                'cart_id', ci.cart_id,
                'product_id', ci.product_id,
                'quantity', ci.quantity,
                'unit_amount', ci.unit_amount,
                'line_total', ci.line_total,
                'product_name', p.name,
                'product_price', p.price,
                'product_pack_price', p.pack_price,
                'product_created_at', p.created_at,
                'product_updated_at', p.updated_at,
                'created_at', ci.created_at,
                'updated_at', ci.updated_at
              )),
              '[]'::json
            )
            FROM cart_items ci
            JOIN products p ON p.id = ci.product_id
            WHERE ci.cart_id = c.id
          ) AS items
        FROM carts c
        WHERE c.user_id = $1 AND c.status = 'open'::cart_status
        LIMIT 1;
    "#;

    pub const GET_CART_BY_USER_ID: &'static str = r#"
        SELECT
          c.id,
          c.user_id,
          c.status,
          c.total_amount,
          c.created_at,
          c.updated_at,
          (
            SELECT COALESCE(
              json_agg(json_build_object(
                'item_id', ci.id,
                'cart_id', ci.cart_id,
                'product_id', ci.product_id,
                'quantity', ci.quantity,
                'unit_amount', ci.unit_amount,
                'line_total', ci.line_total,
                'product_name', p.name,
                'product_price', p.price,
                'product_pack_price', p.pack_price,
                'product_created_at', p.created_at,
                'product_updated_at', p.updated_at,
                'created_at', ci.created_at,
                'updated_at', ci.updated_at
              )),
              '[]'::json
            )
            FROM cart_items ci
            JOIN products p ON p.id = ci.product_id
            WHERE ci.cart_id = c.id
          ) AS items
        FROM carts c
        WHERE c.user_id = $1
        ORDER BY c.created_at DESC;
    "#;

    pub const CREATE_CART_ID: &'static str = r#"
        WITH existing AS (
            SELECT id, user_id, status, total_amount, created_at, updated_at
            FROM carts
            WHERE user_id = $1 AND status = 'open'::cart_status
        ),
        inserted AS (
            INSERT INTO carts (id, user_id, status, total_amount)
            SELECT uuid_generate_v4(), $1, 'open'::cart_status, 0
            WHERE NOT EXISTS (SELECT 1 FROM existing)
            RETURNING id, user_id, status, total_amount, created_at, updated_at
        )
        SELECT * FROM inserted
        UNION ALL
        SELECT * FROM existing
        LIMIT 1;
    "#;
    pub const INSERT_CART_ITEM: &'static str = r#"
        INSERT INTO cart_items (cart_id, product_id, quantity, unit_amount)
        VALUES ($1, $2, $3, $4)
        RETURNING id, cart_id, product_id, quantity, unit_amount, line_total, created_at, updated_at;
    "#;

    pub const UPSERT_CART_ITEM: &'static str = r#"
        INSERT INTO cart_items (cart_id, product_id, quantity, unit_amount)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (cart_id, product_id) DO UPDATE
        SET quantity = cart_items.quantity + EXCLUDED.quantity,
            unit_amount = EXCLUDED.unit_amount,
            updated_at = now()
        RETURNING id, cart_id, product_id, quantity, unit_amount, line_total, created_at, updated_at;
    "#;

    pub const PAY_CART_AND_RETURN_WITH_ITEMS: &'static str = r#"
        WITH updated AS (
            UPDATE carts
            SET status = 'paid'::cart_status,
                updated_at = now()
            WHERE id = $1
              AND status = 'open'::cart_status
            RETURNING id, user_id, status, total_amount, created_at, updated_at
        )
        SELECT
          u.id,
          u.user_id,
          u.status,
          u.total_amount,
          u.created_at,
          u.updated_at,
          COALESCE(
            (
              SELECT json_agg(json_build_object(
                'item_id', ci.id,
                'cart_id', ci.cart_id,
                'product_id', ci.product_id,
                'quantity', ci.quantity,
                'unit_amount', ci.unit_amount,
                'line_total', ci.line_total,
                'product_name', p.name,
                'product_price', p.price,
                'product_pack_price', p.pack_price,
                'product_created_at', p.created_at,
                'product_updated_at', p.updated_at,
                'created_at', ci.created_at,
                'updated_at', ci.updated_at
              ))
              FROM cart_items ci
              JOIN products p ON p.id = ci.product_id
              WHERE ci.cart_id = u.id
            ),
            '[]'::json
          ) AS items
        FROM updated u;
    "#;
}
