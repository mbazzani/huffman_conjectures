use rustc_hash::FxHashMap;
use std::hash::Hash;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use xxhash_rust::xxh3::xxh3_64;
use xxhash_rust::xxh3::Xxh3Builder;
use xxhash_rust::xxh3::Xxh3;

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

pub type Depth = u8;
pub type Code<T> = HashMap<CodeWord<T>, Depth, BuildHasherDefault<Xxh3>>;

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
impl CompetitiveOrd for Code<u32> {
	fn competitive_advantage(&self, other: &Code<u32>) -> Option<i64> {
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
        HashMap::with_hasher(BuildHasherDefault::default())
    }
}
