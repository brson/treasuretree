use anyhow::anyhow;
use borsh::{BorshDeserialize, BorshSerialize};
use geonft_data::{ClaimRequestSolana, GeonftRequestSolana, PlantRequestSolana};
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
        msg!("Geonft account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("Geonft_solana entrypoint.");

    let geonft_data = GeonftRequestSolana::try_from_slice(geonft_data)?;
    match geonft_data {
        GeonftRequestSolana::PlantTreasure(plant_info) => {
            Ok(plant_treasure_with_key(&account, plant_info)?)
        }
        GeonftRequestSolana::ClaimTreasure(claim_info) => {
            Ok(claim_treasure_with_key(&account, claim_info)?)
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
struct Treasure {
    plant_treasure: HashMap<Vec<u8>, PlantRequestSolana>,
    claim_treasure: HashMap<Vec<u8>, ClaimRequestSolana>,
}

pub fn plant_treasure_with_key(
    account: &AccountInfo,
    plant_info: PlantRequestSolana,
) -> Result<(), GeonftError> {
    msg!("plant_treasure_with_key");
/*
    let mut treasure_data = Treasure::try_from_slice(&account.data.borrow())?;
    treasure_data
        .plant_treasure
        .insert(
            plant_info.treasure_public_key.to_vec(),
            plant_info
        );
    
    Ok(treasure_data.serialize(&mut &mut account.data.borrow_mut()[..])?)
     */
    Ok(())
}

pub fn claim_treasure_with_key(
    account: &AccountInfo,
    claim_info: ClaimRequestSolana,
) -> Result<(), GeonftError> {
    msg!("claim_treasure_with_key");

    let treasure_pubkey = &claim_info.treasure_public_key;
    msg!("get treasure_data");
    msg!("account.data: {:?}", &account.data.borrow()[..30]);
    
    let treasure_data = Treasure::try_from_slice(&account.data.borrow());
    msg!("treasure_data result: {:?}", &treasure_data);
/*    if !treasure_data
        .plant_treasure
        .contains_key(treasure_pubkey)
    {
        Err(GeonftError::AnyhowError(anyhow!("Treasure doesn't exist")))
    } else { 
        treasure_data
            .claim_treasure
            .insert(
                treasure_pubkey.to_vec(),
                claim_info
);*/
    msg!("before serialize");
    
//    treasure_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
//    msg!("before ok");
    Ok(())
//    }
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
