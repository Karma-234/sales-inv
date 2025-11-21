# Sales Inv

Inventory & point of sales API built with Rust and Axum.

## Tech Stack
- Rust (axum, tokio) Programming Language
- SQLx (PostgreSQL, async queries, migrations) 
- PostgreSQL (UUID, enum types, generated columns, triggers) Database
- tower (custom auth & permission layers)
- JWT auth (Bearer token middleware)
- utoipa + swagger-ui (OpenAPI generation & interactive docs)
- uuid, chrono (IDs & timestamps)
- Custom error mapping (database -> field errors)

## Core Features
- User registration, login, JWT authentication
- Role-based access (UserRole, CartStatus enums)
- Products CRUD (pricing, pack price, stock quantity)
- Carts:
  - Single open cart per user (partial unique index)
  - Normalized cart_items table with generated line_total
  - Atomic stock adjustments on add/update/remove
  - Open cart retrieval with aggregated JSON items
- Middleware:
  - Auth gate attaches user to request extensions
  - Permission layer (MyAuthPermsLayer) for protected routes
- Structured API responses (MyBaseResponse<T>)
  - Includes db_err mapper for sqlx::Error
  - FieldError list for constraint / validation feedback
- OpenAPI / Swagger:
  - Global Bearer security scheme
  - Per-route security annotations
- Migrations:
  - Enum evolution (cart_status lowercase)
  - Triggers for updated_at and cart total recalculation

## Database Schema Highlights
- products: quantity CHECK (>=0 recommended)
- carts: status cart_status enum, total_amount (DOUBLE PRECISION), unique open per user
- cart_items: GENERATED ALWAYS line_total = unit_amount * quantity
- Triggers keep totals and timestamps consistent.

## Environment Variables (example)
````dotenv
DATABASE_URL=postgres://user:pass@localhost:5432/sales_inv
JWT_SECRET=your_jwt_secret
RUST_LOG=info
