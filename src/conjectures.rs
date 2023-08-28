use crate::code::{Code, CompetitiveOrd, FromNode, MaxDepth};
use crate::node::Node;
use crate::source::Source;

use itertools::Itertools;
use std::iter::{repeat, zip};
use std::ops::Add;

fn pair_combinations_in_range(length: usize) -> Vec<(usize, usize)> {
    (0..length)
        .tuple_combinations::<(_, _)>()
        .collect::<Vec<_>>()
}

fn remove_two<T>(x: usize, y: usize, vec: &mut Vec<T>) -> (T, T) {
    assert!(x != y);
    let mut pair = vec![x, y];
    pair.sort();
    let j = vec.remove(pair[0]);
    let k = vec.remove(pair[1] - 1);
    (j, k)
}

fn join_pair_by_indices<T>(pair_index: (usize, usize), mut nodes: Vec<Node<T>>) -> Vec<Node<T>>
where
    T: Copy + Add<Output = T> + Ord,
{
    let (l, r) = pair_index;
    let (left, right) = remove_two(l, r, &mut nodes);
    nodes.push(Node::new_branch(left, right));
    nodes
}

fn count_same_sequence<T>(vec: &[T]) -> usize
where
    T: Eq + Ord,
{
    assert!(vec.windows(2).all(|w| w[0] <= w[1])); //sorted
    let mut count: usize = 0;
    for (i, elem) in vec.iter().enumerate() {
        if (*elem) != vec[0] {
            break;
        }
        count = i;
    }
    count + 1
}
fn possible_reductions<T>(mut nodes: Vec<Node<T>>) -> Vec<Vec<Node<T>>>
where
    T: Add<Output = T> + Copy + Ord,
{
    assert!(!nodes.is_empty());
    nodes.sort();
    let smallest_probability = nodes[0].probability();
    let num_smallest_nodes = count_same_sequence(&nodes);
    let mut num_next_smallest_nodes = 0;
    for (i, node) in nodes.iter().enumerate() {
        if node.probability() != smallest_probability {
            num_next_smallest_nodes = count_same_sequence(&nodes[i..]);
            break;
        }
    }
    //TODO: Move into own function?
    let possible_pair_indices: Vec<(usize, usize)> =
        match (num_smallest_nodes, num_next_smallest_nodes) {
            (0, _) => panic!("Impossible because of the length assertion"),
            (1, 0) => panic!("Impossible because of the length assertion"),
            (1, 1) => vec![(0, 1)],
            (1, n) => zip(repeat(0usize), 1..(n + 1)).collect_vec(),
            (n, _) => pair_combinations_in_range(n),
        };

    let mut possible_reductions: Vec<Vec<Node<T>>> = vec![];
    for (x, y) in possible_pair_indices.into_iter() {
        possible_reductions.push(join_pair_by_indices((x, y), nodes.clone()));
    }
    possible_reductions
}
fn all_possible_reductions<T>(nodes: Vec<Node<T>>) -> Vec<Node<T>>
where
    T: Add<Output = T> + Copy + Ord,
{
    let mut partial_reductions = vec![nodes];
    let mut completed_reductions: Vec<Node<T>> = vec![];
    while !partial_reductions.is_empty() {
        match partial_reductions.last().unwrap().len() {
            0 | 1 => {
                let mut last = partial_reductions.pop().unwrap();
                completed_reductions.append(&mut last);
            }
            _ => {
                let last = partial_reductions.pop().unwrap();
                partial_reductions.append(&mut possible_reductions(last));
            }
        };
    }
    completed_reductions
}

pub fn no_huffman_code_competitively_dominates_skinniest(
    source_size: usize,
    sources_to_test: u32,
) -> bool {
    let mut sources_tested: u32 = 0;
    while sources_tested < sources_to_test {
        let leaves = Source::new(source_size).to_leaves_vec();
        let huffman_codes: Vec<Code<u32>> = all_possible_reductions(leaves)
            .iter()
            .map(Code::from_node)
            .collect_vec();
        match huffman_codes.len() {
            0 => panic!("There should always exist a huffman code"),
            1 => continue,
            _ => (),
        } //TODO: Check for actual ambiguitites?
        let one_huffman_dominates_other = huffman_codes
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(|(a, b)| a.beats(b))
            .any(|b| b.unwrap());
        if one_huffman_dominates_other {
            //println!("Found pmf where one huffman code dominates another");
            sources_tested += 1;
        } else {
            continue;
        };
        let tallest_huffman_code: &Code<u32> = huffman_codes
            .iter()
            .max_by(|a, b| a.max_depth().cmp(b.max_depth()))
            .unwrap();
        let better_codes = huffman_codes
            .iter()
            .filter(|&a| a.beats(tallest_huffman_code).unwrap())
            .collect_vec();
        if !better_codes.is_empty() {
            println!("Found a code that beats skinny code");
            println!("Skinny code:");
            println!("{:#?}", tallest_huffman_code);
            println!("A better code:");
            println!("{:#?}", better_codes[0]);
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Node;

    #[test]
    fn all_possible_reductions_test() {
        let leaves = vec![
            Node::new_leaf(1, 'a'),
            Node::new_leaf(1, 'b'),
            Node::new_leaf(2, 'c'),
            Node::new_leaf(2, 'd'),
        ];
        let huff_a = 
        Node::new_branch(
            leaves[3].clone(),
	        Node::new_branch(
	            leaves[2].clone(),
	            Node::new_branch(
	                leaves[0].clone(), 
	                leaves[1].clone()
	                ),
                )
            );
        let huff_b = 
        Node::new_branch(
            leaves[2].clone(),
	        Node::new_branch(
	            leaves[3].clone(),
	            Node::new_branch(
	                leaves[0].clone(), 
	                leaves[1].clone()
	                ),
                )
            );

        let huff_c = 
            Node::new_branch(
	            Node::new_branch(
	                leaves[3].clone(),
                    leaves[2].clone(),
                    ),
	            Node::new_branch(
	                leaves[0].clone(), 
	                leaves[1].clone()
	                ),
                );

        let reductions = all_possible_reductions(leaves);
        println!("Reductions: {:#?}", reductions);
        assert!(reductions.iter().any(|node| node.is_same_as(&huff_a)));
        assert!(reductions.iter().any(|node| node.is_same_as(&huff_b)));
        assert!(reductions.iter().any(|node| node.is_same_as(&huff_c)));
    }
}
