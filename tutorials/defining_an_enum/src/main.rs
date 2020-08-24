#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
}

#[derive(Debug)]
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn main() {
  // enum Coinについて.
  let coin_list = [
    Coin::Penny,
    Coin::Nickel,
    Coin::Dime,
    Coin::Quarter(UsState::Alabama),
    Coin::Quarter(UsState::Alaska),
  ];
  for coin in coin_list.iter() {
    println!("{}", value_in_cents(coin));
  }

  // Optionについて.
  let five = Some(5);
  let result = plus_one(five);
  if result != None {
    println!("{:?}", result);
  }
}

fn value_in_cents(coin: &Coin) -> u32 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    },
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}
