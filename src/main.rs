mod node;
mod code;
mod source;
use source::Source;
use code::{Code, CodeWord, CompetitiveOrd, Depth};
use node::{Node, NodeType};
use std::collections::HashMap;
use std::process::exit;
use std::hash::Hash;
use itertools::Itertools;
use std::ops::Add;
use std::iter::{zip, repeat};


fn to_code<T>(node: &Node<T>) -> Code<T> 
where T: Copy + Add<Output = T> + Eq + Hash + Ord {
    to_code_helper(node, 0)
}
fn to_code_helper<T>(node: &Node<T>, depth: Depth) -> Code<T> 
where T: Copy + Add<Output = T> + Eq + Hash + Ord {
    let mut code: Code<T> = HashMap::new();
    match node.node_type() {
        NodeType::Leaf(symbol) => {
            code.insert(CodeWord::new(*symbol, node.probability()), depth);
        }
        NodeType::Branch(children) => {
            code.extend(to_code_helper(&children[0], depth+1));
            code.extend(to_code_helper(&children[1], depth+1));
        },
    };
    code
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
                nodes.push(Node::new_branch(l, r))
            }
        }
    }
}

fn pair_combinations_in_range(length: usize) -> Vec<(usize, usize)> {
    return (0..length).tuple_combinations::<(_,_)>().collect::<Vec<_>>()
}

fn remove_two<T>(x: usize, y:usize, vec: &mut Vec<T>) -> (T, T){
    assert!(x!=y);
    let mut pair = vec![x, y];
    pair.sort();
    let j = vec.remove(pair[0]);
    let k = vec.remove(pair[1]-1);
    (j, k)
}
fn join_pair_by_indices<T>(pair_index: (usize, usize), mut nodes: Vec<Node<T>>) -> Vec<Node<T>> 
where T: Copy + Add<Output= T> + Ord {
    let (l, r) = pair_index;
    let (left, right) =  remove_two(l, r, &mut nodes);
    nodes.push(Node::new_branch(left, right));
    nodes
}

fn count_same_sequence<T>(vec: &[T]) -> usize
where T: Eq + Ord {
    assert!(vec.windows(2).all(|w| w[0] <= w[1])); //sorted
    let mut count: usize = 0;
    for (i, elem) in vec.iter().enumerate() {
        if (*elem)!=vec[0] { break; }
        count=i;
    }
    count+1
}
fn possible_reductions<T>(mut nodes: Vec<Node<T>>) -> Vec<Vec<Node<T>>> 
where T: Add<Output = T> + Copy + Ord {
    assert!(nodes.len()>1);
    nodes.sort();
    let smallest_probability = nodes[0].probability();
    let num_smallest_nodes = count_same_sequence(&nodes);
    let mut num_next_smallest_nodes = 0;
    for (i, node) in nodes.iter().enumerate() {
        if node.probability() != smallest_probability {
            num_next_smallest_nodes = count_same_sequence(&nodes[i..]);
            break
        }
    }
    //TODO: Move into own function?
    let possible_pair_indices: Vec<(usize, usize)>;
    match (num_smallest_nodes, num_next_smallest_nodes) {
        (0, _) => panic!("Should be impossible because of the length assertion"),
        (1, 0) => panic!("Should be impossible because of the length assertion"),
        (1, 1) => possible_pair_indices = vec![(0, 1)],
        (1, n) => possible_pair_indices = zip(repeat(0usize), 1..(n+1)).collect_vec(),
        (n, _) => possible_pair_indices = pair_combinations_in_range(n),
    }

    let mut possible_reductions: Vec<Vec<Node<T>>> = vec![];
    for (x, y) in possible_pair_indices.into_iter() {
        possible_reductions.push(join_pair_by_indices((x, y), nodes.clone()));
    }
    possible_reductions
}
fn all_possible_reductions<T>(nodes: Vec<Node<T>>) -> Vec<Node<T>> 
where T: Add<Output = T> + Copy + Ord {
    let mut partial_reductions = vec![nodes];
    let mut completed_reductions: Vec<Node<T>> = vec![];
    while !partial_reductions.is_empty() {
        match partial_reductions.last().unwrap().len() {
            0 | 1 => {
                let mut last = partial_reductions.pop().unwrap();
                completed_reductions.append(&mut last);
            },
            _ => {
                let last = partial_reductions.pop().unwrap();
                partial_reductions.append(&mut possible_reductions(last));
            },
        };
    };
    completed_reductions
}


//Have tree type
//Implement huffman algorithm that branches at each possible step, 
//returning a vector of Huffman codes that you then join with the previous
//Regenerate PMF until you get one Huffman code beating another
//Check shape of best Huffman code
fn max_depth(code: &Code<u32>) -> &Depth {
    code.iter().max_by_key(|(_, &v)| v).unwrap().1
}

fn main() {
    let mut num_good_sources_tested: u32 = 0;
    while num_good_sources_tested < 1000 {
        let leaves = Source::new(12).to_leaves_vec();
//        vec![
//            Node::new_leaf(12, 'a'),
//            Node::new_leaf(12, 'b'),
//            Node::new_leaf(6, 'c'),
//            Node::new_leaf(2, 'd'),
//            Node::new_leaf(2, 'e'),
//            Node::new_leaf(1, 'f'),
//            Node::new_leaf(1, 'g'),
//        ];
        //println!("Possible reductions: \n{:#?}", all_possible_reductions(leaves.clone()));
        let huffman_codes: Vec<Code<u32>> = all_possible_reductions(leaves)
            .iter().map(to_code).collect_vec();
        match huffman_codes.len() {
            0 => panic!("There should always exist a huffman code"),
            1 => continue,
            _ => (),
        } //TODO: Check for actual ambiguitites?
        let one_huffman_dominates_other = huffman_codes.iter()
            .tuple_combinations::<(_,_)>()
            .map(|(a, b)| a.beats(b))
            .any(|b| b.unwrap());
        if one_huffman_dominates_other {
            //println!("Found pmf where one huffman code dominates another");
            num_good_sources_tested+=1;
        } else { continue };
        let tallest_huffman_code: &Code<u32> = huffman_codes.iter()
            .min_by(|a, b| max_depth(a).cmp(&max_depth(b))).unwrap();
        let better_codes = huffman_codes.iter()
            .filter(|&a| a.beats(tallest_huffman_code).unwrap())
            .collect_vec();
        if better_codes.len()!=0 {
            println!("Found a code that beats skinny code");
            println!("Skinny code:");
            println!("{:#?}", tallest_huffman_code);
            println!("A better code:");
            println!("{:#?}",better_codes[0]);
            exit(0)
        }
    }
}
