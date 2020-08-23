// デバッグ用整形器の注釈を入れることで、println!マクロで構造体の内容を展開できる.
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

// 四角形の面積を求める.
fn main() {
  // let width1 = 30;
  // let height1 = 50;
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  // println!("{}", area(width1, height1));
  println!("{:?}", rect1);
  println!("{}", area(&rect1));
}

// fn area(width: u32, height: u32) -> u32 {
fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}