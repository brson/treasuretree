use serde::{Serialize, Deserialize};
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(Debug)]
#[derive(BorshSerialize, BorshDeserialize)]
#[derive(Serialize, Deserialize)]
#[derive(Hash, Eq, PartialEq)]
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

#[derive(Debug)]
#[derive(BorshSerialize, BorshDeserialize)]
#[derive(Serialize, Deserialize)]
pub enum GeonftRequest {
    PlantTreasure(PlantRequestHash),
    ClaimTreasure(ClaimRequest),
}

#[derive(Debug)]
#[derive(BorshSerialize, BorshDeserialize)]
#[derive(Serialize, Deserialize)]
#[derive(Hash, Eq, PartialEq)]
pub struct PlantRequestHash {
    /// The public key of the account that is planting the treasure
    pub account_public_key: String,
    /// A public key to represent the treasure, bech32 encoded
    pub treasure_public_key: String,
    /// An image hash, base64 encoded
    pub treasure_hash: String,
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

#[derive(Debug)]
#[derive(BorshSerialize, BorshDeserialize)]
#[derive(Serialize, Deserialize)]
#[derive(Hash, Eq, PartialEq)]
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
