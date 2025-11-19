use sqlx::error::DatabaseError;
use utoipa::ToSchema;

#[derive(serde::Serialize, Debug, Clone, ToSchema)]
pub struct FieldError {
    pub field: String,
    pub message: String,
    pub code: String,
}

impl FieldError {
    pub fn new(
        field: impl Into<String>,
        message: impl Into<String>,
        code: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            code: code.into(),
        }
    }
}
pub fn map_pg_database_error(db_err: &(dyn DatabaseError + 'static)) -> FieldError {
    let code = db_err
        .code()
        .map(|c| c.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // best-effort field detection
    let mut field = db_err.constraint().unwrap_or("").to_string();
    if let pg = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>() {
        // pg: &PgDatabaseError
        if let Some(col) = pg.column() {
            field = col.to_string();
        } else if let Some(constraint) = pg.constraint() {
            field = constraint.to_string();
        } else if let Some(table) = pg.table() {
            field = table.to_string();
        }
    }

    match code.as_str() {
        "23505" => FieldError::new(field, "Already exists", "409"),
        "23502" => FieldError::new(field, "Required field missing", "400"),
        "23503" => FieldError::new(field, "Foreign key violation", "400"),
        "23514" => FieldError::new(field, "Check constraint failed", "400"),
        _ => FieldError::new(field, db_err.message(), "500"),
    }
}
