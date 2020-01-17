use crate::{
    camel::{Camel, Race},
    fraction::Fraction,
    tree::{LeafVisitor, Tree},
};
use std::collections::{HashMap, HashSet};

pub fn project(race: &Race, dice: &HashSet<Camel>) -> HashMap<Camel, Fraction> {
    let mut tree = Tree::singleton(race.clone());
    tree.expand(dice);

    let mut counter: LeafCounter = Default::default();
    tree.visit_leaves(&mut counter);

    counter.chances()
}

struct LeafCounter {
    total: usize,
    count: HashMap<Camel, usize>,
}

impl LeafCounter {
    fn chances(&self) -> HashMap<Camel, Fraction> {
        self.count
            .iter()
            .map(|(camel, count)| (*camel, Fraction::new(*count as i64, self.total as u64)))
            .collect()
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
        let mut dice = HashSet::new();
        dice.insert(Camel::Red);
        let chances = project(&race, &dice);

        assert_eq!(chances.get(&Camel::Red), Some(&Fraction::one()));
    }

    #[test]
    fn should_determine_chances() {
        let race = "r,,y".parse::<Race>().expect("to parse");
        let mut dice = HashSet::new();
        dice.insert(Camel::Red);
        let chances = project(&race, &dice);

        assert_eq!(chances.get(&Camel::Red), Some(&Fraction::new(2, 3)));
        assert_eq!(chances.get(&Camel::Yellow), Some(&Fraction::new(1, 3)));
    }
}
