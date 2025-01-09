//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::generated::types::Bin;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

/// An account to contain a range of bin. For example: Bin 100 <-> 200.
/// For example:
/// BinArray index: 0 contains bin 0 <-> 599
/// index: 2 contains bin 600 <-> 1199, ...

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinArray {
    pub discriminator: [u8; 8],
    pub index: i64,
    /// Version of binArray
    pub version: u8,
    pub padding: [u8; 7],
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub lb_pair: Pubkey,
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::Bytes>"))]
    pub bins: [Bin; 70],
}

impl BinArray {
    pub const LEN: usize = 10136;

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for BinArray {
    type Error = std::io::Error;

    fn try_from(
        account_info: &solana_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for BinArray {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for BinArray {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for BinArray {
    fn owner() -> Pubkey {
        crate::LB_CLMM_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for BinArray {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for BinArray {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
