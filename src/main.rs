mod code;
mod conjectures;
mod node;
mod source;

use crate::conjectures::no_huffman_code_competitively_dominates_skinniest;
use crate::node::Node;

use rayon::prelude::*;

fn main() {
    for source_size in 7..24 {
        println!("Source size: {}", source_size);
        let conjecture_test_fn =
            no_huffman_code_competitively_dominates_skinniest;
        //        let num_counterexamples = vec![0; 10]
        //            .par_iter()
        //            .map(|_| conjecture_test_fn(source_size, 120000))
        //            .filter(|&x| !x)
        //            .count();
        let counterexample_exists = vec![0; 10]
            .par_iter()
            .any(|_| !conjecture_test_fn(source_size, 100000));
        if counterexample_exists {
            return;
        }
    }
}
