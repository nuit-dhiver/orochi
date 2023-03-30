mod takeoff;
mod operations;


fn main() {
    if let Err(err) = takeoff::pre_flight_check() {
        eprintln!("Pre-flight check failed: {}", err);
        return;
    }
    
    operations::insert_entity("{}".into());
}
