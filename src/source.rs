use crate::Node;
use rand::{Rng, thread_rng};
use std::iter::zip;
use itertools::Itertools;

pub struct Source<T>(Vec<(char, T)>);

static ASCII: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',

    'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y',
    'Z'
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
        let probabilities = rand_values
            .iter()
            .tuple_windows()
            .map(|(a, b)| b-a)
            .collect_vec();
        assert!(probabilities.len()==len);
        probabilities
    }

    pub fn new(size: usize) -> Source<u32> {
        Source(zip(
                ASCII, 
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
