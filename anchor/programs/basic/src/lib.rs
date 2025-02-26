use anchor_lang::prelude::*;

declare_id!("6z68wfurCMYkZG51s1Et9BJEd9nJGUusjHXNt4dGbNNF");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePoll>, 
        pool_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.poll_id = pool_id;
        pool.description = description;
        pool.poll_start = poll_start;
        pool.poll_end = poll_end;
        pool.candidate_amount = 0;
        Ok(()) 
    }
}

#[derive(Accounts)]
#[instruction(pool_id: u64)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + Pool::INIT_SPACE,
        seeds = [pool_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,

    pub system_program: AccountInfo<'info>,
}

#[account]
#[derive(InitSpace)] 
pub struct Pool {
    pub poll_id: u64,
    #[max_len(200)] 
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}


