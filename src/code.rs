use crate::node::{Node, NodeType};

use std::hash::Hash;
use std::cmp::Ordering;
use std::collections::HashMap;
use xxhash_rust::xxh3::Xxh3Builder;

#[derive(Debug, Clone, Hash)]
pub struct CodeWord<T> {
    source_symbol: char,
    probability: T,
}

impl<T: Eq> PartialEq for CodeWord<T> {
    fn eq(&self, other: &Self) -> bool {
        self.probability == other.probability 
            && self.source_symbol == other.source_symbol
    }
}

impl<T: Eq> Eq for CodeWord<T> {}

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

//TODO: Make generic
impl CompetitiveOrd for Code<Probability> {
	fn competitive_advantage(&self, other: &Code<Probability>) -> Option<i64> {
        let competitive_advantage: i64 = 
            self.iter()
            .map(|(code_word, depth)| 
                match depth.cmp(&other[code_word]) {
                    Ordering::Greater => code_word.probability as i64,
                    Ordering::Equal => 0,
                    Ordering::Less => -(code_word.probability as i64),
                }
            ).sum();
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

pub trait FromNode {
    fn from_node(node: &Node<Probability>) -> Self;

}

fn from_node_helper(node: &Node<Probability>, depth: Depth) -> Vec<(CodeWord<Probability>, Depth)> {
    let mut code: Vec<(CodeWord<Probability>, Depth)> = vec![];
    match node.node_type() {
        NodeType::Leaf(symbol) => {
            code.push((CodeWord::new(*symbol, node.probability()), depth));
        }
        NodeType::Branch(children) => {
            code.append(&mut from_node_helper(&children[0], depth+1));
            code.append(&mut from_node_helper(&children[1], depth+1));
        },
    };
    code
}

impl FromNode for Code<Probability> {
	fn from_node(node: &Node<Probability>) -> Code<Probability> {
	    from_node_helper(node, 0).into_iter().collect()
	}
}

pub trait MaxDepth {
    fn max_depth(&self) -> &Depth;
}

impl MaxDepth for Code<Probability> {
    fn max_depth(&self) -> &Depth {
        self.iter().max_by_key(|(_, &v)| v).unwrap().1
    }
}
