fn main() {
  let x = plus_five(5);
  println!("The value of x is: {}", x);
}

fn plus_five(x: i32) -> i32 {
  x + 5
}
