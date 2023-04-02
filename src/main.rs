mod takeoff;
mod operations;
use std::io::{self, Write};
use serde_json::{Value};
use regex::Regex;


fn main() {
    if let Err(err) = takeoff::pre_flight_check() {
        eprintln!("Pre-flight check failed: {}", err);
        return;
    }
    
    loop {
        let mut input = String::new();
        print!(">: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        input = input.trim().to_string();

        let re = Regex::new(r#"^(insert|pick)\((.*)\)$"#).unwrap();
        if let Some(captures) = re.captures(&input) {
            let function = captures.get(1).unwrap().as_str();
            let arg_string = captures.get(2).unwrap().as_str();
        
            let arg: Value = match serde_json::from_str(arg_string) {
                Ok(val) => val,
                Err(_) => {
                    println!("Error: Invalid data value format");
                    continue;
                }
            };

            match function {
                "inserten" => insert_entity(arg),
                "pick" => println!("To be implemented"),
                _ => println!("Error: Unknown commad"),
            }

        } else {
            println!("Error: Invalid input format");
        }
    }
}
