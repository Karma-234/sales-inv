pub struct CartSQLString;
impl CartSQLString {
    pub const GET_CART_BY_USER_ID: &'static str = r#"
        pub const GET_OPEN_CART_BY_USER_ID: &'static str = r#"
        SELECT
            c.id,
            c.user_id,
            c.status,
            c.total_amount,
            c.created_at,
            c.updated_at,
            COALESCE(
                json_agg(
                json_build_object(
                    'item_id', ci.id,
                    'product_id', p.id,
                    'quantity', ci.quantity,
                    'unit_amount', ci.unit_amount,
                    'line_total', ci.line_total,
                    'product', json_build_object(
                        'id', p.id,
                        'name', p.name,
                        'price', p.price,
                        'quantity', p.quantity,
                        'pack_price', p.pack_price,
                        'created_at', p.created_at,
                        'updated_at', p.updated_at
                    ),
                    'created_at', ci.created_at,
                    'updated_at', ci.updated_at
                )
                ) FILTER (WHERE ci.id IS NOT NULL),
                '[]'
            ) AS items
                FROM carts c
                LEFT JOIN cart_items ci ON ci.cart_id = c.id
                LEFT JOIN products p ON p.id = ci.product_id
                WHERE c.user_id = $1
                GROUP BY c.id, c.user_id, c.status, c.total_amount, c.created_at, c.updated_at
    
    "#;

    pub const GET_OPEN_CART_BY_USER_ID: &'static str = r#"
        SELECT
            c.id,
            c.user_id,
            c.status,
            c.total_amount,
            c.created_at,
            c.updated_at,
            COALESCE(
                json_agg(
                json_build_object(
                    'item_id', ci.id,
                    'product_id', p.id,
                    'quantity', ci.quantity,
                    'unit_amount', ci.unit_amount,
                    'line_total', ci.line_total,
                    'product', json_build_object(
                        'id', p.id,
                        'name', p.name,
                        'price', p.price,
                        'quantity', p.quantity,
                        'pack_price', p.pack_price,
                        'created_at', p.created_at,
                        'updated_at', p.updated_at
                    ),
                    'created_at', ci.created_at,
                    'updated_at', ci.updated_at
                )
                ) FILTER (WHERE ci.id IS NOT NULL),
                '[]'
            ) AS items
                FROM carts c
                LEFT JOIN cart_items ci ON ci.cart_id = c.id
                LEFT JOIN products p ON p.id = ci.product_id
                WHERE c.user_id = $1
                AND c.status = 'Open'
                GROUP BY c.id, c.user_id, c.status, c.total_amount, c.created_at, c.updated_at
    "#;

    pub const CREATE_CART_ID: &'static str = r#"
        WITH existing AS (
            SELECT id, user_id, status, total_amount, created_at, updated_at
            FROM carts
            WHERE user_id = $1 AND status = 'Open'
            ),
            inserted AS (
            INSERT INTO carts (id, user_id, status, total_amount)
            SELECT uuid_generate_v4(), $1, 'Open', 0
            WHERE NOT EXISTS (SELECT 1 FROM existing)
            RETURNING id, user_id, status, total_amount, created_at, updated_at
            )
            SELECT * FROM inserted
            UNION ALL
            SELECT * FROM existing;
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
}
