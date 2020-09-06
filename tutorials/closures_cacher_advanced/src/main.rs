use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, K, V>
  where T: Fn(K) -> V
{
  calculation: T,
  map: HashMap<K, V>
}

impl<T, K: Eq + Hash + Clone, V> Cacher<T, K, V>
  where T: Fn(K) -> V
{
  fn new(calculation: T) -> Cacher<T, K, V> {
    Cacher {
      calculation,
      map: HashMap::new()
    }
  }
  fn value(&mut self, k: K) -> &V {
    if !self.map.contains_key(&k) {
      let v = (self.calculation)(k.clone());
      self.map.insert(k.clone(), v);
    }
    &self.map[&k]
  }
}

fn main() {
  let mut cacher = Cacher::new(|v| {
    println!("Wait a second, now calculating...");
    thread::sleep(Duration::from_secs(2));
    v * 2
  });

  for value in vec![20, 30, 30, 10, 40, 20, 50, 10] {
    println!("{} => {}", value, cacher.value(value));
  }
}
