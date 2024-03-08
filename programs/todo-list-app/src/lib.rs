use anchor_lang::prelude::*;

declare_id!("4fY5Hsxy7FoY16uJYVi6KybEiXSQ4H3V2ShnSWWWJ9tF");

#[program]
pub mod todo_list_app {
    use super::*;

    pub fn adding_task(ctx: Context<AddingTask>, text: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author; // The author account
        let clock = Clock::get().unwrap(); //getting the current timestamp
        if text.chars().count() > 400{
            return Err(ErrorCode::TextTooLong.into());
        }
        task.author=*author.key;
        task.is_done=false;
        task.created_at=clock.unix_timestamp;
        task.updated_at=clock.unix_timestamp;
        task.text=text;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddingTask<'info> {
    #[account(init, payer = author, space=Task::LEN)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Task{
    pub author: Pubkey, // The account that owns the task
    pub is_done:bool, // whether the task is done or not
    pub text: String, // Th actual task todo
    pub created_at:i64, // The timestamp when the task was created
    pub updated_at:i64, // The timestamp when the task was last updated
}

const DISCRIMINATOR: usize=8;
const PUBLIC_KEY_LENGTH: usize=32;
const BOOL_LENGTH: usize=1;
const TEXT_LENGTH: usize=4+400*4; // 400 chars
const TIMESTAMP_LENGTH: usize=8;

impl Task{
    const LEN: usize=DISCRIMINATOR+ //discriminator
    PUBLIC_KEY_LENGTH+ //author
    BOOL_LENGTH+ //is_done
    TEXT_LENGTH+ //text
    TIMESTAMP_LENGTH+ //created_at
    TIMESTAMP_LENGTH; // updated_at
}

#[error_code]
pub enum ErrorCode{
    #[msg("The Text is too Long")]
    TextTooLong,
}
