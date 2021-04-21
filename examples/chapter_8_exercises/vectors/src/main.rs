// Given a list of integers, use a vector and return the mean (the average value), 
// median (when sorted, the value in the middle position), 
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::io;
use std::collections::HashMap;

fn main() {

  loop {
    println!("Please give a comma separated list of integers.");

    let mut numbers = String::new();

    io::stdin()
      .read_line(&mut numbers)
      .expect("Failed to read line");

    let mut numbers: Vec<_> = numbers.trim().split(',').collect();
    numbers.sort_unstable_by(|a, b| b.cmp(a));
    
    let mut map = HashMap::new();

    let mut sum: f32 = 0.0;

    for number in &numbers {
      sum += match number.trim().parse::<f32>() {
        Ok(num) => num,
        Err(_) => 0.0,
      };
      
      let count = map.entry(number).or_insert(0);
      *count += 1;
    };

    let mean = sum / numbers.len() as f32;
    let median = numbers[numbers.len()/2];
    let mode = map.iter()
    .max_by(|a, b| a.1.cmp(&b.1))
    .map(|(k, _v)| k)
    .unwrap();

    println!("This is the map = {:#?}", map);

    println!("The median = {}, the mode = {}, and the mean = {}", median, mode, mean);
  }
}
