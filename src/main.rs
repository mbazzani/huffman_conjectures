mod node;
use node::Node;
use itertools::Itertools;
use std::iter::{zip, repeat};


fn construct_huffman(mut nodes: Vec<Node<u32>>) -> Option<Node<u32>> {
    loop {
        match nodes.len() {
            0 => return None,
            1 => return Some(nodes[0].clone()),
            _ => {
                nodes.sort_by(|a, b| b.cmp(a));
                let l: Node<u32> = nodes.pop().unwrap();
                let r: Node<u32> = nodes.pop().unwrap();
                nodes.push(Node::new_branch(l, r))
            }
        }
    }
}

fn pair_combinations_in_range(length: usize) -> Vec<(usize, usize)> {
    return (0..length).tuple_combinations::<(_,_)>().collect::<Vec<_>>()
}

fn remove_two<T>(x: usize, y:usize, vec: &mut Vec<T>) -> (T, T){
    assert!(x!=y);
    let mut pair = vec![x, y];
    pair.sort();
    let j = vec.remove(pair[0]);
    let k = vec.remove(pair[1]-1);
    (j, k)
}
fn join_pair_by_indices(pair_index: (usize, usize), mut nodes: Vec<Node<u32>>) -> Vec<Node<u32>> {
    let (l, r) = pair_index;
    let (left, right) =  remove_two(l, r, &mut nodes);
    nodes.push(Node::new_branch(left, right));
    nodes
}

fn count_same_sequence<T>(vec: &[T]) -> usize
where T: Eq + Ord {
    assert!(vec.windows(2).all(|w| w[0] <= w[1])); //sorted
    let mut count: usize = 0;
    for (i, elem) in vec.iter().enumerate() {
        if (*elem)!=vec[0] { break; }
        count=i;
    }
    count+1
}
fn possible_reductions(mut nodes: Vec<Node<u32>>) -> Vec<Vec<Node<u32>>> {
    assert!(nodes.len()>1);
    nodes.sort();
    let smallest_probability = nodes[0].probability();
    let num_smallest_nodes = count_same_sequence(&nodes);
    let mut num_next_smallest_nodes = 0;
    for (i, node) in nodes.iter().enumerate() {
        if node.probability() != smallest_probability {
            num_next_smallest_nodes = count_same_sequence(&nodes[i..]);
            break
        }
    }
    //TODO: Move into own function?
    let possible_pair_indices: Vec<(usize, usize)>;
    match (num_smallest_nodes, num_next_smallest_nodes) {
        (0, _) => panic!("Should be impossible because of the length assertion"),
        (1, 0) => panic!("Should be impossible because of the length assertion"),
        (1, 1) => possible_pair_indices = vec![(0, 1)],
        (1, n) => possible_pair_indices = zip(repeat(0usize), 1..(n+1)).collect_vec(),
        (n, _) => possible_pair_indices = pair_combinations_in_range(n),
    }

    let mut possible_reductions: Vec<Vec<Node<u32>>> = vec![];
    for (x, y) in possible_pair_indices.into_iter() {
        possible_reductions.push(join_pair_by_indices((x, y), nodes.clone()));
    }
    possible_reductions
}
fn all_possible_reductions(nodes: Vec<Node<u32>>) -> Vec<Node<u32>> {
    let mut partial_reductions = vec![nodes];
    let mut completed_reductions: Vec<Node<u32>> = vec![];
    while !partial_reductions.is_empty() {
        match partial_reductions.last().unwrap().len() {
            0 | 1 => {
                let mut last = partial_reductions.pop().unwrap();
                completed_reductions.append(&mut last);
            },
            _ => {
                let last = partial_reductions.pop().unwrap();
                partial_reductions.append(&mut possible_reductions(last));
            },
        };
    };
    completed_reductions
}


//Have tree type
//Implement huffman algorithm that branches at each possible step, 
//returning a vector of Huffman codes that you then join with the previous
//Regenerate PMF until you get one Huffman code beating another
//Check shape of best Huffman code
fn main() {
    for _ in 0..1000 {
        let leaves = vec![
            Node::new_leaf(2, '🦀'),
            Node::new_leaf(2, '🦀'),
            Node::new_leaf(2, '🦀'),
            Node::new_leaf(2, '🦀'),
            Node::new_leaf(2, '🦀'),
            Node::new_leaf(2, '🦀'),
            Node::new_leaf(2, '🦀'),
            Node::new_leaf(2, '🦀'),
        ];
        let huffman_codes = all_possible_reductions(leaves);
    }
    //println!("{:#?}", huffman_codes);
}
