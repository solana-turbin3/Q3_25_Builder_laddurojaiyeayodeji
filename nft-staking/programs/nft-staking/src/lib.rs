#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;

use instructions::*;


declare_id!("8t3hctPBzbvrjcSf28CdH9XyDhweCwadXALTW2rNwUKa");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>, points_per_stake: u8, max_stake: u8, points_to_rewards_multiplier: u32, freeze_period: u32) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, points_to_rewards_multiplier, &ctx.bumps)
    }
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }
    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)?;
        ctx.accounts.claim_points()?;
        ctx.accounts.get_available_points()?;
        Ok(())
    }
    pub fn unstake(ctx: Context<Unstake>, points_to_claim: Option<u32>) -> Result<()> {
        ctx.accounts.unstake()?;
        ctx.accounts.claim_rewards(points_to_claim)
    }

}

