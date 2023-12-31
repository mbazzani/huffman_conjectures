use crate::code::{
    possible_codes, possible_length_profiles, Code, CompetitiveOrd, FromNode,
    MaxDepth,
};
use crate::node::{Node, RealNum};
use crate::source::Source;

use itertools::Itertools;
use std::iter::{once, repeat, zip};

fn remove_two<T>(x: usize, y: usize, vec: &mut Vec<T>) -> (T, T) {
    assert!(x != y);
    let (first, second) = if x < y { (x, y) } else { (y, x) };

    let j = vec.remove(first);
    let k = vec.remove(second - 1);
    (j, k)
}

fn join_nodes_by_indices<T>(
    pair_index: (usize, usize),
    mut nodes: Vec<Node<T>>,
) -> Vec<Node<T>>
where
    T: RealNum,
{
    let (left, right) = remove_two(pair_index.0, pair_index.1, &mut nodes);
    nodes.push(Node::new_branch(left, right));
    nodes
}

fn possible_reductions<T>(mut nodes: Vec<Node<T>>) -> Vec<Vec<Node<T>>>
where
    T: RealNum,
{
    //TODO: Add better error handling
    assert!(!nodes.is_empty());
    nodes.sort();
    let num_smallest_nodes =
        nodes.iter().take_while(|&node| *node == nodes[0]).count();
    let num_second_smallest_nodes = nodes[num_smallest_nodes..]
        .iter()
        .take_while(|&node| *node == nodes[num_smallest_nodes])
        .count();

    let possible_pair_indices: Box<dyn Iterator<Item = (usize, usize)>> =
        match (num_smallest_nodes, num_second_smallest_nodes) {
            (0, _) => panic!("Impossible because of the length assertion"),
            (1, 0) => panic!("Impossible because of the length assertion"),
            (1, 1) => Box::new(once((0, 1))),
            (1, n) => Box::new(zip(repeat(0usize), 1..(n + 1))),
            (n, _) => Box::new((0..n).tuple_combinations::<(_, _)>()),
        };

    possible_pair_indices
        .map(|pair| join_nodes_by_indices(pair, nodes.clone()))
        .collect()
}

//Note that this breaks if there exists a zero probablity element
fn all_possible_reductions<T>(nodes: Vec<Node<T>>) -> Vec<Node<T>>
where
    T: RealNum,
{
    let mut partial_reductions = vec![nodes];
    let mut completed_reductions: Vec<_> = vec![];
    while !partial_reductions.is_empty() {
        (completed_reductions, partial_reductions) = partial_reductions
            .into_iter()
            .partition(|nodes| nodes.len() < 2);
        partial_reductions = partial_reductions
            .into_iter()
            .flat_map(|nodes_list| possible_reductions(nodes_list))
            .collect_vec();
    }
    completed_reductions.into_iter().flatten().collect_vec()
}

#[allow(dead_code)]
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
        }
        let one_huffman_dominates_other = huffman_codes
            .iter()
            .tuple_combinations::<(_, _)>()
            .any(|(a, b)| !a.ties(b).unwrap());
        if one_huffman_dominates_other {
            //println!("Found pmf where one huffman code dominates another");
            sources_tested += 1;
        } else {
            continue;
        };
        let tallest_huffman_code: &Code<u32> = huffman_codes
            .iter()
            .max_by(|a, b| a.max_depth().cmp(&b.max_depth()))
            .unwrap();
        let better_codes = huffman_codes
            .iter()
            .filter(|&a| a.beats(tallest_huffman_code).unwrap())
            .collect_vec();
        if !better_codes.is_empty() {
            println!("Found a code that beats skinny code");
            dbg!(tallest_huffman_code);
            dbg!(better_codes[0]);
            return false;
        }
    }
    true
}

pub fn no_huffman_dominates_another_and_is_optimal(
    source_size: usize,
    num_sources: u32,
) -> bool {
    let mut sources_tested = 0;
    let mut sources_that_passed_heuristic = 0;
    let possible_length_profiles =
        possible_length_profiles(source_size).unwrap();
    while sources_tested < num_sources {
        let source = Source::new(source_size);
        let leaves = source.to_leaves_vec();
        let possible_reductions = all_possible_reductions(leaves);
        let huffman_codes = possible_reductions
            .iter()
            .map(|node| (node, Code::from_node(node)))
            .collect_vec();
        match huffman_codes.len() {
            0 => panic!("There should always exist a huffman code"),
            1 => continue,
            _ => (),
        }
        let some_huffman_beat_others = huffman_codes
            .iter()
            .tuple_combinations::<(_, _)>()
            .any(|((_, code_a), (_, code_b))| !code_a.ties(code_b).unwrap());
        if some_huffman_beat_others {
            sources_tested += 1
        } else {
            continue;
        }
        let unbeaten_huffman_codes =
            huffman_codes.iter().filter(|(_, code)| {
                huffman_codes
                    .iter()
                    .all(|(_, other_code)| !other_code.beats(code).unwrap())
            });
        let mut possibly_optimal_codes = unbeaten_huffman_codes
            .filter(|(tree, _)| tree.is_probably_competitively_optimal())
            .map(|(_, code)| code);
        if possibly_optimal_codes.clone().next().is_some() {
            sources_that_passed_heuristic += 1;
        }

        let possible_codes =
            possible_codes(source.clone(), possible_length_profiles.clone());
        let true_optimal_code_exists = possibly_optimal_codes.any(|code| {
            possible_codes.iter().all(|other_code| {
                code.competitive_advantage(other_code).unwrap() <= 0
            })
        });
        if true_optimal_code_exists {
            println!("Found counterexample!!");
            return false;
        }
    }
    dbg!(sources_tested);
    dbg!(sources_that_passed_heuristic);
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
        let huff_a = Node::new_branch(
            leaves[3].clone(),
            Node::new_branch(
                leaves[2].clone(),
                Node::new_branch(leaves[0].clone(), leaves[1].clone()),
            ),
        );
        let huff_b = Node::new_branch(
            leaves[2].clone(),
            Node::new_branch(
                leaves[3].clone(),
                Node::new_branch(leaves[0].clone(), leaves[1].clone()),
            ),
        );

        let huff_c = Node::new_branch(
            Node::new_branch(leaves[3].clone(), leaves[2].clone()),
            Node::new_branch(leaves[0].clone(), leaves[1].clone()),
        );

        let reductions = all_possible_reductions(leaves);
        assert!(reductions.len() >= 1 && reductions.len() <= 4 * 3 * 2);
        assert!(reductions.iter().any(|node| node.is_same_as(&huff_a)));
        assert!(reductions.iter().any(|node| node.is_same_as(&huff_b)));
        assert!(reductions.iter().any(|node| node.is_same_as(&huff_c)));
    }
}
