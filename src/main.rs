// Import the HashMap collection from the standard library.
// A HashMap stores key-value pairs and lets us look up values by key.
use std::collections::HashMap;

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
}