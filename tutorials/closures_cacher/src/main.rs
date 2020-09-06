extern crate rand;

use std::thread;
use std::time::Duration;
use rand::Rng;


// 遅延評価パターン.
struct Cacher<T>
  where T: Fn(u32) -> u32
{
  calculation: T,
  value: Option<u32>
}

impl<T> Cacher<T>
  where T: Fn(u32) -> u32
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None
    }
  }
  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      },
    }
  }
}

fn generate_workout(intensity: u32, random_number: u32) {
  let mut expensive_closure = Cacher::new(|num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 26 {
    println!(
      "Today, do {} pushups!",
      expensive_closure.value(intensity)
    );
    println!(
      "Next, do {} situps!",
      expensive_closure.value(intensity)
    );
  } else {
    if random_number < 6 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} mimnutes!",
        expensive_closure.value(intensity)
      );
    }
  }
}

fn main() {
  let simulated_user_specified_value = rand::thread_rng().gen_range(1, 51);
  let simulated_random_number = rand::thread_rng().gen_range(1, 11);

  generate_workout(
    simulated_user_specified_value,
    simulated_random_number
  );
}
