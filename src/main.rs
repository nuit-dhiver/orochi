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
        }
    }
}
