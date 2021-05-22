use std::{thread, time::Duration, collections::HashMap, hash::Hash};

struct Cacher<T, K, V>
where
  T: Fn(K) -> V,
  K: Hash + Eq + Copy,
  V: Copy
{
  calculation: T,
  values: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
  T: Fn(K) -> V,
  K: Hash + Eq + Copy,
  V: Copy
{
  fn new(calculation: T) -> Cacher<T, K, V> {
    Cacher {
      calculation,
      values: HashMap::new(),
    }
  }

  fn value(&mut self, intensity: K) -> V {
    self.values.entry(intensity).or_insert((self.calculation)(intensity)).clone()
  }
}

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
  let mut expensive_result = Cacher::new(|num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!("Today, do {} pushups!", expensive_result.value(intensity));
    println!("Next, do {} situps!", expensive_result.value(intensity));
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!("Today, run for {} minutes!", expensive_result.value(intensity));
    }
  }
}

#[test]
fn call_with_different_values() {
  let mut c = Cacher::new(|a| a);

  let _v1 = c.value(1);
  let v2 = c.value(2);

  assert_eq!(v2, 2);
}