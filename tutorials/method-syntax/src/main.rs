#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // 関連関数 (第一引数に自身のインスタンスの参照を取らない.)
  // コンストラクタによく使用される.
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size
    }
  }

  // Rectangle構造体上にメソッドを定義.
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  let rect2 = Rectangle {
    width: 10,
    height: 20,
  };
  let rect3 = Rectangle {
    width: 40,
    height: 35,
  };
  let rect4 = Rectangle {
    width: 100,
    height: 10,
  };
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
  println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));

  let rect5 = Rectangle::square(10);
  println!(
    "The area of the rectangle is {} square pixels.",
    rect5.area()
  );
}
