use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  for (i, item) in args.iter().enumerate() {
    println!("args({}): {}", i+1, item);
  }
}
