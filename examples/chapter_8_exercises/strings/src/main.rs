// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word
// and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

use std::io;

fn main() {
  let mut input = String::new();

  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  let input: Vec<_> = input.trim().split(',').collect();

  let vowels = ['a', 'e', 'i', 'o', 'u'];
  let mut piglatin: Vec<String> = vec![];

  for word in input {
    let mut new_word = String::new();
    let mut end = 'h';

    for (i, c) in word.trim().chars().enumerate() {
      if i == 0 && vowels.contains(&c) {
        new_word.push(c);
      } else if i == 0 {
        end = c;
      } else {
        new_word.push(c);
      }
    }

    piglatin.push(new_word + "-" + &end.to_string() + "ay");
  }
  println!("{:#?}", piglatin.connect(", "));
}
