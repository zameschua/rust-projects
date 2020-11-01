# Key Value Store (WIP)

This is a very simple toy CLI-based key-value store
It was built with the intention get used to the Rust language syntax

# Running the program
```
cargo run [path_to_log_file]
```

# Commands
```
// Get the value from a key previously placed in the store
GET key

// Add a new record to the store with key = value
SET key value
```

# Todo
1. Implement concurrency control (MVCC?)
2. Implement a way to handle when there are too many keys to fit into memory
3. Implement compaction to reduce the amount of memory used
4. Implement record deletion (with tombstones)


# Things that I learnt from this project
1. Rust `Result` and `Option` type
2. Basic data structures in Rust like `Vec` and `HashMap`
3. Writing tests in Rust
4. Writing rustdoc comments
5. How a log-based store works (work in progress)
