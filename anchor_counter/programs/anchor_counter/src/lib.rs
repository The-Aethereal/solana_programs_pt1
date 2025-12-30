use anchor_lang::prelude::*;

declare_id!("Adf9VCdjngGn8f6RZftLNFqc6zVUKs3JF5rvqXzZLwaQ");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn increment(ctx: Context<Increment>)->Result<()>{
       return Ok(());
    }
}

#[derive(Accounts)]
pub struct Increment{
}
#[derive(Accounts)]
pub struct Initialize {}
