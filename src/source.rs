use crate::Node;
use rand::{Rng, thread_rng};
use std::iter::zip;
use itertools::Itertools;

pub struct Source<T>(Vec<(char, T)>);

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

//TODO: Resize dynamically
static PROBABILITY_GRANULARITY: u32 = 120;

impl Source<u32> {
    fn uniform_int_probabilities(len: usize) -> Vec<u32> {
        let max_probability: u32 = <usize as TryInto<u32>>::try_into(len).unwrap() 
            * PROBABILITY_GRANULARITY;
        let mut rand_values: Vec<u32> = (0..len+1)
            .map(|_| thread_rng().gen_range(0..=max_probability))
            .collect_vec();
        rand_values.sort();
        rand_values[0] = 0;
        rand_values[len] = max_probability;
        let mut probabilities = vec![];
        for i in 1..rand_values.len() {
            probabilities.push(rand_values[i] - rand_values[i-1]);
        }
        assert!(probabilities.len()==len);
        probabilities
    }
    pub fn new(size: usize) -> Source<u32> {
        Source(zip(
                ASCII_LOWER, 
                Source::uniform_int_probabilities(size),
            ).collect_vec())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn to_leaves_vec(&self) -> Vec<Node<u32>> {
        self.0.iter().map(|(c, p)| Node::new_leaf(*p, *c)).collect_vec()
    }
}
