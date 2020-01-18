use crate::camel::{Dice, Face, Race, Roll};
use std::collections::HashMap;

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn singleton(value: Race) -> Self {
        let root = Node::new(value);
        let nodes = vec![root];

        Self { nodes }
    }

    pub fn expand(&mut self, dice: &Dice) {
        self.expand_node(0, dice);
    }

    fn expand_node(&mut self, index: usize, dice: &Dice) {
        for camel in dice.clone() {
            let remaining_dice = dice.remove(camel);
            for face in Face::values() {
                let roll = Roll::from((camel, face));
                let race = self.perform_on(index, roll);
                let child_index = self.add_child(index, roll, race);
                self.expand_node(child_index, &remaining_dice);
            }
        }
    }

    fn perform_on(&mut self, index: usize, roll: Roll) -> Race {
        self.nodes[index].race.perform(roll)
    }

    fn add_child(&mut self, index: usize, roll: Roll, race: Race) -> usize {
        let child = Node::new(race);
        self.nodes.push(child);
        let child_index = self.nodes.len() - 1;

        self.nodes[index].register_child(roll, child_index);

        child_index
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
    race: Race,
    children: HashMap<Roll, usize>,
}

impl Node {
    fn new(race: Race) -> Self {
        Self {
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
