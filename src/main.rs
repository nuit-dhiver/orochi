fn main() {
    if let Err(err) = pre_flight_check() {
        eprintln!("Pre-flight check failed: {}", err);
        return;
    }
    
    // Rest of program.
}
