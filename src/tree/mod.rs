use crate::camel::{Camel, Face, Race, Roll};
use std::collections::{HashMap, HashSet};

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn singleton(value: Race) -> Self {
        let root = Node::root(value);
        let nodes = vec![root];

        Self { nodes }
    }

    pub fn expand(&mut self, dice: &HashSet<Camel>) {
        self.expand_node(0, dice);
    }

    fn expand_node(&mut self, index: usize, dice: &HashSet<Camel>) {
        for camel in dice {
            let mut remaining_dice = dice.clone();
            remaining_dice.remove(camel);
            for face in Face::values() {
                let roll = Roll::from((camel.clone(), face));
                let race = self.perform_on(index, roll.clone());
                let child_index = self.add_child(index, roll, race);
                self.expand_node(child_index, &remaining_dice);
            }
        }
    }

    fn perform_on(&mut self, index: usize, roll: Roll) -> Race {
        self.nodes[index].race.perform(roll)
    }

    fn add_child(&mut self, index: usize, roll: Roll, race: Race) -> usize {
        let child = Node::child(index, race);
        self.nodes.push(child);
        let child_index = self.nodes.len() - 1;

        self.nodes[index].register_child(roll, child_index);

        child_index
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn visit_leaves(&self, visitor: &mut dyn LeafVisitor) {
        for candidate in &self.nodes {
            if candidate.is_leaf() {
                visitor.visit(&candidate.race);
            }
        }
    }
}

struct Node {
    parent: Option<usize>,
    race: Race,
    children: HashMap<Roll, usize>,
}

impl Node {
    fn root(race: Race) -> Self {
        Self {
            parent: None,
            race,
            children: HashMap::new(),
        }
    }

    fn child(parent: usize, race: Race) -> Self {
        Self {
            parent: Some(parent),
            race,
            children: HashMap::new(),
        }
    }

    fn register_child(&mut self, roll: Roll, child_index: usize) {
        self.children.insert(roll, child_index);
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

pub trait LeafVisitor {
    fn visit(&mut self, race: &Race);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_has_a_size() {
        let race = "r,y".parse::<Race>().expect("to parse");
        let mut tree = Tree::singleton(race);
        let mut dice = HashSet::new();
        dice.insert(Camel::Red);
        tree.expand(&dice);

        assert_eq!(tree.size(), 4);
    }
}
