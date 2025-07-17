use achor_lang::prelude::*;
use crate::state::{Proposal, ProposalStatus};
use crate::error::VotingError;

#[derive(Accounts)]
pub struct OpenProposal<'info>{
    #[account(mut, has_one = creator)]
    pub proposal : Account<'info, Proposal>,
    pub creator: Signer<'info>,
}

pub fn handler(ctx:Context<OpenProposal>)->Result<()>{
    let p = &mut ctx.accounts.proposal;
    reequire!(p.status == ProposalStatus::Draft, VotingError::InvalidProposalStatus);

    let now = Clock::get()?.unix_timestamp;

    require!(now >= p.start_ts && now <= p.end_ts, VotingError::InvalidProposalTiming);

    let now = Clock::get()?.unix_timestamp;
    require!(now >= p.start_ts , VottingError::VotingPeriodNotActive);
    p.status = ProposalStatus::Active;

    Ok(())
}