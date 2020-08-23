struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn main() {
  let user1 = build_user(String::from("someone@example.com"), String::from("hoge"));
  let user2 = User {
    active: false,
    ..user1
  };

  println!("{}, {}, {}, {}", user2.username, user2.email, user2.sign_in_count, user2.active);
}

fn build_user(email: String, username: String) -> User {
  User {
    username,
    email,
    sign_in_count: 1,
    active: true,
  }
}
