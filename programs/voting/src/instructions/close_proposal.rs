use anchor_lang::prelude::*;
use crate::state::{Proposal, ProposalState};
use crate::error::VottingError;

#[derive(Accounts)]
pub struct CloseProposal<'info> {
    #[account(mut,has_one = creator)]
    pub config : Account<'info, Proposal>,
    pub creator: Signer<'info>,
}

pub fn handle(ctx:Context<CloseProposal>)->Result<()>{
    let p = &mut ctx.accounts.config;
    require!(p.status == ProposalState::Active, VottingError::ProposalNotActive);
    
    let now = Clock::get()?.unix_timestamp;
    require!(now >= p.end_ts, VottingError::ProposalNotEnded);

    p.status = ProposalStatus::Closed;
    Ok(())
}