use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct GovernanceConfig {
    pub admin : Pubkey,
    pub proposal_count: u64,
}


impl GovernanceConfig {
    pub const LEN:usize = 8+32+8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ProposalState{
    Draft,
    Active,
    Closed
}
#[account]
pub struct Proposal {
    pub id : u64,
    pub creator : Pubkey,
    pub title: String,
    pub options:Vec<String>,
    pub vote_count: Vec<u64>,
    pub start_ts: i64,
    pub end_ts: i64,
    pub status:ProposalState,
}

impl Proposal {
    pub const LEN:usize = 8+8+32+4+MAX_TITLE_LENGTH+4+MAX_OPTIONS*(4+MAX_OPTION_LENGTH)+4+MAX_OPTIONS*8+8+8+1;
}

#[account]
pub struct Ballot {
    pub voter: Pubkey,
    pub choice_index : u8
}

impl Ballot {
    pub const LEN:usize = 8+32+1;
}