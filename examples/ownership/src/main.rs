// Understanding scope
// fn main() {
//   let s = String::from("hello"); // s comes into scope

//   takes_ownership(s); // s's value moves into the function...
//                       // ... and so is no longer valid here

//   let x = 5; // x comes into scope

//   makes_copy(x); // x would move into the function,
//                  // but i32 is Copy, so it’s okay to still
//                  // use x afterward
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) {
//   // some_string comes into scope
//   println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) {
//   // some_integer comes into scope
//   println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.


// Passing as a reference (borrowing)
// fn main(){
//   let s1 = String::from("hello");

//   let len = calculate_length(&s1);

//   println!("The length of the '{}' is {}", s1, len)
// }

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//   s.len()
// } // Here, s goes out of scope. But because it does not have ownership of what
//   // it refers to, nothing happens.

// Mutable References
// fn main() {
//   let mut s = String:: from("hello");

//   change(&mut s);
// }
// // This works because s is mut and some_string is &mut
// fn change(some_string: &mut String) {
//   some_string.push_str(", world");
// }