#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("8tdPtpAakqfXDUuaz5W5EYri7u2nY4cvS8v1or2ygVsF");

#[program]
pub mod counter {
    use super::*;

    pub fn create_task(
        ctx: Context<CreateTask>,
        task_id: u64,
        description: String,
    ) -> Result<()> {
        let task = &mut ctx.accounts.task;

        task.owner = ctx.accounts.owner.key();
        task.task_id = task_id;
        task.description = description;
        task.is_completed = false;
        task.last_modified = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn update_task(
        ctx: Context<UpdateTask>,
        new_description: String,
    ) -> Result<()> {
        let task = &mut ctx.accounts.task;

        task.description = new_description;
        task.last_modified = Clock::get()?.unix_timestamp;

        Ok(())
    }
    pub fn toggle_task(
        ctx: Context<UpdateTask>,
    ) -> Result<()> {
        let task = &mut ctx.accounts.task;

        task.is_completed = !task.is_completed;
        task.last_modified = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn delete_task(
        _ctx: Context<DeleteTask>,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(task_id: u64)]
pub struct CreateTask<'info> {
    #[account(
        init,
        payer = owner,
        space = Task::INIT_SPACE,
        seeds = [
            b"task",
            owner.key().as_ref(),
            task_id.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTask<'info> {
    #[account(
        mut,
        has_one = owner,
        seeds = [
            b"task",
            owner.key().as_ref(),
            task.task_id.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub task: Account<'info, Task>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTask<'info> {
    #[account(
        mut,
        close = owner,
        has_one = owner,
        seeds = [
            b"task",
            owner.key().as_ref(),
            task.task_id.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub owner: Signer<'info>,
}


#[account]
#[derive(InitSpace)]
pub struct Task {
    pub owner: Pubkey,
    pub task_id: u64,
    #[max_len(280)]
    pub description: String,
    pub is_completed: bool,
    pub last_modified: i64,
}
