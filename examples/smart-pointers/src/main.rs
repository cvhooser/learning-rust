use std::fmt;
use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T)
where
  T: fmt::Debug; //Tuple type

impl<T> MyBox<T>
where
  T: fmt::Debug,
{
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T>
where
  T: fmt::Debug,
{
  type Target = T; // This is an associated type

  fn deref(&self) -> &T {
    &self.0
  }
}

impl<T> Drop for MyBox<T>
where
  T: fmt::Debug,
{
  fn drop(&mut self) {
    println!("Dropping box pointer {:?}.", self.0);
  }
}

// impl <T> fmt::Debug for MyBox<T> where T: fmt::Debug {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     f.debug_struct("Box")
//     .field("0", &self.0)
//     .finish()
//   }
// }

fn main() {
  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);
}
