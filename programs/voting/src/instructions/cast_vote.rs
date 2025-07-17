use anchor_lang::prelude::*;
use crate::state::{Proposal, Ballot,ProposalState};
use crate::error::VottingError ;
use crate::constants::*;

#[derive(Accounts)]
pub struct CastVote<'info>{
    #[account(mut)]
    pub proposal : Account<'info,Proposal>,
    #[account(init, payer = voter, space = Ballot::LEN, seeds = [BALLOT_SEED,proposal.key().as_ref(),voter.key().as_ref()], bump)]
    pub ballot : Account<'info,Ballot>,
    #[account(mut)]
    pub voter : Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx:Context<CastVote>, choice_index:u8) ->Result<()>{
    let p = &mut ctx.accounts.proposal;
    require!(p.state == ProposalState::Active, VottingError::ProposalNotActive);

    let now = Clock::get()?.unix_timestamp;
    require!(now <= p.end_ts, VottingError::VotingPeriodEnded);
    require!((choice_index as usize) < p.options.len(), VottingError::ChoiceIndexOutOfBounds);

    let b = &mut ctx.accounts.ballot;
    b.voter = ctx.accounts.voter.key();
    b.choice_index = choice_index;

    p.vote_counts[choice_index as usize] = p.vote_counts[choice_index as usize].checked_add(1).unwrap();

    Ok(())
}