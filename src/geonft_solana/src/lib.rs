use borsh::{BorshDeserialize, BorshSerialize};
use geonft_nostd::crypto;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::collections::HashMap;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    geonft_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("Geonft_solana entrypoint.");

    let geonft_data = GeonftRequest::try_from_slice(geonft_data).unwrap(); // convert ? to Solana Result

    match geonft_data {
        GeonftRequest::PlantTreasure(plant_info) => {
            msg!("plant info: {:?}", &plant_info);
            Ok(plant_treasure_with_key(&account, plant_info)?)
        }
        GeonftRequest::ClaimTreasure(claim_info) => {
            msg!("claim info: {:?}", &claim_info);
            Ok(())
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum GeonftRequest {
    PlantTreasure(PlantRequest),
    ClaimTreasure(ClaimRequest),
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
struct Treasure {
    plant_treasure: HashMap<String, PlantRequest>,
    claim_treasure: HashMap<String, ClaimRequest>,
}

#[derive(Hash, Eq, PartialEq, BorshSerialize, BorshDeserialize, Debug)]
pub struct PlantRequest {
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ClaimRequest {
    /// The public key of the claiming account, bech32 encoded
    account_public_key: String,
    /// The public key of the treasure, bech32 encoded
    treasure_public_key: String,
    /// A base64-encoded signature by the account key of
    /// the string "claim",
    /// appended by the encoded treasure public key,
    account_signature: String,
    /// A base64-encoded signature by the treasure key of
    /// the string "claim",
    /// appended by the encoded account public key.
    treasure_signature: String,
}

pub fn plant_treasure_with_key(
    account: &AccountInfo,
    plant_info: PlantRequest,
) -> Result<(), GeonftError> {
    let treasure_pubkey_decode =
        crypto::decode_treasure_public_key(&plant_info.treasure_public_key)?;
    let treasure_pubkey_encode = crypto::encode_treasure_public_key(&treasure_pubkey_decode)?;

    let account_pubkey_decode = crypto::decode_account_public_key(&plant_info.account_public_key)?;

    let treasure_signature = crypto::decode_signature(&plant_info.treasure_signature)?;
    let account_signature = crypto::decode_signature(&plant_info.account_signature)?;

    let treasure_hash = &plant_info.treasure_hash;

    crypto::verify_plant_request_for_treasure(
        treasure_pubkey_decode,
        account_pubkey_decode,
        treasure_hash.as_bytes(),
        treasure_signature,
    )?;

    crypto::verify_plant_request_for_account(
        account_pubkey_decode,
        treasure_pubkey_decode,
        account_signature,
    )?;

    let mut treasure_data = Treasure::try_from_slice(&account.data.borrow())?;
    treasure_data
        .plant_treasure
        .insert(treasure_pubkey_encode, plant_info);

    Ok(treasure_data.serialize(&mut &mut account.data.borrow_mut()[..])?)
}

pub enum GeonftError {
    SolanaError(ProgramError),
    AnyhowError(anyhow::Error),
    IOError(std::io::Error),
}

impl From<anyhow::Error> for GeonftError {
    fn from(e: anyhow::Error) -> Self {
        GeonftError::AnyhowError(e)
    }
}

impl From<std::io::Error> for GeonftError {
    fn from(e: std::io::Error) -> Self {
        GeonftError::IOError(e)
    }
}

impl From<GeonftError> for ProgramError {
    fn from(e: GeonftError) -> Self {
        match e {
            GeonftError::SolanaError(e) => e,
            GeonftError::AnyhowError(e) => {
                msg!("{}", e);
                ProgramError::Custom(0)
            }
            GeonftError::IOError(e) => {
                msg!("{}", e);
                ProgramError::Custom(1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
