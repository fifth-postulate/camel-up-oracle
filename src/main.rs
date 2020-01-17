extern crate camel_up;

use camel_up::{
    camel::{Race, Dice},
    oracle::project,
};

fn main() {
    if let (Ok(race), Ok(dice)) = ("r,,y".parse::<Race>(), "r".parse::<Dice>()) {
        let result = project(&race, &dice);
        println!("{:?}", result);
    } else {
        println!("whoops!");
    }
}
