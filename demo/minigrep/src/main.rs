extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
  let args: Vec<String> = env::args().collect();
  // Note: unwrap_or_elseで,panic!ではなく独自のエラー処理を定義できる.
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("{}", err);
    process::exit(1);
  });

  // Note: runがErr値を返したかどうかだけ気にすれば良いので,if-letで記述している.(成功時は()でしかないため)
  if let Err(err) = minigrep::run(config) {
    println!("Application error: {}", err);
    process::exit(1);
  }
}
