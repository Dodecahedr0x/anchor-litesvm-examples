use anchor_lang::prelude::*;

declare_id!("7hCML4igja81mLakZttsUuhPshhrYjZTqVSwEHWLCgTh");

#[program]
pub mod counter {
    use super::*;

    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        msg!("Incrementing counter {:?}", ctx.accounts.counter.key());

        let counter = &mut ctx.accounts.counter;
        counter.value += 1;

        Ok(())
    }
}

#[account]
pub struct Counter {
    pub value: u64,
}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 8,
        seeds = [b"counter", signer.key.as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
