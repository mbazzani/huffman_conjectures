mod node;
mod code;
mod source;
mod conjectures;

use crate::conjectures::no_huffman_code_competitively_dominates_skinniest;
use crate::node::Node;

use rayon::prelude::*;
use std::process::exit;

fn main() {
    for source_size in 7..24 {
        println!("Source size: {}", source_size);
        let conjecture_test_fn = no_huffman_code_competitively_dominates_skinniest;
        let num_counterexamples = vec![0; 10]
            .par_iter()
            .map(|_| conjecture_test_fn(source_size, 1000))
            .filter(|&x| !x)
            .count();
        if num_counterexamples > 0 {
            exit(0);
        }
    }
}
