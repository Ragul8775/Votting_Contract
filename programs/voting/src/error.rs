use anchor_lang::prelude::*;

#[error_code]
pub enum VottingError {
    #[msg("Invalid proposal state for this action.")]
    InvalidProposalState,

    #[msg("Voting period not active.")]
    VotingPeriodNotActive,

    #[msg("Voting period has ended.")]
    VotingPeriodEnded,
    
    #[msg("Proposal not Ended.")]
    ProposalNotEnded,

    #[msg("Choice index out of bounds.")]
    ChoiceIndexOutOfBounds,

    #[msg("Vote has already voted.")]
    AlreadyVoted,

    #[msg("Invalid number of options")]
    InvalidOptionsLength,

    #[msg("Timestamps are invalid.")]
    InvalidTimeStamps,

    #[msg("Proposal not active.")]
    ProposalNotActive,
}