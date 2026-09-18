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
    // Create (or overwrite) the file `data.txt`.
    let mut file = File::create("data.txt").expect("failed to create data.txt");

    // Loop over each key-value pair in the HashMap.
    for (key, value) in store {
        // Build the line "key=value\n" for the current pair.
        let line = format!("{}={}\n", key, value);

        // Write the line to the file.
        file.write_all(line.as_bytes())
            .expect("failed to write to data.txt");
    }
}

// Read the file data.txt (if it exists) and rebuild the HashMap from it.
// Each line is expected to be in the format key=value.
fn load_from_file(store: &mut HashMap<String, String>) {
    // Open data.txt if it exists; if not, there is nothing to load.
    let mut file = match File::open("data.txt") {
        Ok(file) => file,
        Err(_) => return,
    };

    // Read the whole file into one string.
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read data.txt");

    // Loop over each line of the file.
    for line in contents.lines() {
        // Split the line at the first '=' to separate key and value.
        // A line like "name=database_engine" gives key="name", value="database_engine".
        if let Some((key, value)) = line.split_once('=') {
            // Insert the key-value pair into the HashMap.
            store.insert(String::from(key), String::from(value));
        }
    }
}

// Read one line of text typed by the user, without the trailing newline.
fn read_line() -> String {
    // Create an empty String to hold the line.
    let mut line = String::new();

    // Read from standard input and stop the program if reading fails.
    stdin().read_line(&mut line).expect("failed to read input");

    // Remove the trailing newline (and any carriage return) and return the line.
    line.trim().to_string()
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
        let command = read_line();

        // Decide what to do based on the command.
        match command.as_str() {
            // Add a new key-value pair to the HashMap.
            "insert" => {
                // Ask for the key and read it.
                print!("key: ");
                let key = read_line();

                // Ask for the value and read it.
                print!("value: ");
                let value = read_line();

                // Insert the pair into the HashMap.
                store.insert(key, value);
            }

            // Look up a key and print its value if it exists.
            "get" => {
                // Ask for the key and read it.
                print!("key: ");
                let key = read_line();

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
                let key = read_line();

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