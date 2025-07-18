#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::prelude::*;

declare_id!("EM32RusDPns5TWaV7HaBGbQZ8G1qXma3W39VDMud34rE");

pub mod instructions;
use instructions::*;
pub mod state;
// use state::*;


#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    }
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close()
    }
    pub fn refund_and_close(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close()
    }
}


