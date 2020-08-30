use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    println!("Arguments:");
    println!("\t1st: Search string");
    println!("\t2nd: Path to File");
    panic!();
  }
  let query = &args[1];
  let path_to_file = &args[2];

  let mut f = File::open(path_to_file).expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

  println!("With text:\n{}", contents);
}
