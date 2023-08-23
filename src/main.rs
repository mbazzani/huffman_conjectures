use num::rational::Ratio;
use itertools::Itertools;
use std::cmp::Ordering;
use std::error::Error;
use std::ops::Add;

#[derive(Debug, Clone)]
enum NodeType<T: Add<Output = T>> {
    Leaf { symbol: char },
    Branch { left: Box<Node<T>>, right: Box<Node<T>> },
}

#[derive(Debug, Clone)]
struct Node<T: Add<Output = T>> {
    probability: T,
    r#type: NodeType<T>,
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

fn construct_huffman(mut nodes: Vec<Node<u32>>) -> Option<Node<u32>> {
    loop {
        match nodes.len() {
            0 => return None,
            1 => return Some(nodes[0].clone()),
            _ => {
                nodes.sort_by(|a, b| b.cmp(a));
                let l: Node<u32> = nodes.pop().unwrap();
                let r: Node<u32> = nodes.pop().unwrap();
                nodes.push(join(l, r))
            }
        }
    }
}

fn join(left: Node<u32>,  right: Node<u32>) -> Node<u32> {
    Node {
        probability: right.probability + left.probability, 
        r#type: NodeType::Branch {
            left: Box::new((left).clone()),
            right: Box::new((right).clone()),
        }
    }
}

fn get_all_huffman(leaves: Vec<Node<u32>>) -> Vec<Node<u32>> {
    match leaves.len() {
        0 | 1 => return leaves,
        _ => {
            let mut huffman_codes: Vec<Node<u32>> = vec![];
            let min: &Node<u32> = leaves.iter().min().unwrap();
            let (smallest_nodes, other_nodes): (Vec<_>, Vec<_>) =
                leaves.clone().into_iter().partition(|x| x<=min);
            let possible_combinations = smallest_nodes.into_iter()
                    .tuple_combinations::<(_,_)>()
                    .map(|(l, r)| join(l, r));
            //let possible_combinations_ = possible_combinations.clone();
            //println!("{:#?}", possible_combinations_.collect::<Vec<_>>());
            //unimplemented!();
            for joined_node in possible_combinations {
                //let mut other_nodes_ = other_nodes.clone();
                let mut possible_huffman_codes = 
                    get_all_huffman([other_nodes.clone(), vec![joined_node]].concat());
                huffman_codes.append(&mut possible_huffman_codes);
            }
            huffman_codes
        },
    }
}


//Have tree type
//Implement huffman algorithm that branches at each possible step, 
//returning a vector of Huffman codes that you then join with the previous
//Regenerate PMF until you get one Huffman code beating another
//Check shape of best Huffman code
fn main() {
    let leaves = vec![
        Node {
            probability: 2,
            r#type: NodeType::Leaf { symbol: '🦀' },
        },
        Node {
            probability: 2,
            r#type: NodeType::Leaf { symbol: 'A' },
        },
        Node {
            probability: 1,
            r#type: NodeType::Leaf { symbol: 'B' },
        },
        Node {
            probability: 1,
            r#type: NodeType::Leaf { symbol: 'C' },
        },
    ];
    let huffman_codes = get_all_huffman(leaves);
    println!("{:#?}", huffman_codes);
    println!("Hello, world!");
    let vect = vec![1, 1, 1];
    let combs: Vec<_> = vect.iter().tuple_combinations::<(_,_)>().collect();
    println!("{:#?}", combs)
}
