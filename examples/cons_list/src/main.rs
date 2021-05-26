
// First creation
// enum List {
//   Cons(i32, Rc<List>),
//   Nil
// }

// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     let b = Cons(3, Rc::clone(&a));
//     let c = Cons(4, Rc::clone(&a));
// }

// Mutable reference to an immutable value
#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil
}

use crate::List::{Cons, Nil};
use std::{rc::Rc, cell::RefCell};

fn main() {
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // Both are ways to increment
    *value.borrow_mut() += 10;

    if let Cons(v, _) = &*a {
      *v.borrow_mut() += 10;
    }

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// This example shows a reference cycle that will overload the stack
// #[derive(Debug)]
// enum List {
//   Cons(i32, RefCell<Rc<List>>),
//   Nil
// }

// impl List {
//   fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//     match self {
//       Cons(_, item) => Some(item),
//       Nil => None
//     }
//   }
// }

// use crate::List::{Cons, Nil};
// use std::{rc::Rc, cell::RefCell};

// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a initial rc count = {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after changing a = {}", Rc::strong_count(&a));

//     // Uncomment the next line to see that we have a cycle;
//     // it will overflow the stack
//     // println!("a next item = {:?}", a.tail());
// }

