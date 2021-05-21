use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use serde::{Deserialize, Serialize};
use serde_json;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    geonft_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Geonft_solana entrypoint.");

    let geonft_data = serde_json::from_slice(geonft_data).unwrap(); // convert ? to Solana Result
    match geonft_data {
        GeonftRequest::PlantTreasure(plant_info) => { msg!("plant info: {:?}", &plant_info); },
        GeonftRequest::ClaimTreasure(claim_info) => { msg!("claim info: {:?}", &claim_info); },
        _ => unreachable!(),
    };

    msg!("Cool. Finished Geonft part.");

    // content below is Solana's helloworld example
    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();
    
    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
enum GeonftRequest {
    PlantTreasure(PlantRequest),
    ClaimTreasure(ClaimRequest),
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

pub fn plant_treasure_with_key(plant_info: PlantRequest) -> Result<(), ProgramError> {
    /*
    let treasure_key_decode = crypto::decode_treasure_public_key(&plant_info.treasure_public_key)?;
    let treasure_key_encode = crypto::encode_treasure_public_key(&treasure_key_decode)?;

    let account_key_decode = crypto::decode_account_public_key(&plant_info.account_public_key)?;

    let treasure_signature = crypto::decode_signature(&plant_info.treasure_signature)?;
    let account_signature = crypto::decode_signature(&plant_info.account_signature)?;

    // todo check the treasure doesn't exist
    // todo validate image type

    // todo: get_hash from decoded_image
    let treasure_hash = crypto::get_hash(&plant_info.image)?;

    crypto::verify_plant_request_for_treasure(
        treasure_key_decode,
        account_key_decode,
        treasure_hash.as_bytes(),
        treasure_signature,
    )?;

    crypto::verify_plant_request_for_account(
        account_key_decode,
        treasure_key_decode,
        account_signature,
    )?;

    let filename = format!("{}/{key}", data::PLANT_DIR, key = treasure_key_encode);
    fs::create_dir_all(data::PLANT_DIR)?;

    let mut file = File::create(filename)?;
    serde_json::to_writer(file, &plant_info.0)?;
*/
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
