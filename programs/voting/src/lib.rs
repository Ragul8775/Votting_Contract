use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod constants;

declare_id!("6d4FFfXdbfnRLbnCF1C4QLXvWZgvznfBet8fwHtvEGfP");

#[program]
pub mod voting {

    use super::*;

    pub fn initialize_governance(ctx: Context<InitializeGovernance>) -> Result<()> {
        
        instruction::initialize_governance::handler(ctx)
    }

    pub fn create_proposal(ctx: Context<CreateProposal>, title: String, options: Vec<String>, start_ts: i64, end_ts: i64) -> Result<()> {
        
        instruction::create_proposal::handler(ctx, title, options, start_ts, end_ts)
    }
    pub fn open_proposal(ctx:Context<OpenProposal>)->Result<()>{
        instruction::open_proposal::handler(ctx)
    }
    pub fn cast_vote(ctx: Context<CastVote>, choice_index: u8) -> Result<()> {
        
        instruction::cast_vote::handler(ctx, choice_index)
    }
    

    pub fn close_proposal(ctx:Context<CloseProposal>)->Result<()>{
        instruction::close_proposal::handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
