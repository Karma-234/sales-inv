pub fn map_pg_database_error(db_err: &dyn DatabaseError) -> Vec<FieldError> {
    let code = db_err.code().unwrap_or("unknown").to_string();
    let constraint = db_err.constraint().unwrap_or("").to_string();
    let column = db_err.column().unwrap_or("").to_string();

    match code.as_str() {
        "23505" => {
            // unique violation
            let field = if !column.is_empty() {
                column
            } else {
                constraint.clone()
            };
            vec![FieldError::new(field, "Already exists", code)]
        }
        "23502" => {
            // not null violation
            let field = if !column.is_empty() {
                column
            } else {
                constraint.clone()
            };
            vec![FieldError::new(field, "Required field missing", code)]
        }
        "23503" => vec![FieldError::new(constraint, "Foreign key violation", code)],
        "23514" => vec![FieldError::new(constraint, "Check constraint failed", code)],
        _ => vec![FieldError::new(constraint, db_err.message(), code)],
    }
}
