#![allow(unused)]

use crate::back_of_house::Appetizer;
// does not need to be pub to be accessed by eat_at_restaurant because sibling
mod front_of_house {
    // need to be public
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    // if the enum is pub, all variants are pub as well
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
    // structs can be public, fields need to be public as well to be accessable
    #[derive(Debug)]
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
    fn fix_incorrect_order() {
        cook_order();
        // super to access parent modules function
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

// referring to modules using absolute or relative pathes
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // use to import
    use front_of_house::hosting::add_to_waitlist;
    add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {:?} toast please", meal);


    // can't decide about fruit - thats private
    // meal.seasonal_fruit = String::from("blueberries");

    //
    println!("Appetizer is {:?}",Appetizer::Soup);
}
