use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
  pub query: String,
  pub path_to_file: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() != 3 {
      return Err("Invalid argument!!!\n\
        Arguments:\n\
        \t1st: Search string\n\
        \t2nd: Path to a File
      ");
    }
    Ok(Config {
      query: args[1].clone(),
      path_to_file: args[2].clone(),
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
  let mut f = File::open(&config.path_to_file)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  println!("With text:\n{}", contents);

  Ok(())
}
