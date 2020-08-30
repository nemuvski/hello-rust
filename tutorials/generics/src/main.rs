fn largest_i32(list: &[i32]) -> i32 {
  let mut largest = list[0];
  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn largest_char(list: &[char]) -> char {
  let mut largest = list[0];
  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

// スライスの値の型がPartialOrdとCopyを実装する.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];
  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

#[derive(Debug)]
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
  fn y(&self) -> &T {
    &self.y
  }
}

impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
  fn distance(p1: &Point<f32>, p2: &Point<f32>) -> f32 {
    ((&p1.x - &p2.x).powi(2) + (&p1.y - &p2.y).powi(2)).sqrt()
  }
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  // 参照を渡す. (アクセスできるように借用)
  let result = largest_i32(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];
  // 同様.
  let result = largest_char(&char_list);
  println!("The largest char is {}", result);

  let wont_work = Point { x: 5, y: 4 };
  println!("(x,y) = ({},{})", wont_work.x(), wont_work.y());

  let point1 = Point { x: 4.0, y: 3.0 };
  let point2 = Point { x: 8f32, y: 12f32 };
  println!("{:?} : {}", point1, point1.distance_from_origin());
  println!("The distance from {:?} to {:?} is {}",
    point1, point2, Point::distance(&point1, &point2)
  );

  let result = largest(&number_list);
  println!("{}", result);
}
