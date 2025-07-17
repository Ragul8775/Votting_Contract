use anchor_lang::prelude::*;
use crate::state::{GovernanceConfig,Proposal,ProposalStatus};
use create::error::VottingError;
use create::constants::*;

#[derive(Accounts)]
#[instruction(title:String,options:Vec<String>)]
pub struct CreateProposal<'info>{
    #[account(mut, seeds=[GOVERNANCE_SEED],bump)]
    pub config : Account<'info, GovernanceConfig>,
    #[account(init, payer = admin, space = Proposal::LEN, seeds =[PROPOSAL_SEED,config.key().as_ref(), &config.proposal_count.to_le_bytes()], bump)]
    pub proposal : Account<'info, Proposal>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,

}

pub fn handler(ctx:Context<CreateProposal>,title:String,options:Vec<String>, start_ts:i64,end_ts:i64)-> Result<()>{
   let cfg = &mutctx.accounts.config;
   let now = Clock::get()?.unix_timestamp;

   require!((2..= MAX_OPTIONS).contains(&options.len()), VottingError::InvalidOptionsLength);
   require!(start_ts >= now && end_ts > start_ts, VottingError::InvalidStartTime);

    Ok(())
}