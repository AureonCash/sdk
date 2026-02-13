use anchor_lang::prelude::*;

#[account]
pub struct ReserveVault {
    pub authority: Pubkey,
    pub total_reserve: u64,
}
