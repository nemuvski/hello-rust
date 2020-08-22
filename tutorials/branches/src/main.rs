fn main() {
  let number = 3;
  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

  let secondary_number = if number > 0 {
    "positive"
  } else {
    "negative"
  };

  println!("{}", secondary_number);
}
