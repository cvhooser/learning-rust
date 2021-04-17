// Loads files
mod front_of_house;
mod back_of_house;

// Brings it into scope to be used
// These two lines do the same thing when front_of_house is in the same file
// use self::front_of_house::hosting;
pub use crate::front_of_house::hosting;

fn serve_order(){}
pub fn eat_at_restaurant(){
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();

  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);
  // The follow line will not compile
  // meal.seasonal_fruit = String::from("blueberries");

  // // Absolute path
  // crate::front_of_house::hosting::add_to_waitlist();
  // // Relative path
  // front_of_house::hosting::add_to_waitlist();
  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}

// The above is how it is split out.
// This is what it looks like all in one file.
/*
mod back_of_house {
  pub enum Appetizer {
    Soup, 
    Salad
  }
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }
  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }
  fn fix_incorrect_order(){
    cook_order();
    super::serve_order();
  }
  fn cook_order(){}
}

fn serve_order(){}

mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
    fn seat_at_table(){}
  }

  mod serving{
    fn take_order(){}
    fn serve_order(){}
    fn take_payment(){}
  }
}

// These two lines do the same thing
// use crate::front_of_house::hosting;
use self::front_of_house::hosting;

pub fn eat_at_restaurant(){
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();

  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);
  // The follow line will not compile
  // meal.seasonal_fruit = String::from("blueberries");

  // // Absolute path
  // crate::front_of_house::hosting::add_to_waitlist();
  // // Relative path
  // front_of_house::hosting::add_to_waitlist();
  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}
*/