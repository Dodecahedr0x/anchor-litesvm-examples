use anchor_lang::prelude::*;

declare_id!("JD4YNTouRS9Fqh3u7Vhq8RrJsXEVbEXAYdRM5Eqan7b");

#[program]
pub mod logs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
