use std::env;
use std::env::Args;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
  pub query: String,
  pub path_to_file: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: Args) -> Result<Config, &'static str> {
    if args.len() != 3 {
      return Err("Invalid argument!!!\n\
        Arguments:\n\
        \t1st: Search string\n\
        \t2nd: Path to a File
      ");
    }

    // Note: 1番目の引数はプログラム名を指し,ここでは不要.
    args.next();
    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Did not get a query string"),
    };
    let path_to_file = match args.next() {
      Some(arg) => arg,
      None => return Err("Did not get a path to file")
    };
    let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

    Ok(Config {
      query,
      path_to_file,
      case_sensitive,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let mut f = File::open(&config.path_to_file)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_insensitive(&config.query, &contents)
  };

  for line in results {
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

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let query = query.to_lowercase();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
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
  fn case_one_result() {
    let query = "duct";
    let contents = "Rust:\n\
      safe, fast, productive.\n\
      Pick three.";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    );
  }

  #[test]
  fn case_no_result() {
    let query = "ABCDEFG";
    let contents = "hoge hugo\ntest";
    let results = search(query, contents);
    assert_eq!(0, results.len());
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "Rust:\n\
      safe, fast, productive.\n\
      Pick three.\n\
      Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_insensitive(query, contents)
    );
  }

  #[test]
  fn case_sensitive() {
    let query = "rUsT";
    let contents = "Rust:\n\
      safe, fast, productive.\n\
      Pick three.\n\
      Trust me.";
    let results = search(query, contents);
    assert_eq!(0, results.len());
  }
}
