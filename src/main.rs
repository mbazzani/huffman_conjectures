mod code;
mod conjectures;
mod node;
mod source;

use crate::conjectures::no_huffman_dominates_another_and_is_optimal;
use crate::node::Node;

use rayon::prelude::*;

fn main() {
    for source_size in 7..9 {
        println!("Source size: {}", source_size);
        let conjecture_test_fn = no_huffman_dominates_another_and_is_optimal;
        let counterexample_exists = vec![0; 8]
            .par_iter()
            .any(|_| !conjecture_test_fn(source_size, 1200000));
        if counterexample_exists {
            return;
        }
    }
}
