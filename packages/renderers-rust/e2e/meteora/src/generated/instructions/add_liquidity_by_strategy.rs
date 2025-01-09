//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::generated::types::StrategyParameters;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
#[derive(Debug)]
pub struct AddLiquidityByStrategy {
    pub position: solana_program::pubkey::Pubkey,

    pub lb_pair: solana_program::pubkey::Pubkey,

    pub bin_array_bitmap_extension: Option<solana_program::pubkey::Pubkey>,

    pub user_token_x: solana_program::pubkey::Pubkey,

    pub user_token_y: solana_program::pubkey::Pubkey,

    pub reserve_x: solana_program::pubkey::Pubkey,

    pub reserve_y: solana_program::pubkey::Pubkey,

    pub token_x_mint: solana_program::pubkey::Pubkey,

    pub token_y_mint: solana_program::pubkey::Pubkey,

    pub bin_array_lower: solana_program::pubkey::Pubkey,

    pub bin_array_upper: solana_program::pubkey::Pubkey,

    pub sender: solana_program::pubkey::Pubkey,

    pub token_x_program: solana_program::pubkey::Pubkey,

    pub token_y_program: solana_program::pubkey::Pubkey,

    pub event_authority: solana_program::pubkey::Pubkey,

    pub program: solana_program::pubkey::Pubkey,
}

impl AddLiquidityByStrategy {
    pub fn instruction(
        &self,
        args: AddLiquidityByStrategyInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: AddLiquidityByStrategyInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(16 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.position,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.lb_pair,
            false,
        ));
        if let Some(bin_array_bitmap_extension) = self.bin_array_bitmap_extension {
            accounts.push(solana_program::instruction::AccountMeta::new(
                bin_array_bitmap_extension,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LB_CLMM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_token_x,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_token_y,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.reserve_x,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.reserve_y,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_x_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_y_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.bin_array_lower,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.bin_array_upper,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.sender,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_x_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_y_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.event_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = AddLiquidityByStrategyInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AddLiquidityByStrategyInstructionData {
    discriminator: [u8; 8],
}

impl AddLiquidityByStrategyInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [7, 3, 150, 127, 148, 40, 61, 200],
        }
    }
}

impl Default for AddLiquidityByStrategyInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidityByStrategyInstructionArgs {
    pub amount_x: u64,
    pub amount_y: u64,
    pub active_id: i32,
    pub max_active_bin_slippage: i32,
    pub strategy_parameters: StrategyParameters,
}

/// Instruction builder for `AddLiquidityByStrategy`.
///
/// ### Accounts:
///
///   0. `[writable]` position
///   1. `[writable]` lb_pair
///   2. `[writable, optional]` bin_array_bitmap_extension
///   3. `[writable]` user_token_x
///   4. `[writable]` user_token_y
///   5. `[writable]` reserve_x
///   6. `[writable]` reserve_y
///   7. `[]` token_x_mint
///   8. `[]` token_y_mint
///   9. `[writable]` bin_array_lower
///   10. `[writable]` bin_array_upper
///   11. `[signer]` sender
///   12. `[]` token_x_program
///   13. `[]` token_y_program
///   14. `[]` event_authority
///   15. `[]` program
#[derive(Clone, Debug, Default)]
pub struct AddLiquidityByStrategyBuilder {
    position: Option<solana_program::pubkey::Pubkey>,
    lb_pair: Option<solana_program::pubkey::Pubkey>,
    bin_array_bitmap_extension: Option<solana_program::pubkey::Pubkey>,
    user_token_x: Option<solana_program::pubkey::Pubkey>,
    user_token_y: Option<solana_program::pubkey::Pubkey>,
    reserve_x: Option<solana_program::pubkey::Pubkey>,
    reserve_y: Option<solana_program::pubkey::Pubkey>,
    token_x_mint: Option<solana_program::pubkey::Pubkey>,
    token_y_mint: Option<solana_program::pubkey::Pubkey>,
    bin_array_lower: Option<solana_program::pubkey::Pubkey>,
    bin_array_upper: Option<solana_program::pubkey::Pubkey>,
    sender: Option<solana_program::pubkey::Pubkey>,
    token_x_program: Option<solana_program::pubkey::Pubkey>,
    token_y_program: Option<solana_program::pubkey::Pubkey>,
    event_authority: Option<solana_program::pubkey::Pubkey>,
    program: Option<solana_program::pubkey::Pubkey>,
    amount_x: Option<u64>,
    amount_y: Option<u64>,
    active_id: Option<i32>,
    max_active_bin_slippage: Option<i32>,
    strategy_parameters: Option<StrategyParameters>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl AddLiquidityByStrategyBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn position(&mut self, position: solana_program::pubkey::Pubkey) -> &mut Self {
        self.position = Some(position);
        self
    }
    #[inline(always)]
    pub fn lb_pair(&mut self, lb_pair: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lb_pair = Some(lb_pair);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn bin_array_bitmap_extension(
        &mut self,
        bin_array_bitmap_extension: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.bin_array_bitmap_extension = bin_array_bitmap_extension;
        self
    }
    #[inline(always)]
    pub fn user_token_x(&mut self, user_token_x: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_token_x = Some(user_token_x);
        self
    }
    #[inline(always)]
    pub fn user_token_y(&mut self, user_token_y: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_token_y = Some(user_token_y);
        self
    }
    #[inline(always)]
    pub fn reserve_x(&mut self, reserve_x: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reserve_x = Some(reserve_x);
        self
    }
    #[inline(always)]
    pub fn reserve_y(&mut self, reserve_y: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reserve_y = Some(reserve_y);
        self
    }
    #[inline(always)]
    pub fn token_x_mint(&mut self, token_x_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_x_mint = Some(token_x_mint);
        self
    }
    #[inline(always)]
    pub fn token_y_mint(&mut self, token_y_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_y_mint = Some(token_y_mint);
        self
    }
    #[inline(always)]
    pub fn bin_array_lower(
        &mut self,
        bin_array_lower: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.bin_array_lower = Some(bin_array_lower);
        self
    }
    #[inline(always)]
    pub fn bin_array_upper(
        &mut self,
        bin_array_upper: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.bin_array_upper = Some(bin_array_upper);
        self
    }
    #[inline(always)]
    pub fn sender(&mut self, sender: solana_program::pubkey::Pubkey) -> &mut Self {
        self.sender = Some(sender);
        self
    }
    #[inline(always)]
    pub fn token_x_program(
        &mut self,
        token_x_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_x_program = Some(token_x_program);
        self
    }
    #[inline(always)]
    pub fn token_y_program(
        &mut self,
        token_y_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_y_program = Some(token_y_program);
        self
    }
    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.event_authority = Some(event_authority);
        self
    }
    #[inline(always)]
    pub fn program(&mut self, program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.program = Some(program);
        self
    }
    #[inline(always)]
    pub fn amount_x(&mut self, amount_x: u64) -> &mut Self {
        self.amount_x = Some(amount_x);
        self
    }
    #[inline(always)]
    pub fn amount_y(&mut self, amount_y: u64) -> &mut Self {
        self.amount_y = Some(amount_y);
        self
    }
    #[inline(always)]
    pub fn active_id(&mut self, active_id: i32) -> &mut Self {
        self.active_id = Some(active_id);
        self
    }
    #[inline(always)]
    pub fn max_active_bin_slippage(&mut self, max_active_bin_slippage: i32) -> &mut Self {
        self.max_active_bin_slippage = Some(max_active_bin_slippage);
        self
    }
    #[inline(always)]
    pub fn strategy_parameters(&mut self, strategy_parameters: StrategyParameters) -> &mut Self {
        self.strategy_parameters = Some(strategy_parameters);
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
        let accounts = AddLiquidityByStrategy {
            position: self.position.expect("position is not set"),
            lb_pair: self.lb_pair.expect("lb_pair is not set"),
            bin_array_bitmap_extension: self.bin_array_bitmap_extension,
            user_token_x: self.user_token_x.expect("user_token_x is not set"),
            user_token_y: self.user_token_y.expect("user_token_y is not set"),
            reserve_x: self.reserve_x.expect("reserve_x is not set"),
            reserve_y: self.reserve_y.expect("reserve_y is not set"),
            token_x_mint: self.token_x_mint.expect("token_x_mint is not set"),
            token_y_mint: self.token_y_mint.expect("token_y_mint is not set"),
            bin_array_lower: self.bin_array_lower.expect("bin_array_lower is not set"),
            bin_array_upper: self.bin_array_upper.expect("bin_array_upper is not set"),
            sender: self.sender.expect("sender is not set"),
            token_x_program: self.token_x_program.expect("token_x_program is not set"),
            token_y_program: self.token_y_program.expect("token_y_program is not set"),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = AddLiquidityByStrategyInstructionArgs {
            amount_x: self.amount_x.clone().expect("amount_x is not set"),
            amount_y: self.amount_y.clone().expect("amount_y is not set"),
            active_id: self.active_id.clone().expect("active_id is not set"),
            max_active_bin_slippage: self
                .max_active_bin_slippage
                .clone()
                .expect("max_active_bin_slippage is not set"),
            strategy_parameters: self
                .strategy_parameters
                .clone()
                .expect("strategy_parameters is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `add_liquidity_by_strategy` CPI accounts.
pub struct AddLiquidityByStrategyCpiAccounts<'a, 'b> {
    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub lb_pair: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub user_token_x: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_y: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_x: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_y: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_x_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_y_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array_lower: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array_upper: &'b solana_program::account_info::AccountInfo<'a>,

    pub sender: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_x_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_y_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `add_liquidity_by_strategy` CPI instruction.
pub struct AddLiquidityByStrategyCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub lb_pair: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub user_token_x: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_y: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_x: &'b solana_program::account_info::AccountInfo<'a>,

    pub reserve_y: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_x_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_y_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array_lower: &'b solana_program::account_info::AccountInfo<'a>,

    pub bin_array_upper: &'b solana_program::account_info::AccountInfo<'a>,

    pub sender: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_x_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_y_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: AddLiquidityByStrategyInstructionArgs,
}

impl<'a, 'b> AddLiquidityByStrategyCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: AddLiquidityByStrategyCpiAccounts<'a, 'b>,
        args: AddLiquidityByStrategyInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            position: accounts.position,
            lb_pair: accounts.lb_pair,
            bin_array_bitmap_extension: accounts.bin_array_bitmap_extension,
            user_token_x: accounts.user_token_x,
            user_token_y: accounts.user_token_y,
            reserve_x: accounts.reserve_x,
            reserve_y: accounts.reserve_y,
            token_x_mint: accounts.token_x_mint,
            token_y_mint: accounts.token_y_mint,
            bin_array_lower: accounts.bin_array_lower,
            bin_array_upper: accounts.bin_array_upper,
            sender: accounts.sender,
            token_x_program: accounts.token_x_program,
            token_y_program: accounts.token_y_program,
            event_authority: accounts.event_authority,
            program: accounts.program,
            __args: args,
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
        let mut accounts = Vec::with_capacity(16 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lb_pair.key,
            false,
        ));
        if let Some(bin_array_bitmap_extension) = self.bin_array_bitmap_extension {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *bin_array_bitmap_extension.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::LB_CLMM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_token_x.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_token_y.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reserve_x.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reserve_y.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_x_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_y_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.bin_array_lower.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.bin_array_upper.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.sender.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_x_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_y_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.event_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = AddLiquidityByStrategyInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::LB_CLMM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(17 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.position.clone());
        account_infos.push(self.lb_pair.clone());
        if let Some(bin_array_bitmap_extension) = self.bin_array_bitmap_extension {
            account_infos.push(bin_array_bitmap_extension.clone());
        }
        account_infos.push(self.user_token_x.clone());
        account_infos.push(self.user_token_y.clone());
        account_infos.push(self.reserve_x.clone());
        account_infos.push(self.reserve_y.clone());
        account_infos.push(self.token_x_mint.clone());
        account_infos.push(self.token_y_mint.clone());
        account_infos.push(self.bin_array_lower.clone());
        account_infos.push(self.bin_array_upper.clone());
        account_infos.push(self.sender.clone());
        account_infos.push(self.token_x_program.clone());
        account_infos.push(self.token_y_program.clone());
        account_infos.push(self.event_authority.clone());
        account_infos.push(self.program.clone());
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

/// Instruction builder for `AddLiquidityByStrategy` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` position
///   1. `[writable]` lb_pair
///   2. `[writable, optional]` bin_array_bitmap_extension
///   3. `[writable]` user_token_x
///   4. `[writable]` user_token_y
///   5. `[writable]` reserve_x
///   6. `[writable]` reserve_y
///   7. `[]` token_x_mint
///   8. `[]` token_y_mint
///   9. `[writable]` bin_array_lower
///   10. `[writable]` bin_array_upper
///   11. `[signer]` sender
///   12. `[]` token_x_program
///   13. `[]` token_y_program
///   14. `[]` event_authority
///   15. `[]` program
#[derive(Clone, Debug)]
pub struct AddLiquidityByStrategyCpiBuilder<'a, 'b> {
    instruction: Box<AddLiquidityByStrategyCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> AddLiquidityByStrategyCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(AddLiquidityByStrategyCpiBuilderInstruction {
            __program: program,
            position: None,
            lb_pair: None,
            bin_array_bitmap_extension: None,
            user_token_x: None,
            user_token_y: None,
            reserve_x: None,
            reserve_y: None,
            token_x_mint: None,
            token_y_mint: None,
            bin_array_lower: None,
            bin_array_upper: None,
            sender: None,
            token_x_program: None,
            token_y_program: None,
            event_authority: None,
            program: None,
            amount_x: None,
            amount_y: None,
            active_id: None,
            max_active_bin_slippage: None,
            strategy_parameters: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn position(
        &mut self,
        position: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position = Some(position);
        self
    }
    #[inline(always)]
    pub fn lb_pair(
        &mut self,
        lb_pair: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lb_pair = Some(lb_pair);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn bin_array_bitmap_extension(
        &mut self,
        bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.bin_array_bitmap_extension = bin_array_bitmap_extension;
        self
    }
    #[inline(always)]
    pub fn user_token_x(
        &mut self,
        user_token_x: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_token_x = Some(user_token_x);
        self
    }
    #[inline(always)]
    pub fn user_token_y(
        &mut self,
        user_token_y: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_token_y = Some(user_token_y);
        self
    }
    #[inline(always)]
    pub fn reserve_x(
        &mut self,
        reserve_x: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reserve_x = Some(reserve_x);
        self
    }
    #[inline(always)]
    pub fn reserve_y(
        &mut self,
        reserve_y: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reserve_y = Some(reserve_y);
        self
    }
    #[inline(always)]
    pub fn token_x_mint(
        &mut self,
        token_x_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_x_mint = Some(token_x_mint);
        self
    }
    #[inline(always)]
    pub fn token_y_mint(
        &mut self,
        token_y_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_y_mint = Some(token_y_mint);
        self
    }
    #[inline(always)]
    pub fn bin_array_lower(
        &mut self,
        bin_array_lower: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.bin_array_lower = Some(bin_array_lower);
        self
    }
    #[inline(always)]
    pub fn bin_array_upper(
        &mut self,
        bin_array_upper: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.bin_array_upper = Some(bin_array_upper);
        self
    }
    #[inline(always)]
    pub fn sender(
        &mut self,
        sender: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.sender = Some(sender);
        self
    }
    #[inline(always)]
    pub fn token_x_program(
        &mut self,
        token_x_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_x_program = Some(token_x_program);
        self
    }
    #[inline(always)]
    pub fn token_y_program(
        &mut self,
        token_y_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_y_program = Some(token_y_program);
        self
    }
    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.event_authority = Some(event_authority);
        self
    }
    #[inline(always)]
    pub fn program(
        &mut self,
        program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program = Some(program);
        self
    }
    #[inline(always)]
    pub fn amount_x(&mut self, amount_x: u64) -> &mut Self {
        self.instruction.amount_x = Some(amount_x);
        self
    }
    #[inline(always)]
    pub fn amount_y(&mut self, amount_y: u64) -> &mut Self {
        self.instruction.amount_y = Some(amount_y);
        self
    }
    #[inline(always)]
    pub fn active_id(&mut self, active_id: i32) -> &mut Self {
        self.instruction.active_id = Some(active_id);
        self
    }
    #[inline(always)]
    pub fn max_active_bin_slippage(&mut self, max_active_bin_slippage: i32) -> &mut Self {
        self.instruction.max_active_bin_slippage = Some(max_active_bin_slippage);
        self
    }
    #[inline(always)]
    pub fn strategy_parameters(&mut self, strategy_parameters: StrategyParameters) -> &mut Self {
        self.instruction.strategy_parameters = Some(strategy_parameters);
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
        let args = AddLiquidityByStrategyInstructionArgs {
            amount_x: self
                .instruction
                .amount_x
                .clone()
                .expect("amount_x is not set"),
            amount_y: self
                .instruction
                .amount_y
                .clone()
                .expect("amount_y is not set"),
            active_id: self
                .instruction
                .active_id
                .clone()
                .expect("active_id is not set"),
            max_active_bin_slippage: self
                .instruction
                .max_active_bin_slippage
                .clone()
                .expect("max_active_bin_slippage is not set"),
            strategy_parameters: self
                .instruction
                .strategy_parameters
                .clone()
                .expect("strategy_parameters is not set"),
        };
        let instruction = AddLiquidityByStrategyCpi {
            __program: self.instruction.__program,

            position: self.instruction.position.expect("position is not set"),

            lb_pair: self.instruction.lb_pair.expect("lb_pair is not set"),

            bin_array_bitmap_extension: self.instruction.bin_array_bitmap_extension,

            user_token_x: self
                .instruction
                .user_token_x
                .expect("user_token_x is not set"),

            user_token_y: self
                .instruction
                .user_token_y
                .expect("user_token_y is not set"),

            reserve_x: self.instruction.reserve_x.expect("reserve_x is not set"),

            reserve_y: self.instruction.reserve_y.expect("reserve_y is not set"),

            token_x_mint: self
                .instruction
                .token_x_mint
                .expect("token_x_mint is not set"),

            token_y_mint: self
                .instruction
                .token_y_mint
                .expect("token_y_mint is not set"),

            bin_array_lower: self
                .instruction
                .bin_array_lower
                .expect("bin_array_lower is not set"),

            bin_array_upper: self
                .instruction
                .bin_array_upper
                .expect("bin_array_upper is not set"),

            sender: self.instruction.sender.expect("sender is not set"),

            token_x_program: self
                .instruction
                .token_x_program
                .expect("token_x_program is not set"),

            token_y_program: self
                .instruction
                .token_y_program
                .expect("token_y_program is not set"),

            event_authority: self
                .instruction
                .event_authority
                .expect("event_authority is not set"),

            program: self.instruction.program.expect("program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct AddLiquidityByStrategyCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lb_pair: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bin_array_bitmap_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_token_x: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_token_y: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reserve_x: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reserve_y: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_x_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_y_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bin_array_lower: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bin_array_upper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    sender: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_x_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_y_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    amount_x: Option<u64>,
    amount_y: Option<u64>,
    active_id: Option<i32>,
    max_active_bin_slippage: Option<i32>,
    strategy_parameters: Option<StrategyParameters>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
