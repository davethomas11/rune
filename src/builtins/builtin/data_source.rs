use crate::builtins::{BuiltinResult, Context};
use crate::rune_ast::{Section, Value};
use crate::runtime::AppState;
use std::collections::HashMap;
use std::sync::Arc;

use crate::builtins::builtin::postgres::create_or_reuse_postgres_pool;

pub async fn builtin_data_source(
    args: &[String],
    ctx: &mut Context,
    state: &AppState,
    assign_to: Option<&str>,
) -> BuiltinResult {
    if args.is_empty() {
        eprintln!("[ERROR] datasource: missing arguments");
        return BuiltinResult::Error("missing arguments".to_string());
    }
    let action = &args[0];
    let name = if args.len() > 1 { &args[1] } else { "" };
    let action_args = if args.len() > 2 { &args[2..] } else { &[] };

    match action.as_str() {
        "connect" => connect_to_datasource(name, &state).await,
        "fetch_all" => fetch_all_from_datasource(name, action_args, ctx).await,
        "fetch" => fetch_from_datasource(name, action_args, ctx).await,
        "insert" => insert_into_datasource(name, action_args, ctx).await,
        "update" => update_datasource(name, action_args, ctx).await,
        "delete" => delete_from_datasource(name, action_args, ctx).await,
        _ => BuiltinResult::Error(format!("unknown datasource action: {}", action)),
    }
}

fn get_section_by_string_key(
    section: &Section,
    key: &str,
    map: &Arc<HashMap<String, Section>>,
) -> Option<Section> {
    section.kv.get(key).cloned().and_then(|v| {
        if let Value::String(name) = v {
            map.get(&name).cloned()
        } else {
            None
        }
    })
}

pub fn get_data_source_commands(
    method: &str,
    section: Section,
    schemas: &Arc<HashMap<String, Section>>,
    data_sources: &Arc<HashMap<String, Section>>,
    single: bool,
) -> Vec<Value> {
    let schema_name = if let Some(Value::String(name)) = section.kv.get("schema") {
        name.clone()
    } else {
        return vec![Value::String("respond 500 \"Schema name not provided\"".into())];
    };
    let data_source_name = if let Some(Value::String(name)) = section.kv.get("data_source") {
        name.clone()
    } else {
        return vec![Value::String("respond 500 \"Data source name not provided\"".into())];
    };
    let schema = get_section_by_string_key(&section, "schema", schemas);
    let data_source = get_section_by_string_key(&section, "data_source", data_sources);

    if data_source.is_none() {
        return vec![Value::String(format!("respond 500 \"Data source not configured\" with name {}", data_source_name).into())];
    }

    if schema.is_none() {
        return vec![Value::String("respond 500 \"Schema not found\"".into())];
    }

    let connection_command = format!("datasource connect {}", data_source_name);
    let create_table_command = format!(
        "datasource create_table {} in {}",
        schema_name, data_source_name
    );

    let fetch_command = if single {
        format!(
            "datasource fetch {} from {} into data",
            schema_name, data_source_name
        )
    } else {
        format!(
            "datasource fetch_all {} from {} into data",
            schema_name, data_source_name
        )
    };

    match method {
        "GET" => vec![
            Value::String(connection_command),
            Value::String(create_table_command),
            Value::String(fetch_command),
            Value::String("respond 200 data".to_string()),
        ],
        "POST" => vec![
            Value::String(connection_command),
            Value::String(create_table_command),
            Value::String(format!(
                "datasource insert {} into {}",
                schema_name, data_source_name
            )),
            Value::String("respond 201 created".to_string()),
        ],
        "PUT" => vec![
            Value::String(connection_command),
            Value::String(create_table_command),
            Value::String(format!(
                "datasource update {} in {}",
                schema_name, data_source_name
            )),
            Value::String("respond 200 data_object".to_string()),
        ],
        "DELETE" => vec![
            Value::String(connection_command),
            Value::String(create_table_command),
            Value::String(format!(
                "datasource delete {} from {}",
                schema_name, data_source_name
            )),
            Value::String("respond 204".to_string()),
        ],
        _ => vec![],
    }
}

pub async fn connect_to_datasource(name: &str, state: &AppState) -> BuiltinResult {
    let data_sources = &state.data_sources;

    if !data_sources.contains_key(name) {
        eprintln!(
            "[ERROR] datasource.connect: data source '{}' not found",
            name
        );
        return BuiltinResult::Error(format!("data source '{}' not found", name));
    }

    let data_source_section = &data_sources[name];
    let conn_type = if let Some(Value::String(t)) = data_source_section.kv.get("type") {
        t.clone()
    } else {
        eprintln!("[ERROR] datasource.connect: connection type not specified");
        return BuiltinResult::Error("connection type not specified".to_string());
    };

    let conn_str = if let Some(Value::String(s)) = data_source_section.kv.get("connection_string") {
        s.clone()
    } else {
        eprintln!("[ERROR] datasource.connect: connection string not specified");
        return BuiltinResult::Error("connection string not specified".to_string());
    };

    match conn_type.as_str() {
        "postgres" => create_or_reuse_postgres_pool(&conn_str)
            .await
            .map(|_| BuiltinResult::Ok)
            .unwrap_or_else(|e| {
                eprintln!(
                    "[ERROR] datasource.connect: failed to connect to postgres: {}",
                    e
                );
                BuiltinResult::Error(format!("failed to connect to postgres: {}", e))
            }),
        _ => {
            eprintln!(
                "[ERROR] datasource.connect: unsupported connection type '{}'",
                conn_type
            );
            BuiltinResult::Error(format!("unsupported connection type '{}'", conn_type))
        }
    }
}

pub async fn fetch_all_from_datasource(
    name: &str,
    args: &[String],
    ctx: &mut Context,
) -> BuiltinResult {
    BuiltinResult::Error("Fetch all not implemented".to_string())
}

pub async fn fetch_from_datasource(
    name: &str,
    args: &[String],
    ctx: &mut Context,
) -> BuiltinResult {
    BuiltinResult::Error("Fetch not implemented".to_string())
}

pub async fn insert_into_datasource(name: &str, args: &[String], ctx: &Context) -> BuiltinResult {
    BuiltinResult::Error("Insert not implemented".to_string())
}

pub async fn update_datasource(name: &str, args: &[String], ctx: &Context) -> BuiltinResult {
    BuiltinResult::Error("Update not implemented".to_string())
}

pub async fn delete_from_datasource(name: &str, args: &[String], ctx: &Context) -> BuiltinResult {
    BuiltinResult::Error("Delete not implemented".to_string())
}
