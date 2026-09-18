# Database Engine

> A tiny key-value database, written from scratch in Rust, that lives entirely
> in memory during the day and sleeps on disk at night.

Let me be honest about what this is: it is not trying to compete with
PostgreSQL. It is a deliberately small, deliberately understandable piece of
software that captures the *soul* of a database — a place where data goes in,
gets looked up, gets deleted, and doesn't disappear when you close the
terminal. If you strip away the query planners, the buffer pools, and the
millions of lines of cleverness, this is roughly what's left.

## Why I built it

Databases are these towering black boxes. We throw queries at them and trust
that *somewhere* in there, data is being stored, indexed, cached, and
recovered in ways we never actually see. I wanted to peel back one layer of
that mystery and build the smallest possible thing that still deserves the
name "database engine." No external crate for storage, no ORM, no magic — just
Rust, the standard library, and one plain text file.

## What it can do

The engine exposes a small, predictable command surface:

| Command    | What it does |
|------------|--------------|
| `insert`   | Prompts for a key and a value, then stores the pair. |
| `get`      | Prompts for a key and fetches its value — instantly. |
| `delete`   | Prompts for a key and removes the pair if it exists. |
| `list`     | Dumps every `key=value` pair currently in memory. |
| `exit`     | Flushes everything to `data.txt` and shuts down. |

Under the hood there are three capabilities that make it feel like a real
database:

1. **In-memory storage** — everything lives in a `std::collections::HashMap`,
   which means lookups don't scan. More on that below.

2. **Persistence to disk** — on `exit` the whole store is written to
   `data.txt` (one `key=value` per line). On startup the file is read back and
   the store is rebuilt, so your data survives across every run. It is a
   primitive write-ahead of sorts — a very primitive one.

3. **Graceful failure** — a database that crashes because the user typed an
   empty key is a database nobody can trust. So a missing file just means
   "we start empty," an empty key is politely rejected, and a corrupted line
   in `data.txt` is skipped with a warning. No panics, no data loss, no drama.

## How the persistence works

`data.txt` is deliberately boring:

```text
name=database_engine
version=0.1.0
```

One key, an `=`, a value. That's the entire on-disk format. It's human
readable, trivially debuggable, and it makes it really easy to understand the
two phases of the engine's life cycle:

- **Load** (`load_from_file`): the file is opened, read into a string, and
  each line is split on the first `=`. Well-formed lines become entries;
  malformed lines get a warning and are skipped.
- **Save** (`save_to_file`): the file is rewritten from scratch, one line per
  pair. Overwriting is fine here precisely because the load step is cheap and
  idempotent.

I chose plain text over binary on purpose. It costs a little space and a little
speed, but it buys enormous clarity — you can open the file in an editor,
grep it, diff it, and see exactly what the database believes it knows.

## How lookups stay fast

This is the part I find genuinely interesting. A naive database would answer a
`get` by scanning every entry until it found the key — fine for three rows,
painfully slow for three million. We never scan, because the `HashMap` does the
work of a *hash index*:

- When you insert `name=database_engine`, Rust computes a hash of the key
  `"name"` and drops the entry into the bucket that hash maps to.
- When you `get` the key, Rust hashes it again, lands on the same bucket, and
  returns the value. Average case: **O(1)** — constant time, regardless of how
  many keys are in the store.

Real databases do the same trick in fancier clothes. A **B-tree** keeps sorted
keys in a balanced tree so a lookup touches only O(log N) nodes, all carefully
packed to minimize disk reads. A **hash index** — which is closer to what we're
doing here — hashes the key and jumps straight to the right bucket. But they
all share the same philosophy: build a shortcut to the data so you never have
to look at all of it. Our `HashMap` is that shortcut.

## Running it

You need a Rust toolchain (anything recent; the project targets edition 2024):

```sh
cargo run
```

Then talk to it:

```text
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

Run it again a minute later — `name` is still there. That's persistence doing
its quiet job.

## What building this taught me

- **A database is a data structure with opinions about survival.** The hard
  part isn't storing things; it's deciding what happens on the edges — when
  the file is gone, when the input is nonsense, when the user just wants out.
- **`HashMap` is already an index.** I spent a long time treating collections
  as plumbing until I realized the lookup table *is* the index, just without
  the marketing.
- **Graceful > fast.** The slowest correct path beats the fast crashing one.
  Handling the empty key and the corrupted line made this feel more like a real
  product than any feature ever could.
- **Rust's standard library is enough.** No dependencies were harmed — or
  added — in the making of this database. `File`, `Write`, `Read`, `stdin`,
  and `HashMap` covered everything between first keystroke and a working,
  persistent key-value store.
- **The gap between "code" and "database" is mostly discipline.** Commit to a
  format, persist it, load it back, validate your inputs, and you're 90% of
  the way to something you'd trust with real data.

## A roadmap, if it ever grows up

If this little engine ever wants to be a real database, the natural next steps
are: a proper on-disk format with checksums, a B-tree index instead of
re-loading everything into memory, and transactions so a crash mid-write never
leaves the file half-muttered. For now, it's honest, it's fast, and it works.