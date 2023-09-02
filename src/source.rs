use crate::Node;

use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::iter::zip;

pub struct Source<T>(Vec<(char, T)>);

const ASCII: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D',
    'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

//const PROBABILITY_GRANULARITY: u32 = ;

impl Source<u32> {
    #[allow(dead_code)]
    fn uniform_int_probabilities(len: usize) -> Vec<u32> {
        let mut probabilities = vec![0];
        let mut rng = thread_rng();
        while probabilities.contains(&0) {
            let max_probability: u32 = u32::try_from(len * len).unwrap();
            let mut rand_values: Vec<u32> = (0..len + 1)
                .map(|_| rng.gen_range(1..=max_probability))
                .collect_vec();
            rand_values.sort(); //smallest to biggest
            rand_values[0] = 0;
            rand_values[len] = max_probability;
            probabilities = rand_values
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec();
        }
        probabilities
    }

    fn int_probabilities(len: usize) -> Vec<u32> {
        let mut rng = thread_rng();
        let max_probability: u32 = u32::try_from(len * len).unwrap();
        (0..len)
            .map(|_| rng.gen_range(1..max_probability))
            .collect_vec()
    }

    #[allow(dead_code)]
    pub fn new_int_uniform(size: usize) -> Source<u32> {
        Source(
            zip(ASCII, Source::uniform_int_probabilities(size)).collect_vec(),
        )
    }

    pub fn new(size: usize) -> Source<u32> {
        Source(zip(ASCII, Source::int_probabilities(size)).collect_vec())
    }
    #[allow(dead_code)]
    pub fn from_vec(vec: Vec<(char, u32)>) -> Source<u32> {
        Source(vec)
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn to_leaves_vec(&self) -> Vec<Node<u32>> {
        self.0
            .iter()
            .map(|(c, p)| Node::new_leaf(*p, *c))
            .collect_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::node::Node;

    #[test]
    fn len_test() {
        let size = 33;
        let source = Source::new(size);
        assert!(source.0.len() == size);
    }

    #[test]
    fn new_int_uniform_test() {
        let size_usize: usize = 23;
        let size_u32: u32 = 23;
        let source = Source::new_int_uniform(size_usize);
        assert!(
            source.0.iter().map(|(_, i)| i).fold(0, |acc, x| acc + x)
                == size_u32 * size_u32
        );
        assert!(!source.0.iter().map(|(_, i)| i).contains(&0));
    }

    #[test]
    fn new_test() {
        let size_usize: usize = 23;
        let size_u32: u32 = 23;
        let source = Source::new(size_usize);
        assert!(!source.0.iter().any(|(_, i)| *i > size_u32 * size_u32));
        assert!(!source.0.iter().map(|(_, i)| i).contains(&0));
    }

    #[test]
    fn to_leaves_vec_test() {
        let source =
            Source(vec![('a', 1), ('b', 2), ('c', 3), ('d', 4), ('e', 5)]);
        let mut leaves_vec = source.to_leaves_vec();
        let mut leaves_vec_test = vec![
            Node::new_leaf(1, 'a'),
            Node::new_leaf(2, 'b'),
            Node::new_leaf(3, 'c'),
            Node::new_leaf(4, 'd'),
            Node::new_leaf(5, 'e'),
        ];
        leaves_vec.sort();
        leaves_vec_test.sort();
        assert!(
            zip(leaves_vec.iter(), leaves_vec_test.iter())
                .filter(|(x, y)| (*x) != (*y))
                .collect_vec()
                .len()
                == 0
        );
    }
}
