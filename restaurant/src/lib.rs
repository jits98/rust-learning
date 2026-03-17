mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}


// use std::collections::*;

// use std::io::{self, Write};

// use std::io;
// use std::io::Write;




// use std::{cmp::Ordering, io};

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {

// }

// fn function2() -> IoResult<()> {

// }





// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {}

// fn function2() -> io::Result<()> {}
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting::add_to_waitlist;


// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// use std::collections::HashMap;

// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1,2);
// }
// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;

//     let order2 = back_of_house::Appetizer::Salad;
// }

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer (toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }


// pub fn eat_at_restaurant () {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
// }
// mod front_of_house {
//    pub mod hosting {
//         pub fn add_to_waitlist() {}

//     }
// }

// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waitlist();
    
//     front_of_house::hosting::add_to_waitlist();
// }

// fn deliver_order () {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order() {}
// }
   