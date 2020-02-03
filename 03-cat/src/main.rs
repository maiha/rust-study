use std::env;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let args: Vec<_> = env::args().collect();

  match args.get(1) {
    Some(filename) => {
      let file = File::open(filename).expect("No such file or directory");
      let reader = BufReader::new(file);

      for (_index, line) in reader.lines().enumerate() {
	let line = line.unwrap(); // Ignore errors.
	println!("{}", line);
      }
    },
    None => {
      eprintln!("args(1) not found");
      std::process::exit(1)
    },
  }
}
