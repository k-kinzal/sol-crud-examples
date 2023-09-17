use crate::instruction::Instruction;
use borsh::BorshDeserialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

fn process_create(program_id: &Pubkey, accounts: &[AccountInfo], bytes: &[u8]) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let state = next_account_info(account_iter)?;

    if state.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    if !state.is_signer {
        msg!("Account is not signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let data = &mut state.data.borrow_mut();
    if data.len() != bytes.len() {
        msg!("Account data is not the correct length");
        return Err(ProgramError::InvalidAccountData);
    }
    data.copy_from_slice(bytes);

    Ok(())
}

fn process_update(program_id: &Pubkey, accounts: &[AccountInfo], bytes: &[u8]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let state = next_account_info(account_info_iter)?;

    if state.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let data = &mut state.data.borrow_mut();
    if data.len() != bytes.len() {
        msg!("Account data is not the correct length");
        return Err(ProgramError::InvalidAccountData);
    }
    data.copy_from_slice(bytes);

    Ok(())
}

fn process_delete(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let state = next_account_info(accounts_iter)?;
    let refund = next_account_info(accounts_iter)?;

    if state.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let data = &mut state.data.borrow_mut();
    data.fill(0);

    let source_amount: &mut u64 = &mut state.lamports.borrow_mut();
    let dest_amount: &mut u64 = &mut refund.lamports.borrow_mut();
    *dest_amount = dest_amount.saturating_add(*source_amount);
    *source_amount = 0;

    Ok(())
}

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    msg!("input: {:?}", input);
    let instruction = Instruction::deserialize(&mut &input[..])?;

    match instruction {
        Instruction::Create(bytes) => process_create(program_id, accounts, &bytes),
        Instruction::Update(bytes) => process_update(program_id, accounts, &bytes),
        Instruction::Delete => process_delete(program_id, accounts),
    }
}
