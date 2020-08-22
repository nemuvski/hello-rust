// mainはエントリーポイント.
fn main() {
  // 変数xにあとで値を代入する場合はmutでミュータブルな変数として宣言しなければならない.
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  const MAX_POINTS: u32 = 100_000;
  println!("{}", MAX_POINTS);

  // 型注釈をいれなければエラーになる.
  let guess: u32 = "42".parse().expect("Not a number!");
  println!("{}", guess);

  // タプルは複数の型の値を扱える.
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  println!("{0} {1} {2}", tup.0, tup.1, tup.2);
  let (x, y, z) = tup;
  println!("{0} {1} {2}", x, y, z);

  // 配列は型が統一される.
  let arr = [1, 2, 3];
  println!("{}", arr[0]);

  another_function(4, 5);

  // hoge + 1の文末にセミコロンがない点に注目.
  // 式の終端にセミコロンを付けたら「文」に変えてしまう. そして、文は値を返さないため、エラーになる.
  let hugo = {
    let hoge = 3;
    hoge + 1
  };
  println!("The value of hugo is: {}", hugo);
}

fn another_function(x: i32, y: i32) {
  println!("{}, {}", x, y);
}
