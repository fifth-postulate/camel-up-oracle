//! An oracle is
//!
//! > a person or agency considered to provide wise and insightful counsel or prophetic predictions or precognition of the future, inspired by the gods. As such it is a form of divination.
//!
//! We divine by way of mathematics.
use crate::{
    camel::{Camel, Dice, Race},
    fraction::Fraction,
    tree::{LeafVisitor, Tree},
};
use std::{collections::HashMap, iter::Iterator, ops::Index};

/// Determines the win chances for each camel.
///
/// The `Distribution` returns for each camel present in the race, the chance of winning.
pub fn project(race: &Race, dice: &Dice) -> Distribution {
    let mut tree = Tree::singleton(race.clone());
    tree.expand(dice);

    let mut counter: LeafCounter = Default::default();
    tree.visit_leaves(&mut counter);

    counter.chances()
}

/// The chances for a specific situation for each camel.
pub struct Distribution {
    distribution: HashMap<Camel, Fraction>,
    default: Fraction,
}

impl Distribution {
    /// Returns an iterator that iterates over the chances.
    ///
    /// I.e. iterates over `(&Camel, &Fraction)` values.
    pub fn values(&self) -> impl Iterator<Item = (&Camel, &Fraction)> + '_ {
        self.distribution.iter()
    }
}

impl From<HashMap<Camel, Fraction>> for Distribution {
    fn from(distribution: HashMap<Camel, Fraction>) -> Self {
        Self {
            distribution,
            default: Fraction::default(),
        }
    }
}

impl Index<&Camel> for Distribution {
    type Output = Fraction;

    fn index(&self, camel: &Camel) -> &Self::Output {
        self.distribution.get(camel).unwrap_or(&self.default)
    }
}

struct LeafCounter {
    total: usize,
    count: HashMap<Camel, usize>,
}

impl LeafCounter {
    fn chances(&self) -> Distribution {
        let distribution: HashMap<Camel, Fraction> = self
            .count
            .iter()
            .map(|(camel, count)| (*camel, Fraction::new(*count as i64, self.total as u64)))
            .collect();
        Distribution::from(distribution)
    }
}

impl Default for LeafCounter {
    fn default() -> Self {
        Self {
            total: 0,
            count: HashMap::new(),
        }
    }
}

impl LeafVisitor for LeafCounter {
    fn visit(&mut self, race: &Race) {
        if let Some(winner) = race.winner() {
            *self.count.entry(winner).or_insert(0) += 1;
        };
        self.total += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_have_a_clear_winner() {
        let race = "r,y".parse::<Race>().expect("to parse");
        let dice = "r".parse::<Dice>().expect("to parse");
        let chances = project(&race, &dice);

        assert_eq!(chances[&Camel::Red], Fraction::one());
    }

    #[test]
    fn should_determine_chances() {
        let race = "r,,y".parse::<Race>().expect("to parse");
        let dice = "r".parse::<Dice>().expect("to parse");
        let chances = project(&race, &dice);

        assert_eq!(chances[&Camel::Red], Fraction::new(2, 3));
        assert_eq!(chances[&Camel::Yellow], Fraction::new(1, 3));
    }
}
