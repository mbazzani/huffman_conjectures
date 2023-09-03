use crate::node::{Node, NodeType, RealNum};
use crate::source::Source;

use itertools::Itertools;
use rayon::prelude::*;
use sorted_vec::SortedVec;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::zip;
use xxhash_rust::xxh3::Xxh3Builder;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CodeWord<T> {
    source_symbol: char,
    probability: T,
}

pub type Probability = u32;
pub type Depth = u8;
pub type Code<T> = HashMap<CodeWord<T>, Depth, Xxh3Builder>;

impl<T> CodeWord<T> {
    pub fn new(source_symbol: char, probability: T) -> CodeWord<T> {
        CodeWord {
            source_symbol,
            probability,
        }
    }
}

pub trait CompetitiveOrd {
    fn competitive_advantage(&self, other: &Self) -> Option<i64>;
    fn beats(&self, other: &Self) -> Option<bool>;
    fn loses(&self, other: &Self) -> Option<bool>;
    fn ties(&self, other: &Self) -> Option<bool>;
}

impl CompetitiveOrd for Code<Probability> {
    fn competitive_advantage(&self, other: &Code<Probability>) -> Option<i64> {
        let competitive_advantage: i64 = self
            .iter()
            .map(|(code_word, depth)| match depth.cmp(&other[code_word]) {
                Ordering::Less => code_word.probability as i64,
                Ordering::Equal => 0,
                Ordering::Greater => -(code_word.probability as i64),
            })
            .sum();
        Some(competitive_advantage)
    }
    fn beats(&self, other: &Self) -> Option<bool> {
        Some(self.competitive_advantage(other)? > 0)
    }
    fn loses(&self, other: &Self) -> Option<bool> {
        Some(self.competitive_advantage(other)? < 0)
    }
    fn ties(&self, other: &Self) -> Option<bool> {
        Some(self.competitive_advantage(other)? == 0)
    }
}

pub trait New {
    fn new() -> Self;
}

impl<T> New for Code<T> {
    fn new() -> Self {
        HashMap::with_hasher(Xxh3Builder::default())
    }
}

//trait FromIter<T> {
//    fn from_iter(iter: &mut dyn Iterator<Item = ((char, T), Depth)>) -> Self;
//}
//
//impl<T> FromIter<T> for Code<T> where T: Hash + RealNum {
//    fn from_iter(iter: &mut dyn Iterator<Item = ((char, T), Depth)>) -> Self {
//        iter.map(|((s, p), d)|  (CodeWord { source_symbol: s, probability: p }, d)).collect()
//    }
//}

pub trait FromNode {
    fn from_node(node: &Node<Probability>) -> Self;
}

//TODO: Rewrite to be tail recursive
//and use iterators to avoid needless allocations
impl FromNode for Code<Probability> {
    fn from_node(node: &Node<Probability>) -> Code<Probability> {
        fn helper(
            node: &Node<Probability>,
            depth: Depth,
        ) -> Vec<(CodeWord<Probability>, Depth)> {
            let mut code: Vec<(CodeWord<Probability>, Depth)> = vec![];
            match node.node_type() {
                NodeType::Leaf(symbol) => {
                    code.push((
                        CodeWord::new(*symbol, node.probability()),
                        depth,
                    ));
                }
                NodeType::Branch(children) => {
                    code.append(&mut helper(&children[0], depth + 1));
                    code.append(&mut helper(&children[1], depth + 1));
                }
            };
            code
        }
        helper(node, 0).into_iter().collect()
    }
}

pub trait MaxDepth {
    fn max_depth(&self) -> Depth;
}

impl MaxDepth for Code<Probability> {
    fn max_depth(&self) -> Depth {
        *self.iter().max_by_key(|(_, &v)| v).unwrap().1
    }
}

fn next_length_profiles_from_previous(
    length_profile: &SortedVec<Depth>,
) -> Vec<SortedVec<Depth>> {
    let mut possible_length_profiles = vec![];
    let previous_depth = 0;
    for &depth in length_profile.iter() {
        if previous_depth != depth {
            let mut length_profile = length_profile.clone();
            length_profile.remove_item(&depth);
            length_profile.insert(depth + 1);
            length_profile.insert(depth + 1);
            possible_length_profiles.push(length_profile);
        }
    }
    possible_length_profiles
}

fn next_set_of_length_profiles(
    previous_length_profiles: &HashSet<SortedVec<Depth>, Xxh3Builder>,
) -> HashSet<SortedVec<Depth>, Xxh3Builder> {
    let mut next_length_profiles = HashSet::with_hasher(Xxh3Builder::default());
    for length_profile in previous_length_profiles.iter() {
        next_length_profiles
            .extend(next_length_profiles_from_previous(length_profile))
    }
    next_length_profiles
}

fn permute_length_profiles(
    length_profiles: HashSet<SortedVec<Depth>, Xxh3Builder>,
) -> HashSet<Vec<Depth>, Xxh3Builder> {
    fn permutations(profile: SortedVec<Depth>) -> HashSet<Vec<Depth>> {
        let len = profile.len();
        profile.into_vec().into_iter().permutations(len).collect()
    }
    length_profiles
        .into_iter()
        .flat_map(|profile| permutations(profile))
        .collect()
}

pub fn possible_length_profiles(
    num_leaves: usize,
) -> Option<HashSet<Vec<Depth>, Xxh3Builder>> {
    if num_leaves < 2 {
        return None;
    }
    let mut length_profiles: Vec<HashSet<SortedVec<Depth>, Xxh3Builder>> =
        vec![];
    let mut length_profiles_for_two_nodes =
        HashSet::with_hasher(Xxh3Builder::default());
    length_profiles_for_two_nodes.insert(SortedVec::from_unsorted(vec![1, 1]));
    length_profiles.push(length_profiles_for_two_nodes);
    for i in 0..num_leaves - 2 {
        length_profiles.push(next_set_of_length_profiles(&length_profiles[i]))
    }
    Some(
        length_profiles
            .into_iter()
            .last()
            .into_par_iter()
            .flat_map(|length_profiles| {
                permute_length_profiles(length_profiles)
            })
            .collect(),
    )
}

pub fn possible_codes<T>(source: Source<T>, length_profiles: HashSet<Vec<Depth>, Xxh3Builder>) -> Vec<Code<T>>
where
    T: RealNum + Hash,
{
    let code_words = source
        .into_iter()
        .map(|(s, p)| CodeWord {
            source_symbol: s,
            probability: p,
        })
        .collect_vec();
    length_profiles
        .into_iter()
        .map(|profile| {
            zip(code_words.clone().into_iter(), profile.into_iter()).collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_node_test() {
        let huff = Node::new_huffman(vec![
            Node::new_leaf(1, 'a'),
            Node::new_leaf(2, 'b'),
            Node::new_leaf(3, 'c'),
            Node::new_leaf(4, 'd'),
            Node::new_leaf(5, 'e'),
        ])
        .unwrap();
        let mut huff_code = Code::new();
        huff_code.insert(CodeWord::new('a', 1), 3);
        huff_code.insert(CodeWord::new('b', 2), 3);
        huff_code.insert(CodeWord::new('c', 3), 2);
        huff_code.insert(CodeWord::new('d', 4), 2);
        huff_code.insert(CodeWord::new('e', 5), 2);
        assert_eq!(Code::from_node(&huff), huff_code);
    }

    #[test]
    fn max_depth_test() {
        let mut huff_code = Code::new();
        huff_code.insert(CodeWord::new('a', 1), 5);
        huff_code.insert(CodeWord::new('b', 1), 1);
        huff_code.insert(CodeWord::new('c', 1), 2);
        huff_code.insert(CodeWord::new('d', 1), 8);
        huff_code.insert(CodeWord::new('e', 1), 7);
        assert_eq!(huff_code.max_depth(), 8);
    }

    #[test]
    fn competitive_ord_test() {
        let mut code_a = Code::new();
        code_a.insert(CodeWord::new('a', 1), 3);
        code_a.insert(CodeWord::new('b', 2), 3);
        code_a.insert(CodeWord::new('c', 3), 2);
        code_a.insert(CodeWord::new('d', 4), 1);

        let mut code_b = Code::new();
        code_b.insert(CodeWord::new('a', 1), 3);
        code_b.insert(CodeWord::new('b', 2), 2);
        code_b.insert(CodeWord::new('c', 3), 1);
        code_b.insert(CodeWord::new('d', 4), 3);

        let mut code_c = Code::new();
        code_c.insert(CodeWord::new('a', 1), 3);
        code_c.insert(CodeWord::new('b', 2), 1);
        code_c.insert(CodeWord::new('c', 3), 3);
        code_c.insert(CodeWord::new('d', 4), 2);

        assert!(code_b.beats(&code_a).unwrap());
        assert!(code_c.beats(&code_b).unwrap());
        assert!(code_a.beats(&code_c).unwrap());
    }
}
