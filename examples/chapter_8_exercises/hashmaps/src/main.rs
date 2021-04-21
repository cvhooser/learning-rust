// Using a hash map and vectors,
// create a text interface to allow a user to
// add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

fn main() {
  let mut company: HashMap<String, Vec<String>> = HashMap::new();
  println!("Add an employee with: \"Add <employee> to <department> \"");
  
  loop {

    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let input: Vec<_> = input.trim().split(' ').collect();
    if input[0] == "exit" {
      break;
    };

    let mut previous = "";
    let mut employee = "";
    let mut department = "";

    for word in &input {
      if previous.to_lowercase().eq("add") {
        employee = word;
      } else if previous.to_lowercase().eq("to") {
        department = word;
      }
      previous = word;
    }

    company.entry(String::from(department)).or_insert(vec![]).push(String::from(employee));
  }

  println!("{:#?}", company);
}
