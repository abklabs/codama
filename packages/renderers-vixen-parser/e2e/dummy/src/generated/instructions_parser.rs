//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use dummy_program_sdk::instructions::instruction1::Instruction1 as Instruction1IxAccounts;
use dummy_program_sdk::instructions::instruction2::Instruction2 as Instruction2IxAccounts;
use dummy_program_sdk::instructions::instruction3::{
    Instruction3 as Instruction3IxAccounts, Instruction3InstructionArgs as Instruction3IxData,
};
use dummy_program_sdk::instructions::instruction4::{
    Instruction4 as Instruction4IxAccounts, Instruction4InstructionArgs as Instruction4IxData,
};
use dummy_program_sdk::instructions::instruction5::{
    Instruction5 as Instruction5IxAccounts, Instruction5InstructionArgs as Instruction5IxData,
};
use dummy_program_sdk::instructions::instruction6::Instruction6 as Instruction6IxAccounts;
use dummy_program_sdk::instructions::instruction7::Instruction7 as Instruction7IxAccounts;

/// Dummy Instructions
#[derive(Debug)]
pub enum DummyProgramIx {
    Instruction1(Instruction1IxAccounts, Instruction1IxData),
    Instruction2(Instruction2IxAccounts, Instruction2IxData),
    Instruction3(Instruction3IxAccounts, Instruction3IxData),
    Instruction4(Instruction4IxAccounts, Instruction4IxData),
    Instruction5(Instruction5IxAccounts, Instruction5IxData),
    Instruction6(Instruction6IxAccounts, Instruction6IxData),
    Instruction7(Instruction7IxAccounts, Instruction7IxData),
}

#[derive(Debug, Copy, Clone)]
pub struct InstructionParser;

impl yellowstone_vixen_core::Parser for InstructionParser {
    type Input = yellowstone_vixen_core::InstructionUpdate;
    type Output = DummyProgramIx;

    fn id(&self) -> std::borrow::Cow<str> {
        "Dummy::InstructionParser".into()
    }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        yellowstone_vixen_core::Prefilter::builder()
            .program_ids([crate::DUMMY_ID])
            .build()
            .unwrap()
    }

    async fn parse(
        &self,
        ix_update: &InstructionUpdate,
    ) -> yellowstone_vixen_core::ParseResult<Self::Output> {
        if ix_update.program.equals_ref(crate::DUMMY_ID) {
            InstructionParser::parse_impl(ix_update)
        } else {
            Err(yellowstone_vixen_core::ParseError::Filtered)
        }
    }
}

impl ProgramParser for InstructionParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        crate::DUMMY_ID.to_bytes().into()
    }
}

impl InstructionParser {
    pub(crate) fn parse_impl(
        ix: &InstructionUpdate,
    ) -> yellowstone_vixen_core::ParseResult<DummyProgramIx> {
        let accounts_len = ix.accounts.len();
        let ix_discriminator: [u8; 1] = ix.data[0..1].try_into()?;
        let mut ix_data = &ix.data[1..];

        match ix_discriminator {
            42 => {
                check_min_accounts_req(accounts_len, 0)?;
                let de_ix_data: Instruction3IxData = BorshDeserilaize::deserialize(&mut ix_data)?;
                let ix_accounts = Instruction3IxAccounts {};
                Ok(DummyProgramIx::Instruction3(ix_accounts, de_ix_data))
            }
            _ => Err(yellowstone_vixen_core::ParseError::from(
                "Invalid Instruction discriminator".to_owned(),
            )),
        }
    }
}
