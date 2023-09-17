use borsh::BorshSerialize;
use program::Instruction as ProgramInstruction;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;

pub fn create_data(
    program_id: Pubkey,
    account: Pubkey,
    data: &[u8],
) -> anyhow::Result<Instruction> {
    Ok(Instruction::new_with_bytes(
        program_id,
        &ProgramInstruction::Create(data.to_vec()).try_to_vec()?,
        vec![AccountMeta::new(account, true)],
    ))
}

pub fn update_data(
    program_id: Pubkey,
    account: Pubkey,
    data: &[u8],
) -> anyhow::Result<Instruction> {
    Ok(Instruction::new_with_bytes(
        program_id,
        &ProgramInstruction::Update(data.to_vec()).try_to_vec()?,
        vec![AccountMeta::new(account, false)],
    ))
}

pub fn delete_data(
    program_id: Pubkey,
    account: Pubkey,
    refund: Pubkey,
) -> anyhow::Result<Instruction> {
    Ok(Instruction::new_with_bytes(
        program_id,
        &ProgramInstruction::Delete.try_to_vec()?,
        vec![
            AccountMeta::new(account, false),
            AccountMeta::new(refund, true),
        ],
    ))
}
