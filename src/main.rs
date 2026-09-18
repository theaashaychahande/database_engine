// Import the HashMap collection from the standard library.
// A HashMap stores key-value pairs and lets us look up values by key.
use std::collections::HashMap;

// Import the File type so we can write to files.
use std::fs::File;

// Import the Write trait, which gives Files the `.write_all` method.
use std::io::Write;

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

fn main() {
    // Create a new empty HashMap.
    // The key type (String) is noted with `Key`, the value type (String) with `Value`.
    let mut store: HashMap<String, String> = HashMap::new();

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