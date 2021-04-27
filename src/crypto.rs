use anyhow::{Result, bail};
use ed25519_dalek::{PublicKey, SecretKey, Keypair, Signature};
use rand::thread_rng;

use bech32::{FromBase32, ToBase32, Variant};

#[path = "crypto_shared.rs"]
mod crypto_shared;

pub use crypto_shared::*;

pub fn new_keypair() -> Keypair {
    Keypair::generate(&mut thread_rng())
}

