use anchor_lang::prelude::*;
mod state;
mod instructions;
mod errors;

use instructions::*;

declare_id!("Aureon1111111111111111111111111111111111");

#[program]
pub mod aureon {
    use super::*;

    pub fn init_reserve(ctx: Context<InitReserve>) -> Result<()> {
        instructions::init_reserve::handler(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }
}
