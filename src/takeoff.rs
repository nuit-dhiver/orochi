use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use std::path::Path;

fn pre_flight_check() -> Result<(), Error> {
    let entities_path = Path::new("entities.json");
    let relations_path = Path::new("relations.json");

    if !entities_path.exists() {
        let entities_file = File::create(entities_path)?;
        let empty_entities_json = "{}";
        entities_file.write_all(empty_entities_json.as_bytes())?;
    }

    if !relations_path.exists() {
        let relations_file = File::create(relations_path)?;
        let empty_relations_json = "{}";
        relations_file.write_all(empty_relations_json.as_bytes())?;
    }

    Ok(())
}
