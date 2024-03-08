use anchor_lang::prelude::*;

declare_id!("4fY5Hsxy7FoY16uJYVi6KybEiXSQ4H3V2ShnSWWWJ9tF");

#[program]
pub mod todo_list_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
