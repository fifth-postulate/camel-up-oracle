extern crate camel_up;

use camel_up::{
    camel::{Camel, Race},
    oracle::project,
};
use std::collections::HashSet;

fn main() {
    if let Ok(race) = "r,,y".parse::<Race>() {
        let mut dice = HashSet::new();
        dice.insert(Camel::Red);
        let result = project(&race, &dice);
        println!("{:?}", result);
    } else {
        println!("whoops, world!");
    }
}
