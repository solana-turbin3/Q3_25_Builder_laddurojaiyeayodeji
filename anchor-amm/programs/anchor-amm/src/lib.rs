#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::prelude::*;

declare_id!("3dkfbTVg2ic1DfpZ5qCwLATm6zDiDQ6CELLDoXQBFtnF");

pub mod instructions;
use instructions::*;
pub mod state;
pub mod error;



#[program]
pub mod anchor_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.initialize(seed, fee, authority, &ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)
    }
}
