pub mod camel;
pub mod fraction;
#[deny(missing_docs)]
pub mod oracle;
mod tree;

pub mod prelude {
    //! Easy access to a good combination of camel up related concepts.

    pub use crate::camel::{Camel, Dice, Race};
    pub use crate::fraction::Fraction;
    pub use crate::oracle::project;
}
