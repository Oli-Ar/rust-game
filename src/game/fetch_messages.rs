use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Function to fetch messages from file
pub fn fetch_messages() -> HashMap<String, String> {
    let mut msg_map: HashMap<String, String> = HashMap::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("assets/messages.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(message) = line {
                if message.is_empty() || message.starts_with("//") {
                    continue;
                };
                let message_components: Vec<&str> = message.splitn(2, ": ").collect();
                msg_map.insert(
                    message_components[0].to_string(),
                    message_components[1].to_string(),
                );
            }
        }
    };
    msg_map
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
