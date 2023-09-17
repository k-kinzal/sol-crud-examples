use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize, BorshSchema, PartialEq)]
pub enum Instruction {
    Create(Vec<u8>),
    Update(Vec<u8>),
    Delete,
}
