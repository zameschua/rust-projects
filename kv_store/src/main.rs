use std::str::FromStr;
use std::io;
use std::env;

mod store;

const DEFAULT_LOG_FILE_NAME: &str = "./kv_log.csv";

fn main() {
    // 1. Parse command line argument for custom log file name
    let command_line_arguments: Vec<String> = env::args().collect();
    let log_file_name: &str = if command_line_arguments.len() >= 2 {
      &command_line_arguments[1]
    } else {
      DEFAULT_LOG_FILE_NAME
    };

     // 2. Populate in memory store
    let mut store: store::Store = match store::Store::rehydrate_from(log_file_name) {
      Ok(store) => store,
      Err(_) => panic!("Failed to rehydrate store")
    };

    // 3. Main loop: Parse and execute commands
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
      let command: Command = match Command::from_str(line_values[0]) {
        Ok(command) => command,
        Err(()) => {
          println!("Received an invalid command, command should be one of GET or SET");
          continue;
        }
      };

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
            continue;
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
