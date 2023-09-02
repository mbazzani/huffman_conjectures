use std::cmp::Ordering;
use std::ops::*;
use std::sync::Arc;

pub trait RealNum:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Copy + Ord
where
    Self: std::marker::Sized,
{
}
impl<T> RealNum for T where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Ord
{
}

#[derive(Debug, Clone)]
pub enum NodeType<T>
where
    T: RealNum,
{
    Leaf(char),
    Branch(Arc<[Node<T>; 2]>),
}

#[derive(Debug, Clone)]
pub struct Node<T>
where
    T: RealNum,
{
    probability: T,
    node_type: NodeType<T>,
}

impl<T> PartialEq for Node<T>
where
    T: RealNum,
{
    fn eq(&self, other: &Self) -> bool {
        self.probability == other.probability
    }
}

impl<T> Eq for Node<T> where T: RealNum {}

impl<T> PartialOrd for Node<T>
where
    T: RealNum,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Node<T>
where
    T: RealNum,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.probability.cmp(&other.probability)
    }
}

impl<T> Node<T>
where
    T: RealNum,
{
    pub fn new_leaf(probability: T, symbol: char) -> Node<T>
    where
        T: RealNum,
    {
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
            (NodeType::Leaf(symbol), NodeType::Leaf(other_symbol)) => {
                symbol == other_symbol
            }
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
    //Checks whether any node is
    pub fn is_probably_competitively_optimal(&self) -> bool {
        fn helper(
            node: &Node<u32>,
            mut higher_node_differences: Vec<u32>,
            prev_sibling_difference: u32,
        ) -> bool {
            match node.node_type.clone() {
                NodeType::Leaf(_) => higher_node_differences
                    .into_iter()
                    .all(|p| p <= node.probability),
                NodeType::Branch(children) => {
                    //let (l, r) = (children[0].clone(), children[1].clone());
                    let (l, r) = (&children[0], &children[1]);
                    let (bigger, smaller) =
                        if *l > *r { (l, r) } else { (r, l) };
                    let sibling_difference =
                        bigger.probability - smaller.probability;
                    let one_child_optimal = helper(
                        smaller,
                        higher_node_differences.clone(),
                        sibling_difference,
                    );
                    dbg!(higher_node_differences.clone());
                    higher_node_differences.push(prev_sibling_difference);
                    dbg!(higher_node_differences.clone());
                    let other_child_optimal = helper(
                        bigger,
                        higher_node_differences,
                        sibling_difference,
                    );
                    dbg!(one_child_optimal);
                    dbg!(other_child_optimal);
                    one_child_optimal && other_child_optimal
                }
            }
        }
        helper(self, vec![], u32::MIN)
    }

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

    #[test]
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

    #[test]
    fn is_probably_competitively_optimal_test() {
        let code_a = Node::new_branch(
            Node::new_leaf(4, 'a'),
            Node::new_branch(
                Node::new_leaf(3, 'b'),
                Node::new_branch(
                    Node::new_leaf(2, 'c'),
                    Node::new_leaf(1, 'd'),
                ),
            ),
        );

        let code_b = Node::new_branch(
            Node::new_leaf(4, 'a'),
            Node::new_branch(Node::new_leaf(3, 'b'), Node::new_leaf(2, 'c')),
        );

        let code_c = Node::new_branch(
            Node::new_branch(
                Node::new_branch(
                    Node::new_leaf(512, 'a'),
                    Node::new_leaf(512, 'b'),
                ),
                Node::new_branch(
                    Node::new_leaf(512, 'c'),
                    Node::new_leaf(512, 'd'),
                ),
            ),
            Node::new_branch(
                Node::new_branch(
                    Node::new_leaf(512, 'e'),
                    Node::new_leaf(512, 'f'),
                ),
                Node::new_branch(
                    Node::new_branch(
                        Node::new_leaf(256, 'g'),
                        Node::new_leaf(256, 'h'),
                    ),
                    Node::new_branch(
                        Node::new_leaf(256, 'i'),
                        Node::new_branch(
                            Node::new_leaf(128, 'j'),
                            Node::new_branch(
                                Node::new_leaf(64, 'k'),
                                Node::new_branch(
                                    Node::new_leaf(32, 'l'),
                                    Node::new_branch(
                                        Node::new_leaf(16, 'm'),
                                        Node::new_branch(
                                            Node::new_leaf(8, 'n'),
                                            Node::new_branch(
                                                Node::new_leaf(4, 'o'),
                                                Node::new_branch(
                                                    Node::new_leaf(2, 'p'),
                                                    Node::new_branch(
                                                        Node::new_leaf(1, 'q'),
                                                        Node::new_leaf(1, 'r'),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );
        assert!(!code_a.is_probably_competitively_optimal());
        println!("Starting code b");
        assert!(code_b.is_probably_competitively_optimal());
        assert!(code_c.is_probably_competitively_optimal());
    }
}
