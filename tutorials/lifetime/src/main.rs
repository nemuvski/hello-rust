fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  // ライフタイム注釈をいれないと、引数のx,またはyのどちらか
  // が返されるか実行しないとわからないため、コンパイルエラーとなる.
  // ここでは'aと記述している. (多くの場合、こんな感じらしい)
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn main() {
  let string1 = String::from("abcd");
  let string2 = "xyz";
  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);

  // 以下の例だと、string4がスコープ外で参照される可能性がある. (longlong_stringに入る可能性があるため)
  // let string3 = String::from("long string is long");
  // let longlong_string;
  // {
  //   let string4 = String::from("xyz");
  //   longlong_string = longest(string3.as_str(), string4.as_str());
  // }
  // println!("The longest string is {}", longlong_string);
}
