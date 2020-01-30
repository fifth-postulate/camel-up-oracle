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
pub fn project(race: &Race, dice: &Dice) -> Chances {
    let mut tree = Tree::singleton(race.clone());
    tree.expand(dice);

    let mut counter: LeafCounter = Default::default();
    tree.visit_leaves(&mut counter);

    counter.chances()
}

/// All the relevant chances for each camel.
/// 
/// I.e. which camel is winning, which is losing, which is the runner up.
pub struct Chances {
    /// Distribution of the chance to win.
    pub winner: Distribution,
    /// Distribution of the chance to be runner up.
    pub runner_up: Distribution,
    /// Distribution of the chance to lose.
    pub loser: Distribution,
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
    winner: HashMap<Camel, usize>,
    runner_up: HashMap<Camel, usize>,
    loser: HashMap<Camel, usize>,
}

impl LeafCounter {
    fn chances(&self) -> Chances {
        let winner: HashMap<Camel, Fraction> = self
            .winner
            .iter()
            .map(|(camel, count)| (*camel, Fraction::new(*count as i64, self.total as u64)))
            .collect();
        let runner_up: HashMap<Camel, Fraction> = self
            .runner_up
            .iter()
            .map(|(camel, count)| (*camel, Fraction::new(*count as i64, self.total as u64)))
            .collect();
        let loser: HashMap<Camel, Fraction> = self
            .loser
            .iter()
            .map(|(camel, count)| (*camel, Fraction::new(*count as i64, self.total as u64)))
            .collect();
        Chances {
            winner: Distribution::from(winner),
            runner_up: Distribution::from(runner_up),
            loser: Distribution::from(loser),
        }
    }
}

impl Default for LeafCounter {
    fn default() -> Self {
        Self {
            total: 0,
            winner: HashMap::new(),
            runner_up: HashMap::new(),
            loser: HashMap::new(),
        }
    }
}

impl LeafVisitor for LeafCounter {
    fn visit(&mut self, race: &Race) {
        if let Some(winner) = race.winner() {
            *self.winner.entry(winner).or_insert(0) += 1;
        };
        if let Some(runner_up) = race.runner_up() {
            *self.runner_up.entry(runner_up).or_insert(0) += 1;
        };
        if let Some(loser) = race.loser() {
            *self.loser.entry(loser).or_insert(0) += 1;
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

        assert_eq!(chances.winner[&Camel::Red], Fraction::one());
    }

    #[test]
    fn should_determine_chances() {
        let race = "r,,y".parse::<Race>().expect("to parse");
        let dice = "r".parse::<Dice>().expect("to parse");
        let chances = project(&race, &dice);

        assert_eq!(chances.winner[&Camel::Red], Fraction::new(2, 3));
        assert_eq!(chances.winner[&Camel::Yellow], Fraction::new(1, 3));
    }
}
