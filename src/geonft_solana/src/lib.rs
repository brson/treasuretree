use anyhow::anyhow;
use borsh::{BorshDeserialize, BorshSerialize};
use geonft_request::{ClaimRequestSolana, GeonftRequestSolana, PlantRequestSolana};
use solana_program::borsh::try_from_slice_unchecked;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::collections::BTreeMap;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    geonft_request: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Geonft account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("Geonft_solana entrypoint");

    if account.data.borrow()[0] == 0 {
        msg!("init starts");
        let init_treasure = Treasure {
            plant_treasure: BTreeMap::new(),
            claim_treasure: BTreeMap::new(),
        };

        init_treasure.serialize(&mut &mut account.data.borrow_mut()[1..])?;
        account.data.borrow_mut()[0] = 1;
    }

    let mut treasure_data = try_from_slice_unchecked(&account.data.borrow()[1..])?;

    let geonft_request = GeonftRequestSolana::try_from_slice(geonft_request)?;
    match geonft_request {
        GeonftRequestSolana::PlantTreasure(plant_info) => {
            plant_treasure(plant_info, &mut treasure_data)?;
        }
        GeonftRequestSolana::ClaimTreasure(claim_info) => {
            claim_treasure(claim_info, &mut treasure_data)?;
        }
    }

    Ok(treasure_data.serialize(&mut &mut account.data.borrow_mut()[1..])?)
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Treasure {
    plant_treasure: BTreeMap<Vec<u8>, PlantTreasure>,
    claim_treasure: BTreeMap<Vec<u8>, ClaimTreasure>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PlantTreasure {
    account_pubkey: Vec<u8>,
    treasure_hash: Vec<u8>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct ClaimTreasure {
    account_pubkey: Vec<u8>,
}

pub fn plant_treasure(
    plant_info: PlantRequestSolana,
    treasure_data: &mut Treasure,
) -> Result<(), GeonftError> {
    msg!("plant_treasure");

    treasure_data.plant_treasure.insert(
        plant_info.treasure_public_key.to_vec(),
        PlantTreasure {
            account_pubkey: plant_info.account_public_key,
            treasure_hash: plant_info.treasure_hash,
        },
    );

    Ok(())
}

pub fn claim_treasure(
    claim_info: ClaimRequestSolana,
    treasure_data: &mut Treasure,
) -> Result<(), GeonftError> {
    msg!("claim_treasure");

    let treasure_pubkey = &claim_info.treasure_public_key;

    if !treasure_data.plant_treasure.contains_key(treasure_pubkey) {
        Err(GeonftError::AnyhowError(anyhow!("Treasure doesn't exist")))
    } else {
        treasure_data.claim_treasure.insert(
            treasure_pubkey.to_vec(),
            ClaimTreasure {
                account_pubkey: claim_info.account_public_key,
            },
        );

        Ok(())
    }
}

pub enum GeonftError {
    SolanaError(ProgramError),
    AnyhowError(anyhow::Error),
    IoError(std::io::Error),
}

impl From<anyhow::Error> for GeonftError {
    fn from(e: anyhow::Error) -> Self {
        GeonftError::AnyhowError(e)
    }
}

impl From<std::io::Error> for GeonftError {
    fn from(e: std::io::Error) -> Self {
        GeonftError::IoError(e)
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
            GeonftError::IoError(e) => {
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
