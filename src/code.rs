use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ordering;

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
pub type Code<T> = HashMap<CodeWord<T>, Depth>;

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

impl CompetitiveOrd for Code<u32> {
    //TODO: Rewrite as closure because I'm not a filthy imperative programmer
	fn competitive_advantage(&self, other: &Code<u32>) -> Option<i64> {
	    let mut competitive_advantage: i64 = 0;
	    for (code_word, depth) in self {
	        match depth.cmp(&other[code_word]) {
	            Ordering::Greater => competitive_advantage += code_word.probability as i64,
	            Ordering::Equal => (),
	            Ordering::Less => competitive_advantage -= code_word.probability as i64,
            }
	    }
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
