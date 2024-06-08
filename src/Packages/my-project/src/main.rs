use crate::garden::vegetables::Asparagus;
use my_project::eat_at_restaurant;
pub mod garden;


fn main() {
    let plant = Asparagus {};

    println!("I'm growing {:?}!", plant);
    println!("Hello, world!");
    eat_at_restaurant();
}
