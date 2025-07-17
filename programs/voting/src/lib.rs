use anchor_lang::prelude::*;

declare_id!("6d4FFfXdbfnRLbnCF1C4QLXvWZgvznfBet8fwHtvEGfP");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
