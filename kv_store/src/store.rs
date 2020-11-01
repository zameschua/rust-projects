use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::{self, Write, LineWriter, BufRead};

pub struct Store {
  hashmap: HashMap<String, String>,
  log_file_name: String,
}

impl Store {
  // Errors from this method are propogated up the stack
  // Rehydrate a store from the file specified at the path,
  // Creating it if it doesn't exist
  pub fn rehydrate_from(log_file_name: &str) -> Result<Store, ()> {
    let mut hashmap: HashMap<String, String> = HashMap::new();

    if let Ok(lines) = read_lines(log_file_name) {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
          if let Ok(line) = line {
              let line_values: Vec<&str> = line.split(',').collect();
              if line_values.len() == 2 {
                let key: String = String::from(line_values[0]);
                let value: String = String::from(line_values[1]);
                hashmap.insert(String::from(key), String::from(value));
              } else {
                panic!("Failed to rehydrate store! Failed at {}", line)
              }
          }
      }
    }

    // and more! See the other methods for more details.
    Ok(Store {
      log_file_name: String::from(log_file_name),
      hashmap: hashmap,
    })
  }
  
  pub fn get(&self, key: &str) -> Option<&String> {
    self.hashmap.get(key)
  }

  pub fn set(&mut self, key: &str, value: &str) -> Result<(), io::Error> {
    // 1. Write into the write-ahead log
    let file: File = fs::OpenOptions::new()
    .append(true)
    .create(true)
    .open(&self.log_file_name)
    .unwrap();
    let mut line_writer = LineWriter::new(file);
    line_writer.write_all(format!("{key},{value}\n", key = key, value = value).as_bytes())?;

    // 2. Update our hashmap
    self.hashmap.insert(String::from(key), String::from(value));
    Ok(())
  }
}

// Helper method to read contents of file by line and return an iterator over the lines
// Create the file if it doesn't exist, open it if it exists
fn read_lines<P>(log_file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file: File = fs::OpenOptions::new()
    .read(true)
    .append(true)
    .create(true)
    .open(log_file_name)
    .unwrap();
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn store_rehydrate_success() {
    const TEST_LOG_FILE_NAME: &str = "./tests/kv_log_tests_success.csv";
    Store::rehydrate_from(TEST_LOG_FILE_NAME).expect("Failed to rehydrate store");
  }

  #[test]
  #[should_panic]
  fn store_rehydrate_failure_should_panic() {
    const TEST_LOG_FILE_NAME: &str = "./tests/kv_log_tests_failure.csv";
    Store::rehydrate_from(TEST_LOG_FILE_NAME).expect("Failed to rehydrate store");
  }

  #[test]
  fn store_rehydrate_non_existant_file() {
    const TEST_LOG_FILE_NAME: &str = "./tests/non_existant_file.csv";
    Store::rehydrate_from(TEST_LOG_FILE_NAME).expect("Failed to rehydrate store");

    // Cleanup
    fs::remove_file(TEST_LOG_FILE_NAME).expect("Failed to remove file after running tests!")
  }

  #[test]
  fn store_get_nonexistent_key_should_handle_gracefully() {
    const TEST_LOG_FILE_NAME: &str = "./tests/temp_file.csv";
    let store = Store::rehydrate_from(TEST_LOG_FILE_NAME).expect("Failed to rehydrate store");
    
    store.get("nonexistent-key-123");

    // Cleanup
    fs::remove_file(TEST_LOG_FILE_NAME).expect("Failed to remove file after running tests!")
  }

  #[test]
  fn store_set_and_get_success() {
    const TEST_LOG_FILE_NAME: &str = "./tests/temp_file.csv";
    let mut store = Store::rehydrate_from(TEST_LOG_FILE_NAME).expect("Failed to rehydrate store");
    
    let key = "key";
    let value = "value";
    store.set(key, value).unwrap();
    let retrieved_value = store.get(key).unwrap();

    // Cleanup
    fs::remove_file(TEST_LOG_FILE_NAME).expect("Failed to remove file after running tests!");


    assert_eq!(value, retrieved_value);
  }
}