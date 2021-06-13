use rand::thread_rng;

pub use geonft_nostd::crypto::*;

pub fn new_keypair() -> Keypair {
    generate_keypair(&mut thread_rng())
}
