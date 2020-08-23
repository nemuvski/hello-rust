// フィボナッチ数列のn番目を生成する。
// https://doc.rust-jp.rs/book/second-edition/ch03-05-control-flow.html

use std::io;

fn main() {
  println!("フィボナッチ数列は何番目を求めますか？");

  let mut n = String::new();
  io::stdin().read_line(&mut n)
    .expect("標準入力の読み込みに失敗しました。");
  let n: u32 = n.trim().parse()
    .expect("正の整数値を入力してください。");

  println!("F({}) = {}", n, fibonacci_sequence(n));
}

fn fibonacci_sequence(n: u32) -> u32 {
  if n == 0 {
    0
  } else if n == 1 {
    1
  } else {
    fibonacci_sequence(n - 2) + fibonacci_sequence(n - 1)
  }
}
