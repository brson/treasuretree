use anyhow::{bail, Result};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};
use rand::thread_rng;

use bech32::{FromBase32, ToBase32, Variant};

pub use geonft_nostd::crypto::*;

pub fn new_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}
