//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use codama_renderers_rust_e2e_dummy::instructions::{
    Instruction1 as Instruction1IxAccounts, Instruction2 as Instruction2IxAccounts,
    Instruction3 as Instruction3IxAccounts, Instruction4 as Instruction4IxAccounts,
    Instruction4InstructionArgs as Instruction4IxData, Instruction5 as Instruction5IxAccounts,
    Instruction5InstructionArgs as Instruction5IxData, Instruction6 as Instruction6IxAccounts,
    Instruction7 as Instruction7IxAccounts,
};
use codama_renderers_rust_e2e_dummy::ID;

/// Dummy Instructions
#[derive(Debug)]
pub enum DummyProgramIx {
    Instruction1(Instruction1IxAccounts),
    Instruction2(Instruction2IxAccounts),
    Instruction3(Instruction3IxAccounts),
    Instruction4(Instruction4IxAccounts, Instruction4IxData),
    Instruction5(Instruction5IxAccounts, Instruction5IxData),
    Instruction6(Instruction6IxAccounts),
    Instruction7(Instruction7IxAccounts),
}

#[derive(Debug, Copy, Clone)]
pub struct InstructionParser;

impl yellowstone_vixen_core::Parser for InstructionParser {
    type Input = yellowstone_vixen_core::instruction::InstructionUpdate;
    type Output = DummyProgramIx;

    fn id(&self) -> std::borrow::Cow<str> {
        "Dummy::InstructionParser".into()
    }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        yellowstone_vixen_core::Prefilter::builder()
            .transaction_accounts([ID])
            .build()
            .unwrap()
    }

    async fn parse(
        &self,
        ix_update: &yellowstone_vixen_core::instruction::InstructionUpdate,
    ) -> yellowstone_vixen_core::ParseResult<Self::Output> {
        if ix_update.program.equals_ref(ID) {
            InstructionParser::parse_impl(ix_update)
        } else {
            Err(yellowstone_vixen_core::ParseError::Filtered)
        }
    }
}

impl yellowstone_vixen_core::ProgramParser for InstructionParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        ID.to_bytes().into()
    }
}

impl InstructionParser {
    pub(crate) fn parse_impl(
        ix: &yellowstone_vixen_core::instruction::InstructionUpdate,
    ) -> yellowstone_vixen_core::ParseResult<DummyProgramIx> {
        let accounts_len = ix.accounts.len();
        let ix_discriminator: [u8; 1] = ix.data[0..1].try_into()?;
        let mut ix_data = &ix.data[1..];
        match ix_discriminator {
            [42] => {
                check_min_accounts_req(accounts_len, 0)?;
                let ix_accounts = Instruction3IxAccounts {};
                Ok(DummyProgramIx::Instruction3(ix_accounts))
            }
            _ => Err(yellowstone_vixen_core::ParseError::from(
                "Invalid Instruction discriminator".to_owned(),
            )),
        }
    }
}

pub fn check_min_accounts_req(
    actual: usize,
    expected: usize,
) -> yellowstone_vixen_core::ParseResult<()> {
    if actual < expected {
        Err(yellowstone_vixen_core::ParseError::from(format!(
            "Too few accounts provided: expected {expected}, got {actual}"
        )))
    } else {
        Ok(())
    }
}