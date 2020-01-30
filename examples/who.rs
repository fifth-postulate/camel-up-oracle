extern crate camel_up;

use camel_up::prelude::*;

fn main() {
    let race = "r,,w".parse::<Race>().expect("to parse");
    let dice = "rw".parse::<Dice>().expect("to parse");

    let result = project(&race, &dice);
    let mut ordered: Vec<(Camel, Fraction)> = result.values().map(|(k, v)| (*k, *v)).collect();
    ordered.sort_by(|(_, left), (_, right)| right.cmp(&left));
    for (camel, fraction) in ordered {
        print!("({:?},{})", camel, fraction);
    }
    println!();
}
