use crate::builtins::{BuiltinResult, Context};
use crate::rune_ast::{Section, Value};
use std::collections::HashMap;
use std::sync::Arc;

pub fn builtin_data_source(
    args: &[String],
    ctx: &mut Context,
    schemas: &Arc<HashMap<String, Section>>,
    assign_to: Option<&str>,
    data_sources: &Arc<HashMap<String, Section>>,
) -> BuiltinResult {
    if args.is_empty() {
        eprintln!("[ERROR] datasource: missing arguments");
        return BuiltinResult::Error("missing arguments".to_string());
    }
    let action = &args[0];
    let name = if args.len() > 1 { &args[1] } else { "" };
    let action_args = if args.len() > 2 { &args[2..] } else { &[] };

    match action.as_str() {
        "connect" => connect_to_datasource(name, action_args, ctx, schemas, data_sources),
        "fetch_all" => {
            fetch_all_from_datasource(name, action_args, ctx, schemas, assign_to, data_sources)
        }
        "fetch" => fetch_from_datasource(name, action_args, ctx, schemas, assign_to, data_sources),
        "insert" => insert_into_datasource(name, action_args, ctx, schemas, data_sources),
        "update" => update_datasource(name, action_args, ctx, schemas, data_sources),
        "delete" => delete_from_datasource(name, action_args, ctx, schemas, data_sources),
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
) -> Vec<Value::String> {

    let schema_name = if let Some(Value::String(name)) = section.kv.get("schema") {
        name.clone()
    } else {
        return vec!["respond 500 \"Schema name not provided\"".into()];
    };
    let data_source_name = if let Some(Value::String(name)) = section.kv.get("data_source") {
        name.clone()
    } else {
        return vec!["respond 500 \"Data source name not provided\"".into()];
    };
    let schema = get_section_by_string_key(&section, "schema", schemas);
    let data_source = get_section_by_string_key(&section, "data_source", data_sources);

    if data_source.is_none() {
        return vec!["respond 500 \"Data source not configured\"".into()];
    }

    if schema.is_none() {
        return vec!["respond 500 \"Schema not found\"".into()];
    }

    let connection_command = format!("connect {}", data_source_name);
    let create_table_command = format!("create_table {}", schema_name);

    match method {
        "GET" => vec![
            Value::String(connection_command),
            Value::String(create_table_command),
            Value::String("fetch_all <name> [<query>] [<params>]".to_string()),
            Value::String("respond 200 data_object".to_string()),
        ],
        "POST" => vec![
            Value::String(connection_command),
            Value::String("insert <name> <data_object>".to_string()),
            Value::String("respond 201 data_object".to_string()),
        ],
        "PUT" => vec![
            Value::String(connection_command),
            Value::String("update <name> <data_object> [<query>] [<params>]".to_string()),
            Value::String("respond 200 data_object".to_string()),
        ],
        "DELETE" => vec![
            Value::String(connection_command),
            Value::String("delete <name> [<query>] [<params>]".to_string()),
            Value::String("respond 204".to_string()),
        ],
        _ => vec![],
    }
}

pub fn connect_to_datasource(
    name: &str,
    args: &[String],
    ctx: &Context,
    schemas: &Arc<HashMap<String, Section>>,
    data_sources: &Arc<HashMap<String, Section>>,
) -> BuiltinResult {
    let conn_type = if args.len() > 0 { &args[0] } else { "" };
    let conn_str = if args.len() > 1 { &args[1] } else { "" };

    if conn_type.is_empty() || conn_str.is_empty() {
        eprintln!("[ERROR] datasource.connect: missing connection type or string");
        return BuiltinResult::Error("missing connection type or string".to_string());
    }

    if !data_sources.contains_key(name) {
        eprintln!("[ERROR] datasource.connect: data source '{}' not found", name);
        return BuiltinResult::Error(format!("data source '{}' not found", name));
    }

    // Implementation goes here
}

pub fn fetch_all_from_datasource(
    name: &str,
    args: &[String],
    ctx: &mut Context,
    schemas: &Arc<HashMap<String, Section>>,
    assign_to: Option<&str>,
    data_sources: &Arc<HashMap<String, Section>>,
) -> BuiltinResult {
    BuiltinResult::Error("Fetch all not implemented".to_string())
}

pub fn fetch_from_datasource(
    name: &str,
    args: &[String],
    ctx: &mut Context,
    schemas: &Arc<HashMap<String, Section>>,
    assign_to: Option<&str>,
    data_sources: &Arc<HashMap<String, Section>>,
) -> BuiltinResult {
    BuiltinResult::Error("Fetch not implemented".to_string())
}

pub fn insert_into_datasource(
    name: &str,
    args: &[String],
    ctx: &Context,
    schemas: &Arc<HashMap<String, Section>>,
    data_sources: &Arc<HashMap<String, Section>>,
) -> BuiltinResult {
    BuiltinResult::Error("Insert not implemented".to_string())
}

pub fn update_datasource(
    name: &str,
    args: &[String],
    ctx: &Context,
    schemas: &Arc<HashMap<String, Section>>,
    data_sources: &Arc<HashMap<String, Section>>,
) -> BuiltinResult {
    BuiltinResult::Error("Update not implemented".to_string())
}

pub fn delete_from_datasource(
    name: &str,
    args: &[String],
    ctx: &Context,
    schemas: &Arc<HashMap<String, Section>>,
    data_sources: &Arc<HashMap<String, Section>>,
) -> BuiltinResult {
    BuiltinResult::Error("Delete not implemented".to_string())
}
