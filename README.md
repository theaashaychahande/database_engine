# Database Engine

A simple database engine written in Rust. It stores key-value pairs in memory
using a `HashMap` and persists them to a plain-text file on disk so the data
survives between runs.

## Features

- **Insert** — add a new `key=value` pair to the store.
- **Get** — look up a key and print its value.
- **Delete** — remove a `key=value` pair from the store.
- **List** — print every `key=value` pair currently stored.
- **Persistent storage** — data is saved to `data.txt` on exit and loaded back
  automatically on the next run.
- **Friendly error handling** — missing files, empty keys, and corrupted lines
  in `data.txt` are reported with a message instead of crashing the program.

## Data format

Each `key=value` pair is stored on its own line in `data.txt`:

```
name=database_engine
version=0.1.0
```

On startup the program reads `data.txt` (if it exists) and rebuilds the store
from it. On `exit` the store is written back to the file.

## Usage

Clone the repository and run the program with Cargo:

```
cargo run
```

The program shows a command prompt. Type one of the following commands:

| Command  | What it does                                            |
|----------|---------------------------------------------------------|
| `insert` | Prompts for a key and a value, then adds the pair.      |
| `get`    | Prompts for a key and prints its value if it exists.    |
| `delete` | Prompts for a key and removes the pair if it exists.    |
| `list`   | Prints all `key=value` pairs.                           |
| `exit`   | Saves the store to `data.txt` and stops the program.    |

Example session:

```
Commands: insert, get, delete, list, exit

> insert
key: name
value: database_engine
inserted

> list
name=database_engine

> get
key: name
database_engine

> exit
saved to data.txt
```

## What I learned

- How a `HashMap` stores key-value pairs and gives us fast `insert`, `get`, and
  `remove` operations.
- How to read from and write to files in Rust (`std::fs::File`,
  `std::io::Write`, `std::io::Read`).
- How to build an interactive command-line loop that reads user input.
- How to handle errors gracefully instead of panicking — missing files,
  invalid input from the user, and corrupt data are all recoverable.
- How a key-value store kept entirely in memory is persisted to disk, which is
  the basic idea behind a real embedded database.
- How a real database indexes its data (with a B-tree or a hash index) so that
  lookups do not have to scan every row, and how Rust's `HashMap` already acts
  as a simple hash index.