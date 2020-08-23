fn main() {
  // 整数はスタック上に保持されるので、実際のコピーをするのは高速.
  // Stringはヒープ上に保持され、yにコピーすると所有権がmoveされる.
  let x = 5;
  let y = x;
  println!("x = {}, y = {}", x, y);

  let s = String::from("hello");
  // 関数にムーブされるので、このスコープではsは有効ではない.
  takes_owernership(s);

  let x = 6;
  makes_copy(x);
  println!("x on main: {}", x);
}

fn takes_owernership(s: String) {
  println!("{}", s);
}

fn makes_copy(x: i32) {
  println!("{}", x);
}
