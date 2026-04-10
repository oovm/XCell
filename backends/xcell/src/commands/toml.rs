use clap::{Arg, ArgAction, Command};
use std::fs;
use std::path::Path;
use oak_toml::{TomlValue, TomlTable, TomlArray, to_string, from_str};

use crate::utils::read_to_string;

pub fn toml_command() -> Command {
    Command::new("toml")
        .about("Edit TOML configuration files")
        .subcommand(
            Command::new("list")
                .about("List fields in a TOML file")
                .arg(Arg::new("file").required(true).help("Path to TOML file")),
        )
        .subcommand(
            Command::new("add")
                .about("Add a field to a TOML file")
                .arg(Arg::new("file").required(true).help("Path to TOML file"))
                .arg(Arg::new("name").required(true).help("Field name"))
                .arg(Arg::new("type").required(true).help("Field type"))
                .arg(Arg::new("comment").help("Field comment"))
                .arg(Arg::new("default").help("Field default value")),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a field from a TOML file")
                .arg(Arg::new("file").required(true).help("Path to TOML file"))
                .arg(Arg::new("name").required(true).help("Field name")),
        )
        .subcommand(
            Command::new("update")
                .about("Update a field in a TOML file")
                .arg(Arg::new("file").required(true).help("Path to TOML file"))
                .arg(Arg::new("name").required(true).help("Field name"))
                .arg(Arg::new("type").help("Field type"))
                .arg(Arg::new("comment").help("Field comment"))
                .arg(Arg::new("default").help("Field default value")),
        )
}

pub fn handle_toml_command(cmd: &clap::ArgMatches) -> anyhow::Result<()> {
    match cmd.subcommand() {
        Some(("list", subcmd)) => handle_toml_list(subcmd),
        Some(("add", subcmd)) => handle_toml_add(subcmd),
        Some(("remove", subcmd)) => handle_toml_remove(subcmd),
        Some(("update", subcmd)) => handle_toml_update(subcmd),
        _ => unreachable!(),
    }
}

fn handle_toml_list(cmd: &clap::ArgMatches) -> anyhow::Result<()> {
    let file = cmd.get_one::<String>("file").unwrap();
    let path = Path::new(file);
    
    if !path.exists() {
        anyhow::bail!("File does not exist: {}", file);
    }
    
    let content = read_to_string(path)?;
    let table: TomlValue = from_str(&content)?;
    
    if let Some(fields) = table.get("fields").and_then(|v| v.as_array()) {
        println!("Fields in {}", file);
        println!("{:-<50}", "");
        for (i, field) in fields.iter().enumerate() {
            let name = field.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let type_name = field.get("type").and_then(|v| v.as_str()).unwrap_or("");
            let comment = field.get("comment").and_then(|v| v.as_str()).unwrap_or("");
            let default = field.get("default").and_then(|v| v.as_str()).unwrap_or("");
            
            println!("{}: {} ({}), comment: {}, default: {}", i + 1, name, type_name, comment, default);
        }
    } else {
        println!("No fields found in {}", file);
    }
    
    Ok(())
}

fn handle_toml_add(cmd: &clap::ArgMatches) -> anyhow::Result<()> {
    let file = cmd.get_one::<String>("file").unwrap();
    let name = cmd.get_one::<String>("name").unwrap();
    let type_name = cmd.get_one::<String>("type").unwrap();
    let comment = cmd.get_one::<String>("comment");
    let default = cmd.get_one::<String>("default");
    
    let path = Path::new(file);
    
    let mut table: TomlValue = if path.exists() {
        let content = read_to_string(path)?;
        from_str(&content)?
    } else {
        TomlValue::Table(TomlTable { dict: std::collections::HashMap::new() })
    };
    
    let fields = table.get_mut("fields").and_then(|v| v.as_array_mut());
    let fields = match fields {
        Some(fields) => fields,
        None => {
            let new_array = TomlValue::Array(TomlArray { list: vec![] });
            if let TomlValue::Table(ref mut table) = table {
                table.dict.insert("fields".to_string(), new_array);
            }
            table.get_mut("fields").and_then(|v| v.as_array_mut()).unwrap()
        }
    };
    
    let mut field = TomlValue::Table(TomlTable { dict: std::collections::HashMap::new() });
    if let TomlValue::Table(ref mut field_table) = field {
        field_table.dict.insert("name".to_string(), TomlValue::String(name.to_string()));
        field_table.dict.insert("type".to_string(), TomlValue::String(type_name.to_string()));
        if let Some(comment) = comment {
            field_table.dict.insert("comment".to_string(), TomlValue::String(comment.to_string()));
        }
        if let Some(default) = default {
            field_table.dict.insert("default".to_string(), TomlValue::String(default.to_string()));
        }
    }
    
    fields.push(field);
    
    let content = to_string(&table)?;
    fs::write(path, content)?;
    
    println!("Added field {} to {}", name, file);
    
    Ok(())
}

fn handle_toml_remove(cmd: &clap::ArgMatches) -> anyhow::Result<()> {
    let file = cmd.get_one::<String>("file").unwrap();
    let name = cmd.get_one::<String>("name").unwrap();
    
    let path = Path::new(file);
    
    if !path.exists() {
        anyhow::bail!("File does not exist: {}", file);
    }
    
    let content = read_to_string(path)?;
    let mut table: TomlValue = from_str(&content)?;
    
    if let Some(fields) = table.get_mut("fields").and_then(|v| v.as_array_mut()) {
        let initial_len = fields.len();
        fields.retain(|field| {
            field.get("name").and_then(|v| v.as_str()) != Some(name)
        });
        
        if fields.len() < initial_len {
            let content = to_string(&table)?;
            fs::write(path, content)?;
            println!("Removed field {} from {}", name, file);
        } else {
            println!("Field {} not found in {}", name, file);
        }
    } else {
        println!("No fields found in {}", file);
    }
    
    Ok(())
}

fn handle_toml_update(cmd: &clap::ArgMatches) -> anyhow::Result<()> {
    let file = cmd.get_one::<String>("file").unwrap();
    let name = cmd.get_one::<String>("name").unwrap();
    let type_name = cmd.get_one::<String>("type");
    let comment = cmd.get_one::<String>("comment");
    let default = cmd.get_one::<String>("default");
    
    let path = Path::new(file);
    
    if !path.exists() {
        anyhow::bail!("File does not exist: {}", file);
    }
    
    let content = read_to_string(path)?;
    let mut table: TomlValue = from_str(&content)?;
    
    if let Some(fields) = table.get_mut("fields").and_then(|v| v.as_array_mut()) {
        let mut found = false;
        for field in fields {
            if let TomlValue::Table(ref mut field_table) = field {
                if field_table.dict.get("name").and_then(|v| v.as_str()) == Some(name) {
                    if let Some(type_name) = type_name {
                        field_table.dict.insert("type".to_string(), TomlValue::String(type_name.to_string()));
                    }
                    if let Some(comment) = comment {
                        field_table.dict.insert("comment".to_string(), TomlValue::String(comment.to_string()));
                    }
                    if let Some(default) = default {
                        field_table.dict.insert("default".to_string(), TomlValue::String(default.to_string()));
                    }
                    found = true;
                    break;
                }
            }
        }
        
        if found {
            let content = to_string(&table)?;
            fs::write(path, content)?;
            println!("Updated field {} in {}", name, file);
        } else {
            println!("Field {} not found in {}", name, file);
        }
    } else {
        println!("No fields found in {}", file);
    }
    
    Ok(())
}
