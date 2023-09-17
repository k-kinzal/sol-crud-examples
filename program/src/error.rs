use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};

#[derive(Clone, Debug, Eq, thiserror::Error, FromPrimitive, PartialEq)]
pub enum Error {
    #[error("dummy")]
    Dummy,
}
impl<T> DecodeError<T> for Error {
    fn type_of() -> &'static str {
        "Error"
    }
}

impl PrintProgramError for Error {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            Self::Dummy => msg!("Error"),
        }
    }
}

impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}