//! Request / response types used by both Rocket and Solana APIs
//!
//! These are all shared between at least two crates.
//!
//! Rocket APIs are JSON-encoded via serde,
//! and Solana APIs are Borsch-encoded.

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

/// A Rocket request to plant a treasure
#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct PlantRequest {
    /// The public key of the account that is planting the treasure
    pub account_public_key: String,
    /// A public key to represent the treasure, bech32 encoded
    pub treasure_public_key: String,
    /// An image, base64 encoded
    pub image: String,
    /// A base64-encoded signature by the account of
    /// the string "plant",
    /// appended by the encoded treasure public key.
    pub account_signature: String,
    /// A base64-encoded signature by the treasure key of
    /// the string "plant",
    /// appended by the encoded account public key,
    /// appended by the binary sha256 hash of the image.
    pub treasure_signature: String,
}

/// A Rocket request to claim a treasure
#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct ClaimRequest {
    /// The public key of the claiming account, bech32 encoded
    pub account_public_key: String,
    /// The public key of the treasure, bech32 encoded
    pub treasure_public_key: String,
    /// A base64-encoded signature by the account key of
    /// the string "claim",
    /// appended by the encoded treasure public key,
    pub account_signature: String,
    /// A base64-encoded signature by the treasure key of
    /// the string "claim",
    /// appended by the encoded account public key.
    pub treasure_signature: String,
}

/// A Solana request
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum GeonftRequestSolana {
    PlantTreasure(PlantRequestSolana),
    ClaimTreasure(ClaimRequestSolana),
}

/// A Solana request to plant a treasure
#[derive(Debug, BorshSerialize, BorshDeserialize, Hash, Eq, PartialEq)]
pub struct PlantRequestSolana {
    /// The public key of the account that is planting the treasure
    pub account_public_key: Vec<u8>,
    /// A public key to represent the treasure, bech32 encoded
    pub treasure_public_key: Vec<u8>,
    /// An image hash, base64 encoded
    pub treasure_hash: Vec<u8>,
}

/// A Solana request to claim a treasure
#[derive(Debug, BorshSerialize, BorshDeserialize, Hash, Eq, PartialEq)]
pub struct ClaimRequestSolana {
    /// The public key of the claiming account, bech32 encoded
    pub account_public_key: Vec<u8>,
    /// A public key to represent the treasure, bech32 encoded
    pub treasure_public_key: Vec<u8>,
}
