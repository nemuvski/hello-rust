fn main() {
  // map()の例.
  let v1: Vec<i32> = vec![1, 2, 3];
  let v2: Vec<i32> = v1.iter().map(|x| x+1).collect();
  println!("{:?}", v2);
}