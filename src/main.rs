extern crate camel_up;
extern crate clap;

use camel_up::{
    camel::{Camel, Dice, Race},
    fraction::Fraction,
    oracle::project,
};
use clap::{App, Arg};

fn main() {
    let matches = App::new("Camel Up")
        .version("1.0")
        .author("Daan van Berkel <daan.v.berkel.1980@gmail.com>")
        .about("Calculates odds of which camel is winning")
        .arg(
            Arg::with_name("race")
                .short("r")
                .long("race")
                .help("describe the current race")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dice")
                .short("d")
                .long("dice")
                .help("determines which dice are present, defaults to all dice")
                .takes_value(true),
        )
        .get_matches();

    let race_description = matches.value_of("race").unwrap();
    let dice_description = matches.value_of("dice").or(Some("roygw")).unwrap();

    if let (Ok(race), Ok(dice)) = (
        race_description.parse::<Race>(),
        dice_description.parse::<Dice>(),
    ) {
        let result = project(&race, &dice);
        let mut ordered: Vec<(Camel, Fraction)> =
            result.winner.values().map(|(k, v)| (*k, *v)).collect();
        ordered.sort_by(|(_, left), (_, right)| right.cmp(&left));
        print(&ordered);
    } else {
        println!("whoops!");
    }
}

fn print(elements: &[(Camel, Fraction)]) {
    for (camel, fraction) in elements {
        print!("({:?},{})", camel, fraction);
    }
    println!()
}
