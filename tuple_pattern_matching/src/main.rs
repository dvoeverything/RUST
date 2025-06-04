fn execute_command_refactored(mode: &str, status: &str) {

    match (mode, status) {

        ("admin","active") => {println!("Admin privileges granted. Executing active command.")},
        ("normal","pending") => println!("Normal operation. Execute pending or active commands."),
        ("maintenance","complete") => println!("Maintenance complete. System can resume normal operation."),
        _ => println!("No action needed or invalid mode/status."),
              
    }
}
 
 
fn main() {
    execute_command_refactored("admin", "active");
    execute_command_refactored("normal", "pending");
    execute_command_refactored("maintenance", "complete");
    execute_command_refactored("admin", "pending");  
}