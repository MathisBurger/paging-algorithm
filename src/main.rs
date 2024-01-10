use rand::Rng;

fn main() {
    println!("Hello, world!");
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
