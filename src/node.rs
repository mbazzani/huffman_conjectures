use std::cmp::Ordering;
use std::ops::Add;

#[derive(Debug, Clone)]
pub enum NodeType<T> {
    Leaf { symbol: char },
    Branch(Box<[Node<T>; 2]>) ,
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    probability: T,
    node_type: NodeType<T>,
}

impl<T: PartialEq> PartialEq for Node<T> {

    fn eq(&self, other: &Self) -> bool {
        self.probability==other.probability
    }
}

impl<T: Eq> Eq for Node<T> {}

impl<T: PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.probability.partial_cmp(&other.probability)
    }
}

impl<T: Ord> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.probability.cmp(&other.probability)
    }
}

impl<T> Node<T> 
where T: Add<Output = T> + Copy {
    pub fn new_leaf(probability: T, symbol: char) -> Node<T> {
        Node {
            probability,
            node_type: NodeType::Leaf { symbol }
        }
    }
    pub fn new_branch(left: Node<T>, right: Node<T>) -> Node<T> {
        Node {
            probability: left.probability + right.probability,
            node_type: NodeType::Branch(Box::new([left, right]))
        }
    }
    pub fn probability(&self) -> T {
        self.probability
    }
    pub fn node_type(&self) -> &NodeType<T> {
        &self.node_type
    }
}
