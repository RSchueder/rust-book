
use crate::garden::vegetables::Asparagus;
//pub mod restaurant;
pub mod garden;

//use crate::restaurant:restaurant_hosting;

fn main() {
    let plant = Asparagus{ripe: true};
    println!("I'm growing  {:?} plant", plant)
    //restaurant_hosting
}
