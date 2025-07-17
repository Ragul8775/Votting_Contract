use anchor_lang::prelude::*;
use crate::state::{Proposal, ProposalState};
use crate::error::VottingError;

#[derive(Accounts)]
pub struct OpenProposal<'info>{
    #[account(mut, has_one = creator)]
    pub proposal : Account<'info, Proposal>,
    pub creator: Signer<'info>,
}

pub fn handler(ctx:Context<OpenProposal>)-> Result<()>{
    let p = &mut ctx.accounts.proposal;
    require!(p.status == ProposalState::Draft, VottingError::InvalidProposalState);

    let now = Clock::get()?.unix_timestamp;

    require!(now >= p.start_ts && now <= p.end_ts, VottingError::VotingPeriodNotActive);

    let now = Clock::get()?.unix_timestamp;
    require!(now >= p.start_ts , VottingError::VotingPeriodNotActive);
    p.status = ProposalStatus::Active;

    Ok(())
}