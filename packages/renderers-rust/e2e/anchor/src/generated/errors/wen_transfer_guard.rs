//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum WenTransferGuardError {
    /// 6000 - Cpi Rule Enforcement Failed
    #[error("Cpi Rule Enforcement Failed")]
    CpiRuleEnforcementFailed = 0x1770,
    /// 6001 - Transfer Amount Rule Enforce Failed
    #[error("Transfer Amount Rule Enforce Failed")]
    TransferAmountRuleEnforceFailed = 0x1771,
    /// 6002 - Metadata Field Does Not Exist
    #[error("Metadata Field Does Not Exist")]
    MetadataFieldDoesNotExist = 0x1772,
    /// 6003 - Metadata Field Does Not Pass
    #[error("Metadata Field Does Not Pass")]
    MetadataFieldDoesNotPass = 0x1773,
    /// 6004 - Guard token amount should be at least 1
    #[error("Guard token amount should be at least 1")]
    GuardTokenAmountShouldBeAtLeastOne = 0x1774,
    /// 6005 - Not owned by token 2022 program
    #[error("Not owned by token 2022 program")]
    NotOwnedByToken2022Program = 0x1775,
    /// 6006 - Must be initialized by Transfer Hook Authority
    #[error("Must be initialized by Transfer Hook Authority")]
    MustBeInitializedByTransferHookAuthority = 0x1776,
    /// 6007 - Mint's assigned Transfer Hook Program is not this one
    #[error("Mint's assigned Transfer Hook Program is not this one")]
    MintAssignedTransferHookProgramIsNotThisOne = 0x1777,
}

impl solana_program::program_error::PrintProgramError for WenTransferGuardError {
    fn print<E>(&self) {
        solana_program::msg!(&self.to_string());
    }
}