mod optimal;
mod last_recently_used;

use rand::Rng;
use crate::last_recently_used::last_recently_used_algorithm;
use crate::optimal::optimal_algorithm;

fn main() {
    let seq = generate_random_reference_sequence();
    let optimal = optimal_algorithm(seq);
    let last_recently_used = last_recently_used_algorithm(seq);
    println!("Optimal: {}", optimal);
    println!("LRU: {}", last_recently_used);
}

fn generate_random_reference_sequence() -> [u8; 1000] {
    let mut rng = rand::thread_rng();
    let mut sequence: [u8; 1000] = [0; 1000];
    for i in 0..sequence.len() {
        let n1: u8 = rng.gen();
        sequence[i] = n1;
    }
    return sequence;
}
