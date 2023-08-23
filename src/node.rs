use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum NodeType<T> {
    Leaf { symbol: char },
    Branch { left: Box<Node<T>>, right: Box<Node<T>> },
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub probability: T,
    pub r#type: NodeType<T>,
}

impl PartialEq for Node<u32> {
    fn eq(&self, other: &Self) -> bool {
        self.probability==other.probability
    }
}

impl Eq for Node<u32> {}

impl PartialOrd for Node<u32> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::Some(self.probability.cmp(&other.probability))
    }
}

impl Ord for Node<u32> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.probability.cmp(&other.probability)
    }
}
