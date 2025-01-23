//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
#[derive(Debug)]
pub struct ClosePresetParameter {
    pub preset_parameter: solana_program::pubkey::Pubkey,

    pub admin: solana_program::pubkey::Pubkey,

    pub rent_receiver: solana_program::pubkey::Pubkey,
}

impl ClosePresetParameter {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.preset_parameter,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.admin, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.rent_receiver,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = ClosePresetParameterInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClosePresetParameterInstructionData {
    discriminator: [u8; 8],
}

impl ClosePresetParameterInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [4, 148, 145, 100, 134, 26, 181, 61],
        }
    }
}

impl Default for ClosePresetParameterInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `ClosePresetParameter`.
///
/// ### Accounts:
///
///   0. `[writable]` preset_parameter
///   1. `[writable, signer]` admin
///   2. `[writable]` rent_receiver
#[derive(Clone, Debug, Default)]
pub struct ClosePresetParameterBuilder {
    preset_parameter: Option<solana_program::pubkey::Pubkey>,
    admin: Option<solana_program::pubkey::Pubkey>,
    rent_receiver: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ClosePresetParameterBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn preset_parameter(
        &mut self,
        preset_parameter: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.preset_parameter = Some(preset_parameter);
        self
    }
    #[inline(always)]
    pub fn admin(&mut self, admin: solana_program::pubkey::Pubkey) -> &mut Self {
        self.admin = Some(admin);
        self
    }
    #[inline(always)]
    pub fn rent_receiver(&mut self, rent_receiver: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent_receiver = Some(rent_receiver);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = ClosePresetParameter {
            preset_parameter: self.preset_parameter.expect("preset_parameter is not set"),
            admin: self.admin.expect("admin is not set"),
            rent_receiver: self.rent_receiver.expect("rent_receiver is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `close_preset_parameter` CPI accounts.
pub struct ClosePresetParameterCpiAccounts<'a, 'b> {
    pub preset_parameter: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent_receiver: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `close_preset_parameter` CPI instruction.
pub struct ClosePresetParameterCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub preset_parameter: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent_receiver: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> ClosePresetParameterCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ClosePresetParameterCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            preset_parameter: accounts.preset_parameter,
            admin: accounts.admin,
            rent_receiver: accounts.rent_receiver,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.preset_parameter.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.admin.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.rent_receiver.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = ClosePresetParameterInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(4 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.preset_parameter.clone());
        account_infos.push(self.admin.clone());
        account_infos.push(self.rent_receiver.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `ClosePresetParameter` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` preset_parameter
///   1. `[writable, signer]` admin
///   2. `[writable]` rent_receiver
#[derive(Clone, Debug)]
pub struct ClosePresetParameterCpiBuilder<'a, 'b> {
    instruction: Box<ClosePresetParameterCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ClosePresetParameterCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ClosePresetParameterCpiBuilderInstruction {
            __program: program,
            preset_parameter: None,
            admin: None,
            rent_receiver: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn preset_parameter(
        &mut self,
        preset_parameter: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.preset_parameter = Some(preset_parameter);
        self
    }
    #[inline(always)]
    pub fn admin(&mut self, admin: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.admin = Some(admin);
        self
    }
    #[inline(always)]
    pub fn rent_receiver(
        &mut self,
        rent_receiver: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.rent_receiver = Some(rent_receiver);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = ClosePresetParameterCpi {
            __program: self.instruction.__program,

            preset_parameter: self
                .instruction
                .preset_parameter
                .expect("preset_parameter is not set"),

            admin: self.instruction.admin.expect("admin is not set"),

            rent_receiver: self
                .instruction
                .rent_receiver
                .expect("rent_receiver is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct ClosePresetParameterCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    preset_parameter: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    admin: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent_receiver: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
