use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();
  // Note: unwrap_or_elseで,panic!ではなく独自のエラー処理を定義できる.
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("{}", err);
    process::exit(1);
  });

  // Note: runがErr値を返したかどうかだけ気にすれば良いので,if-letで記述している.(成功時は()でしかないため)
  if let Err(err) = run(config) {
    println!("Application error: {}", err);
    process::exit(1);
  }
}

fn run(config: Config) -> Result<(), Box<Error>> {
  let mut f = File::open(&config.path_to_file)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;
  println!("With text:\n{}", contents);

  Ok(())
}

struct Config {
  query: String,
  path_to_file: String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str> {
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
