use rand::thread_rng;

pub use geonft_nostd::crypto::*;

pub fn new_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}
