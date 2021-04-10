fn main() {
  // Part 1
  // This will fail
  // let x = 5;
  // println!("The value of x is: {}", x);
  // x = 6;
  // println!("The value of x is: {}", x);

  // Part 2
  // Shadowing
  // let x = 5;
  // let x = x + 1;
  // let x = x * 2;
  // println!("The value of x is: {}", x);

  // Part 3
  // Shadowing use case
  // let spaces = "    ";
  // let spaces = spaces.len();

  // Part 4
  // This won't work, this is why shadowing is useful.
  // let mut spaces = "    ";
  // spaces = spaces.len();

  // Part 5
  // Floating Point examples
  // let x = 2.0; //f64
  // let y: f32 = 3.0; //f32

  // Part 6
  // addition
  // let sum = 5 + 10;

  // // subtraction
  // let difference = 95.5 - 4.3;

  // // multiplication
  // let product = 4 * 30;

  // // division
  // let quotient = 56.7 / 32.2;

  // // remainder
  // let remainder = 43 % 5;

  //Part 7
  // Character examples
  // let c = 'z';
  // let z = 'ℤ';
  // let heart_eyed_cat = '😻';

  //Part 8
  // Tuples
  // let tup: (i32, f64, u8) = (500, 6.4, 1);

  // Omg you can destructure!
  // let tup = (500, 6.4, 1);
  // let (x, y, z) = tup;
  // println!("The value of y is: {}", y);

  // tuple index reads
  // let x: (i32, f64, u8) = (500, 6.4, 1);
  // let five_hundred = x.0;
  // let six_point_four = x.1;
  // let one = x.2;

  // Part 9
  // Arrays
  // let a = [1, 2, 3, 4, 5];
  // let a: [i32; 5] = [1, 2, 3, 4, 5]; // explicit type
  // let a = [3; 5]; // fills array with values (3)

  // array access
  // let a = [1, 2, 3, 4, 5];
  // let first = a[0];
  // let second = a[1];
}
