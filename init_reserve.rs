use anchor_lang::prelude::*;
use crate::state::ReserveVault;

#[derive(Accounts)]
pub struct InitReserve<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub vault: Account<'info, ReserveVault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitReserve>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.authority = ctx.accounts.authority.key();
    vault.total_reserve = 0;
    Ok(())
}
