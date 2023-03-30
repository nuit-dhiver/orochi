use serde_json::json;
use serde_json::Value;
use uuid::Uuid;
use std::fs::OpenOptions;
use std::io::Write;

pub fn insert_entity(value: Value) -> Result<(), Box<dyn std::error::Error>> {
    let uuid = Uuid::new_v4();
    let mut object = json!({"uuid": uuid.hyphenated().to_string()});
    object["data"] = value;
    let json_string = serde_json::to_string_pretty(&object)?;
    let mut entity_file = OpenOptions::new().append(true).open("entities.json")?;
    entity_file.write_all(json_string.as_bytes())?;
    Ok(())
}
