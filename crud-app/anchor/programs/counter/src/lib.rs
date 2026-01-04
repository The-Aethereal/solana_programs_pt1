#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

#[program]
pub mod counter {
    use super::*;

    pub fn create_task(ctx: Context<CreateTaskContext>) -> Result<()>{

    }

}

#[derive(Accounts)]
pub CreateTaskContext<info>{
    #[account(
        init,
        seeds = [b"task",owner.key().as_ref(),//here i want the program address so that this pda serves a link between task data account and the owner program account],
        bump,
        space = 8 + Task::INIT_SPACE,
        payer = owner,
    )]
}

#[account]
pub struct Task{
    pub owner: Pubkey;
    pub task_id: u64;
    pub description : String;
    pub is_completed: bool;
    pub last_modified: i64;
}