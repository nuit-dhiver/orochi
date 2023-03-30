fn command_handler(input: &str) {
    // Split the input string into two parts at the first space character
    let (command, args) = match input.find(' ') {
        Some(i) => input.split_at(i),
        None => (input, ""),
    };
    
    // Trim the leading space character from the arguments string
    let args = args.trim_start();
    
    // Handle different commands based on the main command string
    if command == "command1" {
        // Call a function to handle command1 with the arguments
        handle_command1(args);
    } else if command == "command2" {
        // Call a function to handle command2 with the arguments
        handle_command2(args);
    } else {
        // Handle unknown command error
        println!("Unknown command: {}", command);
    }
}
