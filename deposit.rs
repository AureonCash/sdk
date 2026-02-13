use anchor_lang::prelude::*;
use crate::state::ReserveVault;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, ReserveVault>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.total_reserve += amount;
    Ok(())
}
