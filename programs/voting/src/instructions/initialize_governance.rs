use anchor_lang::prelude::*;
use crate::state::GovernanceConfig;
use crate::constants::*;
use crate::error::VottingError;


#[derive(Accounts)]
pub struct InitializeGovernance<'info>{
    #[account(
        init,
        payer= admin,
        space = GovernanceConfig::LEN,
        seeds = [GOVERNANCE_SEED],
        bump
    )]
    pub config : Account<'info, GovernanceConfig>,
    #[account(mut)]
    pub admin : Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx:Context<InitializeGovernance>)-> Result<()>{
    let cfg = &mut ctx.accounts.config;
    cfg.admin = *ctx.account.admin.key;
    cfg.proposal_count = 0;
    
    Ok(())
}

