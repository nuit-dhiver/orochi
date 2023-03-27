use std::fs::File;
use std::io::prelude::*;
use serde_json::{json, value};
use uuid::Uuid;

fn insert_entity(value: Value) -> Result<(), Box<dyn std::error::Error>> {
    let uuid = Uuid::new_v4();
    let mut object = json!({"uuid": uuid.to_hyphenated().to_string()});
    object["data"] = value;
    let json_string = serde_json::to_string_pretty(&object)?;
    let mut entity_file = OpenOptions::new().append(true).open("entities.json")?;
    entity_file.write_all(json_string.as_bytes())?;
    Ok(())
}
