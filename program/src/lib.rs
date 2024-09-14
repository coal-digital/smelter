mod claim;
mod close;
mod initialize;
mod smelt;
mod open;
mod reset;
mod stake;
mod update;

use claim::*;
use close::*;
use initialize::*;
use smelt::*;
use open::*;
use reset::*;
use stake::*;
use update::*;

use smelter_api::instruction::*;
use solana_program::{
    self, account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&smelter_api::id()) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (tag, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match SmelterInstruction::try_from(*tag).or(Err(ProgramError::InvalidInstructionData))? {
        SmelterInstruction::Claim => process_claim(accounts, data)?,
        SmelterInstruction::Close => process_close(accounts, data)?,
        SmelterInstruction::Smelt => process_smelt(accounts, data)?,
        SmelterInstruction::Open => process_open(accounts, data)?,
        SmelterInstruction::Reset => process_reset(accounts, data)?,
        SmelterInstruction::Stake => process_stake(accounts, data)?,
        SmelterInstruction::Update => process_update(accounts, data)?,
        SmelterInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}
