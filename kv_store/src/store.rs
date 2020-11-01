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
    .write(true)
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
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}