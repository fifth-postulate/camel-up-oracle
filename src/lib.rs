#![deny(missing_docs)]
//! Camel Up is a game of chance.
//!
//! Since probability questions are often very hard. This crate provides an oracle to answer questions concerning which camel will come out ahead in the race.
//!
//! # Example
//! To determine who has the better winning chance of two camels, where one camel has fallen two behind one could use the following code.
//!
//! ```
//! use camel_up::prelude::*;
//! let race = "r,,w".parse::<Race>().expect("to parse");
//! let dice = "rw".parse::<Dice>().expect("to parse");
//!
//! let result = project(&race, &dice);
//! let red_chance = result.winner[&Camel::Red];
//! let white_chance = result.winner[&Camel::White];
//!
//! assert!(white_chance > red_chance);
//! ```
//!
//! ## Parsing of Race
//! As the above example shows one can parse a string and get a `Race`. The following table describes each symbol.alloc
//!
//! | Symbol | Marker      |
//! |--------|-------------|
//! | r      | Red         |
//! | o      | Orange      |
//! | y      | Yellow      |
//! | g      | Green       |
//! | w      | White       |
//! | ,      | Divider     |
//! | +      | Oasis       |
//! | -      | FataMorgana |
//!
//! As per the rules of the game, camels can not be in a oasis or a fata morgana, nor can either of those be next to each other. So the following strings all fail to parse.
//!
//! ```
//! use camel_up::camel::*;
//! assert_eq!("r|y".parse::<Race>(), Err(RaceParseError::NotAMarker(NotAMarker::But("|".to_owned()))));
//! assert_eq!("+r,y".parse::<Race>(), Err(RaceParseError::CamelInOasis));
//! assert_eq!("-r,y".parse::<Race>(), Err(RaceParseError::CamelInFataMorgana));
//! assert_eq!("r,-+,y".parse::<Race>(), Err(RaceParseError::ToManyAdjustmentsInOnePosition));
//! assert_eq!("r,-,+,y".parse::<Race>(), Err(RaceParseError::ConsecutiveAdjustments));
//! ```
//!
//! ## Parsing of Dice
//! Dice can be similarly parsed. The only allowed symbols are the ones for the camels.

pub mod camel;
pub mod fraction;
pub mod oracle;
pub mod vis;
mod tree;

pub mod prelude {
    //! Easy access to a good combination of camel up related concepts.

    pub use crate::camel::{Camel, Dice, Race};
    pub use crate::fraction::Fraction;
    pub use crate::oracle::project;
}
