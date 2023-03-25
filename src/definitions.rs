use std::fs::File;
use std::io::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]

let struct Entity { 
    key: Uuid, 
    value: String 
}

fn insert_entity(passed_value: &str) -> Result<()> {
    let new_record = { key: Uuid::new_v4(), value String::from(passed_value)}
    //let encoded: Vec<u8> = bincode::serialize(&new_record).unwrap()
    let mut entity_file = File::open(./data/entity)?;
    let buffer = serde_bincode::serialize(&new_record)?;
    entity_file.write_all(&buffer[..])?;
    Ok(())
}
