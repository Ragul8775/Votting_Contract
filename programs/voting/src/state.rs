use anchor_lang::prelude::*;
use create::constants::*;

#[accounts]
pub struct GovernaneConfig {
    pub admin : Pubkey,
    pub proposal_count: u64,
}


impl GovernanceConfig {
    pub const LEN:usize = 8+32+8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ProposalStatus{
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
    pub status:ProposalStatus,
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