use std::fs;
use std::fs::File;
use std::io::{self, Write, LineWriter, BufRead};
use std::str::FromStr;
use std::collections::HashMap;
use std::path::Path;


const LOG_FILE_NAME: &str = "./kv_log.txt";

fn main() {
     // 1. Populate in memory store
    let mut store: Store = match Store::rehydrate_from(LOG_FILE_NAME) {
      Ok(store) => store,
      Err(_) => panic!("Failed to rehydrate store")
    };

    // 2. Read command line arguments
    loop {
      let mut line_buffer = String::new();
      io::stdin()
        .read_line(&mut line_buffer)
        .expect("Failed to read line");
        
      let line_values: Vec<&str> = line_buffer.trim().split(' ').collect();
      if line_values.len() <= 1 {
        println!("Expected command like GET (key) or SET (key) (value)");
        continue
      }
      let command: Command = Command::from_str(line_values[0]).expect("Received an invalid command, command should be one of GET or SET");      

      match command {
        Command::GET => {
          if line_values.len() <= 1 {
            println!("Expected command like GET (key)");
            continue
          }
          let key: &str = line_values[1];
          let value: Option<&String> = store.get(key);
          match value {
            Some(value) => println!("{}", value),
            None => println!("No value found!")
          }
        },
        Command::SET => {
          if line_values.len() <= 2 {
            println!("Expected command like SET (key) (value)");
            continue
          }
          let key: &str = line_values[1];
          let value: &str = {
            if line_values.len() >= 2 {
              line_values[2]
            } else {
              ""
            }
          };

          match store.set(key, value) {
            Ok(()) => println!("OK"),
            Err(_) => println!("Error, did not set value")
          }
        }
      }
    }
}


enum Command {
  GET,
  SET
}

// Implement FromStr trait so that we can parse command input into our enum type
impl FromStr for Command {

  type Err = ();

  fn from_str(input: &str) -> Result<Command, Self::Err> {
      match input {
          "GET"  => Ok(Command::GET),
          "SET"  => Ok(Command::SET),
          _      => Err(()),
      }
  }
}

struct Store {
  hashmap: HashMap<String, String>
}



impl Store {
  // Errors from this method are propogated up the stack
  fn rehydrate_from(file_name: &str) -> Result<Store, ()> {
    let mut hashmap: HashMap<String, String> = HashMap::new();

    if let Ok(lines) = read_lines(file_name) {
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
      hashmap: hashmap,
    })
  }
  
  fn get(&self, key: &str) -> Option<&String> {
    self.hashmap.get(key)
  }

  fn set(&mut self, key: &str, value: &str) -> Result<(), io::Error> {
    // 1. Write into the write-ahead log
    let file: File = fs::OpenOptions::new()
    .write(true)
    .append(true)
    .create_new(true)
    .open(LOG_FILE_NAME)
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