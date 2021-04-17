#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, rect: &Rectangle) -> bool {
    self.height > rect.height && self.width > rect.width
  }

  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  println!("rect is {:#?}", rect1);

  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };

  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));

  let sq = Rectangle::square(25);
  println!("Our square is {:#?}", sq);
}