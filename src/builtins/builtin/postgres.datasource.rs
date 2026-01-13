// src/builtins/datasource_postgres.rs

use sqlx::{Pool, Postgres, Row, postgres::PgRow};
use serde_json::{Value as JsonValue, Map};
use crate::builtins::{Context, BuiltinResult};

/// Built-in to execute a PostgreSQL query and store results in context.
/// Expected args: ["query_string", "param1", "param2", ...]
pub async fn builtin_postgres_query(
    args: &[String],
    ctx: &mut Context,
    pool: &Pool<Postgres>,
    assign_to: Option<&str>,
) -> BuiltinResult {
    if args.is_empty() {
        return BuiltinResult::Error("postgres: missing query argument".to_string());
    }

    let query_str = &args[0];
    let mut query = sqlx::query(query_str);

    // Bind parameters from the remaining args
    for arg_name in &args[1..] {
        if let Some(val) = ctx.get(arg_name) {
            match val {
                JsonValue::String(s) => query = query.bind(s),
                JsonValue::Number(n) => {
                    if let Some(f) = n.as_f64() { query = query.bind(f); }
                }
                JsonValue::Bool(b) => query = query.bind(b),
                _ => return BuiltinResult::Error(format!("Unsupported param type for {}", arg_name)),
            }
        }
    }

    match query.fetch_all(pool).await {
        Ok(rows) => {
            let json_rows: Vec<JsonValue> = rows.iter().map(row_to_json).collect();
            if let Some(target) = assign_to {
                ctx.insert(target.to_string(), JsonValue::Array(json_rows));
            }
            BuiltinResult::Ok
        }
        Err(e) => BuiltinResult::Error(format!("Postgres error: {}", e)),
    }
}

/// Helper to convert a PgRow into a JSON Object
fn row_to_json(row: &PgRow) -> JsonValue {
    let mut map = Map::new();
    for column in row.columns() {
        let name = column.name();
        // Dynamic type extraction from row
        let val: JsonValue = match row.try_get::<String, &str>(name) {
            Ok(s) => JsonValue::String(s),
            Err(_) => match row.try_get::<i64, &str>(name) {
                Ok(n) => JsonValue::Number(n.into()),
                Err(_) => match row.try_get::<bool, &str>(name) {
                    Ok(b) => JsonValue::Bool(b),
                    Err(_) => JsonValue::Null,
                },
            },
        };
        map.insert(name.to_string(), val);
    }
    JsonValue::Object(map)
}