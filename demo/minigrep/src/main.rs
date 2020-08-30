use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 3 {
    println!("Arguments:");
    println!("\t1st: Search string");
    println!("\t2nd: File name");
    panic!();
  }

  let query = &args[1];
  let filename = &args[2];

  println!("Searching for {}", query);
  println!("In file {}", filename);
}
