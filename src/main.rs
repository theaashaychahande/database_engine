// Import the HashMap collection from the standard library.
// A HashMap stores key-value pairs and lets us look up values by key.
use std::collections::HashMap;

// Import the File type so we can write to files.
use std::fs::File;

// Import the Write trait, which gives Files the `.write_all` method.
use std::io::Write;

// Import the Read trait, which gives Files the `.read_to_string` method.
use std::io::Read;

// Import stdin so we can read lines typed by the user.
use std::io::stdin;

// Save every key-value pair in the HashMap to a file called data.txt.
// Each pair is written on its own line in the format key=value.
fn save_to_file(store: &HashMap<String, String>) {
    // Try to create (or overwrite) the file `data.txt`.
    // If it fails, print a friendly message and give up without crashing.
    let mut file = match File::create("data.txt") {
        Ok(file) => file,
        Err(error) => {
            println!("error: could not create data.txt: {}", error);
            return;
        }
    };

    // Loop over each key-value pair in the HashMap.
    for (key, value) in store {
        // Build the line "key=value\n" for the current pair.
        let line = format!("{}={}\n", key, value);

        // Try to write the line to the file.
        // If it fails, print a friendly message and stop saving.
        if let Err(error) = file.write_all(line.as_bytes()) {
            println!("error: could not write to data.txt: {}", error);
            return;
        }
    }

    // Tell the user the data was saved.
    println!("saved to data.txt");
}

// Read the file data.txt (if it exists) and rebuild the HashMap from it.
// Each line is expected to be in the format key=value.
fn load_from_file(store: &mut HashMap<String, String>) {
    // Open data.txt if it exists.
    // If it does not exist yet, tell the user and start with an empty store.
    let mut file = match File::open("data.txt") {
        Ok(file) => file,
        Err(_) => {
            println!("data.txt does not exist yet, starting with no saved data");
            return;
        }
    };

    // Try to read the whole file into one string.
    // If it fails, print a friendly message and start with an empty store.
    let mut contents = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
        println!("error: could not read data.txt: {}", error);
        return;
    }

    // Loop over each line of the file.
    for line in contents.lines() {
        // Split the line at the first '=' to separate key and value.
        // A line like "name=database_engine" gives key="name", value="database_engine".
        match line.split_once('=') {
            // The line has an '=', but the key part is empty.
            Some((key, _)) if key.trim().is_empty() => {
                println!("warning: skipping line with empty key: {}", line);
            }
            // The line is well-formed, so insert the key-value pair.
            Some((key, value)) => {
                store.insert(String::from(key), String::from(value));
            }
            // The line has no '=' at all, so it is corrupted. Skip it.
            None => {
                println!("warning: skipping corrupted line: {}", line);
            }
        }
    }
}

// Read one line of text typed by the user, without the trailing newline.
// Returns None if the input ended (for example, end of file).
fn read_line() -> Option<String> {
    // Create an empty String to hold the line.
    let mut line = String::new();

    // Try to read from standard input.
    // If reading fails, print a friendly message and return None.
    if let Err(error) = stdin().read_line(&mut line) {
        println!("error: could not read input: {}", error);
        return None;
    }

    // If zero bytes were read, the input has ended. Return None.
    if line.is_empty() {
        return None;
    }

    // Remove the trailing newline (and any carriage return) and return the line.
    Some(line.trim().to_string())
}

fn main() {
    // Create a new empty HashMap.
    // The key type (String) is noted with `Key`, the value type (String) with `Value`.
    let mut store: HashMap<String, String> = HashMap::new();

    // Load any previously saved data from data.txt.
    load_from_file(&mut store);

    // Print the available commands once so the user knows what they can do.
    println!("Commands: insert, get, delete, list, exit");

    // Keep asking for commands until the user types "exit".
    loop {
        // Ask the user for a command.
        print!("\n> ");
        println!(); // newline after the prompt

        // Read the command the user typed.
        // If the input has ended, stop the loop gracefully.
        let command = match read_line() {
            Some(command) => command,
            None => {
                println!("input ended, saving and exiting");
                save_to_file(&store);
                break;
            }
        };

        // Decide what to do based on the command.
        match command.as_str() {
            // Add a new key-value pair to the HashMap.
            "insert" => {
                // Ask for the key and read it.
                print!("key: ");
                let Some(key) = read_line() else {
                    println!("error: could not read key");
                    continue;
                };

                // Reject an empty key so the data stays clean.
                if key.is_empty() {
                    println!("error: key cannot be empty");
                    continue;
                }

                // Ask for the value and read it.
                print!("value: ");
                let Some(value) = read_line() else {
                    println!("error: could not read value");
                    continue;
                };

                // Insert the pair into the HashMap.
                store.insert(key, value);
                println!("inserted");
            }

            // Look up a key and print its value if it exists.
            "get" => {
                // Ask for the key and read it.
                print!("key: ");
                let Some(key) = read_line() else {
                    println!("error: could not read key");
                    continue;
                };

                // Reject an empty key so the data stays clean.
                if key.is_empty() {
                    println!("error: key cannot be empty");
                    continue;
                }

                // Look up the key in the HashMap.
                match store.get(&key) {
                    // The key was found, so print its value.
                    Some(value) => println!("{}", value),
                    // The key was not found, so say so.
                    None => println!("key not found"),
                }
            }

            // Remove a key-value pair from the HashMap.
            "delete" => {
                // Ask for the key and read it.
                print!("key: ");
                let Some(key) = read_line() else {
                    println!("error: could not read key");
                    continue;
                };

                // Reject an empty key so the data stays clean.
                if key.is_empty() {
                    println!("error: key cannot be empty");
                    continue;
                }

                // Remove the key from the HashMap. If it was present, say so.
                match store.remove(&key) {
                    Some(_) => println!("deleted"),
                    None => println!("key not found"),
                }
            }

            // Print every key-value pair currently stored.
            "list" => {
                // Print each pair in the HashMap.
                for (key, value) in &store {
                    println!("{}={}", key, value);
                }
            }

            // Save everything and stop the loop.
            "exit" => {
                // Save the HashMap to data.txt.
                save_to_file(&store);
                break;
            }

            // Any other input is not a valid command.
            _ => println!("unknown command"),
        }
    }
}