// Import the HashMap collection from the standard library.
// A HashMap stores key-value pairs and lets us look up values by key.
use std::collections::HashMap;

// Import the File type so we can write to files.
use std::fs::File;

// Import the Write trait, which gives Files the `.write_all` method.
use std::io::Write;

// Import the Read trait, which gives Files the `.read_to_string` method.
use std::io::Read;

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

fn main() {
    // Create a new empty HashMap.
    // The key type (String) is noted with `Key`, the value type (String) with `Value`.
    let mut store: HashMap<String, String> = HashMap::new();

    // Load any previously saved data from data.txt.
    load_from_file(&mut store);

    // Insert the first key-value pair into the HashMap.
    store.insert(
        String::from("name"),
        String::from("database_engine"),
    );

    // Insert a second key-value pair into the HashMap.
    store.insert(
        String::from("version"),
        String::from("0.1.0"),
    );

    // Print the entire HashMap so we can see its contents.
    println!("{:#?}", store);

    // Save the HashMap to data.txt.
    save_to_file(&store);
}