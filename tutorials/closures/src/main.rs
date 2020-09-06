extern crate rand;

use std::thread;
use std::time::Duration;
use rand::Rng;

fn generate_workout(intensity: u32, random_number: u32) {
  // |num: u32| -> u32 とも型注釈を明示することも可能だが、冗長.
  // 基本的にクロージャは狭い範囲で利用される.
  let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  };

  if intensity < 26 {
    println!(
      "Today, do {} pushups!",
      expensive_closure(intensity)
    );
    println!(
      "Next, do {} situps!",
      expensive_closure(intensity)
    );
  } else {
    if random_number < 6 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} mimnutes!",
        expensive_closure(intensity)
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
