use std::collections::HashMap;
use std::hash::Hash;

pub struct Tree<T, E>
where
    E: PartialEq + Eq + Hash,
{
    nodes: Vec<Node<T, E>>,
}

impl<T, E> Tree<T, E>
where
    E: PartialEq + Eq + Hash,
{
    pub fn empty() -> Self {
        Self { nodes: vec![] }
    }

    pub fn singleton(value: T) -> (Self, usize) {
        let root = Node::root(value);
        let nodes = vec![root];

        (Self { nodes }, 0)
    }
}

struct Node<T, E>
where
    E: PartialEq + Eq + Hash,
{
    parent: Option<usize>,
    value: T,
    children: HashMap<E, usize>,
}

impl<T, E> Node<T, E>
where
    E: PartialEq + Eq + Hash,
{
    fn root(value: T) -> Self {
        Self {
            parent: None,
            value,
            children: HashMap::new(),
        }
    }
}
