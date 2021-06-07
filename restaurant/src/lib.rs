#[allow(dead_code)]
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
  }

  pub mod serving {
    pub fn serve_order() {}

    fn take_order() {}

    fn take_payment() {}
  }
}

#[allow(dead_code)]
mod back_of_house {
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  pub enum Appetizer {
    Soup,
    Salad,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }

  fn fix_incorrect_order() {
    cook_order();
    super::front_of_house::serving::serve_order();
  }

  fn cook_order() {}
}

// similar to creating a simlink to the hosting module in the current directory
// add pub before the use here to expose the fn import to external callers.
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  // Absolute Path - preferred if needing to refactor I think.
  // crate::front_of_house::hosting::add_to_waitlist();
  hosting::add_to_waitlist();

  // Relative Path
  // front_of_house::hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();

  // Public enums expose all variants.
  // let order1 = back_of_house::Appetizer::Soup;
  // let order2 = back_of_house::Appetizer::Salad;

  // Order a breakfast in the summer with Rye
  let mut meal = back_of_house::Breakfast::summer("Rye");
  // Change the toast
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // This wont compile if we uncomment it because we're not allowed
  // to see or modify the seasonal fruit with the meal.println! Error is
  //  field `seasonal_fruit` of struct `Breakfast` is private
  //  meal.seasonal_fruit = String::from("Blueberries");
}
