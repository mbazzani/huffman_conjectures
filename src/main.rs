mod node;
use node::Node;
use node::NodeType;
use itertools::Itertools;

fn construct_huffman(mut nodes: Vec<Node<u32>>) -> Option<Node<u32>> {
    loop {
        match nodes.len() {
            0 => return None,
            1 => return Some(nodes[0].clone()),
            _ => {
                nodes.sort_by(|a, b| b.cmp(a));
                let l: Node<u32> = nodes.pop().unwrap();
                let r: Node<u32> = nodes.pop().unwrap();
                nodes.push(join(l, r))
            }
        }
    }
}

fn join(left: Node<u32>,  right: Node<u32>) -> Node<u32> {
    Node::new_branch(left, right)
}


fn possible_pair_indices(length: u32) -> Vec<(u32, u32)> {
    return (0..length).tuple_combinations::<(_,_)>().collect::<Vec<_>>()
}
fn join_pair(pair_indices: (u32, u32), nodes: Vec<Node<u32>>) -> Vec<Node<u32>> {
    unimplemented!();
}

fn get_possible_reductions(nodes: Vec<Node<u32>>) -> Vec<Vec<Node<u32>>> {
    let min: &Node<u32> = nodes.iter().min().unwrap();
    let length = unimplemented!();
    let pair_indices = (0..length)
        .tuple_combinations::<(_,_)>()
        .collect::<Vec<_>>();
    for (first, second) in pair_indices {}
    unimplemented!();
}

fn get_all_huffman(leaves: Vec<Node<u32>>) -> Vec<Node<u32>> {
    match leaves.len() {
        0 | 1 => return leaves,
        _ => {
            let mut huffman_codes: Vec<Node<u32>> = vec![];
            let min: &Node<u32> = leaves.iter().min().unwrap();
            let (smallest_nodes, other_nodes): (Vec<_>, Vec<_>) =
                leaves.clone().into_iter().partition(|x| x<=min);
            let possible_combinations = smallest_nodes.into_iter()
                    .tuple_combinations::<(_,_)>()
                    .map(|(l, r)| join(l, r));
            //let possible_combinations_ = possible_combinations.clone();
            //println!("{:#?}", possible_combinations_.collect::<Vec<_>>());
            //unimplemented!();
            for joined_node in possible_combinations {
                //let mut other_nodes_ = other_nodes.clone();
                let mut possible_huffman_codes = 
                    get_all_huffman([other_nodes.clone(), vec![joined_node]].concat());
                huffman_codes.append(&mut possible_huffman_codes);
            }
            huffman_codes
        },
    }
}


//Have tree type
//Implement huffman algorithm that branches at each possible step, 
//returning a vector of Huffman codes that you then join with the previous
//Regenerate PMF until you get one Huffman code beating another
//Check shape of best Huffman code
fn main() {
    let leaves = vec![
        Node::new_leaf(2, '🦀'),
        Node::new_leaf(2, '🍉'),
        Node::new_leaf(1, '🏅'),
        Node::new_leaf(1, '🦬'),
    ];
    let huffman_codes = get_all_huffman(leaves);
    println!("{:#?}", huffman_codes);
    println!("Hello, world!");
    let vect = vec![1, 1, 1];
    let combs: Vec<_> = vect.iter().tuple_combinations::<(_,_)>().collect();
    println!("{:#?}", combs)
}
