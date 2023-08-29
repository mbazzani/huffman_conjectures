use crate::source::Source;

use std::cmp::Ordering;
use std::ops::Add;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum NodeType<T>
where
    T: Copy + Add<Output = T> + Eq + Ord,
{
    Leaf(char),
    Branch(Arc<[Node<T>; 2]>),
}

#[derive(Debug, Clone)]
pub struct Node<T>
where
    T: Copy + Add<Output = T> + Eq + Ord,
{
    probability: T,
    node_type: NodeType<T>,
}

impl<T> PartialEq for Node<T>
where
    T: Copy + Add<Output = T> + Eq + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.probability == other.probability
    }
}

impl<T> Eq for Node<T> where T: Eq + Copy + Add<Output = T> + Ord {}

impl<T> PartialOrd for Node<T>
where
    T: Copy + Add<Output = T> + Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.probability.partial_cmp(&other.probability)
    }
}

impl<T> Ord for Node<T>
where
    T: Copy + Add<Output = T> + Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.probability.cmp(&other.probability)
    }
}

impl<T> Node<T>
where
    T: Copy + Add<Output = T> + Eq + Ord,
{
    pub fn new_leaf(probability: T, symbol: char) -> Node<T> {
        Node {
            probability,
            node_type: NodeType::Leaf(symbol),
        }
    }
    pub fn new_branch(left: Node<T>, right: Node<T>) -> Node<T> {
        Node {
            probability: left.probability + right.probability,
            node_type: NodeType::Branch(Arc::new([left, right])),
        }
    }
    pub fn probability(&self) -> T {
        self.probability
    }
    pub fn node_type(&self) -> &NodeType<T> {
        &self.node_type
    }

    //Used for testing only, very slow
    #[allow(dead_code)]
    pub fn is_same_as(&self, other: &Node<T>) -> bool {
        if self.probability != other.probability {
            return false;
        }
        match (&self.node_type, &other.node_type) {
            (NodeType::Leaf(symbol), NodeType::Leaf(other_symbol)) => symbol == other_symbol,
            (NodeType::Branch(children), NodeType::Branch(other_children)) => {
                (children[0].is_same_as(&other_children[0])
                    && children[1].is_same_as(&other_children[1]))
                    || (children[0].is_same_as(&other_children[1])
                        && children[1].is_same_as(&other_children[0]))
            }
            (_, _) => false,
        }
    }
}

impl Node<u32> {
    #[allow(dead_code)]
    pub fn new_huffman(mut nodes: Vec<Node<u32>>) -> Option<Node<u32>> {
        loop {
            match nodes.len() {
                0 => return None,
                1 => return Some(nodes[0].clone()),
                _ => {
                    nodes.sort_by(|a, b| b.cmp(a));
                    let l: Node<u32> = nodes.pop().unwrap();
                    let r: Node<u32> = nodes.pop().unwrap();
                    nodes.push(Node::new_branch(l, r))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_huffman_test() {
        let leaves = vec![
            Node::new_leaf(1, 'a'),
            Node::new_leaf(2, 'b'),
            Node::new_leaf(3, 'c'),
            Node::new_leaf(4, 'd'),
        ];

        let huff = Node::new_branch(
            leaves[3].clone(),
            Node::new_branch(
                leaves[2].clone(),
                Node::new_branch(leaves[0].clone(), leaves[1].clone()),
            ),
        );
        assert!(huff.is_same_as(&Node::new_huffman(leaves).unwrap()));
    }

    fn is_same_as_test() {
        let leaves = vec![
            Node::new_leaf(1, 'a'),
            Node::new_leaf(1, 'b'),
            Node::new_leaf(2, 'c'),
            Node::new_leaf(2, 'd'),
        ];

        let code_a = Node::new_branch(
            leaves[3].clone(),
            Node::new_branch(
                leaves[2].clone(),
                Node::new_branch(leaves[0].clone(), leaves[1].clone()),
            ),
        );

        let code_a_ = Node::new_branch(
            Node::new_branch(
                leaves[2].clone(),
                Node::new_branch(leaves[0].clone(), leaves[1].clone()),
            ),
            leaves[3].clone(),
        );

        let code_b = Node::new_branch(
            leaves[2].clone(),
            Node::new_branch(
                leaves[3].clone(),
                Node::new_branch(leaves[0].clone(), leaves[1].clone()),
            ),
        );

        let code_b_ = Node::new_branch(
            leaves[2].clone(),
            Node::new_branch(
                Node::new_branch(leaves[1].clone(), leaves[0].clone()),
                leaves[3].clone(),
            ),
        );

        let code_c = Node::new_branch(
            Node::new_branch(leaves[3].clone(), leaves[2].clone()),
            Node::new_branch(leaves[0].clone(), leaves[1].clone()),
        );

        let code_c_ = Node::new_branch(
            Node::new_branch(leaves[0].clone(), leaves[1].clone()),
            Node::new_branch(leaves[2].clone(), leaves[3].clone()),
        );
        assert!(!code_a.is_same_as(&code_b));
        assert!(!code_b.is_same_as(&code_c));
        assert!(!code_c.is_same_as(&code_a));

        assert!(code_a.is_same_as(&code_a));
        assert!(code_b.is_same_as(&code_b));
        assert!(code_c.is_same_as(&code_c));

        assert!(code_a.is_same_as(&code_a_));
        assert!(code_b.is_same_as(&code_b_));
        assert!(code_c.is_same_as(&code_c_));
    }
}
