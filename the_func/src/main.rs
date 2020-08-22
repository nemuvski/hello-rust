fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}

fn main() {
  println!("five(): {}", five());
  println!("plus_one(): {}", plus_one(5));
}
