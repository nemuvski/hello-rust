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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let mut f = File::open(&config.path_to_file)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  for line in search(&config.query, &contents) {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

// Note: testコマンドを実行した時のみコンパイルされる.
#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_one_result() {
    let query = "duct";
    let contents = "Rust:\n\
      safe, fast, productive.\n\
      Pick three.";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    );
  }
}
