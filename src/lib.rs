#![deny(missing_docs)]
//! Camel Up is a game of chance.
//!
//! Since probability questions are often very hard. This crate provides an oracle to answer questions concerning which camel will come out ahead in the race this round.
//!
//! # Example
//! To determine who has the better winning change of two camels, where one camel has fallen two behind one could use the following code.
//!
//! ```
//! use camel_up::prelude::*;
//! let race = "r,,w".parse::<Race>().expect("to parse");
//! let dice = "rw".parse::<Dice>().expect("to parse");
//!
//! let result = project(&race, &dice);
//! let red_chance = result[&Camel::Red];
//! let white_chance = result[&Camel::White];
//!
//! assert!(white_chance > red_chance);
//! ```

pub mod camel;
pub mod fraction;
pub mod oracle;
mod tree;

pub mod prelude {
    //! Easy access to a good combination of camel up related concepts.

    pub use crate::camel::{Camel, Dice, Race};
    pub use crate::fraction::Fraction;
    pub use crate::oracle::project;
}
