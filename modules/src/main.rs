use modules::eat_at_restaurant;

use crate::garden::vegetables::Asparagus;

// include the code from src/garden.rs
mod garden;

fn main() {
    println!("Modules and visibility");
    let plant = Asparagus { color: 2 };
    println!("I'm growing {:?}!", plant);
    eat_at_restaurant();
}
