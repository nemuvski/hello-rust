// 温度を華氏と摂氏で変換する。
// https://doc.rust-jp.rs/book/second-edition/ch03-05-control-flow.html

use std::io;

fn main() {
  println!("摂氏を入力してください。");

  let mut celsius = String::new();
  io::stdin().read_line(&mut celsius)
    .expect("標準入力の読み込み失敗しました。");
  let celsius: f32 = celsius.trim().parse()
    .expect("入力値に誤りがあります。");

  let fahrenheit = celsius_to_fahrenheit(celsius);

  println!("摂氏: {}°C , 華氏: {}°F", celsius, fahrenheit);
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
  celsius * 1.8 + 32.0
}
